extern crate glam;

use glam::DVec3;

pub struct Ray {
    orig: DVec3,
    dir: DVec3,
}

impl Ray {
    pub fn new(orig: DVec3, dir: DVec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir,
        }
    }

    pub fn origin(&self) -> DVec3 {
        self.orig
    }

    pub fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.orig + t * self.dir
    }
}
