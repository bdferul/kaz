use crate::{
    Chess,
    Class::*,
    Side::{self, *},
};

impl Chess {
    pub fn is_checkmate(&self) -> bool {
        let Some(in_check) = self.check else {
            return false;
        };

        let friendlies = self.positions_of_side(in_check);

        friendlies.iter().all(|f| self.choices(*f).is_empty())
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
            .all(|f| self.choices(*f).is_empty())
    }

    /// Checks if the given turn is in check
    pub(super) fn in_check(&self, turn: Side) -> bool {
        let mut enemy_board = self.clone();
        enemy_board.turn = !turn;
        let mut enemies = self
            .board
            .iter()
            .enumerate()
            .filter(|(_, x)| {
                let Some(p) = x else {
                return false;
            };

                p.side != turn
            })
            .map(|(i, _)| i);

        enemies.any(|enemy| {
            let ebu = enemy_board.basic_choices(enemy);
            ebu.into_iter().any(|e_move| {
                if let Some(p) = self.board[e_move] {
                    p.side == turn && p.class == King
                } else {
                    false
                }
            })
        })
    }

    pub fn set_check(&mut self) {
        self.check = match (self.in_check(White), self.in_check(Black)) {
            (true, _) => Some(White),
            (_, true) => Some(Black),
            _ => None,
        };
    }
}

#[cfg(test)]
mod tests {}
