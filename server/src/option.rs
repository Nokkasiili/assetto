use crate::weather::SunAngle;
use crate::weather::Weather;
use rand::seq::SliceRandom;
use std::collections::HashMap;

//use crate::config::Weather;
use std::fs;

pub struct ServerOptions {
    weathers: Vec<Weather>,
    current_weather: Weather,
    sun_angle: SunAngle,
    checksums: HashMap<String, String>,
    stability_allowed: bool,
    autoclutch_allowed: bool,
}

pub type CarString = String;

impl ServerOptions {
    pub fn update_weather(&mut self) {
        if let Some(weather) = self.weathers.choose(&mut rand::thread_rng()) {
            self.current_weather = weather.clone();
        }
        self.current_weather.update();
    }

    pub fn update_cheksums(
        &mut self,
        cars: &[String],
        track: String,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut ret = HashMap::new();

        for car in cars.iter() {
            let path = format!("content/cars/{}/data.acd", car);
            let content = fs::read(path)?;
            let md5 = md5::compute(content);
            let md5_string = format!("{:x}", md5);
            ret.insert(car.clone(), md5_string);
        }
        /*CHECKSUM: system/data/surfaces.ini=41949b9f7045cad2af3f5eb951d170a9
        CHECKSUM: content/tracks/acu_bathurst/data/surfaces.ini=7a95627e91bf7b3eba312333392ce3f2
        CHECKSUM: content/tracks/acu_bathurst/models.ini=b42cd49a8e3ab30797f9d6857ad4d2e6*/
        Ok(ret)
    }
}
