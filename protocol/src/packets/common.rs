packets!(
    Vec3f{
        x f32;
        y f32;
        z f32;
    }
);

impl Default for Vec3f {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

pub const PROTOCOL_VERSION: u16 = 202;
