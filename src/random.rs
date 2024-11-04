use rand::thread_rng;
use rand::prelude::*;

pub struct Random<T> {
  _marker: ::core::marker::PhantomData<T>
}

impl<T> Random<T>
where rand::distributions::Standard: Distribution<T> {
  pub fn get_vec(size: usize) -> Vec<T> {
    let mut rng = thread_rng();

    let mut v = Vec::<T>::new();
    for _ in 0..size {
      v.push(rng.gen());
    }

    v
  }
}
