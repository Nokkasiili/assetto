use crate::config::Config;
use protocol::{
    json::Car as JsonCar,
    packets::server::{Car as PacketCar, CarList},
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct NoSlotsForCar;

#[derive(Debug, Clone)]
pub struct Driver {
    pub name: String,
    pub team: String,
    pub nation: String,
    pub guid: String,
}
#[derive(Debug, Clone)]
pub struct Car {
    pub driver: Option<Driver>,
    pub model: String,
    pub skin: String,
    pub session_id: usize,
    pub damage: f32,
    pub damage1: f32,
    pub damage2: f32,
    pub damage3: f32,
    pub damage4: f32,
}
impl From<&Cars> for Vec<PacketCar> {
    fn from(cars: &Cars) -> Vec<PacketCar> {
        cars.lock()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, car)| {
                if let Some(driver) = &car.driver {
                    PacketCar {
                        index: i as u8,
                        car_model: car.model.clone(),
                        car_skin: car.skin.clone(),
                        driver_name: driver.name.clone(),
                        driver_team: driver.team.clone(),
                        driver_nation: driver.nation.clone(),
                        is_spectator: false, //TODO
                        damage: car.damage,
                        damage1: car.damage1,
                        damage2: car.damage2,
                        damage3: car.damage3,
                        damage4: car.damage4,
                    }
                } else {
                    PacketCar {
                        index: i as u8,
                        car_model: String::default(),
                        car_skin: String::default(),
                        driver_name: String::default(),
                        driver_team: String::default(),
                        driver_nation: String::default(),
                        is_spectator: false, //TODO
                        damage: car.damage,
                        damage1: car.damage1,
                        damage2: car.damage2,
                        damage3: car.damage3,
                        damage4: car.damage4,
                    }
                }
            })
            .collect()
    }
}

pub struct Cars(Mutex<Vec<Car>>);

impl std::ops::Deref for Cars {
    type Target = Mutex<Vec<Car>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct CarId(usize);

impl From<Car> for JsonCar {
    fn from(c: Car) -> Self {
        let driver = Driver {
            name: "".into(),
            team: "".into(),
            nation: "".into(),
            guid: "".into(),
        };

        let is_connected = c.driver.is_some();
        let driver = c.driver.unwrap_or(driver);
        Self {
            model: c.model,
            skin: c.skin,
            driver_name: driver.name,
            driver_team: driver.team,
            driver_nation: driver.nation,
            is_connected,
            is_requested_guid: false,
            is_entry_list: true,
        }
    }
}
impl From<&Car> for JsonCar {
    fn from(c: &Car) -> Self {
        let driver = Driver {
            name: "".into(),
            team: "".into(),
            nation: "".into(),
            guid: "".into(),
        };

        let is_connected = c.driver.is_some();
        let driver = c.driver.as_ref().unwrap_or(&driver);
        Self {
            model: c.model.clone(),
            skin: c.skin.clone(),
            driver_name: driver.name.clone(),
            driver_team: driver.team.clone(),
            driver_nation: driver.nation.clone(),
            is_connected,
            is_requested_guid: false,
            is_entry_list: true,
        }
    }
}

impl Cars {
    pub fn new(config: Arc<Config>) -> Self {
        let cars = config
            .cars
            .iter()
            .enumerate()
            .map(|(i, c)| Car {
                driver: None,
                session_id: i,
                model: c.model.clone(),
                skin: c.skin.clone(),
                damage: 0.0,
                damage1: 0.0,
                damage2: 0.0,
                damage3: 0.0,
                damage4: 0.0,
            })
            .collect();

        Cars {
            0: Mutex::new(cars),
        }
    }

    pub fn try_add_car(&self, req: String, driver: Driver) -> Result<(usize, Car), NoSlotsForCar> {
        let mut cars = self.lock().unwrap();
        for (i, car) in cars.iter_mut().enumerate() {
            if car.model == req && car.driver.is_none() {
                log::debug!("Adding car {} for {}", car.model, driver.name);
                car.driver = Some(driver);
                return Ok((i, car.clone()));
            }
        }
        Err(NoSlotsForCar)
    }
    pub fn remove_car(&self, id: usize) {
        if let Some(car) = self.lock().unwrap().get_mut(id) {
            if let Some(driver) = &car.driver {
                log::debug!("Removing car {} from driver {}", car.model, driver.name);
            };
            car.driver = None;
        }
    }
    pub fn num_of_clients(&self) -> u16 {
        self.lock()
            .unwrap()
            .iter()
            .filter(|x| x.driver.is_some())
            .count() as u16
    }

    pub fn cars(&self) -> Vec<String> {
        self.lock()
            .unwrap()
            .iter()
            .map(|v| v.model.clone())
            .collect()
    }

    pub fn max_clients(&self) -> u16 {
        self.lock().unwrap().len() as u16
    }

    pub fn to_json(&self) -> Vec<JsonCar> {
        self.lock()
            .unwrap()
            .iter()
            .map(|v| JsonCar::from(v))
            .collect()
    }

    pub fn to_packet(&self, from_session_id: u8) -> CarList {
        let driver = Driver {
            name: "".into(),
            team: "".into(),
            nation: "".into(),
            guid: "".into(),
        };

        let cars: Vec<PacketCar> = self
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .skip(from_session_id as usize)
            .map(|(i, car)| {
                let driver = car.driver.as_ref().unwrap_or_else(|| &driver);

                PacketCar {
                    index: i as u8,
                    car_model: car.model.clone(),
                    car_skin: car.skin.clone(),
                    driver_name: driver.name.clone(),
                    driver_team: driver.team.clone(),
                    driver_nation: driver.nation.clone(),
                    is_spectator: false, //TODO
                    damage: car.damage,
                    damage1: car.damage1,
                    damage2: car.damage2,
                    damage3: car.damage3,
                    damage4: car.damage4,
                }
            })
            .collect();

        CarList {
            from_session_id,
            cars,
        }
    }
}
