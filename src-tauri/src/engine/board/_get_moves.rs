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
            if piece.color == opposite_color {
                continue;
            }

            let square = *square;
            match piece.r#type {
                Pawn => {
                    let multiplier = self.turn.get_multiplier();

                    if square.rank == self.turn.get_pawn_rank()
                        && self
                            .pieces
                            .get(&square.offset(0, 1 * multiplier).unwrap())
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
                        if let Some(target_square) = square.offset(-1, 1 * multiplier) {
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
                        let target_square = square.offset(0, 1 * multiplier).unwrap();
                        if let None = self.pieces.get(&target_square) {
                            for r#type in [Queen, Rook, Bishop, Knight] {
                                moves.push(Move::from_promotion(square, target_square, r#type));
                            }
                        }
                        if let Some(target_square) = square.offset(1, 1 * multiplier) {
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
                                if let Some(left_target_square) = square.offset(-1, 1 * multiplier)
                                {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            left_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -1 * multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, 1 * multiplier)
                                {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            right_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -1 * multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, 1 * multiplier) {
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
                        let target_square = square.offset(0, 1 * multiplier).unwrap();
                        if let None = self.pieces.get(&target_square) {
                            moves.push(Move::from_normal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, 1 * multiplier) {
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
                                            // IGNORE ALL ENPASSANTS, THOSE ARE HANDLED LATER
                                            moves.retain(|m| {
                                                (m.from == king_square
                                                    && !attack_line.contains(&m.to))
                                                    || resolving_squares.contains(&m.to)
                                                    || m.r#type == Enpassant
                                            });
                                        }
                                        1 => {
                                            // Move that pins another piece
                                            // IGNORE ALL ENPASSANTS, THOSE ARE HANDLED LATER
                                            moves.retain(|m| {
                                                !resolving_squares.contains(&m.from)
                                                    || resolving_squares.contains(&m.to)
                                                    || m.r#type == Enpassant
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

        // Determining whether an Enpassant move leaves a King in check is determined here
        moves.retain(|m| {
            if m.r#type == Enpassant {
                let king = *self.kings.get(&self.turn).unwrap();
                let captured = m.captured.as_ref().unwrap();

                if in_check {
                    // If the King is in check
                    // Only allow the Enpassant move if the captured piece attacks the King
                    return captured
                        .get_attack_lines(square!(m.to.file m.from.rank))
                        .contains(&vec![king]);
                }

                if king.rank == m.from.rank {
                    // King is on the same rank as the Enpassant pawn

                    let king_file_index: i8 = king.file.into();

                    // Whether the scan to the left or right has reached either of the pawn
                    // Once this value is 2, look out for a rook or a queen
                    let mut scanned_pawns: i8 = 0;

                    if let Some(bool) =
                        self.enpassant_scan_rank(m, (0..king_file_index).rev(), &mut scanned_pawns)
                    {
                        // If when scanning to the left we encounter a need to return
                        return bool;
                    }

                    if scanned_pawns == 2 {
                        // If the code reached here, it scanned to the left of the board and found no pieces
                        // and no XRay was found
                        return true;
                    }

                    if let Some(bool) =
                        self.enpassant_scan_rank(m, (king_file_index + 1)..=7, &mut scanned_pawns)
                    {
                        // If when scanning to the right we encounter a need to return
                        return bool;
                    }
                }
            }

            true
        });

        moves
    }

    fn enpassant_scan_rank<T>(
        &self,
        r#move: &Move,
        file_indexes: T,
        scanned_pawns: &mut i8,
    ) -> Option<bool>
    where
        T: IntoIterator<Item = i8>,
    {
        let king = *self.kings.get(&self.turn).unwrap();

        for file_index in file_indexes {
            if let Ok(file) = File::try_from(file_index) {
                if file == r#move.from.file || file == r#move.to.file {
                    *scanned_pawns += 1;
                    continue;
                }

                match scanned_pawns {
                    2 => {
                        if let Some(piece) = self.pieces.get(&square!(file  king.rank)) {
                            if piece.color == self.turn.opposite()
                                && (piece.r#type == Queen || piece.r#type == Rook)
                            {
                                return Some(false);
                            } else {
                                return Some(true);
                            }
                        }
                    }
                    0 => {
                        if self.pieces.get(&square!(file  king.rank)).is_some() {
                            return Some(true);
                        }
                    }
                    _ => panic!("???"),
                }
            }
        }

        None
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
