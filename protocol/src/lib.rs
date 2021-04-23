pub mod codec;
pub mod io;
pub mod json;
pub mod packets;
pub use codec::Codec;
use io::{Readable, Writeable};
use packets::{client::*, server::*};

const PROTOCOL_VERSION: usize = 202;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtocolState {
    Handshake,
    Play,
}

pub struct ClientPacketCodec {
    state: ProtocolState,
    codec: Codec,
}

impl Default for ClientPacketCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientPacketCodec {
    pub fn new() -> Self {
        Self {
            state: ProtocolState::Handshake,
            codec: Codec::new(),
        }
    }

    pub fn set_state(&mut self, state: ProtocolState) {
        self.state = state
    }

    /// Decodes a `ClientPacket` using the provided data.
    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<Option<ClientPacket>> {
        self.codec.accept(data);
        match self.state {
            ProtocolState::Handshake => self
                .codec
                .next_packet::<TestClient>()
                .map(|opt| opt.map(ClientPacket::from)),
            ProtocolState::Play => self
                .codec
                .next_packet::<TestClient>()
                .map(|opt| opt.map(ClientPacket::from)),
        }
    }

    /// Encodes a `ClientPacket` into a buffer.
    pub fn encode(&mut self, packet: &ClientPacket, buffer: &mut Vec<u8>) {
        match packet {
            ClientPacket::TestClient(packet) => self.codec.encode(packet, buffer).unwrap(),
            //ClientPacket::Play(packet) => self.codec.encode(packet, buffer).unwrap(),
        }
    }
}

/// Similar to `ClientPacketCodec` but for server-sent packets.
pub struct ServerPacketCodec {
    state: ProtocolState,
    codec: Codec,
}

impl Default for ServerPacketCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerPacketCodec {
    pub fn new() -> Self {
        Self {
            state: ProtocolState::Handshake,
            codec: Codec::new(),
        }
    }

    pub fn set_state(&mut self, state: ProtocolState) {
        self.state = state
    }

    /// Decodes a `ServerPacket` using the provided data.
    pub fn decode(&mut self, data: &[u8]) -> anyhow::Result<Option<ServerPacket>> {
        self.codec.accept(data);
        match self.state {
            //            ProtocolState::Handshake => Err(anyhow!("server sent data during handshake state")),
            ProtocolState::Handshake => self
                .codec
                .next_packet::<TestServer>()
                .map(|opt| opt.map(ServerPacket::from)),
            ProtocolState::Play => self
                .codec
                .next_packet::<TestServer>()
                .map(|opt| opt.map(ServerPacket::from)),
        }
    }

    /// Encodes a `ServerPacket` into a buffer.
    pub fn encode(&mut self, packet: &ServerPacket, buffer: &mut Vec<u8>) {
        match packet {
            ServerPacket::TestServer(packet) => self.codec.encode(packet, buffer).unwrap(),
        }
    }
}

/// A packet sent by the client from any one of the packet stages.
#[derive(Debug, Clone)]
pub enum ClientPacket {
    TestClient(TestClient),
}

impl From<TestClient> for ClientPacket {
    fn from(packet: TestClient) -> Self {
        ClientPacket::TestClient(packet)
    }
}

/// A packet sent by the server from any one of the packet stages.
#[derive(Debug, Clone)]
pub enum ServerPacket {
    TestServer(TestServer),
}

impl From<TestServer> for ServerPacket {
    fn from(packet: TestServer) -> Self {
        ServerPacket::TestServer(packet)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
