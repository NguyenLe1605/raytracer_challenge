mod utils;
use std::ops::{Add, Div, Mul, Neg, Sub};

use utils::float_cmp;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Self { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        utils::float_cmp(self.w, 1.0f64)
    }

    pub fn is_vector(&self) -> bool {
        utils::float_cmp(self.w, 0.0f64)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 1.0f64)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 0.0f64)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(self, rhs: Self) -> Self {
        Tuple::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_cmp(self.x, other.x)
            && float_cmp(self.y, other.y)
            && float_cmp(self.z, other.z)
            && float_cmp(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Div<f64> for &Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

#[cfg(test)]
mod tests {
    use utils::float_cmp;

    use super::*;
    #[test]
    fn tuple_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(float_cmp(a.x, 4.3));
        assert!(float_cmp(a.y, -4.2));
        assert!(float_cmp(a.z, 3.1));
        assert!(float_cmp(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(float_cmp(a.x, 4.3));
        assert!(float_cmp(a.y, -4.2));
        assert!(float_cmp(a.z, 3.1));
        assert!(float_cmp(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn create_tuple_with_point() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn create_tuple_with_vector() {
        let p = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_two_tuples() {
        assert_eq!(
            Tuple::new(3.0, -2.0, 5.0, 1.0) + Tuple::new(-2.0, 3.0, 1.0, 0.0),
            Tuple::new(1.0, 1.0, 6.0, 1.0)
        );
    }

    #[test]
    fn subtracting_two_points() {
        let p = Tuple::point(3.0, 2.0, 1.0) - Tuple::point(5.0, 6.0, 7.0);
        assert!(p.is_vector());
        assert_eq!(p, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0);
        assert!(p.is_point());
        assert_eq!(p, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let p = Tuple::vector(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0);
        assert!(p.is_vector());
        assert_eq!(p, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substracting_a_vector_from_zero() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        assert_eq!(
            -Tuple::new(1.0, -2.0, 3.0, -4.0),
            Tuple::new(-1.0, 2.0, -3.0, 4.0)
        );
    }

    #[test]
    fn mult_a_tuple_by_a_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) * 3.5,
            Tuple::new(3.5, -7.0, 10.5, -14.0)
        );
    }

    #[test]
    fn mult_a_tuple_by_a_fraction() {
        assert_eq!(
            0.5 * Tuple::new(1.0, -2.0, 3.0, -4.0),
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        );
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        assert_eq!(
            Tuple::new(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::new(0.5, -1.0, 1.5, -2.0)
        )
    }

    #[test]
    fn magnitude_of_unit_vector_in_x() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert!(float_cmp(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_unit_vector_in_y() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert!(float_cmp(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_unit_vector_in_z() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert!(float_cmp(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_positive_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(float_cmp(v.magnitude(), f64::sqrt(14.0)));
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert!(float_cmp(v.magnitude(), f64::sqrt(14.0)));
    }

    #[test]
    fn normalizing_vectors() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));

        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(
            norm,
            Tuple::vector(
                1.0 / f64::sqrt(14.0),
                2.0 / f64::sqrt(14.0),
                3.0 / f64::sqrt(14.0)
            )
        );
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
