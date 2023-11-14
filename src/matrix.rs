use super::num::Num;

use core::fmt;
use std::ops::Index;

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum MatrixEnum<N: Clone + Num> {
    Matrix(Vec<MatrixEnum<N>>),
    Element(N),
}

impl<N: Clone + Num + fmt::Display> MatrixEnum<N> {
    #[allow(unused)]
    pub fn zeros(dimensions: Vec<usize>) -> Self {
        let mut matrix = MatrixEnum::Matrix(vec![
            MatrixEnum::Element(N::additive_identity());
            *dimensions.last().unwrap()
        ]);

        for dim in dimensions.iter().rev().skip(1) {
            matrix = fill_matrix(matrix, dim);
        }

        matrix
    }
}

#[allow(unused)]
fn fill_matrix<N: Num>(matrix: MatrixEnum<N>, length: &usize) -> MatrixEnum<N> {
    MatrixEnum::Matrix(vec![matrix; *length])
}

impl<N: Num + Clone + fmt::Display> Index<usize> for MatrixEnum<N> {
    type Output = MatrixEnum<N>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Matrix(inner) => &inner[index],
            Self::Element(inner) => panic!("You cannot index the element: {}", inner),
        }
    }
}
