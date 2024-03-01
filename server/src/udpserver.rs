use crate::config::Config;
use crate::option::ServerOptions;

use crate::Cars;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use flume::Receiver;
use flume::Sender;
use log::debug;
use protocol::io::Writeable;

use core::panic;
use protocol::packets::client::TestClient;
use protocol::packets::server::TestServer;
use protocol::Codec;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::net::UdpSocket;
pub struct UdpServer {
    /*     config: Arc<Config>,
    options: Arc<RwLock<ServerOptions>>,
    cars: Arc<Cars>,*/
    received_packets_tx: Sender<UdpClientMessage>,
    received_packets_rx: Receiver<UdpClientMessage>,
    packets_to_send_tx: Sender<UdpServerMessage>,
    packets_to_send_rx: Receiver<UdpServerMessage>,
    socket: UdpSocket,
}

pub struct UdpClientMessage {
    pub addr: SocketAddr,
    pub packet: TestClient,
}
pub struct UdpServerMessage {
    pub addr: SocketAddr,
    pub packet: TestServer,
}

impl UdpServer {
    pub async fn bind(config: Arc<Config>) -> Result<Self> {
        let (received_packets_tx, received_packets_rx) = flume::bounded(32);
        let (packets_to_send_tx, packets_to_send_rx) = flume::unbounded();

        let address = format!("{}:{}", config.server.address, config.server.udp_port);
        let socket = UdpSocket::bind(address)
            .await
            .context("failed to bind to udp port - maybe a server is already running?")?;

        let udpserver = UdpServer {
            received_packets_tx,
            received_packets_rx,
            packets_to_send_rx,
            packets_to_send_tx,
            socket,
        };

        log::info!(
            "Server is listening udp on {}:{}",
            config.server.address,
            config.server.udp_port
        );
        Ok(udpserver)
    }
    /*
    pub async fn run(self) {
        tokio::task::spawn(async move {
            loop {
                let _ = self.listen().await;
                let _ = self.send_udp().await;
            }
        });
    }*/

    //UDP packets dont have len before packet
    pub fn send_udp(&self) {
        for i in self.packets_to_send_rx.try_iter() {
            let mut buffer = Vec::new();
            if i.packet.write(&mut buffer).is_ok() {
                log::debug!("sent: {:?}", i.packet);
                let _ = self.socket.try_send_to(&buffer, i.addr);
            }
        }
    }

    pub fn listen(&self) {
        let mut buf = vec![0; 512];
        let mut codec = Codec::new();
        if let Ok((len, addr)) = self.socket.try_recv_from(&mut buf) {
            if let Ok(Some(packet)) = codec.decode::<TestClient>(&mut buf[..len].to_vec()) {
                log::trace!("{:?}", packet);
                let _ = self
                    .received_packets_tx
                    .try_send(UdpClientMessage { addr, packet });
            } else {
                log::error!("Failed to decode:{:?}", buf);
            }
        }
    }
    pub fn received_packets(&self) -> Receiver<UdpClientMessage> {
        self.received_packets_rx.clone()
    }

    pub fn packets_to_send(&self) -> Sender<UdpServerMessage> {
        self.packets_to_send_tx.clone()
    }
}
