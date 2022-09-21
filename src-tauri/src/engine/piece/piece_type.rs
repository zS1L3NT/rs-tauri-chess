use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, PartialEq, Serialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl std::fmt::Debug for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}
