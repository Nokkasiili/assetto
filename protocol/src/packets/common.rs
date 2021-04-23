use super::*;
packets!(
    Vec3f{
        x f32;
        y f32;
        z f32;
    }
);
#[derive(Debug)]
pub enum Optional<T> {
    None,
    Some(T),
}
