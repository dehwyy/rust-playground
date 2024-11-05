use playground as pg;

use pg::matrix::{Matrix, Dim};


fn main() {
    let mut matrix = Matrix::new(Dim::new(4, 4));
    matrix.fill_random_in_range(1, 10);


    let (echo, _) = matrix.as_echelon_form();
    println!("{:#}", echo);
}
