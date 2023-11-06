use std::ops::{Add, Mul, Sub};

/// This trait reprsents a number that can be used in a matrix
pub trait Num: Sub<Output = Self> + Add<Output = Self> + Mul<Output = Self> + Clone + Copy {
    /// This function returns the multiplicative identity also know as the indentity element for its type.
    /// For numbers this is typically 1.
    /// ```
    /// let identity = f64::multiplicative_identity();
    /// assert_eq!(identity, 1.0);
    /// ```

    // also known as the identity element
    fn multiplicative_identity() -> Self;

    /// This function returns the additive identity for its type.
    /// For numbers this is typically 0.
    /// ```
    /// let identity = u8::multiplicative_identity();
    /// assert_eq!(identity, 0);
    /// ```

    fn additive_identity() -> Self;
}

// macro to implement Num for some amount of tupes
macro_rules! implement_num {
    ($m:literal, $a:literal, $($t:ty),*) => {
        $(impl Num for $t {
            fn multiplicative_identity() -> Self {
                $m
            }

            fn additive_identity() -> Self {
                $a
            }
        })*
    };
}

// ========================================================
// implementing Num for Rust number types
// floats
implement_num!(1.0, 0.0, f64, f32);
// implement_num!(1.0, 0.0);
// ints
implement_num!(1, 0, u128, i128, usize, isize, u64, i64, u32, i32, u16, i16, u8, i8);
