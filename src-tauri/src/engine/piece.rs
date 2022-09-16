use super::{attack_lines::AttackLines, board::Board, color::Color, square::Square};

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
                        attack_lines.push(vec![square, target_square]);
                    }
                }
            }
            PieceType::Knight => {
                for (file, rank) in Directions::Knight {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![square, target_square]);
                    }
                }
            }
            PieceType::Bishop => {
                for (file, rank) in Directions::Bishop {
                    let mut current_square = square;
                    let mut line = vec![current_square];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }
                    attack_lines.push(line);
                }
            }
            PieceType::Rook => {
                for (file, rank) in Directions::Rook {
                    let mut current_square = square;
                    let mut line = vec![current_square];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }
                    attack_lines.push(line);
                }
            }
            PieceType::Queen => {
                for (file, rank) in Directions::Queen {
                    let mut current_square = square;
                    let mut line = vec![current_square];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        if target_square == opposite_king {
                            lines_with_king = Some(attack_lines.len());
                        }

                        line.push(target_square);
                        current_square = target_square;
                    }
                    attack_lines.push(line);
                }
            }
            PieceType::King => {
                for (file, rank) in Directions::King {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![square, target_square]);
                    }
                }
            }
        }

        AttackLines::new(attack_lines, lines_with_king)
    }
}
