use std::{
    any::Any,
    cell::{Cell, RefCell, RefMut},
    net::{IpAddr, SocketAddr},
    ops::{Add, Sub},
    option,
    sync::{atomic::AtomicI16, Arc, RwLock},
    time::Instant,
};

use crate::{option::ServerOptions, udpserver::UdpServerMessage};
use crate::{server::NewPlayer, udpserver::UdpClientMessage};
use flume::{Receiver, Sender};
use protocol::packets::{
    client::{CarUpdate, LapCompleted, TestClient},
    common::Vec3f,
    server::{
        Bop, Bops, ChangeTireCompound, ClientDisconnect, DamageUpdate, MandatoryPit, P2PCount,
        PositionUpdate, TestServer, UpdateSession,
    },
};
use slab::Slab;
use std::cell::Ref;
#[derive(Debug)]
pub struct Client {
    packets_to_send: Sender<TestServer>,
    received_packets: Receiver<TestClient>,
    udp_packets_to_send: Sender<UdpServerMessage>,
    pub car_id: usize,
    pub guid: String,
    pub ip: IpAddr,
    pub udp: Cell<Option<SocketAddr>>,
    pub has_valid_checksum: Cell<bool>,
    //pub p2p_count: Cell<i16>,
    disconnected: Cell<bool>,
    pub has_sent_first_update: Cell<bool>,
    booked_as_admin: bool,
    status: RefCell<ClientStatus>,
}
#[derive(Debug)]
pub struct ClientStatus {
    pub laps: u32,
    pub pos: Vec3f,
    pub rotation: Vec3f,
    pub velocity: Vec3f,
    pub gear: u8,
    pub pak_sequence_id: u8,
    pub time_stamp: i64,
    pub tyre_angular_speed: [u8; 4],
    pub steer_angle: u8,
    pub wheel_angle: u8,
    pub engine_rpm: u16,
    pub last_lap_timestamp: Instant,
    pub status_bytes: u32,
    pub current_tyre_compound: String,
    pub ballast_kg: f32,
    pub restrictor: f32,
    pub normalized_pos: f32,
    pub damage_zone_level: [f32; 5],
    pub performance_delta: i16,
    pub gas: u8,
    pub mandatory_pit: bool,
    pub p2p_count: i16,
    pub last_ping_time: Instant,
    pub last_pong_time: Instant,
    pub ping: u32,
    pub time_offset: u32,
}

impl Default for ClientStatus {
    fn default() -> Self {
        Self {
            laps: Default::default(),
            pos: Default::default(),
            rotation: Default::default(),
            velocity: Default::default(),
            gear: Default::default(),
            pak_sequence_id: Default::default(),
            time_stamp: Default::default(),
            tyre_angular_speed: Default::default(),
            steer_angle: Default::default(),
            wheel_angle: Default::default(),
            engine_rpm: Default::default(),
            last_lap_timestamp: Instant::now(),
            status_bytes: Default::default(),
            current_tyre_compound: Default::default(),
            ballast_kg: Default::default(),
            restrictor: Default::default(),
            normalized_pos: Default::default(),
            damage_zone_level: Default::default(),
            performance_delta: Default::default(),
            gas: Default::default(),
            mandatory_pit: Default::default(),
            p2p_count: Default::default(),
            last_ping_time: Instant::now(),
            ping: Default::default(),
            time_offset: Default::default(),
            last_pong_time: Instant::now(),
        }
    }
}

impl ClientStatus {
    fn update(&mut self, u: CarUpdate) {
        self.pos = u.pos;
        self.rotation = u.rotation;
        self.velocity = u.velocity;
        self.gear = u.gear;
        self.pak_sequence_id = u.pak_sequence_id;
        self.time_stamp = u.timestamp.into();
        self.tyre_angular_speed = [
            u.tyre_angular_speed,
            u.tyre_angular_speed1,
            u.tyre_angular_speed2,
            u.tyre_angular_speed3,
        ];
        self.steer_angle = u.steer_angle;
        self.wheel_angle = u.wheel_angle;
        self.engine_rpm = u.engine_rpm;
        self.status_bytes = u.status;
        self.normalized_pos = u.normalized_pos;
        self.performance_delta = u.performance_delta;
        self.gas = u.gas;
    }
}

impl From<&Client> for PositionUpdate {
    fn from(c: &Client) -> Self {
        let status = c.status();
        Self {
            car_id: c.car_id as u8,
            pak_sequence_id: status.pak_sequence_id,
            timestamp: status.time_stamp as u32,
            pos: status.pos.clone(),
            rotation: status.rotation.clone(),
            velocity: status.velocity.clone(),
            tyre_angular_speed: status.tyre_angular_speed[0],
            tyre_angular_speed1: status.tyre_angular_speed[1],
            tyre_angular_speed2: status.tyre_angular_speed[2],
            tyre_angular_speed3: status.tyre_angular_speed[3],
            streer_angle: status.steer_angle,
            wheel_angle: status.wheel_angle,
            engine_rpm: status.engine_rpm,
            gear: status.gear,
            status: status.status_bytes,
        }
    }
}

impl From<&Client> for ChangeTireCompound {
    fn from(c: &Client) -> Self {
        Self {
            car_id: c.car_id as u8,
            tire_compound: c.status().current_tyre_compound.clone(),
        }
    }
}

impl From<&Client> for DamageUpdate {
    fn from(c: &Client) -> Self {
        let status: Ref<'_, ClientStatus> = c.status();
        Self {
            car_id: c.car_id as u8,
            damage: status.damage_zone_level[0],
            damage1: status.damage_zone_level[1],
            damage2: status.damage_zone_level[2],
            damage3: status.damage_zone_level[3],
            damage4: status.damage_zone_level[4],
        }
    }
}

impl From<&Client> for ClientDisconnect {
    fn from(c: &Client) -> Self {
        Self {
            car_id: c.car_id as u8,
        }
    }
}

impl From<&Client> for MandatoryPit {
    fn from(c: &Client) -> Self {
        let status: Ref<'_, ClientStatus> = c.status();
        Self {
            car_id: c.car_id as u8,
            mandatory_pit: status.mandatory_pit,
        }
    }
}

impl From<&Client> for P2PCount {
    fn from(c: &Client) -> Self {
        let status: Ref<'_, ClientStatus> = c.status();

        Self {
            car_id: c.car_id as u8,
            p2p_count: status.p2p_count,
            active: false, //?
        }
    }
}

impl From<&Client> for Bop {
    fn from(c: &Client) -> Self {
        let status: Ref<'_, ClientStatus> = c.status();

        Self {
            car_id: c.car_id as u8,
            ballast: status.ballast_kg,
            restrictor: status.restrictor,
        }
    }
}

impl Client {
    pub fn new(player: NewPlayer) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            car_id: player.car_id,
            guid: player.guid,
            disconnected: false.into(),
            ip: player.ip,
            udp: None.into(),
            has_sent_first_update: false.into(),
            booked_as_admin: player.booked_as_admin,
            udp_packets_to_send: player.udp_packets_to_send,
            status: RefCell::new(ClientStatus::default()),
            has_valid_checksum: false.into(),
        }
    }

    pub fn udp(&self) -> Option<SocketAddr> {
        self.udp.get()
    }

    pub fn update_damage(&self, d: protocol::packets::client::DamageUpdate) {
        let mut status = self.status_mut();
        status.damage_zone_level[0] = d.damage;
        status.damage_zone_level[1] = d.damage1;
        status.damage_zone_level[2] = d.damage2;
        status.damage_zone_level[3] = d.damage3;
        status.damage_zone_level[4] = d.damage4;
    }
    pub fn add_lap(&self) {
        let mut status = self.status_mut();
        status.laps = status.laps.add(1);
    }

    pub fn send_session_update(&self, options: Arc<RwLock<ServerOptions>>) {
        let options = options.read().unwrap();
        let session = options.sessions.get_current_session();

        let packet = TestServer::UpdateSession(UpdateSession {
            session_name: session.name.clone(),
            session_index: options.sessions.get_current() as u8,
            session_type: session.session_type.clone(),
            session_time: session.end.as_secs() as u16,
            session_laps: session.laps,
            grip_level: options.grip_level.grip(),
            grid_position: vec![1],
            time: 1337,
        });
        self.send_packet(packet);
    }

    pub fn send_udp_packet(&self, packet: TestServer) {
        if let Some(addr) = self.udp() {
            let _ = self
                .udp_packets_to_send
                .try_send(UdpServerMessage { addr, packet });
        }
    }
    pub fn send_packet(&self, packet: TestServer) {
        log::debug!("Sending:{:?}", packet);
        let _ = self.packets_to_send.try_send(packet);
    }
    pub fn received_packets(&self) -> impl Iterator<Item = TestClient> + '_ {
        self.received_packets.try_iter()
    }

    pub fn set_udp(&self, udp: SocketAddr) {
        self.udp.set(Some(udp));
    }

    pub fn disconnect(&self) {
        self.disconnected.set(true);
    }
    pub fn p2p_count_dec(&self) {
        let p2p = self.status().p2p_count;
        self.status_mut().p2p_count = p2p.saturating_sub(-1);
    }

    pub fn status(&self) -> Ref<'_, ClientStatus> {
        self.status.borrow()
    }

    pub fn status_mut(&self) -> RefMut<'_, ClientStatus> {
        self.status.borrow_mut()
    }
    pub fn update_car(&self, u: CarUpdate) {
        self.status_mut().update(u);
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

impl From<&Clients> for Bops {
    fn from(c: &Clients) -> Self {
        let cars: Vec<Bop> = c.iter().map(Into::into).collect();
        Self { cars }
    }
}
