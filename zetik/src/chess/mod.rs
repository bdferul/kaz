mod fen;
mod lib;
mod moves;

use lib::*;
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
    pub board: Vec<u8>,
    /// true for white, false for black
    pub turn: bool,
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
    /// This number is incremented by one every time Black moves.
    pub fullmoves: u32,
}

impl Chess {
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
    pub fn to_symbol(a: u8, whitespace: char) -> char {
        if a < 12 {
            char::from_u32(0x2654 + a as u32).unwrap_or_default()
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
        let ts = Self::to_symbol;
        for y in 0..8 {
            for x in 0..8 {
                print!("{} ", ts(self.board[ndx(x, y)], '-'));
            }
            println!();
        }
    }

    /// Returns the value of the piece located at the given coordinates
    pub fn sqr(&self, x: usize, y: usize) -> u8 {
        self.board[ndx(x, y)]
    }
}

impl Default for Chess {
    /// Returns the standard chess board with black on top
    #[rustfmt::skip]
    fn default() -> Self {
        Chess {
            board: vec![
                08, 10, 09, 07, 06, 09, 10, 08,
                11, 11, 11, 11, 11, 11, 11, 11,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12,
                05, 05, 05, 05, 05, 05, 05, 05,
                02, 04, 03, 01, 00, 03, 04, 02,
            ],
            turn: true,
            en_passant: None,
            new_en_passant: false,
            castle: [true;4],
            halfmoves: 0,
            fullmoves: 1,
        }
    }
}
