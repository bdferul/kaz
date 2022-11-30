use super::lib::*;
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

        if match self.board[src] {
            0 | 6 => todo!(),  //king
            1 | 7 => todo!(),  //queen
            2 | 8 => todo!(),  //rook
            3 | 9 => todo!(),  //bishop
            4 | 10 => todo!(), //knight
            5 | 11 => self.pawn(src, dst),
            _ => return Err(()),
        } {
            self.board[dst] = self.board[src];
            self.board[src] = 12;
            if !self.turn {
                self.fullmoves += 1;
            }
            if !self.new_en_passant {
                self.en_passant = None;
            }
            self.new_en_passant = false;
            self.turn = !self.turn;
            return Ok(());
        }
        Err(())
    }

    pub fn capturable(&self, pos: usize) -> bool {
        ((0..=5).contains(&self.board[pos]) && !self.turn)
            || ((6..=11).contains(&self.board[pos]) && self.turn)
    }

    fn pawn(&mut self, src: usize, dst: usize) -> bool {
        let mut r = false;

        let dir = if self.turn { -1 } else { 1 };

        let (src_x, src_y) = fndx(src);
        let (dst_x, dst_y) = fndx(dst);
        let (src_x, src_y, dst_x, dst_y) = (src_x as i32, src_y as i32, dst_x as i32, dst_y as i32);

        if src_x == 0 && dir == -1 {
            return false;
        }

        if self.board[dst] != 12 && dst == self.en_passant.unwrap_or(64) {
            return false;
        }

        //single step
        let single_step_y = src_y as i32 + dir;
        if 0 <= single_step_y && single_step_y < 64 {
            if ndx(src_x as usize, single_step_y as usize) == dst {
                r = true;
            }
        }

        //capture & en passant
        if dst_y == src_y + dir && (src_x - dst_x == 1 || src_x - dst_x == -1) {
            if self.capturable(dst) {
                r = true;
            }

            if let Some(ep) = self.en_passant {
                if ep == dst {
                    self.board[ndx(dst_x as usize, src_y as usize)];
                    r = true;
                }
            }
        }

        //double step **MUST GO LAST** (because of 'new_en_passant')
        if self.board[ndx(dst_x as usize, single_step_y as usize)] == 12
            && ((self.turn && src_y == 6) || (!self.turn && src_y == 1))
        {
            let dy = src_y + (2 * dir);
            if 0 <= dy && dy < 64 {
                if ndx(src_x as usize, dy as usize) == dst {
                    self.en_passant = Some(ndx(src_x as usize, single_step_y as usize));
                    self.new_en_passant = true;
                    r = true;
                }
            }
        }

        self.halfmoves = 0;
        return r;
    }
}

#[cfg(test)]
mod tests {
    use crate::{chess::lib::ndx, chess::Chess};

    #[test]
    fn wrong_color_turn() {
        let mut chess = Chess::from_fen("8/8/8/p6P/8/8/8/8 w".to_string()).unwrap();
        assert!(chess.mv(ndx(0, 3), ndx(0, 2)).is_err()); // try move the black pawn like a white pawn
        assert!(chess.turn == true); //ensure it is still white's turn after a failed move
    }

    #[test]
    fn simple_pawn_move() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(2, 6), ndx(2, 5)).is_ok());
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
        assert!(chess.mv(ndx(2, 6), ndx(2, 4)).is_err())
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
        let bd = Chess::from_fen("b7/8/8/8/8/8/8/7B w".to_string()).unwrap();
        bd.pretty_print();
    }
}
