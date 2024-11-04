use crate::random::Random;

const GAP_Y: usize = 2;
const GAP_X: usize = 2;
const GAP_FILL: &str = " ";
const X_TOP_BORDER: &str = "─";
const X_BOTTOM_BORDER: &str = "‾";
const Y_BORDER: &str = "│";

macro_rules! impl_matrix {
    ($($t:ty),+) => {
      $(impl Matrix<$t> {
        pub fn new_random(size: usize) -> Matrix<$t> {

            let mut matrix_data = Vec::new();

            for _ in 0..size {
              matrix_data.push(Random::<$t>::get_vec(size));
            }

            Matrix {
              data: matrix_data,
              size: (size, size)
            }
        }

        pub fn print_pretty(&self) {
          let max_el_size= self.get_max_element_size();
          let matrix_print_size_no_borders = self.size.0 * (max_el_size + GAP_X) + GAP_X;
          let matrix_print_size = matrix_print_size_no_borders + 2;

          println!("{}", X_TOP_BORDER.repeat(matrix_print_size));

          self.for_each_el(
            |el| {
              print!("{el: >width$}{gap}", width = self.get_max_element_size(), gap=GAP_FILL.repeat(GAP_X));
            },
            || {
              print!("{}", Y_BORDER);
              print!("{}", GAP_FILL.repeat(GAP_X));
            },
            ||  {
              println!("{}", Y_BORDER);
              for _ in 0..(GAP_Y - 1) {
                println!("{Y_BORDER}{}{Y_BORDER}", GAP_FILL.repeat(matrix_print_size_no_borders));
              }
            }
          );

          println!("{}", X_BOTTOM_BORDER.repeat(matrix_print_size));
        }

        fn for_each_el(
          &self, mut f:
          impl FnMut($t),
          line_start_f: impl Fn(),
          line_end_f: impl Fn()
        )
        {
          for i in 0..self.data.len() {
            line_start_f();

            for j in 0..self.data.len() {
              f(self.data[i][j]);
            }

            line_end_f();

          }
        }

        fn get_max_element_size(&self) -> usize {
          let mut max = 0;

          self.for_each_el(
            |el| {
              if el.to_string().len() > max {
                max = el.to_string().len();
              }
            },
            || {},
            || {}
          );

          max
        }
      })*

    };
}

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    size: (usize, usize),
}

impl_matrix!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
