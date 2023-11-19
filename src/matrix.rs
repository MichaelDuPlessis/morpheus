use crate::{
    index::{element_index::GetElement, GetError, GetResult},
    num::Num,
};

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

impl<N: Num> GetElement<N> for Matrix<N> {
    fn get_mut(&mut self, index: impl AsRef<[usize]>) -> GetResult<&mut N> {
        let index = index.as_ref();

        // checking if matrix and index has same shape
        if self.shape.len() != index.len() {
            return Err(GetError::MismatchedShape(self.shape.len(), index.len()));
        }

        // checking bounds

        Ok(self.get_unchecked_mut(index))
    }

    fn get_unchecked_mut(&mut self, index: impl AsRef<[usize]>) -> &mut N {
        todo!()
    }
}
