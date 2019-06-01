use std::ops;

pub fn clip_number<T>(n: T, lower: T, upper: T) -> T
    where
        T: PartialOrd,
{
    if n < lower {
        return upper;
    }
    if n > upper {
        return lower;
    }
    n
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub const fn empty() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }

    pub fn clamp(&mut self) {
        self.x = clip_number(self.x, -89.0, 89.0);
        self.y = clip_number(self.y % 360.0, -180.0, 180.0);
        self.z = clip_number(self.z, -50.0, 50.0);
    }

    pub fn normalized(&self) -> Self {
        let mut to_norm = self.clone();
        to_norm.normalize();
        to_norm
    }

    pub fn normalize(&mut self) {
        Self::normalize_vector(self);
    }

    pub fn length_sqrt(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        const SQUARED: fn(f32) -> f32 = |n| n * n;

        SQUARED(self.x) + SQUARED(self.y) + SQUARED(self.z)
    }

    pub fn dot(&self, other: &Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn normalize_vector(vec: &mut Vector3D) -> f32 {
        let length = vec.length_sqrt();
        *vec = if length != 0.0 {
            Vector3D::new(vec.x / length, vec.y / length, vec.z / length)
        } else {
            Vector3D::new(0.0, 0.0, 1.0)
        };
        return length;
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        *self = Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        *self = Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl ops::MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, rhs: Vector3D) {
        *self = Vector3D::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}

impl ops::MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}
