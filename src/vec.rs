use core::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Vec3<T: Add + Sub> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output=T> + Sub> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, _rhs: Vec3<T>) -> Self {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<T: Sub<Output = T> + Add> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, _rhs: Vec3<T>) -> Self {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Add + Sub> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, _rhs: Vec3<T>) -> Self {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl<T: Add + Sub + Copy> Vec3<T> {
    ///Constructor that fill every dimension with the same value
    pub fn fill(value: T) -> Self {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    ///Constructor to avoid explicip declaration of x, y and z
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

#[derive(Clone, Copy)]

pub struct Vec2<T: Add + Sub> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Sub> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, _rhs: Vec2<T>) -> Self {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Add> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, _rhs: Vec2<T>) -> Self {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl<T: Add + Sub + Copy> Vec2<T> {
    pub fn fill(value: T) -> Self {
        Vec2 { x: value, y: value }
    }
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

pub fn dot<T: Add<Output=T> + Mul<Output=T> + Sub>(veca: Vec3<T>, vecb: Vec3<T>) -> T {
    let x = veca.x * vecb.x;
    let y = veca.y * vecb.y;
    let z = veca.z * vecb.z;
    let output = x+y+z;
    output
}
