use crate::{config::Config, session::Sessions, weather::SunAngle};
use crate::{
    dynamictrack::DynamicTrack,
    weather::{Temperature, Weather, Wind},
};

use protocol::packets::server::Lap;
use rand::seq::SliceRandom;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

use md5::Digest;
//use crate::config::Weather;
use std::fs;

#[derive(Debug, Clone)]
pub struct ServerOptions {
    pub weathers: Vec<Weather>,
    pub current_weather: Weather,
    pub sun_angle: SunAngle,
    pub checksums: HashMap<String, String>,
    pub grip_level: DynamicTrack,
    pub sessions: Sessions,
    pub laps: Laps,
}

#[derive(Debug, Default, Clone)]
pub struct Laps {
    laps: Vec<Lap>,
}

/*
 struct main.Lap {
    struct string DriverName;
    struct string DriverGuid;
    uint32 CarId;
    struct string CarModel;
    uint32 Timestamp;
    uint32 LapTime;
    struct []uint32 Sectors;
    uint32 Cuts;
    float32 BallastKG;
    struct string Tyre;
    float32 Restrictor;
};
 */

impl Laps {
    pub fn add_lap(&mut self, lap: Lap) {
        self.laps.push(lap);
    }

    pub fn laps(&self) -> Vec<Lap> {
        self.laps.clone()
    }
}

pub struct Inner {}

pub type CarString = String;

impl ServerOptions {
    pub fn new(conf: Arc<Config>) -> Arc<RwLock<Self>> {
        let mut weathers: Vec<Weather> = Vec::new();
        for i in conf.weathers.iter() {
            let road = Temperature::new(i.base_road, i.variation_road);
            let ambient = Temperature::new(i.base_ambient, i.variation_ambient);
            let wind = Wind::new(
                i.wind.base_speed_min,
                i.wind.base_speed_max,
                i.wind.base_direction,
                i.wind.variation_direction,
            );
            weathers.push(Weather::new(i.graphics.clone(), ambient, road, wind));
        }
        let current_weather = weathers.first().unwrap().clone();

        Arc::new(RwLock::new(Self {
            weathers,
            sun_angle: SunAngle::new(conf.sun_angle, conf.time_of_day_multiplier),
            checksums: ServerOptions::get_checksums(&conf.cars, conf.track.clone()),
            current_weather,
            grip_level: DynamicTrack::from(&conf.dynamictrack),
            sessions: Sessions::from(&conf.sessions.sessions),
            laps: Laps::default(),
        }))
    }
    pub fn update_weather(&mut self) {
        if let Some(weather) = self.weathers.choose(&mut rand::thread_rng()) {
            self.current_weather = weather.clone();
        }
        self.current_weather.update();
    }

    pub fn get_checksums(cars: &[crate::config::Car], track: String) -> HashMap<String, String> {
        let mut ret = HashMap::new();

        for car in cars.iter() {
            let path = format!("content/cars/{}/data.acd", car.model);
            if let Ok(content) = fs::read(path) {}
            //            let md5 = md5::compute(content);
            //let md5_string = format!("{:x}", md5);
            ret.insert(car.model.clone(), "fak".into());
        }
        /*CHECKSUM: system/data/surfaces.ini=41949b9f7045cad2af3f5eb951d170a9
        CHECKSUM: content/tracks/acu_bathurst/data/surfaces.ini=7a95627e91bf7b3eba312333392ce3f2
        CHECKSUM: content/tracks/acu_bathurst/models.ini=b42cd49a8e3ab30797f9d6857ad4d2e6*/
        ret
    }

    pub fn current_weather(&self) -> &Weather {
        &self.current_weather
    }
}
