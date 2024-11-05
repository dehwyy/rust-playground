use std::{cmp::Ordering, fmt::Display};

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

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Dim(u8, u8);

impl Dim {
  #[inline]
  const fn get_m(&self) -> u8 {
    self.0
  }

  #[inline]
  const fn get_n(&self) -> u8 {
    self.1
  }

  pub fn new(m: impl Into<u8>, n: impl Into<u8>) -> Self {
    Self(m.into(), n.into())
  }

  pub fn flip(&mut self) {
    std::mem::swap(&mut self.0, &mut self.1);
  }
}

#[derive(Clone)]
pub struct Matrix {
  data: Vec<Vec<u8>>,
  dim: Dim
}

impl Matrix {
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
    self.into_iter()
  }

  pub fn iter(&self) -> impl Iterator<Item = &u8> {
    self.into_iter()
  }
}

impl Matrix {
  const GAP_Y: usize = 2;
  const GAP_X: usize = 2;
  const GAP_FILL: &str = " ";
  const X_TOP_BORDER: &str = "─";
  const X_BOTTOM_BORDER: &str = "‾";
  const Y_BORDER: &str = "│";

  fn empty_matrix_data(dim: &Dim) -> Vec<Vec<u8>> {
    let mut matrix_data = Vec::with_capacity(dim.get_m().into());

    for _ in 0..matrix_data.capacity() {
      let mut v = Vec::with_capacity(dim.get_n().into());

      for _ in 0..v.capacity() {
        v.push(u8::default());
      }

      matrix_data.push(v);
    }

    matrix_data
  }

  fn get_i32(&self, i: u8, j: u8) -> i32 {
    self.data[i as usize][j as usize].into()
  }

  pub fn new(dim: Dim) -> Self {
    Self {
      data: Self::empty_matrix_data(&dim),
      dim
    }
  }

  pub fn fill_random(&mut self) {
    self.iter_mut().for_each(|v| *v = Random::get());
  }

  pub fn fill_random_in_range(&mut self, min: u8, max: u8) {
    self.iter_mut().for_each(|v| *v = Random::get_in_range(min, max));
  }

  pub fn as_echelon_form(&self) -> (Self, u8) {
    let mut raw_echelon = self.clone();

    todo!()
  }

  /// @Returns `determinant` of the matrix.
  pub fn det(&self) -> i32 {
    let lhs = self.get_i32(0, 0) * self.get_i32(1, 1);
    let rhs = self.get_i32(0, 1) * self.get_i32(1, 0);

    lhs - rhs
  }
}



impl PartialEq for Matrix {
  fn eq(&self, other: &Self) -> bool {
    if self.dim != other.dim {
      return false;
    }

    self.iter().zip(other.iter()).all(|(lhs, rhs)| *lhs == *rhs)
  }
}

impl Eq for Matrix {}

impl PartialOrd for Matrix {
  fn ge(&self, other: &Self) -> bool {
    self.det() > other.det() || self.det() == other.det()
  }

  fn gt(&self, other: &Self) -> bool {
    self.det() > other.det()
  }

  fn le(&self, other: &Self) -> bool {
    self.det() < other.det() || self.det() == other.det()
  }

  fn lt(&self, other: &Self) -> bool {
    self.det() < other.det()
  }

  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self.det(), other.det()) {
      (a, b) if a == b => Some(Ordering::Equal),
      (a, b) if a > b => Some(Ordering::Greater),
      (a, b) if a < b => Some(Ordering::Less),
      _ => None
    }

  }
}

impl Ord for Matrix {
  fn min(self, other: Self) -> Self {
    match self < other {
      true => self,
      false => other
    }
  }

  fn max(self, other: Self) -> Self {
    match self > other {
      true => self,
      false => other
    }
  }

  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }

  fn clamp(self, min: Self, max: Self) -> Self {
    match (self, min, max) {
      (a, b, _) if a < b => b,
      (a, _, c) if a > c => c,
      (a, _, _) => a
    }
  }
}

impl Display for Matrix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

    match f.alternate() {
      true => f.debug_struct("Matrix").field("data", &self.data).finish(),
      false =>  f.debug_list().entries(self.iter()).finish()
    }
  }
}

macro_rules! impl_into_iter {
    ($($s:ty, $item_type:ty, $fn:ident),*) => {
        $(impl<'a> IntoIterator for $s {
            type Item = $item_type;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.data.$fn().flatten().collect::<Vec<_>>().into_iter()
            }
        })*
    };
}

impl_into_iter!(
  Matrix, u8, into_iter,
  &'a Matrix, &'a u8, iter,
  &'a mut Matrix, &'a mut u8, iter_mut
);
