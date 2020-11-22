use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig : Vec3,
    pub dir : Vec3,
}

impl Ray {

    pub fn new() -> Ray {
        Ray {
            orig: Vec3::new(),
            dir: Vec3::new()
        }
    }

    pub fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t*self.dir 
    }
} 