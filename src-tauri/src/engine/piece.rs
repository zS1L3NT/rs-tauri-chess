use super::{board::Board, color::Color, r#move::Move, square::Rank};

#[derive(Debug, PartialEq)]
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

impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }

    pub fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = vec![];
        let square = board.get_square(self);

        match self.r#type {
            PieceType::Pawn => match self.color {
                Color::White => {
                    if square.rank == Rank::_2
                        && board.get_piece(square.offset(0, 1).unwrap()).is_none()
                        && board.get_piece(square.offset(0, 2).unwrap()).is_none()
                    {
                        moves.push(Move::fromPawnJump(square, square.offset(0, 2).unwrap()));
                    }

                    if square.rank == Rank::_7 {
                        if let Some(target_square) = square.offset(-1, 1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Queen,
                                        board.determine_threat(PieceType::Queen, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Rook,
                                        board.determine_threat(PieceType::Rook, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Bishop,
                                        board.determine_threat(PieceType::Bishop, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                        let target_square = square.offset(0, 1).unwrap();
                        if let None = board.get_piece(target_square) {
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Queen,
                                board.determine_threat(PieceType::Queen, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Rook,
                                board.determine_threat(PieceType::Rook, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Bishop,
                                board.determine_threat(PieceType::Bishop, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Knight,
                                false,
                            ));
                        }
                        if let Some(target_square) = square.offset(1, 1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Queen,
                                        board.determine_threat(PieceType::Queen, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Rook,
                                        board.determine_threat(PieceType::Rook, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Bishop,
                                        board.determine_threat(PieceType::Bishop, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                    } else {
                        if let Some(enpassant_square) = board.enpassent_square {
                            if enpassant_square.rank == Rank::_5 && square.rank == Rank::_5 {
                                if let Some(left_target_square) = square.offset(-1, 0) {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            left_target_square.offset(0, 1).unwrap(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, 0) {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            right_target_square.offset(0, 1).unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, 1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromCapture(square, target_square));
                                }
                            }
                        }
                        let target_square = square.offset(0, 1).unwrap();
                        if let None = board.get_piece(target_square) {
                            moves.push(Move::fromNormal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, 1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromCapture(square, target_square));
                                }
                            }
                        }
                    }
                }
                Color::Black => {
                    if square.rank == Rank::_7
                        && board.get_piece(square.offset(0, -1).unwrap()).is_none()
                        && board.get_piece(square.offset(0, -2).unwrap()).is_none()
                    {
                        moves.push(Move::fromPawnJump(square, square.offset(0, -2).unwrap()));
                    }

                    if square.rank == Rank::_2 {
                        if let Some(target_square) = square.offset(-1, -1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Queen,
                                        board.determine_threat(PieceType::Queen, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Rook,
                                        board.determine_threat(PieceType::Rook, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Bishop,
                                        board.determine_threat(PieceType::Bishop, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                        let target_square = square.offset(0, -1).unwrap();
                        if let None = board.get_piece(target_square) {
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Queen,
                                board.determine_threat(PieceType::Queen, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Rook,
                                board.determine_threat(PieceType::Rook, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Bishop,
                                board.determine_threat(PieceType::Bishop, target_square),
                            ));
                            moves.push(Move::fromPromotion(
                                square,
                                target_square,
                                PieceType::Knight,
                                false,
                            ));
                        }
                        if let Some(target_square) = square.offset(1, -1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Queen,
                                        board.determine_threat(PieceType::Queen, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Rook,
                                        board.determine_threat(PieceType::Rook, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Bishop,
                                        board.determine_threat(PieceType::Bishop, target_square),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        target_square,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                    } else {
                        if let Some(enpassant_square) = board.enpassent_square {
                            if enpassant_square.rank == Rank::_4 && square.rank == Rank::_4 {
                                if let Some(left_target_square) = square.offset(-1, 0) {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            left_target_square.offset(0, -1).unwrap(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, 0) {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            right_target_square.offset(0, -1).unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, -1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromCapture(square, target_square));
                                }
                            }
                        }
                        let target_square = square.offset(0, -1).unwrap();
                        if let None = board.get_piece(target_square) {
                            moves.push(Move::fromNormal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, -1) {
                            if let Some(target_piece) = board.get_piece(target_square) {
                                if target_piece.color != self.color {
                                    moves.push(Move::fromCapture(square, target_square));
                                }
                            }
                        }
                    }
                }
            },
            PieceType::Knight => {
                for (file, rank) in [
					(1, 2),
					(2, 1),
					(2, -1),
					(1, -2),
					(-1, -2),
					(-2, -1),
					(-2, 1),
                    (-1, 2),
                ] {
                    if let Some(target_square) = square.offset(file, rank) {
                        if let Some(target_piece) = board.get_piece(target_square) {
                            if target_piece.color != self.color {
                                moves.push(Move::fromCapture(square, target_square));
                            }
                        } else {
                            moves.push(Move::fromNormal(square, target_square));
                        }
                    }
                }
            }
            PieceType::Bishop => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
            PieceType::Rook => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
            PieceType::Queen => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
            PieceType::King => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
        }

        moves
    }
}
