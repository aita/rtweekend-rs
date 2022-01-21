use crate::Ray;
use glam::DVec3;

pub struct HitRecord {
    pub p: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub const EMPTY: HitRecord = HitRecord {
        p: DVec3::ZERO,
        normal: DVec3::ZERO,
        t: 0.0,
        front_face: false,
    };

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            let mut temp_rec = HitRecord::EMPTY;
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                *rec = temp_rec
            }
        }

        hit_anything
    }
}
