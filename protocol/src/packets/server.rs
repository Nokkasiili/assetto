use std::mem::size_of;

use super::*;
use crate::io::BigWideString;
use crate::Readable;
use crate::{io::Writeable, packets::common::Vec3f};
use std::iter;
use std::option::Option;

def_enum! {
    OnOffFactoryOption (u8) {
        0 = Denied,
        1 = Factory,
        2 = Forced,
    }
}

impl From<u8> for OnOffFactoryOption {
    fn from(x: u8) -> Self {
        match x {
            0 => OnOffFactoryOption::Denied,
            2 => OnOffFactoryOption::Forced,
            _ => OnOffFactoryOption::Factory,
        }
    }
}

def_enum! {
    KickReason (u8) {
        0 = Kick,
        1 = KickBan,
        2 = KickBan2,
        3 = Checksum,
    }
}

def_enum! {
    SessionType (u8) {
        0 = Booking,
        1 = Practice,
        2 = Qualify,
        3 = Race,
    }
}

impl From<u8> for SessionType {
    fn from(x: u8) -> Self {
        match x {
            0 => SessionType::Booking,
            1 => SessionType::Practice,
            2 => SessionType::Qualify,
            3 => SessionType::Race,
            _ => SessionType::Practice,
        }
    }
}

packets! {
    SessionU{
        session_type SessionType;
        laps u16;
        time u16;
    }
    Setup{
        name String;
        value f32;
    }
    Bop{
        car_id u8;
        ballast f32;
        restrictor f32;
    }
    DRSZone{
        unknown f32;
        unknown2 f32;
    }
    Car{
        index u8;
        car_model String;
        car_skin String;
        driver_name String;
        driver_team String;
        driver_nation String;
        is_spectator bool;
        damage f32;
        damage1 f32;
        damage2 f32;
        damage3 f32;
        damage4 f32;
    }
    Name{
        car_id u8;
        name WideString;
    }
    Lap{
        //%d) %s BEST: %s TOTAL: %s Laps:%d SesID:%d HasFinished:%t
        car_id u8;
        laptime u32;
      //  total_time u32;
        lap_count u16;
        has_completed_last_lap bool;
    }
    RaceBest{
        //"%d) %s BEST: %s TOTAL: %s Laps:%d SesID:%d Rank:%d
        car_id u8;
        best_lap u32;
    //    total_time u32;
        lap_count u16;
    }
}

packets! {
    NewCarConnection {
        server_name  WideString;
        server_port u16;
        tickrate u8;
        track String;
        track_config String;
        car_model String;
        car_skin String;
        sun_angle f32;
        allowed_tyres i16;
        tyre_blankets_allowed bool;
        tc_allowed OnOffFactoryOption;
        abs_allowed OnOffFactoryOption;
        stability_allowed bool;
        autoclutch_allowed bool;
//START_RULE=0         ; 0 is car locked until start;   1 is teleport   ; 2 is drivethru (if race has 3 or less laps then the Teleport penalty is enabled)
        start_rule u8;
        damage_multiplier f32;
        fuel_rate f32;
        tyre_wear_rate f32;
        force_mirror bool;

        max_contacts_per_km u8;
        race_over_time u32;
        result_screen_time u32;
        has_extra_lap bool;
        race_gas_penalty_disabled bool;
        pit_window_start u16;
        pit_window_end u16;
//0 = no additional race, 1toX = only those position will be reversed for the next race, -1 = all the position will be reversed (Retired players will be on the last positions)
        inverted_grid_positions i16;

        session_id u8;
        sessions BytePrefixedVec<SessionU>;
        session_name String;
        session_index u8;
        session_type SessionType;
        session_time u16;
        session_laps u16;

        grip_level f32;
        player_position u8;
        session_start_time i64;
        checksum_files BytePrefixedVec<String>;
        legal_tyres String;

        random_seed u32;
        server_time u32;
        }
    NoSlotsForCarModel{}

    Chat{
        car_id u8;
        msg WideString;
    }

    Weather{
        ambient u8;
        road u8;
        name WideString;
        wind_speed i16;
        wind_direction i16;
    }
    Kick{
        car_id u8;
        reason u8;
    }
    LapCompleted{
        car_id u8;
        laptime u32;
        cuts u8;
        laps BytePrefixedVec<Lap>;
        grip_level f32;
    }
    MandatoryPit{
        car_id u8;
        mandatory_pit bool;
    }

    ChangeTireCompound{
        car_id u8;
        tire_compound String;
    }

    CarSetup{
        unknown u8;
        fixed bool;
        setups BytePrefixedVec<Setup>;
    }

    SunAngle{
        sun_angle f32;
    }

    Ping {
        last_ping_time u32;
        ping u16;
    }

    Bops{
        cars BytePrefixedVec<Bop>;
    }

    SessionClosed{}

    Unknown{
        unknown WideString;
    }

    WrongPassword{
    }
    WrongProtocol{
        protocol_version u16;
    }
    Banned{
    }
    ClientDisconnect{
        car_id u8;
    }
    RaceStart{
        unknown i16; //timestatus
        unknown2 u16; // time
        unknown3 u32; // time
        ping u16;
    }
    DamageUpdate {
        car_id u8;
        damage f32; //engine?
        damage1 f32; //gear
        damage2 f32; //f sus
        damage3 f32; //steering
        damage4 f32; //r sus
        //damage5 f32; //chasis
    }
    DRSZones{
        zones BytePrefixedVec<DRSZone>;
    }

    SectorSplit{
        car_id u8;
        unknown2 u8;
        unknown3 u32;
        unknown4 u8;
    }

    CarList{
        from_session_id u8;
        cars BytePrefixedVec<Car>;
    }
    EndSessionPlugin{

    }
    Names{
        driver_names  BytePrefixedVec<Name>;
    }
    P2PCount{
        car_id u8;
        p2p_count i16;
        active bool;
    }
    WelcomeMessage{
        unknown u8;//always zero
        welcome_msg BigWideString;
    }

    CarConnected{
        car_id u8;
        name String;
        nation String;
    }
    SessionTimeLeft{
        session_time_left u32;
    }
    PingCache{
        unknown u8;
        ping_cache u8;
    }
    NextSessionVote{
        useless u8;
        unknown u8;
        unknown1 u8;
        dead_line u32;
        last_voter u8;
        last_vote bool;
    }
    KickVote{
        car_id u8;
        unknown u8;
        unknown1 u8;
        dead_line u32;
        last_voter u8;
        last_vote bool;
    }
    RestartSessionVote{
        useless u8;
        unknown u8;
        unknown1 u8;
        dead_line u32;
        last_voter u8;
        last_vote bool;
    }
    Unknown3{
        unknown WideString;
    }
    ClientFirstUpdateUdp{
        car_id u8;
    }
    LobbyCheckMessage{
        http_port u16;
    }
    UpdateUpdAddress{}

    Unknown4{
        unknown u64;
        unknown2 u32;
    }
    PingCache2{
        useless u8;
        ping_cache u8;
    }
    UdpError{
        error WideString;
    }
}
#[derive(Debug, Clone)]
pub struct RaceOver {
    pub lap_data: Vec<RaceBest>,
    pub unknown: bool, // true if session == race, race => over . resets stats?
}

impl Writeable for RaceOver {
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        self.lap_data.iter().try_for_each(|x| x.write(buffer))?;
        self.unknown.write(buffer)?;
        Ok(())
    }
}

impl Readable for RaceOver {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self, anyhow::Error> {
        let mut lap_data: Vec<RaceBest> = Vec::new();
        let len = buffer.get_ref().len() - size_of::<bool>(); //this is wrong

        if 0 < len {
            let length = len / size_of::<RaceBest>() + 1;
            lap_data = iter::repeat_with(|| RaceBest::read(buffer))
                .take(length)
                .collect::<anyhow::Result<Vec<RaceBest>>>()?;
        }
        let unknown = bool::read(buffer)?;

        Ok(Self { lap_data, unknown })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateSession {
    pub session_name: String,
    pub session_index: u8,
    pub session_type: SessionType,
    pub session_time: u16,
    pub session_laps: u16,
    pub grip_level: f32,
    //grid_position: u8,
    pub grid_position: Vec<u8>,
    pub time: i64,
}
impl Writeable for UpdateSession {
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        self.session_name.write(buffer)?;
        self.session_index.write(buffer)?;
        self.session_type.write(buffer)?;
        self.session_time.write(buffer)?;
        self.session_laps.write(buffer)?;
        self.grip_level.write(buffer)?;
        for i in self.grid_position.iter() {
            i.write(buffer)?;
        }

        self.time.write(buffer)?;

        Ok(())
    }
}
impl Readable for UpdateSession {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self, anyhow::Error> {
        let length = buffer.get_ref().len();
        let session_name = String::read(buffer)?;
        let session_index = u8::read(buffer)?;
        let session_type = SessionType::read(buffer)?;
        let session_time = u16::read(buffer)?;
        let session_laps = u16::read(buffer)?;
        let grip_level = f32::read(buffer)?;

        let count = length - buffer.position() as usize - size_of::<i64>();

        let grid_position = iter::repeat_with(|| u8::read(buffer))
            .take(count)
            .collect::<anyhow::Result<Vec<u8>>>()?;

        let time = i64::read(buffer)?;
        Ok(Self {
            session_name,
            session_index,
            session_type,
            session_time,
            session_laps,
            grip_level,
            grid_position,
            time,
        })
    }
}

packets! {
    NewCarConnectionPlugin{
        name WideString;
        guid WideString;
        car_id u8;
        car_model String;
        car_skin String;
    }

    SendVersionPlugin{
        version u8; //4
    }
    ClientEventPlugin {
        event_type u8;
        car_id u8;

        other_car Option<u8>;//optional

        impact_speed f32;
        world_pos Vec3f;
        real_pos Vec3f;
    }

    SessionInfoPlugin{
        protocol_version u8;
        session_index u8;
        sessions_len u8;
        server_name WideString;
        track String;
        track_config String;
        name String;
        typ u8;
        time u16;
        laps u16;
        wait_time u16;
        ambient_temp u8;
        road_temp u8;
        weather_graphics String;
        elapsed_ms i32;
    }
    SessionInfoPlugin1{
        protocol_version u8;
        session_index u8;
        sessions_len u8;
        server_name WideString;
        track String;
        track_config String;
        name String;
        typ u8;
        time u16;
        laps u16;
        wait_time u16;
        ambient_temp u8;
        road_temp u8;
        weather_graphics String;
        elapsed_ms i32;
    }
    ChatPlugin{
        car_id u8;
        msg WideString;
    }
    ConnectionClosedPlugin{
        name WideString;
        guid WideString;
        car_id u8;
        car_model String;
        car_skin String;
    }
    MegaPacket{
        timestamp u32;
        ping u16;
        position_updates BytePrefixedVec<PositionUpdate>;
    }

    PositionUpdate{
        car_id u8;
        pak_sequence_id u8;
        timestamp u32;
        pos Vec3f;
        rotation Vec3f;
        velocity Vec3f;
        tyre_angular_speed u8;
        tyre_angular_speed1 u8;
        tyre_angular_speed2 u8;
        tyre_angular_speed3 u8;

        streer_angle u8;
        wheel_angle u8;
        engine_rpm u16;
        gear u8;
        status u32;
 //       performance_delta i16;
//        gas u8;
    }
}

packet_enum!(UdpPlugin{
    0x32 = SessionInfoPlugin,
    0x33 = NewCarConnectionPlugin,
    0x34 = ConnectionClosedPlugin,
    0x37 = EndSessionPlugin,
    0x38 = SendVersionPlugin,
    0x39 = ChatPlugin,
    0x3b = SessionInfoPlugin1,
    0x82 = ClientEventPlugin,
});

packet_enum!(HandShake{
    0x3e = NewCarConnection,
});

packet_enum!(TestServer {
    0x0d = P2PCount,
    0xe = MandatoryPit,
    //0x36 =
    0x3a = ClientFirstUpdateUdp, //udp
    0x3b = Banned,
    0x3c = WrongPassword,
    0x3c = UdpError,
    0x3e = NewCarConnection,
    0x40 = CarList,
    0x41 = SessionTimeLeft,
    0x42 = WrongProtocol,
    0x45 = NoSlotsForCarModel,
    0x47 = Chat,
    0x48 = MegaPacket,
    0x49 = LapCompleted,
    0x4a = UpdateSession,
    0x4b = RaceOver,
    0x4d = ClientDisconnect,
    0x4e = UpdateUpdAddress,//udp
    0x50 = ChangeTireCompound,
    0x51 = WelcomeMessage,
    0x52 = CarSetup,
    0x53 = DRSZones,
    0x54 = SunAngle,
    0x56 = DamageUpdate,
    0x57 = RaceStart,
    0x58 = SectorSplit,
    0x5a = CarConnected,
    0x5b = Names,
    0x64 = NextSessionVote,
    0x65 = RestartSessionVote,
    0x66 = KickVote,
    0x68 = Kick,
    0x6e = SessionClosed,
    0x6f = Unknown3,
    0x6f = Unknown,
    0x70 = Bops,
    0x78 = Weather,
    0x82 = PingCache2,
    0x8c = PingCache,
    0xc8 = LobbyCheckMessage, //udp
    0xf8 = Unknown4, //udp
    0xf9 = Ping,
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn update_session_test() {
        let buffer: Vec<u8> = vec![
            7, 81, 117, 97, 108, 105, 102, 121, 1, 2, 10, 0, 0, 0, 0, 0, 128, 63, 0, 1, 2, 3, 4,
            146, 205, 0, 0, 0, 0, 0, 0,
        ];

        let mut cursor = Cursor::new(&buffer[..]);
        let p = UpdateSession::read(&mut cursor).unwrap();
        println!("{:?}", p);
        assert_eq!(cursor.position() as usize, buffer.len());
    }
    #[test]
    fn lap_completed_test() {
        let buffer: Vec<u8> = vec![
            255, 0, 0, 0, 0, 0, 5, 0, 255, 201, 154, 59, 0, 0, 0, 1, 255, 201, 154, 59, 0, 0, 0, 2,
            255, 201, 154, 59, 0, 0, 0, 3, 255, 201, 154, 59, 0, 0, 0, 4, 255, 201, 154, 59, 0, 0,
            0, 0, 0, 128, 63,
        ];

        let mut cursor = Cursor::new(&buffer[..]);
        let p = LapCompleted::read(&mut cursor).unwrap();
        println!("{:?}", p);
        assert_eq!(cursor.position() as usize, buffer.len());
    }
    #[test]
    fn race_best_test() {
        //75
        let buffer: Vec<u8> = vec![
            0, 120, 140, 2, 0, 1, 0, 1, 255, 201, 154, 59, 0, 0, 2, 255, 201, 154, 59, 0, 0, 3,
            255, 201, 154, 59, 0, 0, 4, 255, 201, 154, 59, 0, 0, 1,
        ];
        let mut cursor = Cursor::new(&buffer[..]);
        let p = RaceOver::read(&mut cursor).unwrap();
        println!("{:?}", p);

        assert_eq!(cursor.position() as usize, buffer.len());
    }
    #[test]
    fn car_list_test() {
        let buffer: Vec<u8> = vec![
            0x00, 0x03, 0x00, 0x14, 0x6b, 0x73, 0x5f, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x64, 0x65,
            0x73, 0x5f, 0x31, 0x39, 0x30, 0x5f, 0x65, 0x76, 0x6f, 0x32, 0x06, 0x42, 0x6c, 0x75,
            0x65, 0x37, 0x31, 0x0b, 0x4e, 0xc3, 0xb8, 0x6b, 0x6b, 0x61, 0x73, 0x69, 0x69, 0x6c,
            0x69, 0x00, 0x03, 0x46, 0x49, 0x4e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x14, 0x6b, 0x73, 0x5f, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x64, 0x65, 0x73, 0x5f, 0x31,
            0x39, 0x30, 0x5f, 0x65, 0x76, 0x6f, 0x32, 0x06, 0x42, 0x6c, 0x75, 0x65, 0x37, 0x31,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x14, 0x6b, 0x73,
            0x5f, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x64, 0x65, 0x73, 0x5f, 0x31, 0x39, 0x30, 0x5f,
            0x65, 0x76, 0x6f, 0x32, 0x06, 0x42, 0x6c, 0x75, 0x65, 0x37, 0x31, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut cursor = Cursor::new(&buffer[..]);
        let p = CarList::read(&mut cursor).unwrap();
        let mut buffer1: Vec<u8> = Vec::new();
        p.write(&mut buffer1).unwrap();
        println!("{:?} {}", p, buffer.len());
        assert_eq!(buffer, buffer1);
    }
}
