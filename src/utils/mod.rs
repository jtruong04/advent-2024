use std::ops::{Add, Div, Mul, Sub};


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

pub fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}