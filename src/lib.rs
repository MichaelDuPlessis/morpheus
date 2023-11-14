mod dimension;
mod matrix;
mod num;

#[cfg(test)]
mod tests {
    use crate::matrix::MatrixEnum;

    #[test]
    fn index() {
        let test_matrix: MatrixEnum<f64> = MatrixEnum::zeros(vec![2, 2]);

        println! {"{:#?}", test_matrix};

        println!(
            "================================\nIndex 0:\n{:#?}",
            test_matrix[0][0]
        );
    }
}
