use crate::Chess;
use super::lib::*;

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
            _ => return Err(())
        } {
            self.board[b] = self.board[a];
            self.board[a] = 12;
            return Ok(())
        }
        Err(())
    }

    fn king(&self, a: usize, b: usize) -> bool {
        todo!()
    }
}