use crate::{
    car::{self, Cars, Driver},
    config::Config,
    option::ServerOptions,
    readwrite::{Reader, Writer},
    server::NewPlayer,
    udpserver::UdpMessage,
};
use anyhow::{bail, Context};
use flume::{Receiver, Sender};
use futures_lite::FutureExt;
use protocol::{
    io::{Readable, Writeable},
    packets::{
        client::{CarlistRequest, JoinRequest, TestClient},
        common::PROTOCOL_VERSION,
        server::{
            Car, CarList, NewCarConnection, NoSlotsForCarModel, TestServer, WrongPassword,
            WrongProtocol,
        },
    },
    Codec,
};
use serde::de;
use std::{
    any::Any,
    borrow::Borrow,
    f32::consts::E,
    fmt::Debug,
    io::{self, ErrorKind},
    net::{IpAddr, SocketAddr},
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};
pub struct Listener {
    listener: TcpListener,
    config: Arc<Config>,
    options: Arc<RwLock<ServerOptions>>,
    cars: Arc<Cars>,
    new_players: Sender<NewPlayer>,
    udp_messages: Sender<UdpMessage>,
}

impl Listener {
    pub async fn start(
        config: Arc<Config>,
        options: Arc<RwLock<ServerOptions>>,
        cars: Arc<Cars>,
        new_players: Sender<NewPlayer>,
        udp_messages: Sender<UdpMessage>,
    ) -> anyhow::Result<()> {
        let address = format!("{}:{}", config.server.address, config.server.tcp_port);
        let listener = TcpListener::bind(&address)
            .await
            .context("failed to bind to port - maybe a server is already running?")?;
        let listener = Listener {
            listener,
            config,
            options,
            cars,
            new_players,
            udp_messages,
        };

        tokio::spawn(async move {
            listener.run().await;
        });
        Ok(())
    }
    async fn run(mut self) {
        loop {
            if let Ok((stream, addr)) = self.listener.accept().await {
                log::debug!("Accepting Connection: {}", addr);
                self.accept(stream, addr).await;
            }
        }
    }
    async fn accept(&mut self, stream: TcpStream, addr: SocketAddr) {
        let worker = Worker::new(
            stream,
            addr,
            self.config.clone(),
            self.options.clone(),
            self.cars.clone(),
            self.new_players.clone(),
            self.udp_messages.clone(),
        );
        worker.start();
    }
}

pub struct Worker {
    reader: Reader<OwnedReadHalf>,
    writer: Writer<OwnedWriteHalf>,
    packets_to_send_tx: Sender<TestServer>,
    received_packets_rx: Receiver<TestClient>,
    config: Arc<Config>,
    cars: Arc<Cars>,
    new_players: Sender<NewPlayer>,
    ip: IpAddr,
    options: Arc<RwLock<ServerOptions>>,
    udp_messages: Sender<UdpMessage>,
}
impl Worker {
    pub fn new(
        stream: TcpStream,
        _addr: SocketAddr,
        config: Arc<Config>,
        options: Arc<RwLock<ServerOptions>>,
        cars: Arc<Cars>,
        new_players: Sender<NewPlayer>,
        udp_messages: Sender<UdpMessage>,
    ) -> Self {
        let ip = stream.peer_addr().unwrap().ip();
        let (reader, writer) = stream.into_split();

        let (received_packets_tx, received_packets_rx) = flume::bounded(32);
        let (packets_to_send_tx, packets_to_send_rx) = flume::unbounded();
        let reader = Reader::new(reader, received_packets_tx);
        let writer = Writer::new(writer, packets_to_send_rx);

        Self {
            reader,
            writer,
            packets_to_send_tx,
            received_packets_rx,
            config,
            options,
            cars,
            ip,
            new_players,
            udp_messages,
        }
    }

    pub fn start(mut self) {
        tokio::task::spawn(async move {
            self.run().await;
        });
    }

    async fn handle_joinrequest(
        &mut self,
        joiner: JoinRequest,
    ) -> anyhow::Result<(usize, bool, String)> {
        log::debug!("{} requesting {}", joiner.driver_name, joiner.car_name);
        if joiner.protocol_version != PROTOCOL_VERSION {
            log::debug!("Unexpected Protocol:{}", joiner.protocol_version);
            self.write(TestServer::WrongProtocol(WrongProtocol {
                protocol_version: PROTOCOL_VERSION,
            }))
            .await?;
            bail!("Unexpected Protocol")
        }

        let mut admin = false;
        if let Some(admin_password) = &self.config.game.admin_password {
            admin = joiner.server_password == *admin_password;
        }

        if !admin {
            if let Some(password) = &self.config.game.password {
                if joiner.server_password != *password {
                    self.write(TestServer::WrongPassword(WrongPassword {}))
                        .await?;
                    bail!("Unexpected Password")
                }
            }
        }

        let driver = Driver {
            name: joiner.driver_name,
            team: "".into(),
            nation: joiner.driver_country,
            guid: joiner.guid.clone(),
        };
        if let Ok((index, car)) = self.cars.try_add_car(joiner.car_name, driver) {
            let options = self.options.read().unwrap().clone();
            self.write(TestServer::NewCarConnection(NewCarConnection {
                server_name: self.config.server.name.clone(),
                server_port: self.config.server.tcp_port,
                tickrate: self.config.server.client_send_interval_hz,
                track: self.config.track.clone(),
                track_config: self.config.get_track_config(),
                car_model: car.model.clone(),
                car_skin: car.skin.clone(),
                sun_angle: options.sun_angle.get(),
                allowed_tyres: self.config.game.allowed_tyres,
                tyre_blankets_allowed: self.config.game.tyre_blankets_allowed,
                tc_allowed: self.config.game.tc_allowed.into(),
                abs_allowed: self.config.game.abs_allowed.into(),
                stability_allowed: self.config.game.stability_allowed,
                autoclutch_allowed: self.config.game.autoclutch_allowed,
                start_rule: self.config.game.start_rule,
                damage_multiplier: self.config.game.damage_multiplier,
                fuel_rate: self.config.game.fuel_rate,
                tyre_wear_rate: self.config.game.tyre_wear_rate,
                force_mirror: self.config.game.force_virtual_mirror,
                max_contacts_per_km: self.config.game.max_contacts_per_km,
                race_over_time: self.config.sessions.race_over_time,
                result_screen_time: self.config.sessions.result_screen_time,
                has_extra_lap: true,             //todo
                race_gas_penalty_disabled: true, //todo
                pit_window_start: 0,             //Todo
                pit_window_end: 5,               //Todo
                inverted_grid_positions: 4,      //todo
                session_id: index as u8,         //todo
                sessions: options.sessions.clone().into(),
                session_name: options.sessions.get_current_session().name.clone(),
                session_index: options.sessions.get_current() as u8,
                session_type: options.sessions.get_current_session().session_type.clone(),
                session_time: options.sessions.get_current_session().end.as_secs() as u16,
                session_laps: options.sessions.get_current_session().laps,
                grip_level: options.grip_level.grip(),
                player_position: 0,    //TODO
                session_start_time: 0, //TODO,
                checksum_files: options.checksums.keys().cloned().collect(),
                legal_tyres: "TODO".to_owned(),
                random_seed: 1337,
                server_time: 90,
            }))
            .await?;
            return Ok((index, admin, joiner.guid));
        } else {
            self.write(TestServer::NoSlotsForCarModel(NoSlotsForCarModel {}))
                .await?;
            log::debug!("No Slots");
            bail!("NoSlots");
        }
    }

    async fn run(mut self) -> anyhow::Result<()> {
        match self.read::<TestClient>().await? {
            TestClient::JoinRequest(joiner) => {
                log::debug!("Sending JoinRequest");

                if let Ok((id, admin, guid)) = self.handle_joinrequest(joiner).await {
                    let new_player = NewPlayer {
                        received_packets: self.received_packets(),
                        packets_to_send: self.packets_to_send(),
                        upd_messages: self.udp_messages(),
                        car_id: id,
                        ip: self.ip,
                        booked_as_admin: admin,
                        guid,
                    };
                    let _ = self.new_players.send_async(new_player).await;
                    self.split(id);
                }
            }
            _ => bail!("Unexpected packet"),
        }
        Ok(())
    }

    pub async fn read<P: Readable>(&mut self) -> anyhow::Result<P> {
        self.reader.read().await
    }

    pub async fn write(&mut self, packet: impl Writeable + Debug) -> anyhow::Result<()> {
        self.writer.write(packet).await
    }

    pub fn split(self, id: usize) {
        let Self {
            reader,
            writer,
            cars,
            ..
        } = self;
        let reader = tokio::task::spawn(async move { reader.run().await });
        let writer = tokio::task::spawn(async move { writer.run().await });

        tokio::task::spawn(async move {
            let result = reader.race(writer).await.expect("task panicked");
            if let Err(e) = result {
                //let message = disconnected_message(e);
                //self.cars.remove_car(id);
                log::debug!("{} lost connection: {}", id, e);
                cars.remove_car(id);
            }
        });
    }

    pub fn packets_to_send(&self) -> Sender<TestServer> {
        self.packets_to_send_tx.clone()
    }

    pub fn received_packets(&self) -> Receiver<TestClient> {
        self.received_packets_rx.clone()
    }

    pub fn udp_messages(&self) -> Sender<UdpMessage> {
        self.udp_messages.clone()
    }
}
