use crate::engine::{
    color::Color,
    piece::PieceType,
    r#move::{Move, MoveType},
    square::{File, Square},
};

use super::Board;

impl Board {
    pub fn execute(&mut self, r#move: Move) {
        match r#move.r#type {
            MoveType::Normal | MoveType::PawnJump => {
                let piece = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Capture => {
                let piece = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);
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
            }
            MoveType::Castle => {
                let king = self.pieces.remove(&r#move.from).unwrap();
                self.attack_lines.remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, king.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, king);

                let (rook_square_from, rook_square_to) = if r#move.to.file == File::C {
                    (
                        Square::from(File::A, r#move.to.rank),
                        Square::from(File::D, r#move.to.rank),
                    )
                } else {
                    (
                        Square::from(File::H, r#move.to.rank),
                        Square::from(File::F, r#move.to.rank),
                    )
                };
                let rook = self.pieces.remove(&rook_square_from).unwrap();
                self.attack_lines.remove(&rook_square_from);
                self.attack_lines
                    .insert(rook_square_to, rook.get_attack_lines(rook_square_to));
                self.pieces.insert(rook_square_to, rook);
            }
        }

        if let PieceType::King = self.pieces.get(&r#move.to).unwrap().r#type {
            self.kings.insert(
                if self.history.len() % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                },
                r#move.to,
            );
        }

        self.history.push(r#move);
    }
}
