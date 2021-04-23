use super::*;
use crate::packets::common::Vec3f;

packets! {
    MD5{

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
        unknown u8;
    }
    Disconnect{}
    ChangeTireCompound{
        tire_compound String;
    }
    Ping{
        unknown u32;
        unknown2 u16;
    }
    Unknown{
        unknown u8;
    }
    SectorSplit{
        unknown u8;
        unknown2 u32;
        unknown3 u8;
    }
    Vote1{
        unknown u8;
    }
    Vote2{
        unknown u8;
    }

    Vote3{
        unknown u8;
        unknown2 u8;
    }
    CarUpdate{
        unknown u8;
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
        performance_delta i16;
        gas u8;
        normalized_pos f32;



    }
    SessionRequest{
        unknown u8;
    }
    Checksum{
        checksums BytePrefixedVec<MD5>;
    }

    DamageUpdate {
        damage f32; //engine?
        damage1 f32; //gear
        damage2 f32; //f sus
        damage3 f32; //steering
        damage4 f32; //r sus
        damage5 f32; //chasis
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

packets! {
    AdminCommandPlugin{
        cmd WideString;
    }
    NextSessionPlugin{}
    RestartSessionPlugin{}
    SessionInfoPlugin{
        session_infos BytePrefixedVec<SessionInfoU>;
    }
    KickPlugin{
        session_id u8;
    }
}

packet_enum!(UdpPlugin {
    0xd1 = AdminCommandPlugin,
    0xcf = NextSessionPlugin,
    0xd0 = RestartSessionPlugin,
    0xcd = SessionInfoPlugin,
    0xce = KickPlugin,

});
packet_enum!(TestClient {
    0xe = Unknown,
    0xf9 = Ping,
    0x3d = JoinRequest,
    0x3f = CarlistRequest,
    0x43 = Disconnect,
    0x4f = SessionRequest,
    0x44 = Checksum,
    0x46 = CarUpdate,
    0x50 = ChangeTireCompound,
    0x56 = DamageUpdate,
    0x58 = SectorSplit,
    0x64 = Vote1,
    0x65 = Vote2,
    0x66 = Vote3,
  //  0x82 = Event,
});
