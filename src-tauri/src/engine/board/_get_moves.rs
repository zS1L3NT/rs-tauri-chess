use crate::engine::{
    color::Color,
    piece::{Directions, Piece, PieceType},
    r#move::{Move, MoveType},
    square::{File, Rank, Square},
};

use super::Board;

impl Board {
    pub fn get_moves(&self) -> Vec<Move> {
        let color = if self.history.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };

        let mut moves = vec![];
        let opposite_color = color.opposite();

        let initial_king_rank = if color == Color::White {
            Rank::_1
        } else {
            Rank::_8
        };
        let mut in_check = false;
        let mut unchecked_squares = vec![
            Square::from(File::C, initial_king_rank),
            Square::from(File::D, initial_king_rank),
            Square::from(File::F, initial_king_rank),
            Square::from(File::G, initial_king_rank),
        ];

        let get_straight_moves = |moves: &mut Vec<Move>, piece: &Piece, directions: &[(i8, i8)]| {
            let square = self.get_square(piece).unwrap();
            for (file, rank) in directions {
                let mut file_offset = *file;
                let mut rank_offset = *rank;
                while let Some(target_square) = square.offset(file_offset, rank_offset) {
                    if let Some(target_piece) = self.pieces.get(&target_square) {
                        if target_piece.color != color {
                            moves.push(Move::from_capture(
                                square,
                                target_square,
                                target_piece.clone(),
                            ));
                        }
                        break;
                    } else {
                        moves.push(Move::from_normal(square, target_square));
                    }
                    file_offset += file;
                    rank_offset += rank;
                }
            }
        };

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
                                            target_piece.clone(),
                                            r#type,
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
                                moves.push(Move::from_promotion(square, target_square, r#type));
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
                                            target_piece.clone(),
                                            r#type,
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        if let Some(last_move) = self.history.last() {
                            if last_move.r#type == MoveType::PawnJump {
                                let enpassant_square = last_move.to;
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
                                                self.pieces.get(&enpassant_square).unwrap().clone(),
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
                                                self.pieces.get(&enpassant_square).unwrap().clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
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
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
                PieceType::Knight => {
                    for (file, rank) in Directions::KNIGHT {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            } else {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
                PieceType::Bishop => get_straight_moves(&mut moves, &piece, &Directions::BISHOP),
                PieceType::Rook => get_straight_moves(&mut moves, &piece, &Directions::ROOK),
                PieceType::Queen => get_straight_moves(&mut moves, &piece, &Directions::QUEEN),
                PieceType::King => {
                    for (file, rank) in Directions::KING {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != color {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            } else {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
            }
        }

        let king_square = *self.kings.get(&color).unwrap();
        for square in Square::ALL {
            if let Some(piece) = self.pieces.get(&square) {
                if piece.color != color {
                    // This block of code runs where {piece} is every piece
                    // on the enemy team

                    let attack_lines = piece.get_attack_lines(square);
                    if let Some(index) =
                        attack_lines.iter().position(|al| al.contains(&king_square))
                    {
                        let attack_line = attack_lines
                            .get(index)
                            .expect("Invalid lines_with_king index");

                        let mut blocking_pieces = 0;
                        let mut resolving_squares = vec![square];

                        for square in attack_line {
                            let square = *square;

                            if let Some(_) = self.pieces.get(&square) {
                                if square == king_square {
                                    match blocking_pieces {
                                        0 => {
                                            in_check = true;

                                            // Move that checks the King
                                            moves.retain(|m| {
                                                (m.from == king_square
                                                    && !attack_line.contains(&m.to))
                                                    || resolving_squares.contains(&m.to)
                                            });
                                        }
                                        1 => {
                                            // Move that pins another piece
                                            moves.retain(|m| {
                                                !resolving_squares.contains(&m.from)
                                                    || resolving_squares.contains(&m.to)
                                            });
                                        }
                                        _ => {}
                                    }

                                    break;
                                } else {
                                    blocking_pieces += 1;
                                }
                            }

                            resolving_squares.push(square);
                        }
                    }

                    // This block of code filter moves that regard our King
                    // moving into a check
                    for attack_line in &attack_lines {
                        moves.retain(|m| {
                            if m.from == king_square {
                                for square in attack_line {
                                    if let Some(_) = self.pieces.get(&square) {
                                        return *square != m.to;
                                    } else if *square == m.to {
                                        return false;
                                    }
                                }
                            }

                            true
                        });

                        if unchecked_squares.iter().any(|s| attack_line.contains(s)) {
                            for square in attack_line {
                                if let Some(index) =
                                    unchecked_squares.iter().position(|s| s == square)
                                {
                                    unchecked_squares.remove(index);
                                    break;
                                }

                                if self.pieces.get(square).is_some() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let is_untouched = |square: Square| {
            self.history
                .iter()
                .find(|m| m.from == square || m.to == square)
                .is_none()
        };
        if !in_check && is_untouched(Square::from(File::E, initial_king_rank)) {
            if [
                Square::from(File::B, initial_king_rank),
                Square::from(File::C, initial_king_rank),
                Square::from(File::D, initial_king_rank),
            ]
            .iter()
            .all(|s| self.pieces.get(s).is_none())
                && is_untouched(Square::from(File::A, initial_king_rank))
                && unchecked_squares.contains(&Square::from(File::C, initial_king_rank))
                && unchecked_squares.contains(&Square::from(File::D, initial_king_rank))
            {
                moves.push(Move::from_castle(
                    Square::from(File::E, initial_king_rank),
                    Square::from(File::C, initial_king_rank),
                ));
            }

            if [
                Square::from(File::F, initial_king_rank),
                Square::from(File::G, initial_king_rank),
            ]
            .iter()
            .all(|s| self.pieces.get(s).is_none())
                && is_untouched(Square::from(File::H, initial_king_rank))
                && unchecked_squares.contains(&Square::from(File::F, initial_king_rank))
                && unchecked_squares.contains(&Square::from(File::G, initial_king_rank))
            {
                moves.push(Move::from_castle(
                    Square::from(File::E, initial_king_rank),
                    Square::from(File::G, initial_king_rank),
                ));
            }
        }

        moves
    }
}
