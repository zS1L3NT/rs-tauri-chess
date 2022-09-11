use super::{board::Board, color::Color, r#move::Move, square::Rank};

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }

    pub fn get_moves(&self, board: Board) {}
}
