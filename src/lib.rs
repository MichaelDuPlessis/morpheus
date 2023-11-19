mod dimension;
mod matrix;
mod num;

#[cfg(test)]
mod tests {
    use crate::matrix::{ElementIndex, Matrix, MatrixIndex};

    #[test]
    fn index() {
        let matrix: Matrix<usize> = Matrix::zeros([2, 2]);

        let x = ElementIndex::index(&matrix, 0);
        // let y = MatrixIndex::index(&matrix, [0, 1, 4]);
    }
}
