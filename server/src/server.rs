use std::{net::IpAddr, sync::Arc, time::Instant};

use flume::{Receiver, Sender};
use hyper::client;
use protocol::{
    packets::{
        client::TestClient,
        server::{TestServer, UpdateSession},
    },
    ClientPacket, ServerPacket,
};
use rand::distributions::uniform::UniformSampler;

use crate::{
    car::{Car, Cars},
    client::Clients,
    config::Config,
    listener::Listener,
    ServerOptions,
};
use crate::{client::Client, udpserver::UdpServer};
use crate::{client::ClientId, udpserver::UdpMessage};
use std::sync::RwLock;
pub struct Server {
    pub config: Arc<Config>,
    pub options: Arc<RwLock<ServerOptions>>,
    pub clients: Clients,
    pub new_players: Receiver<NewPlayer>,
    pub udp_messages: Receiver<UdpMessage>,
    pub udp_messages_tx: Sender<UdpMessage>,

    pub last_keepalive_time: Instant,
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
    pub upd_messages: Sender<UdpMessage>,
}

impl Server {
    pub async fn bind(
        config: Arc<Config>,
        cars: Arc<Cars>,
        options: Arc<RwLock<ServerOptions>>,
    ) -> anyhow::Result<Self> {
        let (new_players_tx, new_players) = flume::bounded(4);
        let (udp_messages_tx, udp_messages) = flume::bounded(4);

        Listener::start(
            Arc::clone(&config),
            Arc::clone(&options),
            cars.clone(),
            new_players_tx,
            udp_messages_tx.clone(),
        )
        .await?;

        log::info!(
            "Server is listening tcp on {}:{}",
            config.server.address,
            config.server.tcp_port
        );
        //let mut udp_server = UdpServer::bind(cars.clone(), config.clone(), options.clone()).await?;

        UdpServer::bind(
            udp_messages_tx.clone(),
            udp_messages.clone(),
            cars.clone(),
            Arc::clone(&config),
            Arc::clone(&options),
        )
        .await?;

        Ok(Self {
            config,
            options,
            clients: Clients::new(),
            new_players,
            udp_messages,
            udp_messages_tx,
            last_keepalive_time: Instant::now(),
            cars,
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

    pub fn handle_udp_message(&mut self) {
        for message in self.udp_messages.try_iter() {
            if let Some(packet) = message.packet.as_client() {
                match packet {
                    TestClient::UpdateUpdAddress(m) => {
                        if let Some(client) = self.clients.get_from_ip(message.socket.ip()) {
                            client.set_udp(message.socket);
                        }
                    }
                    _ => log::debug!("{} sent unknown udp message", message.socket.ip()),
                }
            }
        }
    }
}
