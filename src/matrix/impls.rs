use super::*;

fn new_empty_matrix_data(dim: &Dim) -> Vec<Vec<ItemCell>> {
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
    fn get_data(&self) -> &'_ Vec<Vec<ItemCell>>;
    fn get_dim(&self) -> Dim;
}

pub trait MatrixOperations: MatrixRepr
where
    Self: Sized + Clone,
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

    fn get_min(&self) -> MatrixItem {
        self.iter()
            .min_by(|a, b| a.get().partial_cmp(&b.get()).unwrap())
            .expect("Empty iterator!")
            .get()
    }

    fn get_max(&self) -> MatrixItem {
        self.iter()
            .max_by(|a, b| a.get().partial_cmp(&b.get()).unwrap())
            .expect("Empty iterator!")
            .get()
    }

    /// @Mutate matrix row | [fold_to_row_idx]
    ///
    /// @Leaves unchanged elements in | [fold_from_row_idx]
    fn fold_row(&self, to_idx: MatrixDim, from_idx: MatrixDim, k: f32) {
        self.row(to_idx).fold(self.row(from_idx), k);
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

    fn to_echelon_form(self) -> Self {
        // TODO: zero-checks,
        // for each column
        for i in 0..self.get_dim().get_n() {
            let pivot_el = self.get(i, i).get();

            if pivot_el == 0. {
                // TODO: perform checks
            }

            for j in i + 1..self.get_dim().get_n() {
                let factor = self.get(j, i).get() / pivot_el;

                self.fold_row(j, i, -factor);
            }
        }

        self
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
        todo!()
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
    fn get_data(&self) -> &Vec<Vec<ItemCell>> {
        &self.data
    }

    fn get_dim(&self) -> Dim {
        self.dim
    }
}

impl MatrixRepr for SquareMatrix {
    fn get_data(&self) -> &Vec<Vec<ItemCell>> {
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

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(print::PRECISION);
        // TODO: no `to_string` type shit
        let width = ((self.get_max() as i32).to_string().len() + 1)
            .max((self.get_min() as i32).to_string().len())
            + precision
            + print::ITEM_X_GAP;

        for i in 0..self.dim.get_m() {
            for j in 0..self.dim.get_n() {
                write!(f, "{:>width$.precision$}", self.get(i, j).get())?;

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
