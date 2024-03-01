use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSON {
    #[serde(rename = "Cars")]
    pub cars: Vec<Car>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Skin")]
    pub skin: String,
    #[serde(rename = "DriverName")]
    pub driver_name: String,
    #[serde(rename = "DriverTeam")]
    pub driver_team: String,
    #[serde(rename = "DriverNation")]
    pub driver_nation: String,
    #[serde(rename = "IsConnected")]
    pub is_connected: bool,
    #[serde(rename = "IsRequestedGUID")]
    pub is_requested_guid: bool,
    #[serde(rename = "IsEntryList")]
    pub is_entry_list: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub ip: String,
    pub port: u16,  //UDP
    pub cport: u16, //HTTP
    pub name: String,
    pub clients: u16,
    pub maxclients: u16,
    pub track: String,
    pub cars: Vec<String>,
    pub timeofday: u64,
    pub session: u16,
    pub sessiontypes: Vec<u8>,
    pub durations: Vec<i64>,
    pub timeleft: u64,
    pub country: Vec<String>,
    pub pass: bool,
    pub timestamp: u64,
    pub json: Value,
    pub l: bool,
    pub pickup: bool,
    pub tport: u16, //TCP
    pub timed: bool,
    pub extra: bool,
    pub pit: bool,
    pub inverted: u8,
}
