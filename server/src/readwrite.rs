use flume::Receiver;
use flume::Sender;
use protocol::{
    io::{Readable, Writeable},
    packets::{client::TestClient, server::TestServer},
    Codec,
};
use std::fmt::Debug;
use std::io::{self, ErrorKind};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};

trait StreamTraitRead: Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error>;
}
trait StreamTraitWrite: Send + Sync {
    async fn write_all(&mut self, buffer: &[u8]) -> anyhow::Result<()>;
}

impl StreamTraitWrite for OwnedWriteHalf {
    async fn write_all(&mut self, buffer: &[u8]) -> anyhow::Result<()> {
        tokio::io::AsyncWriteExt::write_all(self, buffer).await?;
        Ok(())
    }
}
impl StreamTraitRead for OwnedReadHalf {
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        tokio::io::AsyncReadExt::read(self, buffer).await
    }
}
/*
impl StreamTraitWrite for UdpSocket {
    async fn write_all(&mut self, buffer: &[u8]) -> anyhow::Result<()> {
        self.send(buffer).await?;
        Ok(())
    }
}
impl StreamTraitRead for UdpSocket {
    type ReadResult = (usize, SocketAddr);
    async fn read(&mut self, buffer: &mut [u8]) -> Result<(usize, SocketAddr), io::Error> {
        self.recv_from(buffer).await
    }
}*/

pub struct Reader<T>
where
    T: StreamTraitRead,
{
    stream: T,
    codec: Codec,
    buffer: [u8; 512],
    received_packets: Sender<TestClient>,
}

impl<T> Reader<T>
where
    T: StreamTraitRead,
{
    pub fn new(stream: T, received_packets: Sender<TestClient>) -> Self {
        Self {
            stream,
            codec: Codec::new(),
            buffer: [0; 512],
            received_packets,
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            log::debug!("reading packet");

            let packet = self.read().await?;

            let result = self.received_packets.send_async(packet).await;
            if result.is_err() {
                // server dropped connection
                log::debug!("drop");
                return Ok(());
            }
        }
    }

    pub async fn read<P: Readable>(&mut self) -> anyhow::Result<P> {
        // Keep reading bytes and trying to get the packet.
        loop {
            if let Some(packet) = self.codec.next_packet::<P>()? {
                return Ok(packet);
            }

            let duration = Duration::from_secs(10);
            let read_bytes = timeout(duration, self.stream.read(&mut self.buffer)).await??;
            if read_bytes == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "read 0 bytes").into());
            }

            let bytes = &self.buffer[..read_bytes];
            self.codec.accept(bytes);
        }
    }
}

pub struct Writer<T>
where
    T: StreamTraitWrite,
{
    stream: T,
    codec: Codec,
    packets_to_send: Receiver<TestServer>,
    buffer: Vec<u8>,
}

impl<T> Writer<T>
where
    T: StreamTraitWrite,
{
    pub fn new(stream: T, packets_to_send: Receiver<TestServer>) -> Self {
        Self {
            stream,
            codec: Codec::new(),
            packets_to_send,
            buffer: Vec::new(),
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(packet) = self.packets_to_send.recv_async().await {
            self.write(packet).await?;
        }
        Ok(())
    }

    pub async fn write(&mut self, packet: impl Writeable + Debug) -> anyhow::Result<()> {
        self.codec.encode(&packet, &mut self.buffer)?;
        self.stream.write_all(&self.buffer).await?;
        self.buffer.clear();
        Ok(())
    }
}
