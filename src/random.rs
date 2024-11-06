use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;
use rand::thread_rng;

pub struct Random<T> {
    _marker: ::core::marker::PhantomData<T>,
}

impl<T> Random<T>
where
    rand::distributions::Standard: Distribution<T>,
    T: PartialOrd + SampleUniform,
{
    pub fn get_vec(size: usize) -> Vec<T> {
        let mut rng = thread_rng();

        let mut v = Vec::<T>::new();
        for _ in 0..size {
            v.push(rng.gen());
        }

        v
    }

    pub fn get() -> T {
        random()
    }

    pub fn get_in_range(min: T, max: T) -> T {
        thread_rng().gen_range(min..max)
    }
}
