
pub struct Chess {
    pub board: Vec<u8>,
}

impl Chess {
    /**
    Each piece value corresponds to the unicode position of its symbol
     
    White
     * 0: ♔
     * 1: ♕
     * 2: ♖
     * 3: ♗
     * 4: ♘
     * 5: ♙
     
    Black
     * 6: ♚
     * 7: ♛
     * 8: ♜
     * 9: ♝
     * 10: ♞
     * 11: ♟︎
     */
    pub fn to_symbol(a: u8) -> char {
        if a < 12 {
            char::from_u32(0x2654+a as u32).unwrap_or_default()
        } else {
            char::default()
        }
    }

    /**
    ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖

    ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
        
    ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 

    ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
     */
    pub fn pretty_print(&self) {
        let ts = Self::to_symbol;
        for y in 0..8 {
            for x in 0..8 {
                print!("{} ", ts(self.board[ndx(x, y)]));
            }
            println!();
        }
    }
}

impl Default for Chess {
    fn default() -> Self {
        Chess { 
            board: vec![
                02,04,03,01,00,03,04,02,
                05,05,05,05,05,05,05,05,
                12,12,12,12,12,12,12,12,
                12,12,12,12,12,12,12,12,
                12,12,12,12,12,12,12,12,
                12,12,12,12,12,12,12,12,
                11,11,11,11,11,11,11,11,
                08,10,09,07,06,09,10,08,
            ],
        }
    }
}

fn ndx(x: usize, y: usize) -> usize {
    x + (y * 8)
}