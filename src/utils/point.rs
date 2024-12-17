use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<T>(pub T, pub T);

impl<T,U> Add<Point<U>> for Point<T>
where
    T: Add<U, Output = T>,
{
    type Output = Self;

    fn add(self, other: Point<U>) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T, U> Div<U> for Point<T>
where
    T: Div<U, Output = T>,
    U: Copy,
{
    type Output = Self;

    fn div(self, rhs: U) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Mul<U, Output = T>,
    U: Copy,
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<T, U> Rem<U> for Point<T>
where
    T: Rem<U, Output = T>,
    U: Copy,
{
    type Output = Self;

    fn rem(self, rhs: U) -> Self {
        Self(self.0 % rhs, self.1 % rhs)
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod Direction {
    use crate::utils::point::Point;

    pub const UP: Point<i64> = Point(-1, 0);
    pub const DOWN: Point<i64> = Point(1, 0);
    pub const LEFT: Point<i64> = Point(0, -1);
    pub const RIGHT: Point<i64> = Point(0, 1);
}
