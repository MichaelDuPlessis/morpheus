use std::ops::{Add, Sub, Index, IndexMut};
use super::num::Num;
use super::dimension::Dimension;

pub struct MatrixXd<T: Num, D: Dimension> {
    elements: Vec<T>,
    dimensions: D,
}

impl<T: Num, D: Dimension> MatrixXd<T, D> {
    pub fn new(dimensions: D) -> Self {
        // calculate the number of elements in the flat array by multiplying the dimesntions together
        let flat_size: usize = dimensions.into().iter().product();

        // create the elements using `with_capacity` so that no unnecesary memeory is requested
        let mut elements = Vec::with_capacity(flat_size);
        (0..flat_size).into_iter().for_each(|_| elements.push(T::additive_identity()));

        Self { elements, dimensions }
    }

    fn with_elements(dimensions: D, elements: Vec<T>) -> Self {
        let flat_size: usize = dimensions.into().iter().product();

        assert_eq!(flat_size, elements.len());

        Self {
            dimensions,
            elements
        }
    }
}

impl<T: Num, D: Dimension> Add for MatrixXd<T, D> {
    type Output = MatrixXd<T, D>;

    fn add(self, rhs: Self) -> Self::Output {
        let d1: Vec<usize> = self.dimensions.into();
        let d2: Vec<usize> = rhs.dimensions.into();

        if d1 != d2 {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", d1, d2);
        }

        let elements: Vec<T> = self.elements.iter().zip(rhs.elements.iter()).map(|(&lhs_element, &rhs_element)| lhs_element + rhs_element).collect();

        Self {
            dimensions: d1.clone().into(),
            elements 
        }
    }  
}

impl<T: Num, D: Dimension> Sub for MatrixXd<T, D> {
    type Output = MatrixXd<T, D>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimensions.into() != rhs.dimensions.into() {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", self.dimensions.into(), rhs.dimensions.into());
        }

        let elements: Vec<T> = self.elements.iter().zip(rhs.elements.iter()).map(|(&lhs_element, &rhs_element)| lhs_element - rhs_element).collect();

        Self {
            dimensions: self.dimensions,
            elements 
        }
    }  
}

impl<T: Num, D: Dimension> Index<usize> for MatrixXd<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index] 
    }
}

impl<T: Num, D: Dimension> IndexMut<usize> for MatrixXd<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T: Num, D: Dimension> Index<Vec<usize>> for MatrixXd<T, D> {
    type Output = T;

    fn index(&self, index: Vec<usize>) -> &Self::Output {
        if index.len() != self.dimensions.into().len() {
            panic!("Attempt to index a matrix with incompatible index length:\n\tindex length: {}\n\tmatrix dimensions: {}", index.len(), self.dimensions.into().len());
        }

        let mut flat_index: usize = 0;

        let num_dimensions = index.len();

        for i in 0..(num_dimensions - 1) {
            flat_index += index[i] * ((i+1)..num_dimensions).into_iter().map(|j| self.dimensions.into()[j]).product::<usize>();
        }

        flat_index += index.last().unwrap();
        println!("\n flat index: {}", flat_index);

        &self[flat_index]
    } 
}
