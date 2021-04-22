use super::*;

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
    //DamageUpdate{}
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

}

packet_enum!(TestClient {
    0xe = Unknown,
    0xf9 = Ping,
    0x3d = JoinRequest,
    0x3f = CarlistRequest,
    0x43 = Disconnect,
   // 0x44 = Checksum,
    0x50 = ChangeTireCompound,
   // 0x56 = DamageUpdate,
    0x58 = SectorSplit,
    0x64 = Vote1,
    0x65 = Vote2,
    0x66 = Vote3,
  //  0x82 = Event,
});
