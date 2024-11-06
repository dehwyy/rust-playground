use std::{
    cell::Cell,
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use super::{Matrix, MatrixColumn, MatrixItem, MatrixRow};

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
            _ => None,
        }
    }
}

impl Ord for Matrix {
    fn min(self, other: Self) -> Self {
        match self < other {
            true => self,
            false => other,
        }
    }

    fn max(self, other: Self) -> Self {
        match self > other {
            true => self,
            false => other,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        match (self, min, max) {
            (a, b, _) if a < b => b,
            (a, _, c) if a > c => c,
            (a, _, _) => a,
        }
    }
}

macro_rules! impl_into_iter {
    ($($t:ty, $item_type:ty, $fn:ident),*) => {
        $(impl<'a> IntoIterator for $t {
            type Item = $item_type;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.data.$fn().flatten().collect::<Vec<_>>().into_iter()
            }
        })*
    };

    ($t:ty) => {
      impl_into_iter!($t, Cell<MatrixItem>, into_iter);
      impl_into_iter!(&'a $t, &'a Cell<MatrixItem>, iter);
      impl_into_iter!(&'a mut $t, &'a mut Cell<MatrixItem>, iter_mut);
    };
}

macro_rules! impl_matrix_space_ops {
  ($trait:ident, $traitfn:ident, $op:tt, $impl_output:ident, $impl_for:ty) => {
          impl $trait for $impl_for {
              type Output = $impl_output;
              fn $traitfn(self, rhs: Self) -> Self::Output {
                  assert_eq!(self.len(), rhs.len());

                  $impl_output(self.iter().zip(rhs.iter()).map(|(a, b)| a $op b).collect())
              }
          }
  };
  (impl_op $trait:ident, $traitfn:ident, $op:tt, $s:ident) => {
    impl_matrix_space_ops!($trait, $traitfn, $op, $s, $s);
    impl_matrix_space_ops!($trait, $traitfn, $op, $s, &$s);
    impl_matrix_space_ops!($trait, $traitfn, $op, $s, &mut $s);
  };

  (add $($s:ident),*) => {
    $(impl_matrix_space_ops!(impl_op Add, add, +, $s);)*
  };
  (sub $($s:ident),*) => { $(impl_matrix_space_ops!(impl_op Sub, sub, -, $s);)* };
  (mul $($s:ident),*) => { $(impl_matrix_space_ops!(impl_op Mul, mul, *, $s);)* };
  (div $($s:ident),*) => { $(impl_matrix_space_ops!(impl_op Div, div, /, $s);)* };

  (all $($s:ident),*) => {
    $(impl_matrix_space_ops!(add $s);)*
    $(impl_matrix_space_ops!(sub $s);)*
    $(impl_matrix_space_ops!(mul $s);)*
    $(impl_matrix_space_ops!(div $s);)*
   }
}

impl_into_iter!(Matrix);
// impl_matrix_space_ops!(all MatrixColumn, MatrixRow);
