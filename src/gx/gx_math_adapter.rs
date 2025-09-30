//! GX adapter for random ranges
//! Provides usize random generation using gx::rand module.

/// Random usize in [min, max] inclusive, using gx::rand.
pub fn rand_usize_inclusive(min: usize, max: usize) -> usize {
    if min >= max {
        return min;
    }
    crate::gx::rand::random_int_range(min as i64, max as i64) as usize
}
