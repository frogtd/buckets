#![feature(generic_const_exprs)]

use core::mem;

// traits

pub trait Bucket {
    const BUCKET_COUNT: usize;
    type BucketType: Fundemental;
    fn split(&self) -> [Self::BucketType; Self::BUCKET_COUNT];
}

pub trait Fundemental {
    const STATES: usize;
    fn state(&self) -> usize;
}

pub trait SortWithBucket {
    fn sort_with_bucket(&mut self);
}

// impls
macro_rules! impl_bucket {
    ($($t:ty),*) => {
        $(
            impl Bucket for $t {
                const BUCKET_COUNT: usize = mem::size_of::<Self>();
                type BucketType = u8;
                fn split(&self) -> [Self::BucketType; Self::BUCKET_COUNT] {
                    self.to_ne_bytes()
                }
            }
        )*
    };
    () => {

    };
}

impl_bucket!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Fundemental for u8 {
    const STATES: usize = 256;
    fn state(&self) -> usize {
        *self as usize
    }
}

trait WeirdTrick: Sized {
    const VEC: Vec<Self>;
}

impl<T: Sized> WeirdTrick for T {
    const VEC: Vec<Self> = Vec::new();
}

impl<T: Bucket> SortWithBucket for Vec<T>
where
    [(); T::BucketType::STATES]:,
    [(); T::BUCKET_COUNT]:,
{
    fn sort_with_bucket(&mut self) {
        let mut buckets = [T::VEC; T::BucketType::STATES];
        for x in 0..T::BUCKET_COUNT {
            for y in self.drain(..) {
                let bucket = &y.split()[x];
                buckets[bucket.state()].push(y);
            }
            for bucket in buckets.iter_mut() {
                self.append(bucket);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vector = vec![1u16, 7, 3, 64444, 5, 4214, 5, 8, 9, 10000];
        let mut vector_clone = vector.clone();
        vector_clone.sort();
        vector.sort_with_bucket();

        assert_eq!(vector, vector_clone);
    }
}
