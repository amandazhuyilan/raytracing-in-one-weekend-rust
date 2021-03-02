use crate::math;

pub struct Ray {
    pub origin: math::Vec3,
    pub direction: math::Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> math::Vec3 {
        let ori = self.origin;
        let dir = self.direction;

        ori + dir * t
    }

}