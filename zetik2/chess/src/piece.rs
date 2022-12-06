use std::ops::Not;

use Class::*;
use Side::*;

pub type P = Option<Piece>;
pub const WK: P = Some(Piece::from(White, King));
pub const WQ: P = Some(Piece::from(White, Queen));
pub const WB: P = Some(Piece::from(White, Bishop));
pub const WN: P = Some(Piece::from(White, Knight));
pub const WR: P = Some(Piece::from(White, Rook));
pub const WP: P = Some(Piece::from(White, Pawn));
pub const BK: P = Some(Piece::from(Black, King));
pub const BQ: P = Some(Piece::from(Black, Queen));
pub const BB: P = Some(Piece::from(Black, Bishop));
pub const BN: P = Some(Piece::from(Black, Knight));
pub const BR: P = Some(Piece::from(Black, Rook));
pub const BP: P = Some(Piece::from(Black, Pawn));
pub const NO: P = None;
const ORDER: [P; 12] = [WK, WQ, WR, WB, WN, WP, BK, BQ, BR, BB, BN, BP];

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Class {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Side {
    White,
    Black,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl Side {
    pub fn flip(&mut self) {
        *self = !*self;
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Piece {
    pub side: Side,
    pub class: Class,
}

impl Piece {
    pub const fn from(side: Side, rank: Class) -> Piece {
        Piece { side, class: rank }
    }

    pub fn value(&self) -> usize {
        let order = ORDER
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<Piece>>();

        order.iter().position(|x| x == self).unwrap()
    }

    pub fn fen_ndx(a: usize) -> P {
        ORDER[a]
    }
}
