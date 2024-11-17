use playground as pg;

use pg::matrix::{Dim, Matrix};

fn main() {
    let mut matrix = Matrix::new(Dim::new(3, 3));
    matrix.fill_random_in_range(1., 4.);

    // let (echo, _) = matrix.as_echelon_form();
    println!("{}", matrix);
    println!("");
    println!("{}", matrix.as_echelon_form());
    println!("");
    println!("{}", matrix.det().unwrap());
}
