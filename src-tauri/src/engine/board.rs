use std::collections::HashMap;

use crate::{bishop, king, knight, pawn, queen, rook, square};

use super::{
    color::Color,
    piece::{Directions, Piece, PieceType},
    r#move::Move,
    square::{Rank, Square},
};

pub struct Board {
    pub pieces: HashMap<Square, Piece>,
    /// Lines that pin the enemy King
    ///
    /// Doesn't matter if there are multiple pieces between the attacker and the King
    ///
    /// The order of the Vector starts from the attacker
    pub pinning_lines: HashMap<Color, Vec<Vec<Square>>>,
    /// Lines or single tiles that attack the enemy King
    ///
    /// The order of the Vector starts from the attacker and ends at the end of the board
    pub threatening_lines: HashMap<Color, Vec<Vec<Square>>>,
    pub enpassent_square: Option<Square>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: HashMap::from([
                (square!(A _8), rook!(0, Black)),
                (square!(B _8), knight!(1, Black)),
                (square!(C _8), bishop!(2, Black)),
                (square!(D _8), queen!(3, Black)),
                (square!(E _8), king!(4, Black)),
                (square!(F _8), bishop!(5, Black)),
                (square!(G _8), knight!(6, Black)),
                (square!(H _8), rook!(7, Black)),
                (square!(A _7), pawn!(8, Black)),
                (square!(B _7), pawn!(9, Black)),
                (square!(C _7), pawn!(10, Black)),
                (square!(D _7), pawn!(11, Black)),
                (square!(E _7), pawn!(12, Black)),
                (square!(F _7), pawn!(13, Black)),
                (square!(G _7), pawn!(14, Black)),
                (square!(H _7), pawn!(15, Black)),
                (square!(A _2), pawn!(16, White)),
                (square!(B _2), pawn!(17, White)),
                (square!(C _2), pawn!(18, White)),
                (square!(D _2), pawn!(19, White)),
                (square!(E _2), pawn!(20, White)),
                (square!(F _2), pawn!(21, White)),
                (square!(G _2), pawn!(22, White)),
                (square!(H _2), pawn!(23, White)),
                (square!(A _1), rook!(24, White)),
                (square!(B _1), knight!(25, White)),
                (square!(C _1), bishop!(26, White)),
                (square!(D _1), queen!(27, White)),
                (square!(E _1), king!(28, White)),
                (square!(F _1), bishop!(29, White)),
                (square!(G _1), knight!(30, White)),
                (square!(H _1), rook!(31, White)),
            ]),
            pinning_lines: HashMap::from([(Color::White, vec![]), (Color::Black, vec![])]),
            threatening_lines: HashMap::from([(Color::White, vec![]), (Color::Black, vec![])]),
            enpassent_square: None,
        }
    }

    pub fn empty() -> Board {
        Board {
            pieces: HashMap::from([
                (square!(E _8), king!(4, Black)),
                (square!(E _1), king!(28, White)),
            ]),
            pinning_lines: HashMap::from([(Color::White, vec![]), (Color::Black, vec![])]),
            threatening_lines: HashMap::from([(Color::White, vec![]), (Color::Black, vec![])]),
            enpassent_square: None,
        }
    }

    pub fn get_square(&self, piece: &Piece) -> Option<Square> {
        self.pieces
            .iter()
            .find_map(|(s, p)| if p.id == piece.id { Some(*s) } else { None })
    }

    pub fn get_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = vec![];

        let opposite_color = color.opposite();

        for (square, piece) in &self.pieces {
            if piece.color == opposite_color {
                continue;
            }

            let square = *square;
            match piece.r#type {
                PieceType::Pawn => {
                    let team_multiplier = if color == Color::White { 1 } else { -1 };
                    let team_rank = |white_value: Rank, black_value: Rank| -> Rank {
                        if color == Color::White {
                            white_value
                        } else {
                            black_value
                        }
                    };

                    if square.rank == team_rank(Rank::_2, Rank::_7)
                        && self
                            .pieces
                            .get(&square.offset(0, 1 * team_multiplier).unwrap())
                            .is_none()
                        && self
                            .pieces
                            .get(&square.offset(0, 2 * team_multiplier).unwrap())
                            .is_none()
                    {
                        moves.push(Move::from_pawn_jump(
                            square,
                            square.offset(0, 2 * team_multiplier).unwrap(),
                        ));
                    }

                    if square.rank == team_rank(Rank::_7, Rank::_2) {
                        if let Some(target_square) = square.offset(-1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    for r#type in [
                                        PieceType::Queen,
                                        PieceType::Rook,
                                        PieceType::Bishop,
                                        PieceType::Knight,
                                    ] {
                                        moves.push(Move::from_promotion_capture(
                                            square,
                                            target_square,
                                            r#type,
                                            self.determine_threat(r#type, target_square),
                                        ));
                                    }
                                }
                            }
                        }
                        let target_square = square.offset(0, 1 * team_multiplier).unwrap();
                        if let None = self.pieces.get(&target_square) {
                            for r#type in [
                                PieceType::Queen,
                                PieceType::Rook,
                                PieceType::Bishop,
                                PieceType::Knight,
                            ] {
                                moves.push(Move::from_promotion(
                                    square,
                                    target_square,
                                    r#type,
                                    self.determine_threat(r#type, target_square),
                                ));
                            }
                        }
                        if let Some(target_square) = square.offset(1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    for r#type in [
                                        PieceType::Queen,
                                        PieceType::Rook,
                                        PieceType::Bishop,
                                        PieceType::Knight,
                                    ] {
                                        moves.push(Move::from_promotion_capture(
                                            square,
                                            target_square,
                                            r#type,
                                            self.determine_threat(r#type, target_square),
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        if let Some(enpassant_square) = self.enpassent_square {
                            if enpassant_square.rank == team_rank(Rank::_5, Rank::_4)
                                && square.rank == team_rank(Rank::_5, Rank::_4)
                            {
                                if let Some(left_target_square) = square.offset(-1, 0) {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            left_target_square
                                                .offset(0, 1 * team_multiplier)
                                                .unwrap(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, 0) {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            right_target_square
                                                .offset(0, 1 * team_multiplier)
                                                .unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(square, target_square));
                                }
                            }
                        }
                        let target_square = square.offset(0, 1 * team_multiplier).unwrap();
                        if let None = self.pieces.get(&target_square) {
                            moves.push(Move::from_normal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(square, target_square));
                                }
                            }
                        }
                    }
                }
                PieceType::Knight => {
                    for (file, rank) in Directions::Knight {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(square, target_square));
                                }
                            } else {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
                PieceType::Bishop => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::Bishop)
                }
                PieceType::Rook => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::Rook)
                }
                PieceType::Queen => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::Queen)
                }
                PieceType::King => {
                    for (file, rank) in Directions::King {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(square, target_square));
                                }
                            } else {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
            }
        }

        let (king_square, king_piece) = self
            .pieces
            .iter()
            .find(|(_, p)| p.color == color && p.r#type == PieceType::King)
            .expect("Could not find king");
        let king_square = *king_square;

        let pinned_lines = self.pinning_lines.get(&opposite_color).unwrap();
        if pinned_lines.len() > 0 {
            moves.retain(|r#move| {
                // If a the piece moving is in the pinned line
                if let Some(pinned_line) =
                    pinned_lines.iter().find(|line| line.contains(&r#move.from))
                {
                    // Loop through every pinned square except the pinning piece's tile
                    for pinned_square in pinned_line[1..].iter() {
                        let pinned_square = *pinned_square;

                        // If the move is not the candidate move AND another piece is in the way of the king
                        if pinned_square != r#move.from && self.pieces.get(&pinned_square).is_some()
                        {
                            // Ignore the move since it isn't actually pinning the king
                            return true;
                        }
                    }

                    // No other pieces in the way of the king except candidate, restrict candidate
                    if !pinned_line.contains(&r#move.to) {
                        return false;
                    }
                }

                return true;
            });
        }

        let threatened_lines = self.threatening_lines.get(&opposite_color).unwrap();
        if threatened_lines.len() > 0 {
            // If there is more than 1 threat, the king is the only piece that can resolve that
            if threatened_lines.len() > 1 {
                moves.retain(|r#move| {
                    // ! Moves that keep king in check
                    r#move.from == king_square
                        && !threatened_lines
                            .iter()
                            .any(|line| line.contains(&r#move.to))
                });
            } else {
                let threatened_line = threatened_lines[0];
                moves.retain(|r#move| {
                    // If the move is made by the King
                    if r#move.from == king_square {
                        // ! Moves that keep king in check
                    } else {
                        return threatened_line.contains(&r#move.to);
                    }
                });
            }
        }

        moves
    }

    fn get_straight_moves(
        &self,
        piece: &Piece,
        color: Color,
        moves: &mut Vec<Move>,
        directions: &[(i8, i8)],
    ) {
        let square = self.get_square(piece).unwrap();
        for (file, rank) in directions {
            let mut file_offset = *file;
            let mut rank_offset = *rank;
            while let Some(target_square) = square.offset(file_offset, rank_offset) {
                if let Some(target_piece) = self.pieces.get(&target_square) {
                    if target_piece.color != color {
                        moves.push(Move::from_capture(square, target_square));
                    }
                    break;
                } else {
                    moves.push(Move::from_normal(square, target_square));
                }
                file_offset += file;
                rank_offset += rank;
            }
        }
    }

    pub fn execute(r#move: Move) {}

    pub fn determine_threat(&self, r#type: PieceType, square: Square) -> bool {
        true
    }
}
