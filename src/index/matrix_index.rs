use crate::{matrix::Matrix, num::Num};

pub trait MatrixIndex<N: Num> {
    fn index(&self, index: impl AsRef<[usize]>) -> Matrix<N>;
}
