use playground as pg;

use pg::core::term::print::Print;
use pg::core::term::print::{Align, PrintConfig};
use pg::matrix::{Dim, Matrix, MatrixOperations, SquareMatrix};

fn main() {
    let mut matrix = Matrix::new(Dim::new(3, 3));
    matrix.fill_random_in_range(1., 4.);

    let sq_matrix = SquareMatrix::try_from(&matrix).unwrap();

    let cfg = PrintConfig::new().align(Align::MiddleCenter).precision(1);
    sq_matrix.print(&cfg);

    let mut new_sq = SquareMatrix::new(3);
    new_sq.fill_random_in_range(1., 5.);

    new_sq.print(&cfg);
    println!("{}", new_sq.det());
}
