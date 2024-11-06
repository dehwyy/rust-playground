use playground as pg;

use pg::matrix::{Dim, Matrix};

fn main() {
    let mut matrix = Matrix::new(Dim::new(4, 4));
    matrix.fill_random_in_range(1., 10.);

    // let (echo, _) = matrix.as_echelon_form();
    println!("{}", matrix);

    println!("{}", matrix.as_echelon_form());
}
