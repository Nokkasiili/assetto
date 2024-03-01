use std::{
    cell::{Cell, RefCell},
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use crate::udpserver::PacketEnum;
use crate::{car::Car, config::Config, server::NewPlayer, udpserver::UdpMessage};
use flume::{Receiver, Sender};
use protocol::{
    packets::{client::TestClient, server::TestServer},
    ClientPacket, ServerPacket,
};
use slab::Slab;
#[derive(Debug)]
pub struct Client {
    packets_to_send: Sender<TestServer>,
    received_packets: Receiver<TestClient>,
    udp_packets_to_send: Sender<UdpMessage>,
    pub car_id: usize,
    pub guid: String,
    pub damage: f32,
    pub damage1: f32,
    pub damage2: f32,
    pub damage3: f32,
    pub damage4: f32,
    pub ip: IpAddr,
    pub udp: Cell<Option<SocketAddr>>,
    disconnected: Cell<bool>,
    has_sent_first_update: Cell<bool>,
    booked_as_admin: bool,
}

impl Client {
    pub fn new(player: NewPlayer) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            car_id: player.car_id,
            guid: player.guid,
            disconnected: false.into(),
            damage: f32::default(),
            damage1: f32::default(),
            damage2: f32::default(),
            damage3: f32::default(),
            damage4: f32::default(),
            ip: player.ip,
            udp: None.into(),
            has_sent_first_update: false.into(),
            booked_as_admin: player.booked_as_admin,
            udp_packets_to_send: player.upd_messages,
        }
    }

    pub fn udp(&self) -> Option<SocketAddr> {
        self.udp.get()
    }
    pub fn send_udp_packet(&self, packet: TestServer) {
        if let Some(socket) = self.udp() {
            let _ = self.udp_packets_to_send.try_send(UdpMessage {
                socket,
                packet: PacketEnum::Server(packet),
            });
        }
    }
    pub fn send_packet(&self, packet: TestServer) {
        let _ = self.packets_to_send.try_send(packet);
    }
    pub fn received_packets(&self) -> impl Iterator<Item = TestClient> + '_ {
        self.received_packets.try_iter()
    }

    pub fn set_udp(&self, udp: SocketAddr) {
        self.udp.set(Some(udp));
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(usize);

/// Stores all `Client`s.
#[derive(Default)]
pub struct Clients {
    arena: Slab<Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, client: Client) -> ClientId {
        ClientId(self.arena.insert(client))
    }
    pub fn remove(&mut self, id: ClientId) -> Client {
        self.arena.remove(id.0)
    }

    pub fn get_from_ip(&self, ip: IpAddr) -> Option<&Client> {
        self.arena
            .iter()
            .find(|(_i, client)| client.ip == ip)
            .map(|(_i, client)| client)
    }

    pub fn get(&self, id: ClientId) -> Option<&Client> {
        self.arena.get(id.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Client> + '_ {
        self.arena.iter().map(|(_i, client)| client)
    }
}
