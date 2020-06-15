use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use std::fmt::Debug;

/// Returns a function that calculates the integer square root of a number.
/// The returned function can very efficiently produce such a square root
/// if the input value is near the previous input value (or the init value,
/// if this is the first call).
/// ```
/// let to_isqrt = gradual_int_sqrt::floor::int_sqrt_gradually_changing_from::<u16, u8>(0);
/// let result: Vec<u8> = (0u16..10).chain((0u16..10).rev())
///     .map(to_isqrt)
///     .collect();
/// let expected: Vec<u8> = vec![
///     //1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0     // n
///     0,1,1,1,2,2,2,2,2,3,3,2,2,2,2,2,1,1,1,0     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_changing_from<Num, Sqrt>(init: Sqrt) -> impl FnMut(Num) -> Sqrt where
    Num:  Debug + Add<Output = Num>  + AddAssign + SubAssign + Copy + From<u8> + Sub<Output = Num> + Mul<Output = Num> + Ord,
    Sqrt: Debug + Add<Output = Sqrt> + AddAssign + SubAssign + Copy + From<u8> + Into<Num>
{
    let mut sqrt: Sqrt = init; // the current square root
    let s: Num = init.into();
    let mut lo: Num = s * s;
    let mut hi: Num = lo + (s * 2.into());   // (s + 1)^2 - 1 without overflowing
    move |n: Num| {
        // If the current sqrt doesn't work for this n,
        // increment it until it does.
        if n > hi {
            while n > hi {
                sqrt += 1.into();
                let s: Num = sqrt.into();
                lo = hi + 1.into();
                hi = lo + s + s;
            }
        } else {
            while n < lo {
                sqrt -= 1.into();
                let s: Num = sqrt.into();
                hi = lo - 1.into();
                lo = hi - s - s;
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
/// let to_isqrt = gradual_int_sqrt::floor::int_sqrt_gradually_ascending_from::<u16, u8>(0);
/// let result: Vec<u8> = (0u16..17).map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
///        0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_ascending_from<Num, Sqrt>(init: Sqrt) -> impl FnMut(Num) -> Sqrt where
    Num:  Debug + Add<Output = Num>  + AddAssign + Copy + From<u8> + Mul<Output = Num> + Ord,
    Sqrt: Debug + Add<Output = Sqrt> + AddAssign + Copy + From<u8> + Into<Num>
{
    let mut sqrt: Sqrt = init; // the current square root
    let s: Num = init.into();
    let mut hi: Num = s * (s + 2.into());   // (s + 1)^2 - 1 without overflowing
    move |n: Num| {
        // If the current sqrt doesn't work for this n,
        // increment it until it does.
        while n > hi {
            sqrt += 1.into();
            let s: Num = sqrt.into();
            hi += s + s + 1.into();
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
/// let to_isqrt = gradual_int_sqrt::floor::int_sqrt_gradually_descending_from::<u16, u8>(5);
/// let result: Vec<u8> = (0u16..17).rev().map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0     // n
///         4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1, 1, 0     // isqrt(n)
/// ];
/// assert_eq!(result, expected);
/// ```
pub fn int_sqrt_gradually_descending_from<Num, Sqrt>(init: Sqrt) -> impl FnMut(Num) -> Sqrt where
    Num:  Debug + Add<Output = Num>  + SubAssign + Copy + From<u8> + Mul<Output = Num> + Ord,
    Sqrt: Debug + Add<Output = Sqrt> + SubAssign + Copy + From<u8> + Into<Num>
{
    let mut sqrt: Sqrt = init;   // the current square root
    let s: Num = init.into();
    let mut lo: Num = s * s;
    move |n: Num| {
        // If the current sqrt doesn't work for this n,
        // decrement it until it does.
        while n < lo {
            sqrt -= 1.into();
            let s: Num = sqrt.into();
            lo -= s + s + 1.into();
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
    fn test_asc_u16_u8() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u16, u8>(0);
        let result: Vec<u8> = (0u16..17)
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
               0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4     // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_entire_asc_range_u16_u8() {
        let mut to_isqrt = int_sqrt_gradually_ascending_from::<u16, u8>(0);
        for n in 0u16..65535 {  // For every possible u16 value ...
            let s = to_isqrt(n);
            let t = s as u32;
            let n = n as u32;
            assert_le!(t*t, n);         // the isqrt is not too high:  isqrt(n)^2 <= n
            let t = t + 1;
            assert_gt!(t*t, n);         // the isqrt is high enough:  (isqrt(n) + 1)^2 > n
        }
    }

    #[test]
    fn test_asc_u16_u16() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u16, u16>(0);
        let result: Vec<_> = (0u16..17)
            .map(to_isqrt)
            .collect();
        let expected: Vec<u16> = vec![
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
               0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4     // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_asc_u32_u16() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u32, u16>(0);
        let result: Vec<_> = (0u32..17)
            .map(to_isqrt)
            .collect();
        let expected: Vec<u16> = vec![
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16     // n
               0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4     // isqrt(n)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_asc_i32_i16() {
        // Overflow is possible with i32_i16, but not with these numbers.
        let to_isqrt = int_sqrt_gradually_ascending_from::<i32, i16>(0);
        let result: Vec<_> = (0i32..10)
            .map(to_isqrt)
            .collect();
        let expected: Vec<i16> = vec![
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scaled_asc_u32_u16() {
        let mut to_isqrt = int_sqrt_gradually_ascending_from::<u32, u16>(0);
        let result: Vec<_> = (0u32..10)
            .map(|n| to_isqrt(1024*n))
            .collect();
        let expected: Vec<u16> = vec![
            0, 32, 45, 55, 64, 71, 78, 84, 90, 96
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_desc_u16_u8() {
        let to_isqrt = int_sqrt_gradually_descending_from::<u16, u8>(5);
        let result: Vec<u8> = (0u16..10)
            .rev()
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            3, 2, 2, 2, 2, 2, 1, 1, 1, 0,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_u16_u8() {
        let to_isqrt = int_sqrt_gradually_changing_from::<u16, u8>(0);
        let result: Vec<u8> = (0u16..10)
            .chain((0u16..10).rev())
            .map(to_isqrt)
            .collect();
        let expected: Vec<u8> = vec![
            //1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0
            0,1,1,1,2,2,2,2,2,3,3,2,2,2,2,2,1,1,1,0
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

