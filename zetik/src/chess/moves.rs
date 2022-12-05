use super::lib::*;
use super::piece::*;
use super::piece::{
    Class::*,
    Side::{self, *},
};
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

        if let Some(src_p) = self.board[src] {
            if src_p.class == King {
                self.if_castle_move_rooks(src, dst)
            }
        }

        let (possible, double_dst, en_passant) = self.base_choices(src);
        if possible.contains(&dst) {
            let mut tmp_game = self.clone();
            Self::move_unchecked(&mut tmp_game, src, dst, double_dst, en_passant);

            //tmp_game.turn.flip();
            if !tmp_game.in_check(self.turn) {
                //tmp_game.turn.flip();
                self.log(src, dst);
                let log = self.move_log.clone();
                *self = tmp_game;
                self.move_log = log;

                self.set_check();
                self.checkmate = self.is_checkmate();
                self.stalemate = self.is_stalemate();
                return Ok(());
            } else {
                return Err(());
            }
        }
        Err(())
    }
    /// Returns positions available for castling
    fn castle_choices(&self) {
        let srcs = [60, 4]; // [K,k]
        let dsts = [62, 58, 6, 2]; // [K,Q,k,q]
                                   //let iter = self.castle.iter().zip(dsts.into_iter()).filter(|(a,_)| **a);

        let s = 4;
        let d = 6;

        if self.castle[2] {
            let king = self.board[s].unwrap();
            let rook = self.board[d].unwrap();
        }
    }

    pub fn set_check(&mut self) {
        self.check = match (self.in_check(White), self.in_check(Black)) {
            (true, _) => Some(White),
            (_, true) => Some(Black),
            _ => None,
        };
    }

    fn positions_of_side(&self, turn: Side) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, x)| {
                let Some(p) = x else {
                return false;
            };

                p.side != turn
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn is_checkmate(&self) -> bool {
        let Some(in_check) = self.check else {
            return false;
        };

        let friendlies = self.positions_of_side(in_check);

        friendlies.iter().all(|f| self.choices(*f).0.len() == 0)
    }

    pub fn is_stalemate(&self) -> bool {
        if self.checkmate {
            return false;
        }

        if self.halfmoves == 100 {
            return true;
        }

        self.positions_of_side(self.turn)
            .iter()
            .all(|f| self.choices(*f).0.len() == 0)
    }

    fn if_castle_move_rooks(&mut self, src: usize, dst: usize) {
        if matches!(src, 4 | 60) {
            let a = src - 2;
            let b = src + 2;
            if dst == a {
                self.board[a + 1] = self.board[a - 2];
                self.board[a - 2] = NO;
            } else if dst == b {
                self.board[b - 1] = self.board[b + 1];
                self.board[b + 1] = NO;
            }
        }
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
        let src_piece = chess.board[src].unwrap();
        if src_piece.class == King {
            match src_piece.side {
                White => {
                    chess.castle[0] = false;
                    chess.castle[1] = false;
                }
                Black => {
                    chess.castle[2] = false;
                    chess.castle[3] = false;
                }
            }
            chess.if_castle_move_rooks(src, dst);
        }
        if src_piece.class == Rook {
            match src {
                0 => chess.castle[3] = false,
                7 => chess.castle[2] = false,
                56 => chess.castle[1] = false,
                63 => chess.castle[0] = false,
                _ => (),
            };
        }
        chess.check_en_passant(src, dst, double_dst, en_passant);
        if src_piece.class == Pawn || chess.board[dst].is_some() {
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

    pub fn choices(&self, src: usize) -> (Vec<usize>, Option<usize>, Option<usize>) {
        let (a, b, c) = self.base_choices(src);

        (
            a.into_iter()
                .filter(|x| {
                    let mut tmp = self.clone();
                    Chess::move_unchecked(&mut tmp, src, *x, b, c);
                    !tmp.in_check(self.turn)
                })
                .collect(),
            b,
            c,
        )
    }

    pub fn capturable(&self, pos: usize) -> bool {
        let Some(p) = self.board[pos] else {
            return false;
        };

        p.side != self.turn
    }
    /// Returns a vector of the positions of every enemy
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

    pub fn all_choices(&self) -> Vec<(usize, usize)> {
        (0..self.board.len())
            .map(|i| (i, self.choices(i).0))
            .filter(|(_, x)| x.len() > 0)
            .fold(vec![], |mut acc, (src, dsts)| {
                acc.extend(dsts.iter().map(|dst| (src, *dst)));
                acc
            })
    }

    /// returns a vector of possibilities
    pub fn pawn_choices(&self, src: usize) -> (Vec<usize>, Option<usize>, Option<usize>) {
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

    pub fn knight_choices(&self, src: usize) -> Vec<usize> {
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

    pub fn queen_choices(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        r.extend(self.bishop_choices(src));
        r.extend(self.rook_choices(src));

        r
    }

    /// Returns only basic king choices. Does not include castling
    pub fn king_choices(&self, src: usize) -> Vec<usize> {
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

    /// Returns the basic choices available to the selected piece, independent of turn
    pub fn basic_choices(&self, src: usize) -> Vec<usize> {
        let Some(src_p) = self.board[src] else {
            return vec![]
        };

        let mut tmp = self.clone();
        tmp.turn = src_p.side;

        match src_p.class {
            King => tmp.king_choices(src),
            Queen => tmp.queen_choices(src),
            Bishop => tmp.bishop_choices(src),
            Knight => tmp.bishop_choices(src),
            Rook => tmp.rook_choices(src),
            _ => todo!(),
        }
    }

    /// Returns true if the enemy can capture that square
    pub fn is_dangerous(&self, dst: usize) -> bool {
        let enemies = self.enemy_positions();
        //pln!("{:?}", enemies);
        let mut tmp = self.clone();
        tmp.turn.flip();
        let reds = enemies
            .into_iter()
            .map(|x| tmp.base_choices(x).0)
            .fold(vec![], |mut acc, x| {
                acc.extend(x);
                acc
            });
        reds.contains(&dst)
    }

    /// Checks if the given turn is in check
    fn in_check(&self, turn: Side) -> bool {
        let mut enemy_board = self.clone();
        enemy_board.turn = !turn;
        let enemies: Vec<usize> = self
            .board
            .iter()
            .enumerate()
            .filter(|(_, x)| {
                let Some(p) = x else {
                return false;
            };

                p.side != turn
            })
            .map(|(i, _)| i)
            .collect();

        enemies.into_iter().any(|enemy| {
            let ebu = enemy_board.base_choices(enemy).0;
            ebu.into_iter().any(|e_move| {
                if let Some(p) = self.board[e_move] {
                    p.side == turn && p.class == King
                } else {
                    false
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::piece::Side::*,
        chess::Chess,
        chess::{lib::ndx, pln},
    };

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
    fn castle() {
        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPPPPPPR3K2R".to_string()).unwrap();
        assert!(bd.mv(60, 62).is_ok());
        pln!("{}", bd.to_fen());
        assert!(bd.mv(4, 6).is_ok());

        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPPPPPPR3K2R".to_string()).unwrap();
        assert!(bd.mv(60, 58).is_ok());
        pln!("{}", bd.to_fen());
        assert!(bd.mv(4, 2).is_ok());

        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPrPPPPR3K2R".to_string()).unwrap();
        pln!("{}", bd.is_dangerous(59));
        assert!(bd.mv(60, 58).is_err());
    }

    #[test]
    fn is_dangerous() {
        let bd = Chess::from_fen("r3k2rpppppppp8888PPPrPPPPR3K2R".to_string()).unwrap();
        assert!(bd.is_dangerous(59));
        //assert!(bd.mv(60, 58).is_err());
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
