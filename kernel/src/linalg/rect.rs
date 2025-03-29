use crate::num_traits::WrappingSub;

use super::vec::Vec2;

/// A rectangle in 2D space.
///
/// # Invariants
///
/// `min <= max`
#[derive(Clone, Copy)]
pub struct Rect<T> {
    min: Vec2<T>,
    max: Vec2<T>,
}

impl<T> Rect<T> {
    /// # Safety
    ///
    /// `min <= max`
    ///
    /// Violating this condition will not immediately cause problems,
    /// but further operations down the line will rely on this property being upheld.
    pub const unsafe fn new_unchecked(min: Vec2<T>, max: Vec2<T>) -> Self {
        Self { min, max }
    }

    pub const fn min(&self) -> Vec2<T>
    where
        T: Copy,
    {
        self.min
    }

    pub const fn max(&self) -> Vec2<T>
    where
        T: Copy,
    {
        self.max
    }

    /// We use `wrapping_sub` here for speed.
    /// Due to the invariant `min <= max`, we don't need to worry about overflow.
    pub fn width(&self) -> T
    where
        T: WrappingSub + Copy,
    {
        self.max.x.wrapping_sub(self.min.x)
    }

    /// We use `wrapping_sub` here for speed.
    /// Due to the invariant `min <= max`, we don't need to worry about overflow.
    pub fn height(&self) -> T
    where
        T: WrappingSub + Copy,
    {
        self.max.y.wrapping_sub(self.min.y)
    }
}
