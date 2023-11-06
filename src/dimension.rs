pub trait Dimension: AsRef<[usize]> + Clone {}

#[derive(Clone, Copy)]
pub struct StackDims<const D: usize>(pub [usize; D]);

impl<const D: usize> Dimension for StackDims<D> {}

impl<const D: usize> AsRef<[usize]> for StackDims<D> {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}

#[derive(Clone)]
pub struct HeapDims(Vec<usize>);

impl Dimension for HeapDims {}

impl AsRef<[usize]> for HeapDims {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}
