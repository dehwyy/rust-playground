use playground::{self as pg, matrix::SquareMatrix};

use pg::matrix::{Dim, Matrix, MatrixOperations};

fn main() {
    let mut matrix = Matrix::new(Dim::new(3, 3));
    matrix.fill_random_in_range(1., 4.);

    let sq_matrix = SquareMatrix::try_from(&matrix).unwrap();

    // let (echo, _) = matrix.as_echelon_form();
    println!("{:.10}", matrix);
    println!("");
    println!("{:.10}", matrix.to_echelon_form());
    println!("");
    println!("{}", sq_matrix.det());
}
