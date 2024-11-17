mod impls;
pub use impls::*;

use std::{cell::Cell, fmt::Display};

use crate::core as pg;
use crate::random::Random;

//         pub fn print_pretty(&self) {
//           let max_el_size= self.get_max_element_size();
//           let matrix_print_size_no_borders = self.size.0 * (max_el_size + GAP_X) + GAP_X;
//           let matrix_print_size = matrix_print_size_no_borders + 2;

//           println!("{}", X_TOP_BORDER.repeat(matrix_print_size));

//           self.for_each_el(
//             |el| {
//               print!("{el: >width$}{gap}", width = self.get_max_element_size(), gap=GAP_FILL.repeat(GAP_X));
//             },
//             || {
//               print!("{}", Y_BORDER);
//               print!("{}", GAP_FILL.repeat(GAP_X));
//             },
//             ||  {
//               println!("{}", Y_BORDER);
//               for _ in 0..(GAP_Y - 1) {
//                 println!("{Y_BORDER}{}{Y_BORDER}", GAP_FILL.repeat(matrix_print_size_no_borders));
//               }
//             }
//           );

//           println!("{}", X_BOTTOM_BORDER.repeat(matrix_print_size));
//         }

mod print {
    pub const PRECISION: usize = 4;
    pub const ITEM_X_GAP: usize = 4;
}

type MatrixItem = f32;
type ItemCell = Cell<MatrixItem>;
type MatrixDim = usize;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Dim(MatrixDim, MatrixDim);

pub struct MatrixColumn(Vec<ItemCell>);
pub struct MatrixRow<'a>(&'a Vec<ItemCell>);

#[derive(Clone)]
pub struct Matrix {
    data: Vec<Vec<ItemCell>>,
    dim: Dim,
}

/// Guaranteed to be N × N.
#[derive(Clone)]
pub struct SquareMatrix {
    data: Vec<Vec<ItemCell>>,
    size: MatrixDim,
}
