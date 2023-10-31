use morpheus::test::MatrixXd;

fn main() { 
    let matrix: MatrixXd<i32> = MatrixXd::full(2, 4, 0);
    println!("{:?}", matrix);
}
