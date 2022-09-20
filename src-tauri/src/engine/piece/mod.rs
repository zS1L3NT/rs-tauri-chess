use serde::Serialize;

use super::{attack_lines::AttackLines, board::Board, color::Color, square::Square};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ToString for PieceType {
    fn to_string(&self) -> String {
		match self {
			PieceType::Pawn => "pawn".into(),
			PieceType::Knight => "knight".into(),
			PieceType::Bishop => "bishop".into(),
			PieceType::Rook => "rook".into(),
			PieceType::Queen => "queen".into(),
			PieceType::King => "king".into(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Piece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
}

pub struct Directions {}

impl Directions {
    pub const KNIGHT: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    pub const BISHOP: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    pub const ROOK: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub const QUEEN: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub const KING: [(i8, i8); 8] = Directions::QUEEN;
}

impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }

    pub fn get_attack_lines(&self, board: &Board, square: Square) -> AttackLines {
        let mut attack_lines = vec![];
        let mut lines_with_king = None;

        let opposite_king = if self.color == Color::White {
            board.black_king
        } else {
            board.white_king
        };

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
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() > 1 {
                        attack_lines.push(line);
                    }
                }
            }
            PieceType::Rook => {
                for (file, rank) in Directions::ROOK {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() > 1 {
                        attack_lines.push(line);
                    }
                }
            }
            PieceType::Queen => {
                for (file, rank) in Directions::QUEEN {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }

                    if line.len() > 1 {
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

        AttackLines::new(square, attack_lines, lines_with_king)
    }
}
