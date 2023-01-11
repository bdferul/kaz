mod basic_choices;
mod complex_choices;
mod game_over;

use crate::{Chess,piece::{
    Class::*,
    Side::{self, *},
}};

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
    pub fn mv(&mut self, src: usize, dst: usize) -> Result<(), String> {
        if dst >= 64 || src >= 64 {
            return Err(String::from(
                "src or dst is too large. They must be less than 64",
            ));
        }

        if self.capturable(src) {
            return Err("src is considered 'capturable'".to_string());
        }

        if self.board[dst].is_some() && !self.capturable(dst) {
            return Err("dst is not empty and also not 'capturable'".to_string());
        }

        let choices = self.choices(src);
        if choices.contains(&dst) {
            let mut tmp_game = self.clone();
            tmp_game.preform_en_passant(src, dst);
            tmp_game.set_en_passant(src, dst);
            //tmp_game.perform_castle(src, dst);
            Self::move_unchecked(&mut tmp_game, src, dst);

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
                Ok(())
            } else {
                Err("a move cannot place the current player's king in check".to_string())
            }
        } else {
            Err("The available moves for 'src' do not contain 'dst'".to_string())
        }
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

    /// moves the piece from src -> dst, updating all relevant fields, without checking anything
    ///
    /// THIS WILL NOT EVEN CHECK IF YOU ARE MOVING BLANK SQUARES
    pub fn move_unchecked(chess: &mut Chess, src: usize, dst: usize) {
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
            chess.perform_castle(src, dst);
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
        chess.turn.flip();
    }

    pub fn choices(&self, src: usize) -> Vec<usize> {
        let mut choices = self.basic_choices(src);
        choices.extend(self.complex_choices(src));

        choices
            .into_iter()
            .filter(|x| {
                let mut tmp = self.clone();
                Chess::move_unchecked(&mut tmp, src, *x);
                !tmp.in_check(self.turn)
            })
            .collect()
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

    pub fn queen_choices(&self, src: usize) -> Vec<usize> {
        let mut r = vec![];

        r.extend(self.bishop_choices_basic(src));
        r.extend(self.rook_choices_basic(src));

        r
    }

    /// Returns true if the enemy can capture that square
    pub(super) fn is_dangerous(&self, dst: usize) -> bool {
        let enemies = self.enemy_positions();
        //pln!("{:?}", enemies);
        let mut tmp = self.clone();
        tmp.turn.flip();
        let reds = enemies
            .into_iter()
            .map(|x| tmp.basic_choices(x))
            .fold(vec![], |mut acc, x| {
                acc.extend(x);
                acc
            });
        reds.contains(&dst)
    }
}

#[cfg(test)]
mod tests {
    use crate::{mdx, piece::Side::*, Chess};

    #[test]
    fn wrong_color_turn() {
        let mut chess = Chess::from_fen("8/8/8/p6P/8/8/8/8 w".to_string()).unwrap();
        assert!(chess.mv(mdx!(0, 3), mdx!(0, 2)).is_err()); // try move the black pawn like a white pawn
        assert!(chess.turn == White); //ensure it is still white's turn after a failed move
    }

    #[test]
    fn pawn_en_passant() {
        let mut chess = Chess::from_fen("8/8/8/4Pp2/8/8/8/8 w KQkq f6 0 3".to_string()).unwrap();
        assert!(chess.mv(28, 21).is_ok());
    }

    #[test]
    fn bishop() {
        let mut bd = Chess::from_fen("b7/8/8/8/8/8/8/7B w".to_string()).unwrap();
        assert!(bd.mv(63, mdx!(5, 5)).is_ok());
        assert!(bd.mv(0, mdx!(1, 1)).is_ok());
        assert!(bd.mv(mdx!(5, 5), mdx!(5, 4)).is_err());
    }

    #[test]
    fn knight() {
        let mut bd = Chess::default();
        assert!(bd.mv(57, 42).is_ok());
        assert!(bd.mv(6, 23).is_ok());
        assert!(bd.mv(62, 52).is_err());
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
