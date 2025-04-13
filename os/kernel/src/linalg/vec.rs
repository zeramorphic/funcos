use core::ops::Mul;

use crate::num_traits::{One, Zero};

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self
    where
        T: Zero,
    {
        Self::new(Zero::ZERO, Zero::ZERO)
    }

    pub fn one() -> Self
    where
        T: One,
    {
        Self::new(One::ONE, One::ONE)
    }

    pub fn unit_x() -> Self
    where
        T: Zero + One,
    {
        Self::new(One::ONE, Zero::ZERO)
    }

    pub fn unit_y() -> Self
    where
        T: Zero + One,
    {
        Self::new(Zero::ZERO, One::ONE)
    }
}

impl<T, U> Mul<Vec2<U>> for Vec2<T>
where
    T: Mul<U>,
{
    type Output = Vec2<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vec2<U>) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
