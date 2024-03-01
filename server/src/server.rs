use std::ops::Add;
use std::time::Duration;
use std::{net::IpAddr, sync::Arc, time::Instant};

use flume::{Receiver, Sender};

use hyper::client;
use protocol::io::WideString;
use protocol::packets::client::UpdateUpdAddress;
use protocol::packets::server::{
    Bops, CarConnected, CarList, ChangeTireCompound, Chat, ClientDisconnect, DamageUpdate, Lap,
    LapCompleted, LobbyCheckMessage, MegaPacket, P2PCount, Ping, PositionUpdate,
    UpdateUpdAddress as UpdateUpdAddressS, Weather, WelcomeMessage,
};
use protocol::packets::{client::TestClient, server::TestServer};
use rand::distributions::uniform::UniformSampler;

use crate::udpserver::UdpServerMessage;
use crate::{car::Cars, client::Clients, config::Config, listener::Listener, ServerOptions};
use crate::{client::Client, udpserver::UdpServer};
use crate::{client::ClientId, udpserver::UdpClientMessage};
use std::sync::RwLock;
pub struct Server {
    pub config: Arc<Config>,
    pub options: Arc<RwLock<ServerOptions>>,
    pub clients: Clients,
    pub new_players: Receiver<NewPlayer>,

    pub udp_packets: Receiver<UdpClientMessage>,
    pub udp_packets_to_send: Sender<UdpServerMessage>,
    pub start_time: Instant,
    cars: Arc<Cars>,
}

#[derive(Debug)]
pub struct NewPlayer {
    pub car_id: usize,
    pub guid: String,
    pub ip: IpAddr,
    pub booked_as_admin: bool,

    pub received_packets: Receiver<TestClient>,
    pub packets_to_send: Sender<TestServer>,
    pub udp_packets_to_send: Sender<UdpServerMessage>,
}

impl Server {
    pub async fn bind(
        config: Arc<Config>,
        cars: Arc<Cars>,
        options: Arc<RwLock<ServerOptions>>,
        udp_packets: Receiver<UdpClientMessage>,
        udp_packets_to_send: Sender<UdpServerMessage>,
    ) -> anyhow::Result<Self> {
        let (new_players_tx, new_players) = flume::bounded(4);
        let start_time = Instant::now();
        Listener::start(
            start_time.clone(),
            Arc::clone(&config),
            Arc::clone(&options),
            cars.clone(),
            new_players_tx,
            udp_packets_to_send.clone(),
        )
        .await?;

        log::info!(
            "Server is listening tcp on {}:{}",
            config.server.address,
            config.server.tcp_port
        );

        Ok(Self {
            config,
            options,
            clients: Clients::new(),
            new_players,
            cars,
            udp_packets_to_send,
            udp_packets: udp_packets,
            start_time,
        })
    }
    pub fn remove_client(&mut self, id: ClientId) {
        let client = self.clients.remove(id);
        log::debug!("Removed client for {}", client.car_id);
    }

    fn create_client(&mut self, new_player: NewPlayer) -> ClientId {
        log::debug!("Creating client for {}", new_player.car_id);
        let client = Client::new(new_player);
        self.clients.insert(client)
    }

    pub fn accept_new_players(&mut self) -> Vec<ClientId> {
        let mut clients = Vec::new();
        for player in self.new_players.clone().try_iter() {
            let car = self.cars.lock().unwrap().get(player.car_id).unwrap();
            if let Some(old_client) = self.clients.iter().find(|x| x.guid == player.guid) {
                //old_client.disconnect("Logged in from another location!");
            }
            let id = self.create_client(player);
            clients.push(id);
        }
        clients
    }

    pub fn broadcast_with(&self, mut callback: impl FnMut(&Client)) {
        for client in self.clients.iter() {
            callback(client);
        }
    }
    pub fn broadcast_except_with(&self, client: &Client, mut callback: impl FnMut(&Client)) {
        for client in self.clients.iter().filter(|c| c.car_id != client.car_id) {
            callback(client);
        }
    }

    pub fn handle_tcp_packets(&mut self) {
        for client in self.clients.iter() {
            for packet in client.received_packets() {
                match packet {
                    TestClient::P2PCount(p) => {
                        if p.count == -1 {
                            client.send_packet(TestServer::P2PCount(P2PCount {
                                car_id: client.car_id as u8,
                                p2p_count: client.status().p2p_count,
                                active: false, // what is this shit
                            }))
                        } else {
                            client.p2p_count_dec();
                            self.broadcast_except_with(client, |c| {
                                c.send_packet(TestServer::P2PCount(P2PCount {
                                    car_id: client.car_id as u8,
                                    p2p_count: client.status().p2p_count,
                                    active: p.active,
                                }))
                            })
                        }
                    }
                    TestClient::CarlistRequest(carlist_req) => client
                        .send_packet(TestServer::CarList(self.cars.to_packet(carlist_req.index))),
                    TestClient::Disconnect(_) => self.broadcast_except_with(client, |c| {
                        c.send_packet(TestServer::ClientDisconnect(client.into()));
                        c.disconnect();
                    }),

                    TestClient::Checksum(checksum) => {
                        for i in checksum.checksums.iter() {
                            log::debug!("{} checksum {:?}", client.car_id, i);
                        }
                    }
                    TestClient::Chat(chat) => {
                        log::debug!("{}: {}", client.car_id, chat.msg);
                        self.broadcast_except_with(client, |c| {
                            c.send_packet(TestServer::Chat(Chat {
                                car_id: client.car_id as u8,
                                msg: chat.msg.clone().into(),
                            }))
                        });
                    }
                    TestClient::LapCompleted(l) => {
                        let elapsed =
                            client.status().last_lap_timestamp.elapsed().as_millis() as u32;
                        log::debug!("{} completed lap {}/{}", client.car_id, l.laptime, elapsed);

                        client.status_mut().last_lap_timestamp = Instant::now();
                        self.options.write().unwrap().grip_level.on_lap_complete();

                        let has_completed_last_lap = client.status().laps
                            >= self
                                .options
                                .read()
                                .unwrap()
                                .sessions
                                .get_current_session()
                                .laps as u32;

                        self.options.write().unwrap().laps.add_lap(Lap {
                            car_id: client.car_id as u8,
                            laptime: l.laptime,
                            lap_count: client.status().laps as u16,
                            has_completed_last_lap,
                        });

                        client.add_lap();

                        self.broadcast_except_with(client, |c| {
                            c.send_packet(TestServer::LapCompleted(LapCompleted {
                                car_id: client.car_id as u8,
                                grip_level: self.options.read().unwrap().grip_level.grip(),
                                laptime: l.laptime,
                                cuts: l.cuts,
                                laps: self.options.read().unwrap().laps.laps(),
                            }))
                        })
                    }
                    TestClient::Pulse(_) => todo!(),
                    TestClient::ChangeTireCompound(t) => {
                        log::debug!("{} changed tires to {}", client.car_id, t.tire_compound);
                        client.status_mut().current_tyre_compound = t.tire_compound.clone();
                        self.broadcast_except_with(client, |c| {
                            c.send_packet(TestServer::ChangeTireCompound(client.into()))
                        })
                    }
                    TestClient::DamageUpdate(d) => {
                        client.update_damage(d);

                        self.broadcast_except_with(client, |c| {
                            c.send_packet(TestServer::DamageUpdate(client.into()))
                        })
                    }
                    TestClient::SectorSplit(_) => todo!(),
                    TestClient::NextSessionVote(_) => todo!(),
                    TestClient::RestartSessionVote(_) => todo!(),
                    TestClient::KickVote(_) => todo!(),
                    TestClient::Event(_) => todo!(),
                    _ => {}
                }
            }
        }
    }

    pub fn handle_udp_messages(&mut self) {
        for message in self.udp_packets.try_iter() {
            //log::debug!("UDP:{:?}", message.packet);
            match message.packet {
                TestClient::CarUpdate(u) => {
                    if let Some(client) = self.clients.get_from_ip(message.addr.ip()) {
                        if !client.has_sent_first_update.get() {
                            let bops: Bops = (&self.clients).into();
                            let updates: Vec<PositionUpdate> =
                                self.clients.iter().map(Into::into).collect();
                            client.send_udp_packet(TestServer::MegaPacket(MegaPacket {
                                timestamp: self.timestamp(),
                                ping: client.status().ping as u16,
                                position_updates: updates,
                            }));

                            client.send_packet(TestServer::WelcomeMessage(WelcomeMessage {
                                unknown: 0,
                                welcome_msg: self.config.server.name.clone().into(),
                            }));

                            client.send_packet(TestServer::Weather(
                                self.options.read().unwrap().current_weather().into(),
                            ));

                            for i in self.clients.iter() {
                                if i.car_id != client.car_id {
                                    client.send_packet(TestServer::ChangeTireCompound(i.into()));
                                }
                                client.send_packet(TestServer::MandatoryPit(i.into()));
                                client.send_packet(TestServer::P2PCount(i.into()));
                            }
                            client.send_packet(TestServer::Bops(bops));
                            /*client.send_packet(TestServer::LapCompleted(LapCompleted {
                                car_id: (),
                                unknown1: (),
                                unknown2: (),
                                session_bests: (),
                                grip_level: (),
                            }));*/

                            client.has_sent_first_update.set(true);
                        }

                        client.update_car(u);
                    }
                }
                TestClient::UpdateUpdAddress(m) => {
                    if let Some(client) = self.clients.get_from_ip(message.addr.ip()) {
                        if let Some(car) = self.cars.lock().unwrap().get(client.car_id) {
                            if car.session_id != m.car_id as usize {
                                log::debug!("{} tried update wrong car", car.session_id);
                            }
                            client.set_udp(message.addr);
                            client
                                .send_udp_packet(TestServer::UpdateUpdAddress(UpdateUpdAddressS {}))
                        }
                    }
                }
                TestClient::LobbyCheckMessage(_) => {
                    let _ = self.udp_packets_to_send.try_send(UdpServerMessage {
                        addr: message.addr,
                        packet: TestServer::LobbyCheckMessage(LobbyCheckMessage {
                            http_port: self.config.server.http_port as u16,
                        }),
                    });
                }
                TestClient::Pong(p) => {
                    if let Some(client) = self.clients.get_from_ip(message.addr.ip()) {
                        let mut status = client.status_mut();
                        status.ping = self.start_time.elapsed().as_millis() as u32 - p.ping;
                        status.time_offset = status.ping / 2 + p.time_offset;
                        status.last_pong_time = Instant::now();
                    }
                }
                TestClient::SessionRequest(r) => {
                    let session_type = self
                        .options
                        .read()
                        .unwrap()
                        .sessions
                        .get_current_session()
                        .session_type
                        .clone(); // :DD
                    if r.session_type != session_type {
                        if let Some(client) = self.clients.get_from_ip(message.addr.ip()) {
                            client.send_session_update(self.options.clone());
                        }
                    }
                }
                _ => log::debug!("{} sent unknown udp message", message.addr.ip()),
            }
        }
    }

    pub fn send_pings_and_updates(&self) {
        let updates: Vec<PositionUpdate> = self.clients.iter().map(Into::into).collect();
        for client in self.clients.iter() {
            if !client.has_sent_first_update.get() {
                continue;
            }
            if Instant::now().duration_since(client.status().last_ping_time)
                > Duration::from_secs(1)
            {
                client.status_mut().last_ping_time = Instant::now();
                client.send_udp_packet(TestServer::Ping(Ping {
                    last_ping_time: self.timestamp(),
                    ping: client.status().ping as u16,
                }))
            }

            client.send_udp_packet(TestServer::MegaPacket(MegaPacket {
                timestamp: self.timestamp(),
                ping: client.status().ping as u16,
                position_updates: updates.clone(),
            }));

            if Instant::now().duration_since(client.status().last_pong_time)
                > Duration::from_secs(10)
            {
                log::debug!(
                    "disconnecting {} because didnt respond pings",
                    client.car_id
                );
                client.disconnect();
            }
        }
    }

    pub fn timestamp(&self) -> u32 {
        self.start_time.elapsed().as_millis() as u32
    }
}
