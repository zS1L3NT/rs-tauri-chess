use {crate::engine::*, rs_tauri_chess::square};

impl Board {
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let opposite_color = self.turn.opposite();

        let piece_rank = self.turn.get_piece_rank();
        let mut in_check = false;
        let mut unchecked_squares = vec![
            square!(C piece_rank),
            square!(D piece_rank),
            square!(F piece_rank),
            square!(G piece_rank),
        ];

        for (square, piece) in &self.pieces {
            let time = std::time::Instant::now();

            if piece.color == opposite_color {
                continue;
            }

            let square = *square;
            match piece.r#type {
                Pawn => {
					let time = std::time::Instant::now();

                    let multiplier = self.turn.get_multiplier();

                    if square.rank == self.turn.get_pawn_rank()
                        && self
                            .pieces
                            .get(&square.offset(0, multiplier).unwrap())
                            .is_none()
                        && self
                            .pieces
                            .get(&square.offset(0, 2 * multiplier).unwrap())
                            .is_none()
                    {
                        moves.push(Move::from_pawn_jump(
                            square,
                            square.offset(0, 2 * multiplier).unwrap(),
                        ));
                    }

                    if square.rank == self.turn.opposite().get_pawn_rank() {
                        if let Some(target_square) = square.offset(-1, multiplier) {
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
                        let target_square = square.offset(0, multiplier).unwrap();
                        if self.pieces.get(&target_square).is_none() {
                            for r#type in [Queen, Rook, Bishop, Knight] {
                                moves.push(Move::from_promotion(square, target_square, r#type));
                            }
                        }
                        if let Some(target_square) = square.offset(1, multiplier) {
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
                            if enpassant_square.rank == self.turn.opposite().get_enpassant_rank()
                                && square.rank == self.turn.opposite().get_center_rank()
                            {
                                if let Some(left_target_square) = square.offset(-1, multiplier) {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            left_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, multiplier) {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            right_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, multiplier) {
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
                        let target_square = square.offset(0, multiplier).unwrap();
                        if self.pieces.get(&target_square).is_none() {
                            moves.push(Move::from_normal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, multiplier) {
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

					println!("    _get_moves.rs::iter_1::pawn ({:.2?})", time.elapsed());
                }
                Knight => {
                    for (file, rank) in Directions::KNIGHT {
						let time = std::time::Instant::now();

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

						println!("      _get_moves.rs::iter_1::knight::iter ({:.2?}) [{:?}, {:?}]", time.elapsed(), square, piece);
                    }
                }
                Bishop => self.get_straight_moves(&mut moves, piece, &Directions::BISHOP),
                Rook => self.get_straight_moves(&mut moves, piece, &Directions::ROOK),
                Queen => self.get_straight_moves(&mut moves, piece, &Directions::QUEEN),
                King => {
                    for (file, rank) in Directions::KING {
						let time = std::time::Instant::now();

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

						println!("    _get_moves.rs::iter_1::king::iter ({:.2?}) [{:?}, {:?}]", time.elapsed(), file, rank);
                    }
                }
            }

            println!("  _get_moves.rs::iter_1 ({:.2?}) [{:?}, {:?}]", time.elapsed(), square, piece);
        }

        let king_square = *self.kings.get(&self.turn).unwrap();

        for square in Square::ALL {
            let time = std::time::Instant::now();

            if let Some(piece) = self.pieces.get(&square) {
                if piece.color != self.turn {
					let time = std::time::Instant::now();
					
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

                            if self.pieces.get(&square).is_some() {
                                if square == king_square {
                                    match blocking_pieces {
                                        0 => {
                                            in_check = true;

                                            // Move that checks the King
                                            moves.retain(|m| {
                                                (m.from == king_square
                                                    && !attack_line.contains(&m.to))
                                                    || resolving_squares.contains(&m.to)
													// Allow Enpassant where captured piece is checking the King
                                                    || (m.r#type == Enpassant
                                                        && resolving_squares.get(0).unwrap()
                                                            == &square!(m.to.file m.from.rank))
                                            });
                                        }
                                        1 => {
                                            // Move that pins another piece
                                            moves.retain(|m| {
                                                !resolving_squares.contains(&m.from)
                                                    || resolving_squares.contains(&m.to)
                                            });
                                        }
                                        2 => {
                                            // Enpassant that leaves the king in check
                                            // This takes two pieces off the board
                                            moves.retain(|m| {
                                                m.r#type != Enpassant
                                                    || m.from.rank != king_square.rank
                                            })
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
                                    if self.pieces.get(square).is_some() {
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

					println!("    _get_moves.rs::iter_2::has_opponent ({:.2?})", time.elapsed());
                }
            }

            println!("  _get_moves.rs::iter_2 ({:.2?}) [{:?}]", time.elapsed(), square);
        }

        let CastlingRights {
            queenside,
            kingside,
        } = *self.castling_rights.get(&self.turn).unwrap();
        if !in_check {
            if kingside
                && [square!(F piece_rank), square!(G piece_rank)]
                    .iter()
                    .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(F piece_rank))
                && unchecked_squares.contains(&square!(G piece_rank))
            {
                moves.push(Move::from_castle(
                    square!(E piece_rank),
                    square!(G piece_rank),
                ));
            }

            if queenside
                && [
                    square!(B piece_rank),
                    square!(C piece_rank),
                    square!(D piece_rank),
                ]
                .iter()
                .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(C piece_rank))
                && unchecked_squares.contains(&square!(D piece_rank))
            {
                moves.push(Move::from_castle(
                    square!(E piece_rank),
                    square!(C piece_rank),
                ));
            }
        }

        moves
    }

    fn get_straight_moves(&self, moves: &mut Vec<Move>, piece: &Piece, directions: &[(i8, i8)]) {
        let square = self.get_square(piece).unwrap();
        for (file, rank) in directions {
			let time = std::time::Instant::now();

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

			println!("    _get_moves.rs::get_straight_moves ({:.2?}) [{:?}, {:?}]", time.elapsed(), file, rank);
        }
    }
}
