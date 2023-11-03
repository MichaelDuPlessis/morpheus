mod num;
mod test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::test::MatrixXd;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_matrices() {
        let mut matrix1: MatrixXd<isize> = MatrixXd::new(vec![2, 2]);
        let mut matrix2: MatrixXd<isize> = MatrixXd::new(vec![2, 2]);

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
        let mut matrix1: MatrixXd<isize> = MatrixXd::new(vec![2, 2]);
        let mut matrix2: MatrixXd<isize> = MatrixXd::new(vec![2, 2]);

        for i in 0..4 {
            matrix1[i] = (i + 2) as isize;
            matrix2[i] = i as isize;
        }

        let matrix3 = matrix1 - matrix2; 

        for i in 0..4 {
            assert_eq!(matrix3[i], 2);
        }
    }
}
