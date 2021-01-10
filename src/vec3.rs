use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

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

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x * (1.0 / other),
            y: self.y * (1.0 / other),
            z: self.z * (1.0 / other), 
        }
    }
}


impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z, 
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z, 
        }
    }
}

fn get_dot_product(vec3_1: Vec3, vec3_2: Vec3) -> f64 {
    return vec3_1.x * vec3_2.x + vec3_1.y * vec3_2.y + vec3_1.z * vec3_2.z;
}

fn get_cross_product(vec3_1: Vec3, vec3_2: Vec3) -> Vec3 {
    return Vec3 {
        x: vec3_1.y * vec3_2.z - vec3_1.z * vec3_2.y,
        y: vec3_1.z * vec3_2.x - vec3_1.x * vec3_2.z,
        z: vec3_1.x * vec3_2.y - vec3_1.y * vec3_2.x,
    }
}

fn get_length_squared (vec3: Vec3) -> f64 {
    return vec3.x * vec3.x + vec3.y * vec3.y + vec3.z * vec3.z;
}

fn get_length(vec3: Vec3) -> f64 {
    return get_length_squared(vec3).sqrt();
}

fn get_unit_vector(vec3: Vec3) -> Vec3 {
    let vec3_len = get_length(vec3);
    return Vec3 {
        x: vec3.x / vec3_len,
        y: vec3.y / vec3_len,
        z: vec3.z / vec3_len,
    }
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
    fn test_div() {
        assert_eq! (Vec3 {x: 1.0, y: 1.0, z: 1.0} / 2.0, Vec3 {x: 0.5, y: 0.5, z: 0.5});
    }

    #[test]
    fn test_mul_vec3() {
        assert_eq! ((Vec3 {x: 1.0, y: 2.0, z: 3.0} * Vec3 {x: 1.0, y: 1.0, z: 1.0}), Vec3 {x: 1.0, y: 2.0, z: 3.0});
    }

    #[test]
    fn test_mul_f64() {
        assert_eq! ((Vec3 {x: 1.0, y: 2.0, z: 3.0} * 3.0), Vec3 {x: 3.0, y: 6.0, z: 9.0});
    }

    #[test]
    fn test_sub() {
        assert_eq! (Vec3 {x: 2.0, y: 2.0, z: 2.0} - Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 1.0, y: 1.0, z: 1.0});
    }

    #[test]
    fn test_get_cross_product() {
        assert_eq! (get_cross_product(Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 1.0, y: 1.0, z: 1.0}), Vec3 {x: 0.0, y: 0.0, z: 0.0});
    }

    #[test]
    fn test_get_dot_product() {
        assert_eq! (get_dot_product(Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 1.0, y: 1.0, z: 1.0}), 3.0);
    }

    #[test]
    fn test_get_length_squared() {
        assert_eq! (get_length_squared(Vec3 {x: 1.0, y: 1.0, z: 1.0}), 3.0);
    }

    #[test]
    fn test_get_length() {
        assert_eq! (get_length(Vec3 {x: 1.0, y: 2.0, z: 2.0}), 3.0);
    }

    #[test]
    fn test_get_unit_vector() {
        assert_eq! (get_unit_vector(Vec3 {x: 1.0, y: 2.0, z: 2.0}), Vec3 {x: 1.0/3.0, y: 2.0/3.0, z: 2.0/3.0});
    }
}