//! # My Crate
//!
//! Do a very complex calculation

/// add one to the given number
///
/// # examples
///
/// ```
/// let arg = 41;
/// let answer = tcy_my_crate::add_one(arg);
///
/// assert_eq!(42, answer);
/// ```
///
pub fn add_one(nb: i32) -> i32 {
    nb + 1
}
