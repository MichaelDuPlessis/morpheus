use crate::num::Num;
use core::fmt;
use std::error::Error;

pub mod element_index;
pub mod matrix_index;

#[derive(Debug)]
pub enum GetError {
    IndexOutOfBounds,
    MismatchedShape(usize, usize),
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            GetError::IndexOutOfBounds => "Element out of bounds".to_owned(),
            GetError::MismatchedShape(matrix_shape_dims, index_shape_dims) => format!(
                "Index dimensions does not match matrix shape dimensions. Matrix {matrix_shape_dims} != Index {index_shape_dims}",
            ),
        };

        write!(f, "{}", error)
    }
}

impl Error for GetError {
    //TODO: Properly implement methods
}

pub type GetResult<N: Num> = Result<N, GetError>;
