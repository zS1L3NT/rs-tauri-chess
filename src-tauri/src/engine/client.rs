use serde::Serialize;

use super::{
    color::Color,
    piece::{Piece, PieceType},
    r#move::Move,
    square::Square,
};

#[derive(Clone, Serialize)]
pub struct ClientPiece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
    pub square: Square,
}

impl ClientPiece {
    pub fn from(piece: &Piece, square: Square) -> ClientPiece {
        ClientPiece {
            id: piece.id,
            r#type: piece.r#type,
            color: piece.color,
            square,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ClientBoard {
    pieces: Vec<ClientPiece>,
    moves: Vec<Move>,
}

impl ClientBoard {
    pub fn new(pieces: Vec<ClientPiece>, moves: Vec<Move>) -> ClientBoard {
        ClientBoard { pieces, moves }
    }
}
