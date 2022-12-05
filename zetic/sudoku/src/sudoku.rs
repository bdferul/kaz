pub const TYLER: &'static str = "Tyler";

pub struct Sudoku {
    pub puzzle: Vec<u8>,
    pub size: usize,
}

impl Sudoku {
    pub fn new(size: usize) -> Self {
        Self {
            puzzle: vec![0;(size*size)],
            size
        }
    }

    pub fn ndx(&self, x: usize, y: usize) -> u8 {
        self.puzzle[x + (y * self.size)]
    }

    pub fn is_valid(&self) -> bool {
        if self.puzzle.len() != self.size.pow(2) {
            return false
        }
        let test: Vec<u8> = (1..=self.size as u8).collect();
        
        //horizontal
        for y in 0..self.size {
            let mut row = vec![];
            for x in 0..self.size {
                let p = self.ndx(x, y);
                if !test.contains(&p) {
                    return false
                }
                row.push(p);
            }
            for t in test.iter() {
                if !row.contains(t) {
                    return false
                }
            }
        }

        // vertical
        for x in 0..self.size {
            let mut row = vec![];
            for y in 0..self.size {
                let p = self.ndx(x, y);
                if !test.contains(&p) {
                    return false
                }
                row.push(p);
            }
            for t in test.iter() {
                if !row.contains(t) {
                    return false
                }
            }
        }
        true
    }
}