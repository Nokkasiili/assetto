use anyhow::{bail, Context};
use serde::{Deserialize, Deserializer};
use std::{fs, net::Ipv4Addr, path::Path, str::FromStr};

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub base_speed_min: i32,
    pub base_speed_max: i32,
    pub base_direction: i32,
    pub variation_direction: i32,
}
#[derive(Deserialize, Debug)]
pub struct Weather {
    pub graphics: String,
    pub base_ambient: f32,
    pub base_road: f32,
    pub variation_ambient: f32,
    pub variation_road: f32,
    pub wind: Wind,
}
#[derive(Deserialize, Debug)]
struct DynamicTrack {
    pub enabled: bool,
    pub base_grip: f32,
    pub session_start_grip: f32,
    pub grip_per_lap: f32,
    pub random_grip: f32,
    pub session_transfer: f32,
    pub total_lap_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct GameOptions {
    pub legal_tyres: String,
    pub force_virtual_mirror: bool,
    pub tc_allowed: u8,
    pub abs_allowed: u8,
    pub stability_allowed: bool,
    pub autoclutch_allowed: bool,
    pub tyre_blankets_allowed: bool,
}

#[derive(Deserialize, Debug)]
pub struct ServerOptions {
    pub address: Ipv4Addr,
    pub udp_port: u16,
    pub tcp_port: u16,
    pub http_port: u16,
    pub max_clients: u16,
    pub client_send_interval_hz: u16,
}
#[derive(Debug, Deserialize)]
pub struct Log {
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: log::LevelFilter,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerOptions,
    pub game: GameOptions,
    pub weathers: Vec<Weather>,
    pub sun_angle: f32,
    pub tracks: Vec<String>,
    pub cars: Vec<String>,
    pub log: Log,
}

//racewait cannot be lower 20s
//race over lower than 30s
//result time lower 30s

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

/// Loads the config, creating a default config if needed.

fn deserialize_log_level<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<log::LevelFilter, D::Error> {
    let string: String = String::deserialize(deserializer)?;
    let level = log::LevelFilter::from_str(&string).map_err(|_| {
        serde::de::Error::custom(
            "invalid log level: valid options are trace, debug, info, warn, error",
        )
    })?;
    Ok(level)
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Config> {
        let path = Path::new(path);
        let default_config = DEFAULT_CONFIG;

        if !path.exists() {
            println!("Creating default config");
            fs::write(path, default_config)?;
        }

        let config_string = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_string).context("invalid config.toml file")?;

        for (i, w) in config.weathers.iter().enumerate() {
            if w.base_road + w.variation_road >= 75.0 {
                bail!("{}:Road Temperature cannot be over 75c", i)
            }
            if w.base_ambient + w.variation_ambient >= 45.0 {
                bail!("{}:Road Temperature cannot be over 45c", i)
            }

            if w.wind.base_speed_min > w.wind.base_speed_max {
                bail!("{}:Windspeed min cannot be bigger than max", i);
            }
            if w.wind.base_speed_min > 40 || w.wind.base_speed_max > 40 {
                bail!("{}:Windspeed cannot be bigger than 40ms/s", i);
            }
        }

        Ok(config)
    }
}
