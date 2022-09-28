use {
    crate::engine::{
        board::{Board, CastlingRights},
        color::*,
        piece::*,
        r#move::*,
        square::{Rank, Square},
    },
    rs_tauri_chess::square,
};

impl Board {
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let opposite_color = self.turn.opposite();

        let initial_king_rank = if self.turn == White {
            Rank::_1
        } else {
            Rank::_8
        };
        let mut in_check = false;
        let mut unchecked_squares = vec![
            square!(C initial_king_rank),
            square!(D initial_king_rank),
            square!(F initial_king_rank),
            square!(G initial_king_rank),
        ];

        for (square, piece) in &self.pieces {
            if piece.color == opposite_color {
                continue;
            }

            let square = *square;
            match piece.r#type {
                Pawn => {
                    let team_multiplier = if self.turn == White { 1 } else { -1 };
                    let team_rank = |white_value: Rank, black_value: Rank| -> Rank {
                        if self.turn == White {
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
                                if target_piece.color != self.turn {
                                    for r#type in [Queen, Rook, Bishop, Knight] {
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
                            for r#type in [Queen, Rook, Bishop, Knight] {
                                moves.push(Move::from_promotion(square, target_square, r#type));
                            }
                        }
                        if let Some(target_square) = square.offset(1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    for r#type in [Queen, Rook, Bishop, Knight] {
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
                        if let Some(enpassant_square) = self.enpassant_square {
                            if enpassant_square.rank == team_rank(Rank::_6, Rank::_3)
                                && square.rank == team_rank(Rank::_5, Rank::_4)
                            {
                                if let Some(left_target_square) =
                                    square.offset(-1, 1 * team_multiplier)
                                {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            left_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -1 * team_multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) =
                                    square.offset(1, 1 * team_multiplier)
                                {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            right_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -1 * team_multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, 1 * team_multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
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
                                if target_piece.color != self.turn {
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
                Knight => {
                    for (file, rank) in Directions::KNIGHT {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
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
                Bishop => self.get_straight_moves(&mut moves, &piece, &Directions::BISHOP),
                Rook => self.get_straight_moves(&mut moves, &piece, &Directions::ROOK),
                Queen => self.get_straight_moves(&mut moves, &piece, &Directions::QUEEN),
                King => {
                    for (file, rank) in Directions::KING {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
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

        let king_square = *self.kings.get(&self.turn).unwrap();
        for square in Square::ALL {
            if let Some(piece) = self.pieces.get(&square) {
                if piece.color != self.turn {
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
                                    if let Some(_) = self.pieces.get(square) {
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

        let CastlingRights {
            queenside,
            kingside,
        } = *self.castling_rights.get(&self.turn).unwrap();
        if !in_check {
            if queenside
                && [
                    square!(B initial_king_rank),
                    square!(C initial_king_rank),
                    square!(D initial_king_rank),
                ]
                .iter()
                .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(C initial_king_rank))
                && unchecked_squares.contains(&square!(D initial_king_rank))
            {
                moves.push(Move::from_castle(
                    square!(E initial_king_rank),
                    square!(C initial_king_rank),
                ));
            }

            if kingside
                && [square!(F initial_king_rank), square!(G initial_king_rank)]
                    .iter()
                    .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(F initial_king_rank))
                && unchecked_squares.contains(&square!(G initial_king_rank))
            {
                moves.push(Move::from_castle(
                    square!(E initial_king_rank),
                    square!(G initial_king_rank),
                ));
            }
        }

        moves
    }

    fn get_straight_moves(&self, moves: &mut Vec<Move>, piece: &Piece, directions: &[(i8, i8)]) {
        let square = self.get_square(piece).unwrap();
        for (file, rank) in directions {
            let mut file_offset = *file;
            let mut rank_offset = *rank;
            while let Some(target_square) = square.offset(file_offset, rank_offset) {
                if let Some(target_piece) = self.pieces.get(&target_square) {
                    if target_piece.color != self.turn {
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
    }
}
