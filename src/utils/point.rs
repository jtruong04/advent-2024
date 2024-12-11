use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<T>(pub T, pub T);

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
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
