use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other, 
        }
    }
}

fn get_length_squared (vec3: Vec3) -> f64 {
    return vec3.x * vec3.x + vec3.y * vec3.y + vec3.z * vec3.z;
}

fn get_length(vec3: Vec3) -> f64 {
    return get_length_squared(vec3).sqrt();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq! (Vec3 {x: 1.0, y: 1.0, z: 1.0} + Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 2.0, y: 2.0, z: 2.0});
    }

    #[test]
    fn test_mul() {
        assert_eq! ((Vec3 {x: 1.0, y: 2.0, z: 3.0} * 3.0), Vec3 {x: 3.0, y: 6.0, z: 9.0});
    }

    #[test]
    fn test_get_length_squared() {
        assert_eq! (get_length_squared(Vec3 {x: 1.0, y: 1.0, z: 1.0}), 3.0);
    }

    #[test]
    fn test_get_length() {
        assert_eq! (get_length(Vec3 {x: 1.0, y: 2.0, z: 2.0}), 3.0);
    }
}