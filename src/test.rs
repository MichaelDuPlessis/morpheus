use std::ops::{Add, Sub, Index, IndexMut};
use super::num::Num;

pub struct MatrixXd<T: Num> {
    elements: Vec<T>,
    dimensions: Vec<usize>,
}

impl<T: Num + Add<Output = T> + Clone + Copy> MatrixXd<T> {
    pub fn new(dimensions: Vec<usize>) -> Self {
        // calculate the number of elements in the flat array by multiplying the dimesntions together
        let flat_size: usize = dimensions.iter().product();

        Self { elements: vec![T::additive_identity(); flat_size], dimensions }
    }

    fn with_elements(dimensions: Vec<usize>, elements: Vec<T>) -> Self {
        let flat_size: usize = dimensions.iter().product();

        assert_eq!(flat_size, elements.len());

        Self {
            dimensions,
            elements
        }
    }
}

impl<T: Num> Add for MatrixXd<T> {
    type Output = MatrixXd<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dimensions != rhs.dimensions {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", self.dimensions, rhs.dimensions);
        }

        let elements: Vec<T> = self.elements.iter().zip(rhs.elements.iter()).map(|(&lhs_element, &rhs_element)| lhs_element + rhs_element).collect();

        Self {
            dimensions: self.dimensions.clone(),
            elements 
        }
    }  
}

impl<T: Num> Sub for MatrixXd<T> {
    type Output = MatrixXd<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimensions != rhs.dimensions {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", self.dimensions, rhs.dimensions);
        }

        let elements: Vec<T> = self.elements.iter().zip(rhs.elements.iter()).map(|(&lhs_element, &rhs_element)| lhs_element - rhs_element).collect();

        Self {
            dimensions: self.dimensions.clone(),
            elements 
        }
    }  
}

impl<T: Num> Index<usize> for MatrixXd<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index] 
    }
}

impl<T: Num> IndexMut<usize> for MatrixXd<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}
