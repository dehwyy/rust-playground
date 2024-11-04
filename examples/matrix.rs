use playground as pg;

use pg::matrix::Matrix;

fn main() {
    let matrix = Matrix::<u16>::new_random(3);
    matrix.print_pretty();
}
