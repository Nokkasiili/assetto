use crate::config::DynamicTrack as DynamicTrackConfig;

#[derive(Debug, Clone)]
pub struct DynamicTrack {
    enabled: bool,
    session_start_grip: f32,
    base_grip: f32,
    grip_per_lap: f32,
    random_grip: f32,
    laps: i32,
    gained_grip: f32,
    session_transfer: f32,
}

impl Default for DynamicTrack {
    fn default() -> Self {
        Self {
            enabled: false,
            session_start_grip: 0.8,
            base_grip: 0.8,
            grip_per_lap: 0.1,
            random_grip: 0.0,
            laps: 0,
            gained_grip: 0.0,
            session_transfer: 0.0,
        }
    }
}

impl From<&DynamicTrackConfig> for DynamicTrack {
    fn from(x: &DynamicTrackConfig) -> Self {
        Self {
            enabled: x.enabled,
            session_start_grip: x.session_start_grip,
            base_grip: x.base_grip,
            grip_per_lap: x.grip_per_lap,
            random_grip: x.random_grip,
            laps: 0,
            gained_grip: 0.0,
            session_transfer: x.session_transfer,
        }
    }
}

impl DynamicTrack {
    pub fn on_lap_complete(&mut self) {
        self.laps += 1;
    }
    pub fn on_new_session(&mut self) {
        let gained = (self.base_grip - self.grip()) * self.session_transfer;
        self.gained_grip = gained;
        self.laps = 0;
    }
    pub fn grip(&self) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        (self.base_grip + self.gained_grip + self.grip_per_lap * self.laps as f32).clamp(0.0, 1.0)
    }
}
