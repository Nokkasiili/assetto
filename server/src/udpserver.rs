use crate::car::Car;
use crate::client::Client;
use crate::config::Config;
use crate::option::ServerOptions;
use crate::server::Server;
use crate::Cars;
use crate::Clients;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use flume::Receiver;
use flume::Sender;
use protocol::io::Readable;
use protocol::io::Writeable;
use protocol::packets::client::TestClient;
use protocol::packets::server::TestServer;
use protocol::Codec;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::net::UdpSocket;
pub struct UdpServer {
    /*     config: Arc<Config>,
    options: Arc<RwLock<ServerOptions>>,
    cars: Arc<Cars>,*/
    udpmessage: Sender<UdpMessage>,
    udpmessage_recv: Receiver<UdpMessage>,
    socket: UdpSocket,
}
pub enum PacketEnum {
    Client(TestClient),
    Server(TestServer),
}

impl PacketEnum {
    pub fn as_client(&self) -> Option<&TestClient> {
        if let Self::Client(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_server(&self) -> Option<&TestServer> {
        if let Self::Server(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn try_into_server(self) -> Result<TestServer> {
        if let Self::Server(v) = self {
            Ok(v)
        } else {
            bail!("Enum is not Server");
        }
    }
}

pub struct UdpMessage {
    pub socket: SocketAddr,
    pub packet: PacketEnum,
}

impl UdpServer {
    pub async fn bind(
        udpmessage: Sender<UdpMessage>,
        udpmessage_recv: Receiver<UdpMessage>,
        cars: Arc<Cars>,
        config: Arc<Config>,
        options: Arc<RwLock<ServerOptions>>,
    ) -> Result<()> {
        let address = format!("{}:{}", config.server.address, config.server.udp_port);
        let socket = UdpSocket::bind(address)
            .await
            .context("failed to bind to udp port - maybe a server is already running?")?;

        let udpserver = UdpServer {
            udpmessage,
            udpmessage_recv,
            socket,
        };

        tokio::task::spawn(async move {
            udpserver.listen().await;
            udpserver.send_udp().await;
        });

        log::info!(
            "Server is listening udp on {}:{}",
            config.server.address,
            config.server.udp_port
        );
        Ok(())
    }

    pub async fn send_udp(&self) -> Result<()> {
        let mut codec = Codec::new();

        loop {
            for i in self.udpmessage_recv.try_iter() {
                let mut buffer = Vec::new();
                codec.encode(&i.packet.try_into_server()?, &mut buffer);
                self.socket.send_to(&buffer, i.socket).await?;
            }
        }
    }

    pub async fn listen(&self) -> Result<()> {
        let mut buf = [0; 512];
        let mut codec = Codec::new();
        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            codec.accept(&buf);
            if let Some(packet) = codec.next_packet::<TestClient>()? {
                self.udpmessage
                    .send_async(UdpMessage {
                        socket: addr,
                        packet: PacketEnum::Client(packet),
                    })
                    .await?;
            }
            codec.clear();
        }
    }
}
