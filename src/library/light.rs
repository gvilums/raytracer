use nalgebra::Vector3;

pub struct Light {
    pub pos: Vector3<f64>,
    pub brightness: f64,
}

impl Light {
    pub fn new(pos: Vector3<f64>, brightness: f64) -> Self {
        Light { pos, brightness }
    }
}