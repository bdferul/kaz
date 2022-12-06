use super::piece::{Class::*, Piece, Side::*};
use crate::{
    chess::{fndx, Chess},
    mdx,
};

impl Chess {
    pub fn to_std_notation(&self, src: usize, dst: usize) -> Option<String> {
        let Some(src_p) = self.board[src] else {
            return None
        };

        if src_p.class == King {
            match src {
                4 | 60 => {
                    if dst == src + 2 {
                        return Some("0-0".to_string());
                    }
                    if dst == src - 2 {
                        return Some("0-0-0".to_string());
                    }
                }
                _ => (),
            }
        }

        let src_str_chars: Vec<char> = Chess::fen_pos(src).unwrap().chars().collect();
        let mut src_str_x = String::new();
        let mut src_str_y = String::new();

        let same_class: Vec<(usize, usize)> = self
            .board
            .iter()
            .enumerate()
            .filter(|(i, x)| {
                let Some(p) = x else {
                    return false;
                };

                *p == src_p && *i != src
            })
            .map(|(i, _)| i)
            .filter(|i| self.choices(*i).contains(&dst))
            .map(|i| fndx(i))
            .collect();

        let (src_x, src_y) = fndx(src);
        for (x, y) in same_class {
            if x == src_x {
                src_str_x = src_str_chars[0].to_string();
            }
            if y == src_y {
                src_str_y = src_str_chars[1].to_string()
            }
        }

        let p_str = src_p.std_notation_piece();
        let cap = if self.board[dst].is_some() { "x" } else { "" };
        let dst_str = Chess::fen_pos(dst).unwrap_or(String::new());

        Some(format!("{p_str}{src_str_x}{src_str_y}{cap}{dst_str}"))
    }

    pub fn all_choices_note(&self) -> Vec<String> {
        let choices: Vec<(usize, Vec<usize>)> = (0..self.board.len())
            .map(|i| (i, self.choices(i)))
            .filter(|(_, x)| x.len() > 0)
            .collect();

        let mut r = vec![];
        for (src, dsts) in choices.into_iter() {
            r.extend(
                dsts.into_iter()
                    .map(|dst| self.to_std_notation(src, dst).unwrap())
                    .collect::<Vec<String>>(),
            );
        }

        r
    }

    pub fn mv_str(&mut self, note: String) -> Result<(), ()> {
        let res = self.from_std_notation(note);
        println!("{:?}", res);
        let Ok((src,dst)) = res else {
            return Err(())
        };
        self.mv(src, dst)
    }

    /// Returns in the form of `Result<(src, dst)>`
    pub fn from_std_notation(&self, note: String) -> Result<(usize, usize), String> {
        let note = note.trim();

        let castle_src = if self.turn == White {60} else {4};
        match note {
            "0-0"|"O-O" => return Ok((castle_src, castle_src + 2)),
            "0-0-0"|"O-O-O" => return Ok((castle_src, castle_src - 2)),
            _ => (),
        }

        if note.len() < 2 {
            return Err(String::from("Input string is not long enough"));
        }

        let note_chars = note
            .chars()
            .filter(|c| *c != 'x')
            .rev()
            .collect::<Vec<char>>();

        // start dst

        let dst_y = note_chars[0];
        if !('1'..='8').any(|c| c == dst_y) {
            return Err(format!(
                "Final character '{}' is not valid. Must be a number in '1..=8'",
                dst_y
            ));
        };
        let dst_y = 8 - note_chars[0].to_digit(9).unwrap();
        let dst_x = note_chars[1];
        if !('a'..='h').any(|c| c == dst_x) {
            return Err(String::new());
        }
        let dst_x = ('a'..='h').position(|c| c == dst_x).unwrap();

        let dst = mdx!(dst_x, dst_y);

        // end dst
        // start src

        let mut src_class = Pawn;
        let mut src_x = None;
        let mut src_y = None;
        let patterns = ["y", "yx", "yc", "x", "xc", "c"];

        if note_chars.len() > 2 {
            let tail = note_chars[2..]
                .iter()
                .map(|c| match *c {
                    '1'..='8' => 'y',
                    'a'..='h' => 'x',
                    'K' | 'Q' | 'B' | 'N' | 'R' => 'c',
                    _ => 'b', //for bad
                })
                .collect::<String>();
            if patterns.contains(&&tail[..]) {
                if let Some(c) = tail.chars().position(|c| c == 'c') {
                    src_class = match note_chars[2 + c] {
                        'K' => King,
                        'Q' => Queen,
                        'B' => Bishop,
                        'N' => Knight,
                        'R' => Rook,
                        _ => Pawn,
                    };
                }
                if let Some(x) = tail.chars().position(|c| c == 'x') {
                    src_x = Some(('a'..='h').position(|c| c == note_chars[2 + x]).unwrap());
                }
                if let Some(y) = tail.chars().position(|c| c == 'y') {
                    src_y = Some(note_chars[2 + y].to_digit(9).unwrap() as usize);
                }
            } else if tail != "" {
                return Err(format!(
                    "The first character(s) {:?} are invalid as a sequence",
                    tail.chars().rev().collect::<Vec<char>>()
                ));
            }
        }

        let srcs = if src_x.is_some() && src_y.is_some() {
            vec![mdx!(src_x.unwrap(), src_y.unwrap())]
        } else {
            let [x_range, y_range] = [src_x, src_y].map(|λ| match λ {
                Some(x) => vec![x],
                None => (0..8).collect(),
            });

            let mut srcs = vec![];

            for y in y_range.iter() {
                for x in x_range.iter() {
                    if let Some(piece) = self.board[mdx!(*x, *y)] {
                        if piece.side == self.turn && piece.class == src_class {
                            srcs.push(mdx!(x, y));
                        }
                    }
                }
            }

            srcs
        };

        println!("srcs: {:?}", (&srcs, &src_x, &src_y, src_class));
        println!("self.choices(6).0: {:?}", (self.choices(6), dst));

        let valid = srcs
            .into_iter()
            .filter(|x| {
                self.choices(*x).contains(&dst) && self.board[*x].unwrap().class == src_class
            })
            .collect::<Vec<usize>>();

        println!("valid: {:?}", valid);
        if valid.len() != 1 {
            return Err("No valid sources for given destination".to_string());
        }

        Ok((valid[0], dst))
    }

    /// Adds the notation for the move `src` -> `dst`
    ///
    /// Will panic if `self.board[src] == None`
    pub fn log(&mut self, src: usize, dst: usize) {
        self.move_log.push(self.to_std_notation(src, dst).unwrap());
    }
}

impl Piece {
    fn std_notation_piece(&self) -> String {
        match self.class {
            King => "K",
            Queen => "Q",
            Bishop => "B",
            Knight => "N",
            Rook => "R",
            _ => "",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::Chess;

    #[test]
    fn log_every_move() {
        let mut cb = Chess::default();
        assert!(cb.mv(52, 36).is_ok());
        assert_eq!(cb.move_log, ["e4"]);

        let mut cb = Chess::from_fen("Q3Q3/8/8/8/Q7/8/8/8".to_string()).unwrap();
        assert!(cb.mv(0, 36).is_ok());
        assert_eq!(cb.move_log, ["Qa8e4"]);
    }

    #[test]
    fn from_std_notation() {
        let cb = Chess::default();
        assert_eq!(cb.from_std_notation("e4".to_string()), Ok((52, 36)));
        //assert!(cb.from_std_notation("Nf6".to_string()).is_ok());
    }

    #[test]
    fn mv_str() {
        let mut cb = Chess::default();
        assert!(cb.mv_str("e4".to_string()).is_ok());
        assert!(cb.mv_str("Nf6".to_string()).is_ok());
    }
}
