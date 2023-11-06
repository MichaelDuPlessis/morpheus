use super::num::Num;
use std::ops::{Add, Index, IndexMut};

pub struct Matrix<N: Num> {
    elements: Vec<N>,
    dimensions: Vec<usize>
}

impl<N: Num> Matrix<N> {
    pub fn zeros(dimensions: Vec<usize>) -> Self {
        let flat_size = dimensions.iter().product();

        Self {
            elements: vec![N::additive_identity(); flat_size],
            dimensions
        }
    }
}

impl<N: Num> Add for Matrix<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        if self.dimensions != rhs.dimensions {
            panic!("Attempted to add two matrices with difference dimensions.
                        \n\tlhs dimensions: {:?}
                        \n\trhs dimensions: {:?}"
                   , self.dimensions
                   , rhs.dimensions);
        }

        self.elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(&mut mut lhs_element, &rhs_element)| lhs_element += rhs_element);
        
        self
    }
}

impl<N: Num> Index<usize> for Matrix<N> {
    type Output = N;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<N: Num> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}
