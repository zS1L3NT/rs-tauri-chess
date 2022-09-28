use rs_tauri_chess::square;

use crate::engine::{
    color::Color,
    piece::PieceType,
    r#move::{Move, MoveType},
    square::{File, Rank},
};

use super::Board;

impl Board {
    pub fn execute(&mut self, r#move: Move) {
        self.halfmove_clock += 1;
        self.fullmove_number += 1;

        match r#move.r#type {
            MoveType::Normal | MoveType::PawnJump => {
                let piece = self.pieces.remove(&r#move.from).unwrap();
                let r#type = piece.r#type;
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                if r#type == PieceType::Pawn {
                    self.halfmove_clock = 0;
                }

                if r#move.r#type == MoveType::PawnJump {
                    if r#move.from.rank == Rank::_2 {
                        self.enpassant_square = Some(square!(r#move.from.file 3));
                    } else {
                        self.enpassant_square = Some(square!(r#move.from.file 6));
                    }
                }
            }
            MoveType::Capture => {
                let piece = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                self.halfmove_clock = 0;
            }
            MoveType::Promotion => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::PromotionCapture => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                self.halfmove_clock = 0;
            }
            MoveType::Enpassant => {
                let piece = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                let captured_square = &r#move
                    .from
                    .offset(r#move.to.file.to_index() - r#move.from.file.to_index(), 0)
                    .unwrap();
                self.pieces.remove(captured_square);
                self.attack_lines.remove(captured_square);

                self.halfmove_clock = 0;
            }
            MoveType::Castle => {
                let king = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, king.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, king);

                let (rook_square_from, rook_square_to) = if r#move.to.file == File::C {
                    (square!(A r#move.to.rank), square!(D r#move.to.rank))
                } else {
                    (square!(H r#move.to.rank), square!(F r#move.to.rank))
                };
                let rook = self.pieces.remove(&rook_square_from).unwrap();
                self.attack_lines.remove(&rook_square_from);
                self.attack_lines
                    .insert(rook_square_to, rook.get_attack_lines(rook_square_to));
                self.pieces.insert(rook_square_to, rook);

                self.castling_rights.insert(self.turn, [false, false]);
            }
        }

        if let PieceType::King = self.pieces.get(&r#move.to).unwrap().r#type {
            self.kings.insert(self.turn, r#move.to);
        }

        self.history.push(r#move);
        self.turn = self.turn.opposite();

        for color in [Color::White, Color::Black].iter() {
            let [kingside, queenside] = *self.castling_rights.get(color).unwrap();
            let rank = if *color == Color::White {
                Rank::_1
            } else {
                Rank::_8
            };

            if kingside {
                let king = self.pieces.get(&square!(E rank));
                let rook = self.pieces.get(&square!(H rank));

                if king.is_none()
                    || rook.is_none()
                    || king.unwrap().r#type != PieceType::King
                    || rook.unwrap().r#type != PieceType::Rook
                    || king.unwrap().color != rook.unwrap().color
                {
                    self.castling_rights.insert(*color, [false, queenside]);
                }
            }

            if queenside {
                let king = self.pieces.get(&square!(E rank));
                let rook = self.pieces.get(&square!(A rank));

                if king.is_none()
                    || rook.is_none()
                    || king.unwrap().r#type != PieceType::King
                    || rook.unwrap().r#type != PieceType::Rook
                    || king.unwrap().color != rook.unwrap().color
                {
                    self.castling_rights.insert(*color, [kingside, false]);
                }
            }
        }
    }
}
