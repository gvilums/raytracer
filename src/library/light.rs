use nalgebra::Vector3;

pub struct Light {
    pub pos: Vector3<f64>,
    pub color: Vector3<f64>,
}

impl Light {
    pub fn new(pos: Vector3<f64>, color: Vector3<f64>) -> Self {
        Light { pos, color }
    }
}