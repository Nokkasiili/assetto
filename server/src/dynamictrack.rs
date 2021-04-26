use std::cell::Cell;

pub struct DynamicTrack {
    enabled: bool,
    session_start_grip: f32,
    base_grip: f32,
    grip_per_lap: f32,
    random_grip: f32,
    laps: Cell<i32>,
    session_transfer: f32,
}

impl Default for DynamicTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            session_start_grip: 0.8,
            base_grip: 0.8,
            grip_per_lap: 0.1,
            random_grip: 0.0, //not used :D
            laps: Cell::new(0),
            session_transfer: 0.0, // not used :D
        }
    }
}
impl DynamicTrack {
    pub fn on_lap_complete(&self) {
        self.laps.update(|x| x + 1);
        //self.laps.
    }
    pub fn on_new_sesion(&self) {
        self.laps.set(0);
    }
    pub fn grip(&self) -> f32 {
        if !self.enabled {
            1.0
        }
        (self.base_grip + self.grip_per_lap * self.laps.get() as f32).clamp(0.0, 1.0)
    }
}
