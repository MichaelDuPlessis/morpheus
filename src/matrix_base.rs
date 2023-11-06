use super::dimension::*;
use super::num::Num;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub enum MatrixError {}

pub trait Matrix<N: Num>:
    Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + Mul<N>
    + MulAssign
    + MulAssign<N>
    + Div
    + Div<N>
    + DivAssign
    + DivAssign<N>
    + Clone
    + Sized
{
    fn transpose<M: Matrix<N>>(&mut self) -> Result<&mut M, MatrixError>;

    fn transposed<M: Matrix<N>>(self) -> Result<M, MatrixError>;

    fn inverse<M: Matrix<N>>(&mut self) -> Result<&mut M, MatrixError>;

    fn inversed<M: Matrix<N>>(self) -> Result<M, MatrixError>;

    fn determinant(&self) -> Result<f64, MatrixError>;

    fn determinant_f32(&self) -> Result<f32, MatrixError> {
        self.determinant().map(|n| n as f32)
    }

    fn powf<M: Matrix<N>>(self, n: f32) -> Result<M, MatrixError>;

    fn powi<M: Matrix<N>>(self, n: i32) -> Result<M, MatrixError>;

    fn exp<M: Matrix<N>>(&mut self) -> &mut Result<M, MatrixError>;

    fn transpose_unchecked<M: Matrix<N>>(&mut self) -> &mut M;

    fn transposed_unchecked<M: Matrix<N>>(self) -> M;

    fn inverse_unchecked<M: Matrix<N>>(&mut self) -> &mut M;

    fn inversed_unchecked<M: Matrix<N>>(self) -> M;

    fn determinant_unchecked(&self) -> f64;

    fn determinant_f32_unchecked(&self) -> f32 {
        self.determinant_unchecked() as f32
    }

    fn powf_unchecked<M: Matrix<N>>(self, n: f32) -> M;

    fn powi_unchecked<M: Matrix<N>>(self, n: i32) -> M;

    fn exp_unchecked<M: Matrix<N>>(&mut self) -> &mut M;
}

struct Vector<N: Num> {
    elements: Vec<N>,
}

pub struct MatrixXd<T: Num, D: Dimension> {
    elements: Vec<T>,
    dimensions: D,
}

impl<T: Num, D: Dimension> MatrixXd<T, D> {
    pub fn new(dimensions: D) -> Self {
        // calculate the number of elements in the flat array by multiplying the dimesntions together
        let flat_size: usize = dimensions.as_ref().iter().product();

        // create the elements using `with_capacity` so that no unnecesary memeory is requested
        let elements = vec![T::additive_identity(); flat_size];

        Self {
            elements,
            dimensions,
        }
    }

    fn with_elements(dimensions: D, elements: &[T]) -> Self {
        let flat_size: usize = dimensions.as_ref().iter().product();

        assert_eq!(flat_size, elements.len());

        Self {
            dimensions,
            elements: elements.into(),
        }
    }
}

/*
impl<T: Num, D: Dimension> Add for MatrixXd<T, D> {
    type Output = MatrixXd<T, D>;

    fn add(self, rhs: Self) -> Self::Output {
        let d1: Vec<usize> = self.dimensions.into();
        let d2: Vec<usize> = rhs.dimensions.into();

        if d1 != d2 {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", d1, d2);
        }

        let elements: Vec<T> = self
            .elements
            .iter()
            .zip(rhs.elements.iter())
            .map(|(&lhs_element, &rhs_element)| lhs_element + rhs_element)
            .collect();

        Self {
            dimensions: d1.clone().into(),
            elements,
        }
    }
}

impl<T: Num, D: Dimension> Sub for MatrixXd<T, D> {
    type Output = MatrixXd<T, D>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimensions.into() != rhs.dimensions.into() {
            panic!("Attempt to add matrices of different dimensions:\n\tlhs dimensions: {:?}\n\trhs dimensions: {:?}", self.dimensions.into(), rhs.dimensions.into());
        }

        let elements: Vec<T> = self
            .elements
            .iter()
            .zip(rhs.elements.iter())
            .map(|(&lhs_element, &rhs_element)| lhs_element - rhs_element)
            .collect();

        Self {
            dimensions: self.dimensions,
            elements,
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

impl<T: Num, D: Dimension> Index<&[usize]> for MatrixXd<T, D> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        if index.len() != self.dimensions.into().len() {
            panic!("Attempt to index a matrix with incompatible index length:\n\tindex length: {}\n\tmatrix dimensions: {}", index.len(), self.dimensions.into().len());
        }

        let mut flat_index: usize = 0;

        let num_dimensions = index.len();

        for i in 0..(num_dimensions - 1) {
            flat_index += index[i]
                * ((i + 1)..num_dimensions)
                    .into_iter()
                    .map(|j| self.dimensions.into()[j])
                    .product::<usize>();
        }

        flat_index += index.last().unwrap();
        println!("\n flat index: {}", flat_index);

        &self[flat_index]
    }
}
*/

#[macro_export]
macro_rules! matrix {
    ($type:ty, [$($dim:expr),*]) => {
        MatrixXd::<$type, _>::new(StackDims([$($dim),*]))
    };
    ($type:ty, ($($dim:expr),*)) => {
        MatrixXd::<$type, _>::new(StackDims([$($dim),*]))
    };
    ($type:ty, $e:expr) => {
        MatrixXd::<$type, _>::new(StackDims($e))
    };
    ($type:ty, $($dim:expr),*) => {
        MatrixXd::<$type, _>::new(StackDims([$($dim),*]))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = matrix!(f64, 3, 3);
        let x = matrix!(f64, (3, 3, 2, 4, 5, 6, 7, 109, 12, 89, 12, 23, 44, 90));
        let (a, b) = (1, 2);
        let x = matrix!(u8, [a, b]);
    }
}
