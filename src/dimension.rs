pub trait Dimension: Clone + From<Vec<usize>> {
    fn dims(&self) -> &[usize];

    fn flat_size(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct StackDims<const D: usize>(pub [usize; D]);

impl<const D: usize> From<Vec<usize>> for StackDims<D> {
    fn from(value: Vec<usize>) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl<const D: usize> Dimension for StackDims<D> {
    fn dims(&self) -> &[usize] {
        &self.0
    }

    fn flat_size(&self) -> usize {
        self.0.len()
    }
}

// #[derive(Clone)]
// pub struct HeapDims(Vec<usize>);
//
// impl Dimension for HeapDims {
//     fn dims(&self) -> &[usize] {
//         &self.0
//     }
// }
