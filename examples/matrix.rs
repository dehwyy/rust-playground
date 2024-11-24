use playground as pg;

use pg::core::term::print::Print;
use pg::core::term::print::{Align, PrintConfig};
use pg::matrix::{Dim, Matrix, MatrixOperations};

fn main() {
    let mut matrix = Matrix::new(Dim::new(3, 3));
    matrix.fill_random_in_range(1., 4.);

    let mut matrix2 = Matrix::new(Dim::new(3, 2));
    matrix2.fill_random_in_range(1., 4.);

    let matrix3 = matrix.multiply(&matrix2).unwrap();

    let cfg = PrintConfig::new().align(Align::Center).precision(2);

    matrix.print(&cfg);
    matrix2.print(&cfg);
    matrix3.print(&cfg);
}
