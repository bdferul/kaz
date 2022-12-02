use super::lib::*;
use super::piece::{Rank::*, Side::{*, self}};
use crate::chess::Chess;

/**
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

impl Chess {
    /// Attempts to move the piece on square 'a' to square 'b'
    ///
    /// Performs the move and returns 'true' if valid.
    ///
    /// Does nothing and returns 'false' if invalid
    pub fn mv(&mut self, src: usize, dst: usize) -> Result<(), ()> {
        if dst >= 64 || src >= 64 {
            return Err(());
        }

        if self.capturable(src) {
            return Err(());
        }

        if self.board[dst].is_some() && !self.capturable(dst) {
            return Err(());
        }

        let (possible, double_dst, en_passant) = self.possibilities(src);

        if possible.contains(&dst) {
            let mut tmp_game = self.clone();
            Self::move_unchecked(&mut tmp_game, src, dst, double_dst, en_passant);

            //tmp_game.turn.flip();
            if !tmp_game.in_check(self.turn) {
                //tmp_game.turn.flip();
                *self = tmp_game;
                return Ok(());
            } else {
                return Err(());
            }
        }
        Err(())
    }

    /// moves the piece from src -> dst, updating all relevant fields, without checking anything
    ///
    /// THIS WILL NOT EVEN CHECK IF YOU ARE MOVING BLANK SQUARES
    pub fn move_unchecked(
        chess: &mut Chess,
        src: usize,
        dst: usize,
        double_dst: Option<usize>,
        en_passant: Option<usize>,
    ) {
        chess.check_en_passant(src, dst, double_dst, en_passant);
        if chess.board[src].unwrap().rank == Pawn || chess.board[dst].is_some() {
            chess.halfmoves = 0;
        } else {
            chess.halfmoves += 1;
        }
        chess.board[dst] = chess.board[src];
        chess.board[src] = None;
        if chess.turn == Black {
            chess.fullmoves += 1;
        }
        if !chess.new_en_passant {
            chess.en_passant = None;
        }
        chess.new_en_passant = false;
        chess.turn.flip();
    }

    pub fn possibilities(&self, src: usize) -> (Vec<usize>, Option<usize>, Option<usize>) {
        let mut tails = (None, None);
        (
            match self.board[src] {
                Some(piece) => {
                    if piece.side == self.turn {
                        match piece.rank {
                            King => self.king_possible(src), //todo
                            Queen => self.queen_possible(src),
                            Rook => self.rook_possible(src),
                            Bishop => self.bishop_possible(src),
                            Knight => self.knight_possible(src),
                            Pawn => {
                                let p = self.pawn_possible(src);
                                tails = (p.1, p.2);
                                p.0
                            }
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
            .collect(),
            tails.0,
            tails.1,
        )
    }

    pub fn capturable(&self, pos: usize) -> bool {
        let Some(p) = self.board[pos] else {
            return false;
        };

        p.side != self.turn
    }

    pub fn rook_possible(&self, src: usize) -> Vec<usize> {
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

    pub fn enemy_positions(&self) -> Vec<usize> {
        (0..64)
            .filter(|i| {
                let Some(p) = self.board[*i] else {
                return false;
            };

                p.side != self.turn
            })
            .collect()
    }

    pub fn friendly_positions(&self) -> Vec<usize> {
        (0..64)
            .filter(|i| {
                let Some(p) = self.board[*i] else {
                return false;
            };

                p.side == self.turn
            })
            .collect()
    }

    pub fn bishop_possible(&self, src: usize) -> Vec<usize> {
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
                    //println!("{}: {:?}", line!(), (ndx, x, y, i));
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

    fn check_en_passant(
        &mut self,
        src: usize,
        dst: usize,
        double_dst: Option<usize>,
        en_passant: Option<usize>,
    ) {
        if let Some(dd) = double_dst {
            if dd == dst {
                self.en_passant = en_passant;
                self.new_en_passant = true;
                let (_, src_y) = fndx(src);
                let (ep_x, _) = fndx(en_passant.unwrap());
                self.board[mdx!(ep_x, src_y)];
            }
        }
    }

    /// returns a vector of possibilities
    pub fn pawn_possible(&self, src: usize) -> (Vec<usize>, Option<usize>, Option<usize>) {
        let mut r = vec![];
        let mut double_dst = None;
        let mut en_passant = None;

        let (src_x, src_y) = fndx(src);

        if (src_y == 0 && self.turn == White) || (src_y == 7 && self.turn == Black) {
            return (r, double_dst, en_passant);
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
                            en_passant = Some(single_ndx);
                            double_dst = Some(ndx);
                        }
                    }
                }
                _ => (),
            }
        }

        (r, double_dst, en_passant)
    }

    pub fn knight_possible(&self, src: usize) -> Vec<usize> {
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

    pub fn queen_possible(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        r.extend(self.bishop_possible(src));
        r.extend(self.rook_possible(src));

        r
    }

    pub fn king_possible(&self, src: usize) -> Vec<usize> {
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

        r
    }

    fn in_check(&self, turn: Side) -> bool {
        let mut enemy_board = self.clone();
        enemy_board.turn.flip();
        self.friendly_positions().into_iter().any(|i| {
            println!("{}: {:?}", line!(),(i, self.possibilities(i).0, self.turn, turn));
            self.possibilities(i).0.into_iter().any(|j| {
                if let Some(p) = self.board[j] {
                    p.side == turn && p.rank == King
                } else {
                    false
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{chess::lib::ndx, chess::piece::Side::*, chess::Chess};

    #[test]
    fn wrong_color_turn() {
        let mut chess = Chess::from_fen("8/8/8/p6P/8/8/8/8 w".to_string()).unwrap();
        assert!(chess.mv(ndx(0, 3), ndx(0, 2)).is_err()); // try move the black pawn like a white pawn
        assert!(chess.turn == White); //ensure it is still white's turn after a failed move
    }

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
        assert_eq!(
            d.to_fen(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
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

    #[test]
    fn pawn_en_passant() {
        let mut chess = Chess::from_fen("8/8/8/4Pp2/8/8/8/8 w KQkq f6 0 3".to_string()).unwrap();
        assert!(chess.mv(28, 21).is_ok());
    }

    #[test]
    fn bishop() {
        let mut bd = Chess::from_fen("b7/8/8/8/8/8/8/7B w".to_string()).unwrap();
        assert!(bd.mv(63, ndx(5, 5)).is_ok());
        assert!(bd.mv(0, ndx(1, 1)).is_ok());
        assert!(bd.mv(ndx(5, 5), ndx(5, 4)).is_err());
    }

    #[test]
    fn knight() {
        let mut bd = Chess::default();
        assert!(bd.mv(57, 42).is_ok());
        assert!(bd.mv(6, 23).is_ok());
        assert!(bd.mv(62, 52).is_err());
    }

    #[test]
    fn rook() {
        let mut bd = Chess::from_fen("R6r/8/8/8/8/8/8/8".to_string()).unwrap();
        assert!(bd.mv(0, 16).is_ok());
        assert!(bd.mv(7, 4).is_ok());
        assert!(bd.mv(16, 19).is_ok());
        assert!(bd.mv(4, 3).is_ok());
        assert!(bd.mv(19, 19 - 16).is_ok());
    }

    #[test]
    fn in_check() {
        let mut bd = Chess::from_fen("K1b58888888".to_string()).unwrap();
        assert!(bd.mv(0, 9).is_err());
    }
}
