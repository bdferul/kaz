use super::Chess;

impl Chess{

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

    let (possible, double_dst, en_passant) = self.unchecked_possibilities(src);

    if possible.contains(&dst) {
        let mut tmp_game = self.clone();
        Self::move_unchecked(&mut tmp_game, src, dst, double_dst, en_passant);

        //tmp_game.turn.flip();
        if !tmp_game.in_check(self.turn) {
            //tmp_game.turn.flip();
            *self = tmp_game;
            
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

pub fn capturable(&self, pos: usize) -> bool {
    let Some(p) = self.board[pos] else {
        return false;
    };

    p.side != self.turn
}

}