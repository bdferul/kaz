use crate::Chess;
use super::lib::*;

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
    pub fn mv(&mut self, a: usize, b: usize) -> Result<(),()> {
        if a|b >= 64 {
            return Err(())
        }

        if match self.board[a] {
            0|6 => self.king(a, b),
            5|11 => self.pawn(a,b),
            _ => return Err(())
        } {
            self.board[b] = self.board[a];
            self.board[a] = 12;
            return Ok(())
        }
        Err(())
    }

    fn pawn(&self, a: usize, b: usize) -> bool {
        todo!()
    }

    fn king(&self, a: usize, b: usize) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chess, chess::lib::ndx};

    #[test]
    fn simple_pawn_move() {
        let mut d = Chess::default();
        assert!(d.mv(ndx(0, 6), ndx(0, 5)).is_ok());
        assert_eq!(d.to_fen(),"rnbqkbnr/pppppppp/8/8/8/p7/1PPPPPPP/RNBQKBNR b KQkq - 0 0")
    }
}