pub struct Chess {
    pub board: Vec<u8>,
    /// true for white, false for black
    pub turn: bool,
    pub en_passant: Option<usize>,
    pub castle: [bool;4],
    pub halfmoves: u32,
    pub fullmoves: u32,
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
            char::from_u32(0x2654 + a as u32).unwrap_or_default()
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

    /// Returns the FEN char interpretation of the u8 piece value
    fn fencode(a: u8) -> Option<char> {
        if a >= 12 {
            return None;
        }

        let codes = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p'];

        Some(codes[a as usize])
    }

    fn fen_pos(a: usize) -> String {
        let (x,y) = fndx(a);

        format!("{}{}", (('a' as u8) + x as u8) as char, y+1)
    }

    /**
    Returns a url friendly FEN notation with no '/' separations

    Whitespace is replaced with the 'whitespace' var
    
    Replace whitespace with nothing by passing '/' as 'whitespace'
     */
    pub fn to_fen_url(&self, whitespace: char) -> String {
        self.to_fen().chars().map(|c| if c == ' ' {whitespace} else {c}).filter(|c| *c != '/').collect()
    }

    /// Returns the game's FEN notation
    pub fn to_fen(&self) -> String {
        let mut r = String::new();
        for y in (0..8).rev() {
            let mut cnt = 0;
            for x in 0..8 {
                if let Some(f) = Self::fencode(self.sqr(x, y)) {
                    r = format!(
                        "{r}{}{}",
                        if cnt > 0 {
                            cnt.to_string()
                        } else {
                            "".to_string()
                        },
                        f
                    );
                    cnt = 0;
                } else {
                    cnt += 1;
                }
            }
            if y > 0 {
                r = format!(
                    "{r}{}/",
                    if cnt > 0 {
                        cnt.to_string()
                    } else {
                        String::new()
                    }
                )
            }
        }

        let t = if self.turn {'w'} else {'b'};

        let e = if let Some(ep) = self.en_passant {
            Self::fen_pos(ep)
        } else {
            "-".to_string()
        };

        let mut castle = String::new();

        "KQkq".chars().enumerate().for_each(|(i,c)| if self.castle[i] {castle = format!("{castle}{c}")});
        if castle == String::new() {
            castle = "-".to_string();
        }

        r = format!("{r} {t} {e} {castle} {} {}", self.halfmoves, self.fullmoves);

        r
    }

    pub fn sqr(&self, x: usize, y: usize) -> u8 {
        self.board[ndx(x, y)]
    }
}

impl Default for Chess {
    /// Returns the standard chess board with white on top
    #[rustfmt::skip]
    fn default() -> Self {
        Chess {
            board: vec![
                02, 04, 03, 01, 00, 03, 04, 02,
                05, 05, 05, 05, 05, 05, 05, 05,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                11, 11, 11, 11, 11, 11, 11, 11,
                08, 10, 09, 07, 06, 09, 10, 08,
            ],
            turn: true,
            en_passant: Some(ndx(4,2)),
            castle: [true;4],
            halfmoves: 0,
            fullmoves: 0,
        }
    }
}

fn ndx(x: usize, y: usize) -> usize {
    x + (y * 8)
}

fn fndx(p: usize) -> (usize,usize){
    (p%8,p/8)
}