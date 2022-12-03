mod fen;
mod lib;
mod moves;
mod notation;
mod piece;
mod ai;

pub use ai::Ai;
pub use lib::*;

use piece::{
    Side::{self, *},
    *,
};
use std::fmt::Debug;

/**
A chess library built for use with web api

Features
* FEN notation support
* * url friendly
* * export any chess board to valid FEN
* * create chess game from valid FEN
*/
#[derive(Debug, Clone)]
pub struct Chess {
    /// 64 long with black on top
    board: Vec<P>,
    /// true for white, false for black
    pub turn: Side,
    /// The index of a square that is capturable via en passant
    ///
    /// May need to change in the future
    pub en_passant: Option<usize>,
    pub new_en_passant: bool,
    /// Stores the 4 possible castles as bools where 'true' indicates validity
    ///
    /// Stored in the FEN order K, Q, k, q
    pub castle: [bool; 4],
    /// How many moves both players have made since the last pawn advance or piece capture
    ///
    /// The game should stalemate when this counter reaches 100
    pub halfmoves: u32,
    /// The number of completed turns in the game.
    ///
    /// This number starts at 1 and is incremented by one every time Black moves.
    pub fullmoves: u32,

    pub check: Option<Side>,
    pub checkmate: bool,
    pub stalemate: bool,
    pub move_log: Vec<String>,
}

impl Chess {
    /// Clone of the internal board vector
    pub fn board(&self) -> Vec<P> {
        self.board.clone()
    }

    /// Returns Some(*winning side*) or None if there is no winner
    pub fn winner(&self) -> Option<Side> {
        if self.checkmate {
            self.check
        } else {
            None
        }
    }

    /// Mutable reference to the internal board vector
    pub fn board_mut(&mut self) -> &mut Vec<P> {
        &mut self.board
    }
    /**
    Each piece value corresponds to the unicode position of its symbol

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
    pub fn to_symbol(a: P, whitespace: char) -> char {
        if let Some(x) = a {
            char::from_u32(0x2654 + x.value() as u32).unwrap_or_default()
        } else {
            whitespace
        }
    }

    /**
    ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜

    ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟

    ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙

    ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
     */
    pub fn pretty_print(&self) {
        for y in 0..8 {
            for x in 0..8 {
                print!("{} ", Self::to_symbol(self.board[ndx(x, y)], '-'));
            }
            println!();
        }
    }

    /// Returns the value of the piece located at the given coordinates
    pub fn sqr(&self, x: usize, y: usize) -> P {
        self.board[ndx(x, y)]
    }

    /// Returns `Chess::default()` but the board is `vec![None;64]`
    pub fn empty() -> Self {
        let mut chess = Chess::default();
        chess.board = vec![NO; 64];
        chess
    }
}

impl Default for Chess {
    /// Returns the standard chess board with black on top
    #[rustfmt::skip]
    fn default() -> Self {
        Chess {
            board: vec![
                BR,BN,BB,BQ,BK,BB,BN,BR,
                BP,BP,BP,BP,BP,BP,BP,BP,
                NO,NO,NO,NO,NO,NO,NO,NO,
                NO,NO,NO,NO,NO,NO,NO,NO,
                NO,NO,NO,NO,NO,NO,NO,NO,
                NO,NO,NO,NO,NO,NO,NO,NO,
                WP,WP,WP,WP,WP,WP,WP,WP,
                WR,WN,WB,WQ,WK,WB,WN,WR,
            ],
            turn: White,
            en_passant: None,
            new_en_passant: false,
            castle: [true;4],
            halfmoves: 0,
            fullmoves: 1,
            check: None,
            checkmate: false,
            stalemate: false,
            move_log: vec![],
        }
    }
}
