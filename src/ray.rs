use crate::vec3::Vector3;

pub type Point3 = Vector3;
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
