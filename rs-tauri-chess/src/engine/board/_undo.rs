use {crate::engine::*, indexmap::IndexMap, rs_tauri_chess::square};

impl Board {
    pub fn undo(
        &mut self,
        castling_rights: IndexMap<Color, CastlingRights>,
        enpassant_square: Option<Square>,
        halfmove_clock: u32,
    ) -> Option<()> {
        let r#move = self.history.pop()?;
        self.turn = self.turn.opposite();
		self.castling_rights = castling_rights;
		self.enpassant_square = enpassant_square;
		self.halfmove_clock = halfmove_clock;
		self.fullmove_number -= 1;

        match r#move.r#type {
            Normal | PawnJump => {
                let piece = self.pieces.remove(&r#move.to).unwrap();
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);
            }
            Capture => {
                let piece = self.pieces.remove(&r#move.to).unwrap();
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                self.attack_lines
                    .insert(r#move.to, captured.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, captured);
            }
            Promotion => {
                let mut piece = self.pieces.remove(&r#move.to).unwrap();
                piece.r#type = Pawn;
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);
            }
            PromotionCapture => {
                let mut piece = self.pieces.remove(&r#move.to).unwrap();
                piece.r#type = Pawn;
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                self.attack_lines
                    .insert(r#move.to, captured.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, captured);
            }
            Enpassant => {
                let piece = self.pieces.remove(&r#move.to).unwrap();
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                let captured_square = r#move
                    .from
                    .offset(
                        Into::<i8>::into(r#move.to.file) - Into::<i8>::into(r#move.from.file),
                        0,
                    )
                    .unwrap();
                self.attack_lines
                    .insert(captured_square, captured.get_attack_lines(captured_square));
                self.pieces.insert(captured_square, captured);
            }
            Castle => {
                let king = self.pieces.remove(&r#move.to).unwrap();
                self.attack_lines.remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, king.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, king);

                let (rook_square_from, rook_square_to) = if r#move.to.file == File::C {
                    (square!(A r#move.to.rank), square!(D r#move.to.rank))
                } else {
                    (square!(H r#move.to.rank), square!(F r#move.to.rank))
                };
                let rook = self.pieces.remove(&rook_square_to).unwrap();
                self.attack_lines.remove(&rook_square_to);
                self.attack_lines
                    .insert(rook_square_from, rook.get_attack_lines(rook_square_from));
                self.pieces.insert(rook_square_from, rook);
            }
        }

        if let King = self.pieces.get(&r#move.from).unwrap().r#type {
            self.kings.insert(self.turn, r#move.from);
        }

        Some(())
    }
}
