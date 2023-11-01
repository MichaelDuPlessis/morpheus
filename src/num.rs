/// This trait reprsents a number that can be used in a matrix
pub trait Num {
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

// Annoying to do code like this
// maybe a macro could be used instead

impl Num for f64 {
    fn multiplicative_identity() -> Self {
        1.0
    }

    fn additive_identity() -> Self {
        0.0
    }
}

impl Num for f32 {
    fn multiplicative_identity() -> Self {
        1.0
    }

    fn additive_identity() -> Self {
        0.0
    }
}
