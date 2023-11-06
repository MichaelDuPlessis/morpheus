mod num;
mod dimension;
mod matrix_base;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{matrix_base::MatrixXd, dimension::*};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_matrices() {
        let mut matrix1 = matrix2D!((2, 2), isize);
        let mut matrix2 = matrix2D!((2, 2), isize);

        for i in 0..4 {
            matrix1[i] = i as isize;
            matrix2[i] = i as isize;
        }

        let matrix3 = matrix1 + matrix2; 

        for i in 0..4 {
            assert_eq!(matrix3[i], (i + i) as isize);
        }
    }

    #[test]
    fn subtract_matrices() {
        let mut matrix1 = matrix2D!((2, 2), isize);
        let mut matrix2 = matrix2D!((2, 2), isize);

        for i in 0..4 {
            matrix1[i] = (i + 2) as isize;
            matrix2[i] = i as isize;
        }

        let matrix3 = matrix1 - matrix2; 

        for i in 0..4 {
            assert_eq!(matrix3[i], 2);
        }
    }

    #[test]
    fn basic_indexing() {
        let mut matrix1 = matrix2D!((10, 10), f64);
        let mut matrix2 = matrix3D!((5, 5, 4), f64);
        
        for i in 0..100 {
            matrix1[i] = i as f64;
            matrix2[i] = i as f64;
        }

        let output1 = matrix1[vec![5, 0]];
        let output2 = matrix2[vec![2, 2, 2]];

        assert_eq!(output1, 50.0);
        assert_eq!(output2, 50.0);

        let output3 = matrix1[vec![9, 3]];
        let output4 = matrix2[vec![3, 3, 2]];

        assert_eq!(output3, 93.0);
        assert_eq!(output4, 74.0);
    }
}
