pub use Color::*;

use {
    crate::engine::*,
    serde::{Deserialize, Serialize},
    std::fmt::{Debug, Formatter, Result},
};

#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                White => "W",
                Black => "B",
            }
        )
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }

    pub fn get_multiplier(self) -> i8 {
        match self {
            White => 1,
            Black => -1,
        }
    }

    pub fn get_piece_rank(self) -> Rank {
        match self {
            White => Rank::_1,
            Black => Rank::_8,
        }
    }

    pub fn get_pawn_rank(self) -> Rank {
        match self {
            White => Rank::_2,
            Black => Rank::_7,
        }
    }

    pub fn get_enpassant_rank(self) -> Rank {
        match self {
            White => Rank::_3,
            Black => Rank::_6,
        }
    }

    pub fn get_center_rank(self) -> Rank {
        match self {
            White => Rank::_4,
            Black => Rank::_5,
        }
    }
}
