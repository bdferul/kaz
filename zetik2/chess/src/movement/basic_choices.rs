//! Everything in here must return a vector of possible moves independent of turn

use crate::{chess::{Side::*, Class::*, fndx, Chess}, mdx, fmdx, in_range};

impl Chess {
    pub(super) fn basic_choices(&self, src: usize) -> Vec<usize> {
            match self.board[src] {
                Some(piece) => {
                    if piece.side == self.turn {
                        match piece.class {
                            King => self.king_choices_basic(src),
                            Queen => self.queen_choices(src),
                            Rook => self.rook_choices_basic(src),
                            Bishop => self.bishop_choices_basic(src),
                            Knight => self.knight_choices_basic(src),
                            Pawn => self.pawn_choices_basic(src)
                        }
                    } else {
                        vec![]
                    }
                }
                None => vec![],
            }
            .into_iter()
            .filter(|x| match self.board[*x] {
                Some(piece) => piece.side != self.turn,
                None => true,
            })
            .collect()
    }

    /// Returns only basic king choices. Does not include castling
    pub(super) fn king_choices_basic(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        let (src_x, src_y) = fmdx!(src, i32);

        for y in -1..=1 {
            for x in -1..=1 {
                let (dst_x, dst_y) = (src_x + x, src_y + y);
                if in_range!(0, dst_x, 8) && in_range!(0, dst_y, 8) {
                    let ndx = mdx!(src_x + x, src_y + y);
                    if ndx != src {
                        if let Some(p) = self.board[ndx] {
                            if p.side != self.turn {
                                r.push(ndx);
                            }
                        } else {
                            r.push(ndx)
                        }
                    }
                }
            }
        }
        /*
        if let Some(king) = self.board[src] {
            //pln!("{r:?}, {king:?}");
            if self.check != Some(king.side) {
                let castle_dsts = [62, 58, 6, 2]
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| self.castle[*i])
                    .filter(|(_, x)| self.board[*x].is_none())
                    .filter(|(i, _)| {
                        (king.side == White && *i < 2) || (king.side == Black && *i >= 2)
                    })
                    .filter(|(i, _)| {
                        let tmp_dst = match *i {
                            0 => 61,
                            1 => 59,
                            2 => 5,
                            3 => 3,
                            _ => 64,
                        };
                        if tmp_dst == 59 {
                            //pln!("{}", self.is_dangerous(tmp_dst));
                        }
                        !self.is_dangerous(tmp_dst)
                    })
                    .map(|(_, x)| x)
                    .collect::<Vec<usize>>();
                r.extend(castle_dsts);
            }
        }*/

        r
    }

    pub(super) fn bishop_choices_basic(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];
        let (src_x, src_y) = fndx(src);
        let [mut is_ne, mut is_se, mut is_sw, mut is_nw] = [true; 4];

        for i in 1..8 {
            let ex = if src_x + i <= 7 {
                src_x + i
            } else {
                is_ne = false;
                is_se = false;
                0
            };
            let wx = if i <= src_x {
                src_x - i
            } else {
                is_sw = false;
                is_nw = false;
                0
            };
            let sy = if src_y + i <= 7 {
                src_y + i
            } else {
                is_sw = false;
                is_se = false;
                0
            };
            let ny = if i <= src_y {
                src_y - i
            } else {
                is_nw = false;
                is_ne = false;
                0
            };

            let cardinal = [
                (&mut is_ne, ex, ny),
                (&mut is_se, ex, sy),
                (&mut is_sw, wx, sy),
                (&mut is_nw, wx, ny),
            ];

            for (d, x, y) in cardinal {
                if *d {
                    let ndx = mdx!(x, y);
                    if let Some(p) = self.board[ndx] {
                        if p.side != self.turn {
                            r.push(ndx);
                        }
                        *d = false;
                    } else {
                        r.push(ndx);
                    }
                }
            }
        }

        r
    }

    pub(super) fn knight_choices_basic(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        let (src_x, src_y) = fmdx!(src, i32);

        let up1 = src_y >= 1;
        let up2 = src_y >= 2;
        let left1 = src_x <= 6;
        let left2 = src_x <= 5;
        let down1 = src_y <= 6;
        let down2 = src_y <= 5;
        let right2 = src_x >= 2;
        let right1 = src_x >= 1;

        // goes clockwise starting at 1
        let directions = [
            //directions
            (left1 && up2, 1, -2),    // 1:00
            (left2 && up1, 2, -1),    // 2:00
            (left2 && down1, 2, 1),   // 4:00
            (left1 && down2, 1, 2),   // 5:00
            (right1 && down2, -1, 2), // 7:00
            (right2 && down1, -2, 1), // 8:00
            (right2 && up1, -2, -1),  // 10:00
            (right1 && up2, -1, -2),  // 11:00
        ];

        for (a, x, y) in directions {
            if a {
                r.push(mdx!(src_x + x, src_y + y, usize));
            }
        }

        r
    }

    pub(super) fn rook_choices_basic(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];
        let (src_x, src_y) = fmdx!(src, i32);

        let mut checking = [true; 4];

        for i in 1..8 {
            let directions = [[i, 0], [-i, 0], [0, i], [0, -i]];

            for (i, dir) in directions.iter().enumerate() {
                checking[i] = checking[i]
                    && src_x + dir[0] < 8
                    && src_x + dir[0] >= 0
                    && src_y + dir[1] < 8
                    && src_y + dir[1] >= 0;

                if checking[i] {
                    let dst = mdx!(src_x + dir[0], src_y + dir[1]);
                    if let Some(p) = self.board[dst] {
                        checking[i] = false;
                        if p.side != self.turn {
                            r.push(dst);
                        }
                    } else {
                        r.push(dst);
                    }
                }
            }

            if !checking.iter().any(|x| *x) {
                break;
            }
        }

        r
    }

    pub(super) fn pawn_choices_basic(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        let (src_x, src_y) = fndx(src);

        if (src_y == 0 && self.turn == White) || (src_y == 7 && self.turn == Black) {
            return r;
        }

        let dir = match self.turn {
            White => -1,
            Black => 1,
        };

        // single move
        let single_move_y = src_y as i32 + dir;
        if 0 <= single_move_y && single_move_y < 64 {
            let single_ndx = mdx!(src_x, single_move_y);
            if self.board[single_ndx].is_none() {
                r.push(single_ndx);
            }

            // attacks and en passant
            let mut attacks = vec![];
            if src_x > 0 {
                attacks.push(src_x - 1);
            }
            if src_x < 7 {
                attacks.push(src_x + 1);
            }

            for atk_x in attacks {
                let ndx = mdx!(atk_x, single_move_y);
                //r.push(ndx);

                if self.capturable(ndx) {
                    r.push(ndx);
                } else if let Some(en_p) = self.en_passant {
                    if en_p == ndx {
                        r.push(en_p);
                    }
                }
            }

            // double move
            match src_y {
                1 | 6 => {
                    let double_move_y = src_y as i32 + (dir * 2);
                    if 0 <= double_move_y && double_move_y < 8 {
                        let ndx = mdx!(src_x, double_move_y);
                        if self.board[ndx].is_none() {
                            r.push(ndx);
                        }
                    }
                }
                _ => (),
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::ndx;
    use super::*;

    #[test]
    fn simple_pawn_move() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(2, 6), ndx(2, 5)).is_ok());
        println!(
            "{}\n{}",
            d.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1"
        );
        assert_eq!(
            d.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1"
        );
        assert!(d.mv(ndx(5, 1), ndx(5, 2)).is_ok());
        assert_eq!(
            d.to_fen(),
            "rnbqkbnr/ppppp1pp/5p2/8/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 2"
        );
    }

    #[test]
    fn double_pawn_move() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(4, 6), ndx(4, 4)).is_ok());
        let mut chess = Chess::from_fen(
            "rnbqkbnr/pp1ppp1p/8/3P4/5PP1/2p1P1p1/PPP4P/RNBQKBNR w KQkq - 0 7".to_string(),
        )
        .unwrap();
        assert!(chess.mv(ndx(2, 6), ndx(2, 4)).is_ok())
    }

    #[test]
    fn bad_pawn_moves() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(7, 6), ndx(7, 6)).is_err());
        assert!(d.mv(ndx(7, 6), ndx(7, 3)).is_err());
        assert!(d.mv(ndx(2, 6), ndx(3, 5)).is_err());
    }

    #[test]
    fn pawn_capture() {
        let ffp = |s: &str| Chess::from_fen_pos(&s.to_string()).unwrap();
        let mut chess = Chess::from_fen("p7/1P6/8/8/8/8/8/8 w".to_string()).unwrap();
        assert!(chess.mv(ffp("b7"), ffp("a8")).is_ok());
    }
}