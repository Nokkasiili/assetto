use std::collections::HashMap;

use crate::config::Weather;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::fs;

pub struct Options {
    weathers: Vec<Weather>,
    current_weather: CurrentWeather,
    sun_angle: f32,
    checksums: HashMap<String, String>,
}

pub struct CurrentWeather {
    pub graphics: String,
    pub ambient: f32,
    pub road: f32,
    pub wind: CurrentWind,
}

pub struct CurrentWind {
    pub speed: i32,
    pub direction: i32,
}

pub type CarString = String;

impl Options {
    pub fn update_sun_angle(&self) -> f32 {
        let sun_angle = self.sun_angle;
        sun_angle.clamp(-80.0, 80.0)

        /*
        *
        *   main.CurrentSunAngle =
              (float32)((float)main.ServerOptions.baseSunAngle +
                       ((float)(double)CONCAT44(in_stack_ffffff94,fVar17) / 1000.0) * 0.0044 *
                       (float)main.ServerOptions.TimeOfDayMult);*/
    }
    fn normalize_angle(a: i32) -> i32 {
        let mut normalized = a % 360;
        if normalized < 0 {
            normalized += 360;
        }
        normalized
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

    pub fn update_weather(&mut self, weathers: &[Weather]) {
        let mut rng = rand::thread_rng();
        if let Some(weather) = weathers.choose(&mut rng) {
            let road = rng.gen_range(
                weather.base_road - weather.variation_road,
                weather.variation_road + weather.variation_road,
            );
            let ambient = rng.gen_range(
                weather.base_ambient - weather.variation_ambient,
                weather.variation_ambient + weather.variation_ambient,
            );

            let speed = rng.gen_range(weather.wind.base_speed_min, weather.wind.base_speed_max);

            let direction = rng.gen_range(
                weather.wind.base_direction - weather.wind.variation_direction,
                weather.wind.base_direction + weather.wind.variation_direction,
            );

            let direction = Options::normalize_angle(direction);

            self.current_weather = CurrentWeather {
                graphics: weather.graphics.clone(),
                ambient,
                road,
                wind: CurrentWind { speed, direction },
            }
        }
    }
}
