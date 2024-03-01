use std::time::Instant;

use protocol::packets::server::SunAngle as SunAnglePacket;
use protocol::packets::server::Weather as WeatherPacket;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Weather {
    pub graphics: String,
    pub ambient: Temperature,
    pub road: Temperature,
    pub wind: Wind,
}

#[derive(Debug, Clone)]
pub struct Wind {
    pub speed: i32,
    pub direction: i32,
    speed_min: i32,
    speed_max: i32,
    variation_direction: i32,
    base_direction: i32,
}
#[derive(Debug, Clone)]
pub struct SunAngle {
    pub sun_angle: f32,
    base_sun_angle: f32,
    time_of_day_mult: f32,
    start: Instant,
}

#[derive(Debug, Clone)]
pub struct Temperature {
    pub temp: f32,
    base_temp: f32,
    variation: f32,
}

impl From<&Weather> for WeatherPacket {
    fn from(weather: &Weather) -> Self {
        Self {
            ambient: weather.ambient.temp as u8,
            road: weather.road.temp as u8,
            name: weather.graphics.clone(),
            wind_speed: weather.wind.speed as i16,
            wind_direction: weather.wind.direction as i16,
        }
    }
}

impl From<SunAngle> for SunAnglePacket {
    fn from(sun_angle: SunAngle) -> Self {
        Self {
            sun_angle: sun_angle.sun_angle,
        }
    }
}

impl Weather {
    pub fn new(graphics: String, ambient: Temperature, road: Temperature, wind: Wind) -> Self {
        Self {
            graphics,
            ambient,
            road,
            wind,
        }
    }
    pub fn update(&mut self) {
        self.ambient.update();
        self.road.update();
        self.wind.update();
    }
}

impl Temperature {
    pub fn new(base_temp: f32, variation: f32) -> Self {
        Self {
            temp: base_temp,
            base_temp,
            variation,
        }
    }
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.temp = rng.gen_range(
            self.base_temp - self.variation,
            self.base_temp + self.variation,
        );
    }

    pub fn get(&self) -> f32 {
        self.temp
    }
}

impl SunAngle {
    pub fn new(base_sun_angle: f32, time_of_day_mult: f32) -> Self {
        Self {
            sun_angle: base_sun_angle,
            base_sun_angle,
            time_of_day_mult,
            start: Instant::now(),
        }
    }
    pub fn calc(&self) -> f32 {
        /*
        *
        *   main.CurrentSunAngle =
              (float32)((float)main.ServerOptions.baseSunAngle +
                       ((float)(double)CONCAT44(in_stack_ffffff94,fVar17) / 1000.0) * 0.0044 *
                       (float)main.ServerOptions.TimeOfDayMult);*/

        let from_start_as_secs = self.start.elapsed().as_secs() as f32; //TODO
        let sun_angle: f32 =
            self.base_sun_angle + from_start_as_secs * 0.044 * self.time_of_day_mult;
        sun_angle.clamp(-80.0, 80.0)
    }
    pub fn get(&self) -> f32 {
        self.sun_angle
    }
}

impl Wind {
    pub fn new(
        speed_min: i32,
        speed_max: i32,
        base_direction: i32,
        variation_direction: i32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let speed = rng.gen_range(speed_min, speed_max);
        Self {
            speed,
            direction: base_direction,
            speed_min,
            speed_max,
            base_direction,
            variation_direction,
        }
    }
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.speed = rng.gen_range(self.speed_min, self.speed_max);

        let direction = rng.gen_range(
            self.base_direction - self.variation_direction,
            self.base_direction + self.variation_direction,
        );

        self.direction = direction % 360;
    }
}
#[cfg(test)]
mod tests {

    /*    #[test]
    fn sun_angle_calc() {
        let sun_angle = SunAngle::new(-80.0, 1.0); //8:00
        let calc_sun_angle = sun_angle.calc(10.0 * 60.0 * 60.0);

        assert_eq!(calc_sun_angle, 80.0)
    }
    */
}
