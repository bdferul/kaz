use super::lib::*;
use crate::Chess;

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
        if src | dst >= 64 {
            return Err(());
        }

        if match self.board[src] {
            0 | 6 => self.king(src, dst),
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

    fn pawn(&mut self, src: usize, dst: usize) -> bool {
        let mut r = false;

        let dir = if self.turn { -1 } else { 1 };
        let bv = self.board[dst];

        let (ax, ay) = fndx(src);

        if bv != 12 {
            return false;
        }

        //single step
        let sy = ay as i32 + dir;
        if 0 <= sy && sy < 64 {
            if ndx(ax, sy as usize) == dst {
                r = true;
            }
        }

        //double step
        if (self.turn && ay == 6) || (!self.turn && ay == 1) {
            let dy = ay as i32 + (2 * dir);
            if 0 <= dy && dy < 64 {
                if ndx(ax, dy as usize) == dst {
                    self.en_passant = Some(ndx(ax, sy as usize));
                    self.new_en_passant = true;
                    r = true;
                }
            }
        }

        self.halfmoves = 0;
        return r;
    }

    fn king(&self, src: usize, dst: usize) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{chess::lib::ndx, Chess};

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
        )
    }

    #[test]
    fn bad_pawn_moves() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(7, 6), ndx(7, 6)).is_err());
        assert!(d.mv(ndx(7, 6), ndx(7, 3)).is_err());
        assert!(d.mv(ndx(2, 6), ndx(3, 5)).is_err())
    }
}
