//! We define custom versions of the `num-traits` traits that have more `const` and less unnecessary `&`s.
//! The disadvantage is that we can't use the same traits on BigInts,
//! but that's not really our problem in the kernel anyway.

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

pub trait WrappingSub {
    fn wrapping_sub(self, other: Self) -> Self;
}

/// Marker trait for types where `0` is the least element.
pub trait Unsigned {}

macro_rules! impl_num_traits {
    ($t:ident) => {
        impl Zero for $t {
            const ZERO: Self = 0;
        }

        impl One for $t {
            const ONE: Self = 1;
        }

        impl WrappingSub for $t {
            fn wrapping_sub(self, other: Self) -> Self {
                $t::wrapping_sub(self, other)
            }
        }
    };
    ($t:ident $($tail:tt)*) => {
        impl_num_traits!($t);
        impl_num_traits!($($tail)*);
    };
}

impl_num_traits!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! impl_unsigned {
    ($t:ident) => {
        impl Unsigned for $t {}
    };
    ($t:ident $($tail:tt)*) => {
        impl_unsigned!($t);
        impl_unsigned!($($tail)*);
    };
}

impl_unsigned!(u8 u16 u32 u64 u128 usize);
