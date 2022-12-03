use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use std::fmt::Debug;

//   s   lo      hi     s*s
//  --   --      --     ---
//   0    0       0       0
//          +1      +2
//   1    1       2       1
//          +2      +4
//   2    3       6       4
//          +4      +6
//   3    7      12       9
//          +6      +8
//   4   13      20      16
//          +8     +10
//   5   21      30      25
//         +10     +12
//   6   31      42      36
//         +12     +14
//   7   43      56      49
//         +14     +16
//   8   57      72      64

// Ascending:
//  lo(n) = hi(n-1) + 1
//  hi(n) = hi(n-1) + 2*s(n)
//  Have to special-case 2^N-1

// Descending:
//  hi(n) = hi(n+1) - 2*(n+1)
//  lo(n) = hi(n-1) + 1               <===
//  Have to special-case 0

pub trait MaxValue<T> {
    const MAX: T;
}
impl MaxValue<u8>   for u8   { const MAX: u8   =   u8::MAX; }
impl MaxValue<u16>  for u16  { const MAX: u16  =  u16::MAX; }
impl MaxValue<u32>  for u32  { const MAX: u32  =  u32::MAX; }
impl MaxValue<u64>  for u64  { const MAX: u64  =  u64::MAX; }
impl MaxValue<u128> for u128 { const MAX: u128 = u128::MAX; }

/// Returns a function that calculates the integer square root of a number.
/// The returned function can very efficiently produce such a square root
/// if the input value is near the previous input value (or the init value,
/// if this is the first call).
/// ```
/// let to_isqrt = gradual_int_sqrt::closest::int_sqrt_gradually_changing_from::<u16, u8>(0);
/// let result: Vec<u8> = (0u16..10).chain((0u16..10).rev())
///     .map(to_isqrt)
///     .collect();
/// let expected: Vec<u8> = vec![
///     //1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0     // n
///     0,1,1,1,2,2,2,2,2,3,3,2,2,2,2,2,1,1,1,0     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_changing_from<T>(init: T) -> impl FnMut(T) -> T where
    T: Debug + Add<Output = T> + AddAssign + SubAssign + Copy + From<u8> + Sub<Output = T> + CheckedMul<Output = T> + Ord + MaxValue<T>,
{
    let mut sqrt: T = init; // the current square root
    let s: T = init.into();
    let mut lo: T = s * s;
    let mut hi: T =
        match s.checked_mul(s) {
            None => T::MAX,
            Some(p) => p + s
        };
    move |n: T| {
        println!("{:?}: ({:?}, {:?}, {:?})", sqrt, lo, n, hi);
        // If the current sqrt doesn't work for this n,
        // increment/decrement it until it does.
        if n > hi {
            while n > hi {
                sqrt += 1.into();
                let s: T = sqrt.into();
                lo = hi + 1.into();
                hi += s + s;
            }
        } else {
            while n < lo {
                sqrt -= 1.into();
                let s: T = sqrt.into();
                hi = lo - 1.into();
                lo =
                    if hi == 0.into() {
                        hi
                    } else {
                        hi - s - s + 1.into()
                    };
            }
        }
        sqrt
    }
}

/// Returns a function that calculates the integer square root of a number.
/// The returned function can very efficiently produce such a square root
/// if the input value is near the previous input value (or the init value,
/// if this is the first call).  This version assumes that it will be called
/// with increasing values; if you call it with a lower value, the previous
/// isqrt will be returned again.
/// ```
/// let to_isqrt = gradual_int_sqrt::closest::int_sqrt_gradually_ascending_from::<u16, u8>(0);
/// let result: Vec<u8> = (0u16..17).map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
///        0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_ascending_from<T>(init: T) -> impl FnMut(T) -> T where
    T: Debug + Add<Output = T>  + AddAssign + Copy + From<u8> + Mul<Output = T> + Ord + MaxValue<T>,
{
    let mut sqrt: T = init; // the current square root
    let s: T = init.into();
    let mut hi: T =
        if sqrt == T::MAX {
            T::MAX
        } else {
            s * s + s
        };
    move |n: T| {
        // If the current sqrt doesn't work for this n,
        // increment it until it does.
        while n > hi {
            sqrt += 1.into();
            let s: T = sqrt.into();
            hi += s + s;
        }
        sqrt
    }
}

/// Returns a function that calculates the integer square root of a number.
/// The returned function can very efficiently produce such a square root
/// if the input value is near the previous input value (or the init value,
/// if this is the first call).  This version assumes that it will be called
/// with decreasing values; if you call it with a higher value, the previous
/// isqrt will be returned again.
/// ```
/// let to_isqrt = gradual_int_sqrt::closest::int_sqrt_gradually_descending_from::<u16, u8>(5);
/// let result: Vec<u8> = (0u16..17).rev().map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0     // n
///         4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 0     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_descending_from<T>(init: T) -> impl FnMut(T) -> T where
    T: Debug + Add<Output = T> + SubAssign + Copy + From<u8> + Mul<Output = T> + Sub<Output = T> + Ord + MaxValue<T>,
{
    let mut sqrt: T = init;   // the current square root
    let s: T = init.into();
    let mut lo: T =
        if sqrt == 0.into() {
            0.into()
        } else {
            s * s - s + 1.into()
        };
    move |n: T| {
        // If the current sqrt doesn't work for this n,
        // decrement it until it does.
        while n < lo {
            //dbg!(n, lo, sqrt);
            sqrt -= 1.into();
            if sqrt == 0.into() {
                lo = 0.into();
            } else {
                let s: T = sqrt.into();
                lo -= s + s;
            }
        }
        sqrt
    }
}

///////////////////////////////////////////////////////////////////////////

#[cfg(test)]
extern crate more_asserts;

#[cfg(test)]
mod tests {

    use super::*;
    use more_asserts::*;

    #[test]
    fn test_asc_u8() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u8>(0);
        let result: Vec<u8> = (0u8..17)
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
               0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4     // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_entire_u16_asc_range_u16() {
        // Note that for "closest" we need extra width: the closest isqrt of
        // 65535 is greater than 255, and its end value is greater than 65535.
        let mut to_isqrt = int_sqrt_gradually_ascending_from::<u16>(0);
        for n in 0u16..65535 {  // For every possible u16 value ...
            let dist = |s| {
                let s = s as i64;
                (n as i64 - s*s).abs()
            };
            let s = to_isqrt(n);
            // sq(s) should be closer to n than sq(s-1) and sq(s+1)
            let d = dist(s);
            if s > 0 {
                let d1 = dist(s-1);
                assert_le!(d, d1);
            }
            let d1 = dist(s+1);
            assert_le!(d, d1);
        }
    }

    #[test]
    fn test_asc_u16() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u16>(0);
        let result: Vec<_> = (0u16..17)
            .map(to_isqrt)
            .collect();
        let expected: Vec<u16> = vec![
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
               0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4     // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scaled_asc_u16() {
        let mut to_isqrt = int_sqrt_gradually_ascending_from::<u16>(0);
        let result: Vec<_> = (0u16..17)
            .map(|n| to_isqrt(1024*n))
            .collect();
        let expected: Vec<u16> = vec![
            // 0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16    // n
               0, 32, 45, 55, 64, 72, 78, 85, 91, 96,101,106,111,115,120,124,128    // isqrt(n*1024)
            // e.g. 128/32 = 4 is sqrt(16), 124/32 is close to sqrt(15)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_desc_u8() {
        let to_isqrt = int_sqrt_gradually_descending_from::<u8>(5);
        let result: Vec<u8> = (0u8..17)
            .rev()
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            // 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0   // n
                4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 0   // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_u8() {
        let to_isqrt = int_sqrt_gradually_changing_from(0u8);
        let result: Vec<u8> = (0u8..14)
            .chain((0u8..14).rev())
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            // 0 1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0
               0,1,1,2,2,2,2,3,3,3,3,3,3,2,2,2,2,1,1,0
        ];
        assert_eq!(result, expected);
    }

    /*
    // Float types don't implement Ord.  Could make a PartialOrd version.
    #[test]
    fn test_f32_u16() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<f32, u16>(0);
        let result: Vec<u16> = (0f32..10f32).map(to_isqrt).collect();
        let expected: Vec<u16> = vec![
            //1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0
            0,1,1,1,2,2,2,2,2,3,3,2,2,2,2,2,1,1,1,0
        ];
        assert_eq!(result, expected);
    }
     */

}

