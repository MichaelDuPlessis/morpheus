pub mod test;

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
    fn create_matrix() {
        let matrix: MatrixXd<i32> = MatrixXd::full(2, 4, 0);
        println!("{:?}", matrix);

        for i in 0..2 {
            for j in 0..4 {
                assert_eq!(matrix[i][j], 0);
            }
        }

    }
}
