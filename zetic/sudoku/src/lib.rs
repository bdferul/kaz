pub mod sudoku;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tyler() {
        assert_eq!(sudoku::TYLER, "Tyler");
    }

    #[test]
    fn validity() {
        let mut s = sudoku::Sudoku {
            puzzle: vec![
                1,2,3,
                2,3,1,
                3,1,2,
            ],
            size: 3,
        };

        assert!(s.is_valid());
        s.puzzle[0] = 3;
        assert!(!s.is_valid())
    }

    #[test]
    fn three_by_three() {
        todo!()
    }
}
