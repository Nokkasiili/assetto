use super::*;
//use crate::{Readable, Writeable};
/*#[derive(Debug, Clone)]
pub struct Unknown {
    unknown: u8,
    unknown2: u16,
    unknown3: u16,
}

impl Readable for Unknown {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self, anyhow::Error> {
        Ok(Self {
            unknown: u8::read(buffer)?,
            unknown2: u16::read(buffer)?,
            unknown3: u16::read(buffer)?,
        })
    }
}
impl Writeable for Unknown {
    fn write(&self, output: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        self.unknown.write(output)?;
        self.unknown2.write(output)?;
        self.unknown3.write(output)?;

        Ok(())
    }
}*/

def_enum! {
    OnOffFactoryOption (i8) {
        0 = Off,
        1 = Factory,
        2 = On,
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

packets! {
    Session{
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
        unknown3 f32;

        force_mirror bool;
        max_contacts_per_km u8;
        race_over_time u32;
        result_scren_time u32;
        has_extra_lap bool;
        race_gas_penalty_disabled bool;
        pit_window_start u16;
        pit_window_end u16;
        inverted_grid_positions i16;
        session_id u8;

        sessions BytePrefixedVec<Session>;

        session_name String;
        session_index u8;
        session_type u8;
        session_time u16;
        session_laps u16;
        grip_level f32;

        player_position u8;

        time i64;
        checksum_files BytePrefixedVec<String>;

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
    LapCompeleted{
        session_id u8;
        unknown1 u32;
        unknown2 u8;
        players_length u8;
        session_best u32;
        laps u16;
        completed bool;
        grip_level f32;
//"%d) %s BEST: %s TOTAL: %s Laps:%d SesID:%d HasFinished:%

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
        session_id u8;
        session_type u8;
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
        unknown i16; //time
        unknown2 u16; // time
        unknown3 u32; // time
        ping u16;
    }
    RaceOver{
        //missing
    }
    DamageUpdate {
        //Missing
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
}

packet_enum!(TestServer {
    0xe = MandatoryPit,
    0xf9 = Ping,
    0x4a = UpdateSession,
    0x3e = NewCarConnection,
    0x3b = Banned,
    0x3c = WrongPassword,
    0x4d = ClientDisconnect,
    0x4b = RaceOver,
    0x42 = WrongProtocol,
    0x45 = NoSlotsForCarModel,
    0x47 = Chat,
    0x49 = LapCompeleted,
    0x50 = ChangeTireCompound,
    0x52 = CarSetup,
    0x53 = DRSZones,
    0x54 = SunAngle,
    0x56 = DamageUpdate,
    0x57 = RaceStart,
    0x58 = SectorSplit,
    0x68 = Kick,
    0x6f = Unknown,
    0x6e = SessionClosed,
    0x70 = Bops,
    0x78 = Weather,
});
