//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient. 
/// Adds one to the number given.
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_one_works() {
        let arg = 5;
        let answer = add_one(arg);

        assert_eq!(6, answer);
    }
}
