use super::lib::*;
use crate::Chess;

impl Chess {
    /// Returns the FEN char interpretation of the u8 piece value
    fn fencode(a: u8) -> Option<char> {
        if a >= 12 {
            return None;
        }

        let codes = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p'];

        Some(codes[a as usize])
    }

    /// Returns the internal value of a piece based on the provided FEN char
    fn defencode(c: char) -> Option<usize> {
        let codes = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p'];

        codes.iter().position(|x| *x == c)
    }

    /// Returns a FEN formatted index of a square on a board (ie. "e3")
    fn fen_pos(a: usize) -> String {
        let (x, y) = fndx(a);

        format!("{}{}", (('a' as u8) + x as u8) as char, 8 - y)
    }

    // Returns the usize parsed from the FEN formatted string input (ie. "e3")
    fn from_fen_pos(s: &String) -> Result<usize, &'static str> {
        if s.len() != 2 {
            return Err("invalid len");
        }

        let mut x = s.chars().nth(0).unwrap() as usize;
        let h = 'h' as usize;
        let a = 'a' as usize;
        if x <= h && x >= a {
            x = h - x;
        } else {
            return Err("invalid x value");
        }

        let Some(y) = s.chars().nth(1).unwrap().to_digit(10) else {
            return Err("invalid y value");
        };

        Ok(ndx(x, y as usize))
    }

    /// Returns a url friendly FEN notation with no '/' separations and '.' as the whitespace
    pub fn to_fen_url(&self) -> String {
        self.to_fen()
            .chars()
            .map(|c| if c == ' ' { '.' } else { c })
            .filter(|c| *c != '/')
            .collect()
    }

    /// Returns only the first part of a FEN string
    ///
    /// default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    pub fn to_fen_pieces(&self) -> String {
        let mut r = String::new();
        for y in 0..8 {
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

            r = format!(
                "{r}{}/",
                if cnt > 0 {
                    cnt.to_string()
                } else {
                    String::new()
                }
            );
            if y == 7 {
                r.pop();
            }
        }

        r
    }

    /// Returns the game's FEN notation
    pub fn to_fen(&self) -> String {
        let mut r = self.to_fen_pieces();

        let t = if self.turn { 'w' } else { 'b' };

        let e = if let Some(ep) = self.en_passant {
            Self::fen_pos(ep)
        } else {
            "-".to_string()
        };

        let mut castle = String::new();

        "KQkq".chars().enumerate().for_each(|(i, c)| {
            if self.castle[i] {
                castle = format!("{castle}{c}")
            }
        });
        if castle == String::new() {
            castle = "-".to_string();
        }

        r = format!("{r} {t} {castle} {e} {} {}", self.halfmoves, self.fullmoves);

        r
    }

    /// Returns a vector inferred by the first part of a FEN string
    pub fn from_fen_pieces(s: &String) -> Result<Vec<u8>, &'static str> {
        let mut board = vec![];
        for c in s.chars() {
            if board.len() < 64 {
                let e = Err("invalid piece sequece");
                if let Some(d) = c.to_digit(10) {
                    if board.len() + d as usize > 64 {
                        return e;
                    }
                    for _ in 0..d {
                        board.push(12);
                    }
                    continue;
                }

                if c == '/' {
                    if board.len() % 8 == 0 {
                        continue;
                    } else {
                        return Err("improper use of '/'");
                    }
                }

                if let Some(p) = Self::defencode(c) {
                    board.push(p as u8);
                } else {
                    return e;
                }
            }
        }

        Ok(board)
    }

    /// Returns chess struct derrived from the parsed FEN string
    pub fn from_fen(fen: String) -> Result<Chess, &'static str> {
        let items: Vec<String> = fen.split(&[' ', '.'][..]).map(|s| s.to_string()).collect();

        let ffp = Chess::from_fen_pieces(&items[0]);
        let Ok(board) = ffp else {
            return Err(ffp.err().unwrap());
        };

        let mut r = Self::default();

        //turn
        if items.len() > 1 {
            r.turn = match &items[1][..] {
                "w" => true,
                "b" => false,
                _ => return Err("unable to parse turn"),
            }
        }

        //castle
        if items.len() > 2 {
            let mut a = [false; 4];
            if items[3] != "-" {
                "KQkq".chars().enumerate().for_each(|(i, c)| {
                    if items[3].contains(c) {
                        a[i] = true;
                    }
                })
            }
            r.castle = a;
        }

        //en passant
        if items.len() > 3 {
            if items[2] != "-" {
                let Ok(a) = Chess::from_fen_pos(&items[2]) else {
                    return Err("unable to parse en passant")
                };
                r.en_passant = Some(a);
            }
        }

        //halfmoves
        if items.len() > 4 {
            let Ok(a) = items[4].parse::<u32>() else {
                return Err("unable to parse halfmoves")
            };
            println!("{items:?}, {a}");
            r.halfmoves = a;
        }

        //fullmoves
        if items.len() > 5 {
            let Ok(a) = items[5].parse::<u32>() else {
                return Err("unable to parse fullmoves")
            };
            r.fullmoves = a;
        }

        r.board = board;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::Chess;

    #[test]
    fn fen_from_default() {
        let d = Chess::default();
        assert_eq!(
            d.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        )
    }

    #[test]
    fn fen_from_other() {
        let s = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50".to_string();
        let ch = Chess::from_fen(s.clone()).unwrap();

        assert_eq!(ch.to_fen(), s);
    }
}