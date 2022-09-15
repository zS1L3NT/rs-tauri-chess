use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
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

pub struct Directions {}

impl Directions {
    pub const Knight: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    pub const Bishop: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    pub const Rook: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub const Queen: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub const King: [(i8, i8); 8] = Directions::Queen;
}

impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }
}
