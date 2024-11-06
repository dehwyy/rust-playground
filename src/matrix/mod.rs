mod implementation;
pub use implementation::*;

use std::{
    cell::Cell,
    fmt::{Display, Write},
};

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

type MatrixItem = f32;
type ItemCell = Cell<MatrixItem>;
type MatrixDim = usize;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Dim(MatrixDim, MatrixDim);

impl Dim {
    #[inline]
    fn get_m(&self) -> MatrixDim {
        self.0
    }

    #[inline]
    fn get_n(&self) -> MatrixDim {
        self.1
    }

    // todo: using `where`?
    pub fn new(m: MatrixDim, n: MatrixDim) -> Self {
        Self(m, n)
    }

    pub fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1);
    }
}

pub struct MatrixColumn(Vec<ItemCell>);

pub struct MatrixRow<'a>(&'a Vec<ItemCell>);

impl MatrixRow<'_> {
    fn iter(&self) -> impl Iterator<Item = &ItemCell> {
        self.0.iter()
    }

    pub fn fold(&self, row: MatrixRow, k: f32) {
        for (v1, v2) in self.iter().zip(row.iter()) {
            v1.set(v1.get() + k * v2.get());
        }
    }
}

#[derive(Clone)]
pub struct Matrix {
    data: Vec<Vec<ItemCell>>,
    dim: Dim,
}

impl Matrix {
    pub fn iter(&self) -> impl Iterator<Item = &ItemCell> {
        self.into_iter()
    }

    pub fn row(&self, row_i: MatrixDim) -> MatrixRow {
        MatrixRow(self.data.get(row_i).unwrap())
    }

    pub fn col(&self, col_i: MatrixDim) -> MatrixColumn {
        MatrixColumn(self.data.iter().map(|row| row[col_i].clone()).collect())
    }

    /// @Mutate matrix row | [fold_to_row_idx]
    ///
    /// @Leaves unchanged elements in | [fold_from_row_idx]
    pub fn fold_row(&self, to_idx: MatrixDim, from_idx: MatrixDim, k: f32) {
        self.row(to_idx).fold(self.row(from_idx), k);
    }
}

impl Matrix {
    // const GAP_Y: usize = 2;
    // const GAP_X: usize = 2;
    // const GAP_FILL: &str = " ";
    // const X_TOP_BORDER: &str = "─";
    // const X_BOTTOM_BORDER: &str = "‾";
    // const Y_BORDER: &str = "│";

    fn empty_matrix_data(dim: &Dim) -> Vec<Vec<ItemCell>> {
        let mut matrix_data = Vec::with_capacity(dim.get_m().into());

        for _ in 0..matrix_data.capacity() {
            let mut v = Vec::with_capacity(dim.get_n());

            for _ in 0..v.capacity() {
                v.push(Cell::new(MatrixItem::default()));
            }

            matrix_data.push(v);
        }

        matrix_data
    }

    fn get(&self, i: MatrixDim, j: MatrixDim) -> &ItemCell {
        &self.data[i][j]
    }

    pub fn new(dim: Dim) -> Self {
        Self {
            data: Self::empty_matrix_data(&dim),
            dim,
        }
    }

    pub fn fill_random(&mut self) {
        self.iter().for_each(|v| v.set(Random::get()));
    }

    pub fn fill_random_in_range(&mut self, min: MatrixItem, max: MatrixItem) {
        // ? cast?? tf
        self.iter()
            .for_each(|v| v.set(Random::get_in_range(min as u8, max as u8) as f32));
    }

    pub fn as_echelon_form(&self) -> Self {
        // let mut raw_echelon = self.clone();
        // TODO: zero-checks,

        let echelon_form = self.clone();

        // for each column
        for i in 0..self.dim.get_n() {
            let pivot_el = echelon_form.get(i, i).get();

            if pivot_el == 0. {
                // TODO: perform checks
            }

            for j in i + 1..self.dim.get_n() {
                let factor = echelon_form.get(j, i).get() / pivot_el;

                echelon_form.fold_row(j, i, -factor);
            }
        }

        echelon_form
    }

    /// @Returns `determinant` of the matrix.
    pub fn det(&self) -> i32 {
        todo!()
        // let lhs = self.get_i32(0, 0) * self.get_i32(1, 1);
        // let rhs = self.get_i32(0, 1) * self.get_i32(1, 0);

        // lhs - rhs
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.dim.get_m() {
            for j in 0..self.dim.get_n() {
                write!(f, "{}", self.get(i, j).get())?;

                if j != self.dim.get_n() - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.dim.get_m() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
