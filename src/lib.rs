//!
//! Interesting article relating to the part for trig functions:
//!
//! http://files.righto.com/calculator/sinclair_scientific_simulator.html
//!   c = c - s / 1000
//!   s = s + c / 1000
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
//! let to_isqrt = gradual_int_sqrt::floor::int_sqrt_gradually_ascending_from::<u16, u8>(0);
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
//! let mut to_isqrt = gradual_int_sqrt::floor::int_sqrt_gradually_ascending_from::<u16, u8>(0);
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

pub mod closest;
pub mod floor;

