use std::mem::size_of;

use super::*;
use crate::Readable;
use crate::{io::Writeable, packets::common::Vec3f};
use std::iter;

def_enum! {
    OnOffFactoryOption (i8) {
        0 = Denied,
        1 = Factory,
        2 = Forced,
    }
}
def_enum! {
    KickReason (i8) {
        0 = Kick,
        1 = KickBan,
        2 = KickBan2,
        3 = Checksum,
    }
}

def_enum! {
    SessionType (i8) {
        0 = Booking,
        1 = Practice,
        2 = Qualify,
        3 = Race,
    }
}

packets! {
    SessionU{
        unknown u8;
        unknown2 u16;
        time u16;
    }
    Setup{
        name String;
        value f32;
    }
    Bop{
        session_id u8;
        ballast f32;
        unknown f32;
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
        session_id u8;
        name WideString;
    }
    SessionBest{
        //%d) %s BEST: %s TOTAL: %s Laps:%d SesID:%d HasFinished:%t
        session_id u8;
        best_lap u32;
      //  total_time u32;
        lap_count u16;
        has_completed_last_lap bool;
    }
    RaceBest{
        //"%d) %s BEST: %s TOTAL: %s Laps:%d SesID:%d Rank:%d
        session_id u8;
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
        unknown2 f32;
        allowed_tyres i16;

        tyre_blankets_allowed bool;
        tc_allowed OnOffFactoryOption;
        abs_allowed OnOffFactoryOption;
        stability_allowed bool;
        autoclutch_allowed bool;
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

        time i64;
        checksum_files BytePrefixedVec<String>;
        legal_tyres String;

        random_seed u32;
        unknown4 u32;
        }
    NoSlotsForCarModel{}

    Chat{
        useless u8;
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
        session_id u8;
        reason u8;
    }
    LapCompleted{
        session_id u8;
        unknown1 u32;
        unknown2 u8;
        session_bests BytePrefixedVec<SessionBest>;
        grip_level f32;
    }
    MandatoryPit{
        session_id u8;
        mandatory_pit bool;
    }

    ChangeTireCompound{
        session_id u8;
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
        unknown u32;
        unknown1 u16;
    }

    UpdateSession {
        session_name String;
        session_index u8;
        session_type SessionType;
        session_time u16;
        session_laps u16;
        grip_level f32;
        grid_position u8;
        time i64;
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
        session_id u8;
    }
    RaceStart{
        unknown i16; //timestatus
        unknown2 u16; // time
        unknown3 u32; // time
        ping u16;
    }
    DamageUpdate {
        session_id u8;
        damage f32; //engine?
        damage1 f32; //gear
        damage2 f32; //f sus
        damage3 f32; //steering
        damage4 f32; //r sus
        damage5 f32; //chasis
    }
    DRSZones{
        zones BytePrefixedVec<DRSZone>;
    }
    SectorSplit{
        session_id u8;
        unknown2 u8;
        unknown3 u32;
        unknown4 u8;
    }
    Session {
        session_name String;
        session_index u8;
        session_type SessionType;
        session_time u16;
        session_laps u16;
        grip_level f32;
        time u64;
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
        session_id u8;
        p2p_count i16;
        unknown u8;
    }
    WelcomeMessage{
        unknown u8;//always zero
        welcome_msg WideString;// wrong
    }

    Unknown2{
        session_id u8;
        unknown String;
        unknown2 String;
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
        session_id u8;
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
        session_id u8;
    }
}
#[derive(Debug, Clone)]
pub struct RaceOver {
    lap_data: Vec<RaceBest>,
    unknown: bool, // true if session == race, race => over . resets stats?
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

        //let len = buffer.clone().into_inner().len() - size_of::<bool>();

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

packets! {
    NewCarConnectionPlugin{
        name WideString;
        guid WideString;
        session_id u8;
        car_model String;
        car_skin String;
    }

    SendVersionPlugin{
        version u8; //4
    }
    ClientEventPlugin {
        event_type u8;
        session_id u8;

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
        session_id u8;
        msg WideString;
    }
    ConnectionClosedPlugin{
        name WideString;
        guid WideString;
        session_id u8;
        car_model String;
        car_skin String;
    }
}

packet_enum!(UdpPlugin{
    0x32 = SessionInfoPlugin,
    0x3b = SessionInfoPlugin1,
    0x33 = NewCarConnectionPlugin,
    0x34 = ConnectionClosedPlugin,
    0x37 = EndSessionPlugin,
    0x38 = SendVersionPlugin,
    0x39 = ChatPlugin,
    0x82 = ClientEventPlugin,
});

packet_enum!(HandShake{
    0x3e = NewCarConnection,
});

packet_enum!(TestServer {
    0xe = MandatoryPit,
    0x0d = P2PCount,
    0xf9 = Ping,
    0x4a = UpdateSession,
    0x3e = NewCarConnection,
    0x3a = ClientFirstUpdateUdp,
    0x3b = Banned,
    0x3c = WrongPassword,
    //0x3c = UdpError WideString,
    0x4a = Session,
    0x4d = ClientDisconnect,
    0x4b = RaceOver,
    0x5a = Unknown2,
    0x5b = Names,
    0x6f = Unknown3,
    //0x36 =
    0x40 = CarList,
    0x41 = SessionTimeLeft,
    0x42 = WrongProtocol,
    0x45 = NoSlotsForCarModel,
    0x47 = Chat,
    0x49 = LapCompleted,
    0x50 = ChangeTireCompound,
    0x51 = WelcomeMessage,
    0x52 = CarSetup,
    0x53 = DRSZones,
    0x54 = SunAngle,
    0x56 = DamageUpdate,
    0x57 = RaceStart,
    0x58 = SectorSplit,
    0x64 = NextSessionVote,
    0x65 = RestartSessionVote,
    0x66 = KickVote,
    0x68 = Kick,
    0x6f = Unknown,
    0x6e = SessionClosed,
    0x70 = Bops,
    0x78 = Weather,
    0x8c = PingCache,
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
}
