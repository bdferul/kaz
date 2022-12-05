pub mod macros;
//pub use crate::macros::pln;

pub fn add(left: usize, right: usize) -> usize {
    pln!();
    pln!("{left} + {right}");
    pln!("{} + {} = {}", left, right, left+right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
