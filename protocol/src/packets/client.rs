use super::*;
use crate::{
    io::{Readable, Writeable},
    packets::common::Vec3f,
    SessionType,
};

def_enum! {
    CollisionType (u8) {
        0xa = CarCollision,
        0xb = WorldCollision,
        0xc = UnknownCollision, //send 0xc
    }
}
packets! {
    AdminCommandPlugin{
        cmd WideString;
    }
    NextSessionPlugin{}
    RestartSessionPlugin{}
    SetSessionInfoPlugin{
        session_infos BytePrefixedVec<SessionInfoU>;
    }
    KickPlugin{
        car_id u8;
    }
    SetRealTimePosPlugin{
        realtime_pos u16;
    }
    RequestCarInfoPlugin{
        car_id u8;
    }
    ChatPlugin{
        car_id u8;
        msg WideString;
    }
    BroadcastPlugin{
        msg WideString;
    }
    SessionInfoPlugin{
        unknown u16;
    }
}
packets! {
    JoinRequest {
        protocol_version u16;
        guid String;
        driver_name  WideString;
        unknown u8;
        driver_country String;
        car_name  String;
        server_password  String;
    }
    CarlistRequest {
        index u8;
    }
    Disconnect{}
    ChangeTireCompound{
        tire_compound String;
    }
    Pong{
        ping u32;
        time_offset u32;
    }
    Pulse{}
    Ping{
        unknown u32;
        unknown2 u16;
    }
    MandatoryPit{
        unknown u8;
    }
    SectorSplit{
        unknown u8;
        unknown2 u32;
        unknown3 u8;
    }
    NextSessionVote{
        unknown u8;
    }
    RestartSessionVote{
        unknown u8;
    }

    KickVote{
        car_id u8;
        unknown2 u8;
    }
    CarUpdate{
       // car_id u8;
        pak_sequence_id u8;
        timestamp u32;
        pos Vec3f;
        rotation Vec3f;
        velocity Vec3f;
        tyre_angular_speed u8;
        tyre_angular_speed1 u8;
        tyre_angular_speed2 u8;
        tyre_angular_speed3 u8;

        steer_angle u8;
        wheel_angle u8;
        engine_rpm u16;
        gear u8;
        status u32;
        performance_delta i16;
        gas u8;
        normalized_pos f32;
    }
    SessionRequest{
        session_type SessionType;
    }
    /*Checksum{
        checksums BytePrefixedVec<MD5Array>;
    }*/

    DamageUpdate {
        damage f32;
        damage1 f32;
        damage2 f32;
        damage3 f32;
        damage4 f32;
    //    damage5 f32; //chasis
    }
    P2PCount {
        count i16;
        active bool;
    }

    LapCompleted{
        timestamp u32;
        laptime u32;
        //unknown u8;
        splits BytePrefixedVec<u32>;
        cuts u8;
        numlap u8;
    }
    Chat{
        unknown u8;
        msg WideString;
    }
    Event{
        events U16PrefixedVec<EventS>;
    }
    LobbyCheckMessage{

    }
    UpdateUpdAddress{
        car_id u8;
    }


}
#[derive(Debug, Clone)]
pub struct Checksum {
    pub checksums: Vec<MD5Array>,
}

impl Writeable for Checksum {
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        self.checksums.iter().try_for_each(|x| x.write(buffer))?;
        Ok(())
    }
}
impl Readable for Checksum {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self, anyhow::Error> {
        let mut checksums: Vec<MD5Array> = Vec::new();

        //let len = buffer.clone().into_inner().len();
        let len = buffer.get_ref().len(); //this is wrong
        if 0 < len {
            let length = len / 16;
            checksums = std::iter::repeat_with(|| MD5Array::read(buffer))
                .take(length)
                .collect::<anyhow::Result<Vec<MD5Array>>>()?;
        }
        Ok(Checksum { checksums })
    }
}

#[derive(Debug, Clone)]
pub struct EventS {
    //event_length: u16,
    event_type: CollisionType,
    other_car: Option<u8>,
    impact_speed: f32,
    world_pos: Vec3f,
    real_pos: Vec3f,
}
impl Writeable for EventS {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        self.event_type.write(buffer)?;
        if let Some(other_car) = self.other_car {
            other_car.write(buffer)?;
        }
        self.impact_speed.write(buffer)?;
        self.world_pos.write(buffer)?;
        self.real_pos.write(buffer)?;
        Ok(())
    }
}
impl Readable for EventS {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> anyhow::Result<Self> {
        let event_type = CollisionType::read(buffer)?;
        let other_car = {
            match event_type {
                CollisionType::CarCollision => Some(u8::read(buffer)?),
                CollisionType::WorldCollision => None,
                _ => {
                    return Ok(Self {
                        event_type,
                        other_car: None,
                        impact_speed: 0.0,
                        world_pos: Vec3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        real_pos: Vec3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    });
                }
            }
        };
        let impact_speed = f32::read(buffer)?;
        let world_pos = Vec3f::read(buffer)?;
        let real_pos = Vec3f::read(buffer)?;
        Ok(Self {
            event_type,
            other_car,
            impact_speed,
            world_pos,
            real_pos,
        })
    }
}

packets! {
    SessionInfoU{
        unknown WideString;
        unknown2 u8;
        unknown3 u32;
        unknown4 u32;
        unknown5 u32;
    }
}

packet_enum!(UdpPlugin {
    0xc8 = SetRealTimePosPlugin,
    0xc9 = RequestCarInfoPlugin,
    0xca = ChatPlugin,
    0xcb = BroadcastPlugin,
    0xcc = SessionInfoPlugin,
    0xcd = SetSessionInfoPlugin,
    0xce = KickPlugin,
    0xcf = NextSessionPlugin,
    0xd0 = RestartSessionPlugin,
    0xd1 = AdminCommandPlugin,

});

packet_enum!(HandShakeStatus{
    0x3d = JoinRequest,
});

packet_enum!(TestClient {
    0x0d = P2PCount,
    0xe = MandatoryPit,
    0x3d = JoinRequest,
    0x3f = CarlistRequest,
    0x43 = Disconnect,
    0x44 = Checksum,
    0x46 = CarUpdate,
    0x47 = Chat,
    0x49 = LapCompleted,
    0x4c = Pulse,//udp
    0x4e = UpdateUpdAddress, //udp
    0x4f = SessionRequest,//udp
    0x50 = ChangeTireCompound,
    0x56 = DamageUpdate,
    0x58 = SectorSplit,
    0x64 = NextSessionVote,
    0x65 = RestartSessionVote,
    0x66 = KickVote,
    0x82 = Event,
    0xc8 = LobbyCheckMessage,
    0xf8 = Pong, //udp
    0xf9 = Ping,
});

#[cfg(test)]
mod tests {
    use super::*;
    use md5::Digest;
    use std::io::Cursor;
    #[test]
    fn event_test() {
        //130
        let buffer: Vec<u8> = vec![
            1, 0, 11, 116, 62, 45, 66, 238, 67, 167, 195, 168, 53, 128, 62, 133, 102, 34, 194, 0,
            0, 226, 54, 198, 195, 210, 190, 178, 72, 6, 64,
        ];
        let mut cursor = Cursor::new(&buffer[..]);
        let p = Event::read(&mut cursor).unwrap();
        println!("{:?}", p);

        assert_eq!(cursor.position() as usize, buffer.len());
    }
    #[test]
    fn checksums_test() {
        //68
        let buffer: Vec<u8> = vec![
            65, 148, 155, 159, 112, 69, 202, 210, 175, 63, 94, 185, 81, 209, 112, 169, 122, 149,
            98, 126, 145, 191, 123, 62, 186, 49, 35, 51, 57, 44, 227, 242, 180, 44, 212, 154, 142,
            58, 179, 7, 151, 249, 214, 133, 122, 212, 210, 230, 214, 215, 24, 236, 202, 119, 52,
            47, 22, 232, 203, 108, 13, 77, 189, 47,
        ];
        let outputs = vec![
            "41949b9f7045cad2af3f5eb951d170a9",
            "7a95627e91bf7b3eba312333392ce3f2",
            "b42cd49a8e3ab30797f9d6857ad4d2e6",
            "d6d718ecca77342f16e8cb6c0d4dbd2f",
        ];
        let mut cursor = Cursor::new(&buffer[..]);
        let p = Checksum::read(&mut cursor).unwrap();
        p.checksums.iter().enumerate().for_each(|(i, f)| {
            assert_eq!(format!("{:x}", Digest { 0: f.0 }), outputs[i]);
        });

        assert_eq!(cursor.position() as usize, buffer.len());
    }
    #[test]
    fn damage_update_test() {
        //86
        let buffer: Vec<u8> = vec![
            187, 200, 186, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 187, 200, 186, 64,
        ];
        let mut cursor = Cursor::new(&buffer[..]);
        let p = DamageUpdate::read(&mut cursor).unwrap();
        println!("{:?}", p);

        assert_eq!(cursor.position() as usize, buffer.len());
    }
}
