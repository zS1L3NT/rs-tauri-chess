use {crate::engine::*, serde::Serialize};

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
