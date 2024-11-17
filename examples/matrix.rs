use playground::{self as pg, matrix::SquareMatrix, random::Random};

use pg::matrix::{Dim, Matrix, MatrixOperations};

fn main() {
    let mut matrix = Matrix::new(Dim::new(5, 5));
    matrix.fill_random_in_range(1., 4.);

    let sq_matrix = SquareMatrix::try_from(&matrix).unwrap();

    // let (echo, _) = matrix.as_echelon_form();
    // println!("{:.2}", matrix);
    // println!("");
    println!("{:.2}", matrix);
    println!("{:.2}", matrix.clone().to_echelon_form());
    println!("{}", sq_matrix.det());
}
