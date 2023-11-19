use super::{GetError, GetResult};
use crate::num::Num;

pub trait GetElement<N: Num> {
    fn mismatched_shape(&self, index: impl AsRef<[usize]>) -> Option<GetError>;

    fn out_of_bounds_check(&self, index: impl AsRef<[usize]>) -> Option<GetError>;

    fn check_operation(&self, index: impl AsRef<[usize]>) -> Option<GetError> {
        if let Some(err) = self.mismatched_shape(index) {
            return Some(err);
        }

        if let Some(err) = self.out_of_bounds_check(index) {
            return Some(err);
        }

        None
    }

    fn get_mut(&mut self, index: impl AsRef<[usize]>) -> GetResult<&mut N> {
        let index = index.as_ref();

        if let Some(err) = self.check_operation(index) {
            return Err(err);
        }

        Ok(self.get_unchecked_mut(index))
    }

    fn get_unchecked_mut(&mut self, index: impl AsRef<[usize]>) -> &mut N;

    fn get(&self, index: impl AsRef<[usize]>) -> GetResult<&N> {
        let index = index.as_ref();

        if let Some(err) = self.check_operation(index) {
            return Err(err);
        }

        Ok(self.get_unchecked(index))
    }

    fn get_unchecked(&self, index: impl AsRef<[usize]>) -> &N;
}
