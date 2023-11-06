use std::convert::{From, Into};

pub trait Dimension: From<Vec<usize>> + Into<Vec<usize>> + Clone + Copy {}

#[derive(Clone, Copy)]
pub struct Dims2(pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims3(pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims4(pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims5(pub usize, pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims6(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims7(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims8(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims9(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Dims10(pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize, pub usize);


macro_rules! impl_from_tuple {
    ($t:ident, $($idx:tt),*) => {
        impl From<Vec<usize>> for $t {
            fn from(value: Vec<usize>) -> Self {
                $t( $(value[$idx]),* )
            }
        }
    };
}

macro_rules! impl_into_tuple {
    ($t:ident, $($idx:tt),*) => {
        impl Into<Vec<usize>> for $t {
            fn into(self) -> Vec<usize> {
                vec![ $(self.$idx),* ]
            }
        }
    };
}

impl_from_tuple!(Dims2, 0, 1);
impl_from_tuple!(Dims3, 0, 1, 2);
impl_from_tuple!(Dims4, 0, 1, 2, 3);
impl_from_tuple!(Dims5, 0, 1, 2, 3, 4);
impl_from_tuple!(Dims6, 0, 1, 2, 3, 4, 5);
impl_from_tuple!(Dims7, 0, 1, 2, 3, 4, 5, 6);
impl_from_tuple!(Dims8, 0, 1, 2, 3, 4, 5, 6, 7);
impl_from_tuple!(Dims9, 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_from_tuple!(Dims10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

impl_into_tuple!(Dims2, 0, 1);
impl_into_tuple!(Dims3, 0, 1, 2);
impl_into_tuple!(Dims4, 0, 1, 2, 3);
impl_into_tuple!(Dims5, 0, 1, 2, 3, 4);
impl_into_tuple!(Dims6, 0, 1, 2, 3, 4, 5);
impl_into_tuple!(Dims7, 0, 1, 2, 3, 4, 5, 6);
impl_into_tuple!(Dims8, 0, 1, 2, 3, 4, 5, 6, 7);
impl_into_tuple!(Dims9, 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_into_tuple!(Dims10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

impl Dimension for Dims2 {}
impl Dimension for Dims3 {}
impl Dimension for Dims4 {}
impl Dimension for Dims5 {}
impl Dimension for Dims6 {}
impl Dimension for Dims7 {}
impl Dimension for Dims8 {}
impl Dimension for Dims9 {}
impl Dimension for Dims10 {}

pub trait TupleLen {
    fn len(self) -> usize;
}

macro_rules! impl_tuple_len {
    (($($type:ty),*), $num:literal) => {
        impl TupleLen for ($($type),*) {
            fn len(self) -> usize {
                $num
            }
        }
    };
}

impl_tuple_len!((usize, usize), 2);
impl_tuple_len!((usize, usize, usize), 3);
impl_tuple_len!((usize, usize, usize, usize), 4);
impl_tuple_len!((usize, usize, usize, usize, usize), 5);
impl_tuple_len!((usize, usize, usize, usize, usize, usize), 6);
impl_tuple_len!((usize, usize, usize, usize, usize, usize, usize), 7);
impl_tuple_len!((usize, usize, usize, usize, usize, usize, usize, usize), 8);
impl_tuple_len!((usize, usize, usize, usize, usize, usize, usize, usize, usize), 9);
impl_tuple_len!((usize, usize, usize, usize, usize, usize, usize, usize, usize, usize), 10);
