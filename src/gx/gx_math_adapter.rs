//! GX adapter over math (random ranges)
//! Consumer-owned adapter to utilize math::random without coupling macros.

/// Random usize in [min, max] inclusive, using math::random.
pub fn rand_usize_inclusive(min: usize, max: usize) -> usize {
    if min >= max { return min; }
    crate::math::random::random_int_range(min as i64, max as i64) as usize
}

