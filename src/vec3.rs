use std::ops::{Add, Neg, AddAssign, MulAssign, DivAssign, Sub, SubAssign, Mul, Div};


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const X: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const Y: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const Z: Vec3 = Vec3(0.0, 0.0, 1.0);
    pub const UNIT: Vec3 = Vec3(1.0, 1.0, 1.0);

    pub fn random() -> Self {
        Self(super::random_double(),
             super::random_double(),
             super::random_double())
    }

    pub fn random_in_range(min: f32, max: f32) -> Self {
        Self(super::random_double_in_range(min, max),
             super::random_double_in_range(min, max),
             super::random_double_in_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        let a = super::random_double_in_range(0.0, std::f32::consts::PI * 2.0);
        let z = super::random_double_in_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3(
            r * a.cos(),
            r * a.sin(),
            z
        )
    }

    pub fn random_in_hemisphere(&self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if self.dot(in_unit_sphere) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3(self.1 * rhs.2 - self.2 * rhs.1,
             self.2 * rhs.0 - self.0 * rhs.2,
             self.0 * rhs.1 - self.1 * rhs.0)
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(self.0, self.1, self.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        let mut v = self.clone();
        v.0 *= rhs;
        v.1 *= rhs;
        v.2 *= rhs;
        v
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut v = rhs.clone();
        v.0 *= self;
        v.1 *= self;
        v.2 *= self;
        v
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        let mut v = self;
        v /= rhs;
        v
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.clone();
        v += rhs;
        v
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self.clone();
        v -= rhs;
        v
    }
}