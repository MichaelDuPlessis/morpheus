use crate::num::Num;
use std::borrow::Borrow;

pub struct Matrix<N: Num> {
    elements: Vec<N>,
    shape: Vec<usize>,
}

impl<N: Num> Matrix<N> {
    pub fn zeros(shape: impl Into<Vec<usize>>) -> Self {
        let shape = shape.into();
        let elements = vec![N::additive_identity(); shape.iter().copied().product()];

        Self { elements, shape }
    }
}

pub trait MatrixIndex<N: Num> {
    fn index(&self, index: impl Borrow<[usize]>) -> Matrix<N>;
}

pub trait ElementIndex<N: Num> {
    fn index(&self, index: usize) -> N;
}

// impl MatrixIndex for Matrix {
//     fn index(&self, index: impl Borrow<[usize]>) -> Matrix {
//         Matrix {}
//     }
// }

impl<N: Num> ElementIndex<N> for Matrix<N> {
    fn index(&self, index: usize) -> N {
        self.elements[index]
    }
}
