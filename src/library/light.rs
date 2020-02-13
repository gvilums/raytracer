use nalgebra::Vector3;

pub struct Lighting {
    pub points: Vec<PointLight>,
    pub globals: Vec<GlobalLight>,
    pub point_inv_pot: Vector3<f64>,
    pub global_inv_pot: Vector3<f64>,
    pub point_global_ratio: f64,
}

impl Lighting {
    pub fn new(ratio: f64) -> Self {
        Lighting {
            points: Vec::new(),
            globals: Vec::new(),
            point_inv_pot: Vector3::new(1f64, 1f64, 1f64),
            global_inv_pot: Vector3::new(1f64, 1f64, 1f64),
            point_global_ratio: ratio,
        }
    }

    pub fn add_point(&mut self, pos: Vector3<f64>, color: Vector3<f64>) {
        self.points.push(PointLight { pos, color });
        for i in 0..3 {
            self.point_inv_pot[i] = 1. / (1. / self.point_inv_pot[i] + color[i]);
        }
    }

    pub fn add_global(&mut self, dir: Vector3<f64>, color: Vector3<f64>) {
        self.globals.push(GlobalLight { dir, color });
        for i in 0..3 {
            self.global_inv_pot[i] = 1. / (1. / self.global_inv_pot[i] + color[i]);
        }
    }
}


pub struct PointLight {
    pub pos: Vector3<f64>,
    pub color: Vector3<f64>,
}

impl PointLight {
    pub fn new(pos: Vector3<f64>, color: Vector3<f64>) -> Self {
        PointLight { pos, color }
    }
}

pub struct GlobalLight {
    pub dir: Vector3<f64>,
    pub color: Vector3<f64>,
}

impl GlobalLight {
    pub fn new(dir: Vector3<f64>, color: Vector3<f64>) -> Self {
        GlobalLight { dir, color }
    }
}