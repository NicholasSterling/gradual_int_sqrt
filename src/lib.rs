#![no_std]

//!
//! This crate contains functions that generate functions that calculate
//! the integer square root (hereafter "isqrt") of a number.
//! For example, the integer square root of 30 is 5, since 5^2 = 25 and
//! 6^2 = 36.
//! These generated functions are very efficient at processing gradually
//! changing sequences of numbers.
//! They achieve this efficiency by remembering the previous square root
//! calculated and making use of that in calculating the next square root.
//!
//! As a trivial example, if the last value processed was 133, then the
//! algorithm has figured out that the isqrt was 11, and that that value
//! is good up to 143.  So if the next invocation asks for the isqrt of
//! 136, it simply returns the previous answer of 11 again.  For a value
//! of 145 it would recognize that 11 is too low, add 2*12 + 1 to the
//! previous end value of 143 to get 168, and see that 12 is the isqrt.
//! Multiplying by 2 is just a trivial shift left, so as long as the
//! current value is not very far from the previous value, its isqrt
//! costs very little to produce.  If, however, the next invocation
//! asks for the isqrt of 1,000,293, then it will take many iterations
//! to reach the correct isqrt value of 1000.
//!
//! Here is an example involving an ascending sequence:
//! ```
//! let to_isqrt = gradual_int_sqrt::int_sqrt_gradually_ascending_from::<u16, u8>(0);
//! let result: Vec<u8> = (0u16..17).map(to_isqrt).collect();
//! let expected: Vec<u8> = vec![
//!     // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16    // n
//!        0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4    // isqrt(n)
//! ];
//! assert_eq!(result, expected);
//! ```
//!
//! Note that the input type and the isqrt type are separate.  Care is taken
//! to ensure that the algorithm works without overflow for unsigned integers
//! as long as the type of the isqrt is at least half as wide as the type of
//! the inputs.  For example, if the input type is u16, then the output type
//! can be u8 with no possibility of overflow.
//!
//! Generally speaking, this crate should only be used when the following
//! are all true:
//!  1. You have a lot of values, perhaps a continuous stream of them,
//!     for which you need some measure of the square root.
//!  3. Either you really want the integer square root, or you want the
//!     square root but don't need much precision.
//!  2. The input values do not jump around wildly.
//!
//! A good example of where this crate could help is in processing sensor
//! data in embedded systems that do not have floating-point units or even
//! fast integer division circuitry.
//! If you were to take, say, 32 or 64 samples
//! per second from an accelerometer or gyroscope in a hand-held device,
//! the values would rise and fall within some range as the sensors tracked
//! your movements, but would not jump all over the place.
//! You could use isqrt(x<sup>2</sup> + y<sup>2</sup>) as a rough measure
//! of the magnitude of the acceleration in the XY plane (for example).
//!
//! It is possible to increase precision by scaling the input values,
//! usually by a power of 2 to make it faster.  For example, multiplying
//! incoming values by 64 improves the resolution of the function 8X.
//! This will have some performance cost, since the function will have
//! to search more to find the isqrt, especially near 0.
//!
//! ```
//! let mut to_isqrt = gradual_int_sqrt::int_sqrt_gradually_ascending_from::<u16, u8>(0);
//! let result: Vec<u8> = (0u16..17).map(|n| to_isqrt(64*n)).collect();
//! let expected: Vec<u8> = vec![
//!     // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16    // n
//!     //                                             ==       // e.g. n = 15
//!     // 0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4    // isqrt(15) = 3
//!        0, 8,11,13,16,17,19,21,22,24,25,26,27,28,29,30,32    // isqrt(15*64) = 30
//!     // 30/8 = 3.75 is much closer to sqrt(15) than 3 is
//! ];
//! assert_eq!(result, expected);
//! ```
//!
//! There are several versions of the algorithm, allowing for extra
//! efficiency in situations where you know that the values are sorted
//! (or at least are not so unsorted that you would care).
//!
//! In cases where you know that a big jump has occurred (e.g. in
//! a reset function), you can simply regenerate the isqrt function
//! with an appropriate initial isqrt value.
//!

use core::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use core::fmt::Debug;

/// Returns a function that calculates the integer square root of a number.
/// The returned function can very efficiently produce such a square root
/// if the input value is near the previous input value (or the init value,
/// if this is the first call).
/// ```
/// let to_isqrt = gradual_int_sqrt::int_sqrt_gradually_changing_from::<u16, u8>(0);
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
    let mut hi: Num = lo + s + s;   // (s + 1)^2 - 1 without overflowing
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
/// let to_isqrt = gradual_int_sqrt::int_sqrt_gradually_ascending_from::<u16, u8>(0);
/// let result: Vec<u8> = (0u16..10).map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 0  1  2  3  4  5  6  7  8  9     // n
///        0, 1, 1, 1, 2, 2, 2, 2, 2, 3     // isqrt(n)
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
/// let to_isqrt = gradual_int_sqrt::int_sqrt_gradually_descending_from::<u16, u8>(5);
/// let result: Vec<u8> = (0u16..10).rev().map(to_isqrt).collect();
/// let expected: Vec<u8> = vec![
///     // 9  8  7  6  5  4  3  2  1  0     // n
///        3, 2, 2, 2, 2, 2, 1, 1, 1, 0     // isqrt(n)
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
    use arrayvec::ArrayVec;

    #[test]
    fn test_asc_u16_u8() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u16, u8>(0);
        let result: ArrayVec<[u8; 10]> = (0u16..10)
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            // 1  2  3  4  5  6  7  8  9
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3
        ]);
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
        let result: ArrayVec<[u16; 10]> = (0u16..10)
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_asc_u32_u16() {
        let to_isqrt = int_sqrt_gradually_ascending_from::<u32, u16>(0);
        let result: ArrayVec<[u16; 10]> = (0u32..10)
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_asc_i32_i16() {
        // Overflow is possible with i32_i16, but not with these numbers.
        let to_isqrt = int_sqrt_gradually_ascending_from::<i32, i16>(0);
        let result: ArrayVec<[i16; 10]> = (0i32..10)
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scaled_asc_u32_u16() {
        let mut to_isqrt = int_sqrt_gradually_ascending_from::<u32, u16>(0);
        //let result: ArrayVec<[u8; 10]> = (1000u32..1010)
        let result: ArrayVec<[u16; 10]> = (0u32..10)
            //.map(|n| to_isqrt(  64*n))
              .map(|n| to_isqrt( 256*n))
            //.map(|n| to_isqrt(1024*n))
            //.map(|n| to_isqrt(4096*n))
            .collect();
        let expected = ArrayVec::from([
            //         |                   |
            //  0    1    2    3    4    5    6    7    8    9
            //  0,   8,  11,  13,  16,  17,  19,  21,  22,  24    // isqrt(  64*n)
                0,  16,  22,  27,  32,  35,  39,  42,  45,  48    // isqrt( 256*n)
            //  0,  32,  45,  55,  64,  71,  78,  84,  90,  96    // isqrt(1024*n)
            //  0,  64,  90, 110, 128, 143, 156, 169, 181, 192    // isqrt(4096*n)
            //         |                   |
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_desc_u16_u8() {
        let to_isqrt = int_sqrt_gradually_descending_from::<u16, u8>(5);
        let result: ArrayVec<[u8; 10]> = (0u16..10)
            .rev()
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            3, 2, 2, 2, 2, 2, 1, 1, 1, 0,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_u16_u8() {
        let to_isqrt = int_sqrt_gradually_changing_from::<u16, u8>(0);
        let result: ArrayVec<[u8; 20]> = (0u16..10)
            .chain((0u16..10).rev())
            .map(to_isqrt)
            .collect();
        let expected = ArrayVec::from([
            //1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0
            0,1,1,1,2,2,2,2,2,3,3,2,2,2,2,2,1,1,1,0
        ]);
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

