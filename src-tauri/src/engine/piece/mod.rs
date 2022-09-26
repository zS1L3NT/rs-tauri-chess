mod directions;
mod piece_type;
#[cfg(test)]
mod tests;

pub use directions::Directions;
pub use piece_type::PieceType;

use serde::{Deserialize, Serialize};

use super::{color::Color, square::Square};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Piece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:0<2}", self.color, self.r#type, self.id)
    }
}

impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }

    pub fn get_attack_lines(&self, square: Square) -> Vec<Vec<Square>> {
        let mut attack_lines = vec![];
        match self.r#type {
            PieceType::Pawn => {
                let team_multiplier = if self.color == Color::White { 1 } else { -1 };
                for (file, rank) in [(-1, team_multiplier), (1, team_multiplier)] {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
            PieceType::Knight => {
                for (file, rank) in Directions::KNIGHT {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
            PieceType::Bishop => {
                for (file, rank) in Directions::BISHOP {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() >= 1 {
                        attack_lines.push(line);
                    }
                }
            }
            PieceType::Rook => {
                for (file, rank) in Directions::ROOK {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() >= 1 {
                        attack_lines.push(line);
                    }
                }
            }
            PieceType::Queen => {
                for (file, rank) in Directions::QUEEN {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() >= 1 {
                        attack_lines.push(line);
                    }
                }
            }
            PieceType::King => {
                for (file, rank) in Directions::KING {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
        }

        attack_lines
    }
}
