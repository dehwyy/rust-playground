use pg::{math::get_i32_len, term::print::Print};
use print::ITEM_X_GAP;

use super::*;

fn new_empty_matrix_data(dim: &Dim) -> MatrixData {
    let mut matrix_data = Vec::with_capacity(dim.get_m());

    for i in 0..dim.get_m() {
        matrix_data.push(Vec::with_capacity(dim.get_n()));
        for _ in 0..dim.get_n() {
            matrix_data[i].push(ItemCell::default());
        }
    }

    matrix_data
}

pub trait MatrixRepr {
    fn get_data(&self) -> &'_ MatrixData;
    fn get_dim(&self) -> Dim;
}

pub trait MatrixOperations: MatrixRepr
where
    Self: Sized + Clone + Display,
{
    fn iter(&self) -> impl Iterator<Item = &ItemCell> {
        self.get_data().iter().flatten()
    }

    fn row(&self, row_i: MatrixDim) -> MatrixRow {
        MatrixRow(self.get_data().get(row_i).unwrap())
    }

    fn col(&self, col_i: MatrixDim) -> MatrixColumn {
        MatrixColumn(
            self.get_data()
                .iter()
                .map(|row| row[col_i].clone())
                .collect(),
        )
    }

    fn get(&self, i: MatrixDim, j: MatrixDim) -> &ItemCell {
        &self.get_data()[i][j]
    }

    /// @Mutate matrix row | [fold_to_row_idx]
    ///
    /// @Leaves unchanged elements in | [fold_from_row_idx]
    fn fold_row(&self, to_idx: MatrixDim, from_idx: MatrixDim, k: f32) {
        self.row(to_idx).fold(self.row(from_idx), k);
    }

    fn spaw_rows(&mut self, lhs: MatrixDim, rhs: MatrixDim) {
        for i in 0..self.get_dim().get_n() {
            self.get_data()[lhs][i].swap(&self.get_data()[rhs][i]);
        }
    }

    fn fill_random(&mut self) {
        self.iter().for_each(|v| v.set(Random::get()));
    }

    fn fill_random_in_range(&mut self, min: MatrixItem, max: MatrixItem) {
        // ? cast?? tf
        self.iter()
            .for_each(|v| v.set(Random::get_in_range(min as u8, max as u8) as f32));
    }

    fn fill(&mut self, value: MatrixItem) {
        self.iter().for_each(|v| v.set(value));
    }

    fn fill_fn(&mut self, f: impl Fn(MatrixDim, MatrixDim) -> MatrixItem) {
        for i in 0..self.get_dim().get_m() {
            for j in 0..self.get_dim().get_n() {
                self.get_data()[i][j].set(f(i, j));
            }
        }
    }

    fn to_echelon_form(mut self) -> Self {
        let mut start_pivot_idx = 0;
        // for each column
        for i in 0..self.get_dim().get_n() {
            let (pivot_el, pivot_idx) = {
                let mut el = 0.;
                let mut idx = 0;

                for j in start_pivot_idx..self.get_dim().get_m() {
                    el = self.get(j, i).get();
                    if el != 0. {
                        idx = j;
                        break;
                    }
                }

                (el, idx)
            };

            if (pivot_el == 0.) {
                continue;
            }

            if (pivot_idx != start_pivot_idx) {
                self.spaw_rows(pivot_idx, i);
            }

            for j in (start_pivot_idx + 1)..self.get_dim().get_m() {
                let factor = self.get(j, i).get() / pivot_el;

                self.fold_row(j, start_pivot_idx, -factor);
            }

            start_pivot_idx += 1;
        }

        self
    }

    fn multiply(&self, rhs: &Self) -> Option<Matrix> {
        let (m1, n1) = (self.get_dim().get_m(), self.get_dim().get_n());
        let (m2, n2) = (rhs.get_dim().get_m(), rhs.get_dim().get_n());

        if n1 != m2 {
            return None;
        }

        let dim = Dim(m1, n2);

        let data = new_empty_matrix_data(&dim);
        for n in 0..n2 {
            for m in 0..m1 {
                data[m][n].set((0..n1).zip(0..m2).fold(0., |acc, (i, j)| {
                    acc + self.get(m, i).get() * rhs.get(j, n).get()
                }));
            }
        }

        Some(Matrix { data, dim })
    }
}

impl Dim {
    /// m = Rows
    #[inline]
    fn get_m(&self) -> MatrixDim {
        self.0
    }

    /// n = Cols
    #[inline]
    fn get_n(&self) -> MatrixDim {
        self.1
    }

    pub fn new(m: MatrixDim, n: MatrixDim) -> Self {
        Self(m, n)
    }

    pub fn is_square(&self) -> bool {
        self.0 == self.1
    }

    pub fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1);
    }
}
impl MatrixRow<'_> {
    fn iter(&self) -> impl Iterator<Item = &ItemCell> {
        self.0.iter()
    }

    /// @Mutate `self`: `cell[self]` += `k` * `cell[row]`
    ///
    /// @Leaves unchanged `row`
    pub fn fold(&self, row: MatrixRow, k: f32) {
        for (v1, v2) in self.iter().zip(row.iter()) {
            v1.set(v1.get() + k * v2.get());
        }
    }
}

impl Matrix {
    // const GAP_Y: usize = 2;
    // const GAP_X: usize = 2;
    // const GAP_FILL: &str = " ";
    // const X_TOP_BORDER: &str = "─";
    // const X_BOTTOM_BORDER: &str = "‾";
    // const Y_BORDER: &str = "│";

    pub fn new(dim: Dim) -> Self {
        Self {
            data: new_empty_matrix_data(&dim),
            dim,
        }
    }
}

impl SquareMatrix {
    pub fn new(size: MatrixDim) -> Self {
        Self {
            data: new_empty_matrix_data(&Dim(size, size)),
            size,
        }
    }

    pub fn get_size(&self) -> MatrixDim {
        self.size
    }

    pub fn get_main_diagonal(&self) -> Vec<MatrixItem> {
        (0..self.get_size()).map(|i| self.get(i, i).get()).collect()
    }

    /// @Returns `determinant` of the matrix.
    ///
    /// @If matrix is not square, returns `None`.
    pub fn det(&self) -> MatrixItem {
        self.clone()
            .to_echelon_form()
            .get_main_diagonal()
            .iter()
            .fold(1., |acc, el| acc * el)
    }
}

impl MatrixRepr for Matrix {
    fn get_data(&self) -> &MatrixData {
        &self.data
    }
    fn get_dim(&self) -> Dim {
        self.dim
    }
}
impl MatrixRepr for SquareMatrix {
    fn get_data(&self) -> &MatrixData {
        &self.data
    }
    fn get_dim(&self) -> Dim {
        Dim(self.size, self.size)
    }
}

impl MatrixOperations for Matrix {}
impl MatrixOperations for SquareMatrix {}

impl TryFrom<&Matrix> for SquareMatrix {
    type Error = pg::error::Error;
    fn try_from(value: &Matrix) -> Result<Self, Self::Error> {
        let matrix_copy = value.clone();
        SquareMatrix::try_from(matrix_copy)
    }
}
impl TryFrom<Matrix> for SquareMatrix {
    type Error = pg::error::Error;
    fn try_from(value: Matrix) -> Result<Self, Self::Error> {
        match value.dim.is_square() {
            true => Ok(SquareMatrix {
                size: value.dim.get_m(),
                data: value.data,
            }),
            false => Err(Self::Error::default()),
        }
    }
}

impl Display for dyn MatrixRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(print::PRECISION);

        let (max, min) = {
            let mut max = f32::MIN;
            let mut min = f32::MAX;
            self.get_data().iter().flatten().for_each(|v| {
                let val = v.get();
                (val > max).then(|| max = val);
                (val < min).then(|| min = val);
            });

            (max as i32, min as i32)
        };

        let (m, n) = (self.get_dim().get_m(), self.get_dim().get_n());
        let width = get_i32_len(max).max(get_i32_len(min)) + precision + print::ITEM_X_GAP / 2;
        let row_width = n * (width + ITEM_X_GAP / 2);

        writeln!(f, "{:_^row_width$}", "Matrix")?;
        for i in 0..m {
            for j in 0..n {
                write!(f, "{:>width$.precision$}", self.get_data()[i][j].get())?;

                if j != self.get_dim().get_n() - 1 {
                    write!(f, " ")?;
                }
            }

            // At least one newline.
            writeln!(f)?;

            // If not last row -> newline n-1 times.
            if i != m - 1 {
                for _ in 1..print::ITEM_Y_GAP {
                    writeln!(f)?;
                }
            }
        }

        writeln!(f, "{:‾^row_width$}", "")?;

        Ok(())
    }
}

impl Print for Matrix {}
impl Print for SquareMatrix {}
impl Display for SquareMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <dyn MatrixRepr>::fmt(self, f)
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <dyn MatrixRepr>::fmt(self, f)
    }
}
