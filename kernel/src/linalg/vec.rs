use num_traits::{one, zero, One, Zero};

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
        Self::new(zero(), zero())
    }

    pub fn one() -> Self
    where
        T: One,
    {
        Self::new(one(), one())
    }

    pub fn unit_x() -> Self
    where
        T: Zero + One,
    {
        Self::new(one(), zero())
    }

    pub fn unit_y() -> Self
    where
        T: Zero + One,
    {
        Self::new(zero(), one())
    }
}
