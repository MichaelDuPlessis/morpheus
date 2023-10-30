use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// trait to reprsent numbers
pub trait Num:
    Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Sized
{
}

pub struct Array<T: Num> {
    array: Vec<T>,
}
