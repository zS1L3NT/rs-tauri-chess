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

    pub fn get_moves(&self, board: Board) {
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
                        if let Some(square_offset) = square.offset(-1, 1) {
                            if let Some(piece) = board.get_piece(square_offset) {
                                if piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Queen,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Rook,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Bishop,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                        let square_offset = square.offset(0, 1).unwrap();
                        if let Some(piece) = board.get_piece(square_offset) {
                            if piece.color != self.color {
                                moves.push(Move::fromPromotion(
                                    square,
                                    square_offset,
                                    PieceType::Queen,
                                    board.determine_threat(piece),
                                ));
                                moves.push(Move::fromPromotion(
                                    square,
                                    square_offset,
                                    PieceType::Rook,
                                    board.determine_threat(piece),
                                ));
                                moves.push(Move::fromPromotion(
                                    square,
                                    square_offset,
                                    PieceType::Bishop,
                                    board.determine_threat(piece),
                                ));
                                moves.push(Move::fromPromotion(
                                    square,
                                    square_offset,
                                    PieceType::Knight,
                                    false,
                                ));
                            }
                        }
                        if let Some(square_offset) = square.offset(1, 1) {
                            if let Some(piece) = board.get_piece(square_offset) {
                                if piece.color != self.color {
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Queen,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Rook,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Bishop,
                                        board.determine_threat(piece),
                                    ));
                                    moves.push(Move::fromPromotionCapture(
                                        square,
                                        square_offset,
                                        PieceType::Knight,
                                        false,
                                    ));
                                }
                            }
                        }
                    } else {
                        if let Some(enpassant_square) = board.enpassent_square {
                            if enpassant_square.rank == Rank::_5 && square.rank == Rank::_5 {
                                if let Some(left_square) = square.offset(-1, 0) {
                                    if left_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            left_square.offset(0, 1).unwrap(),
                                        ));
                                    }
                                }
                                if let Some(right_square) = square.offset(1, 0) {
                                    if right_square == enpassant_square {
                                        moves.push(Move::fromEnpassent(
                                            square,
                                            right_square.offset(0, 1).unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(square_offset) = square.offset(-1, 1) {
                            if let Some(piece) = board.get_piece(square_offset) {
                                if piece.color != self.color {
                                    moves.push(Move::fromCapture(square, square_offset));
                                }
                            }
                        }
                        let square_offset = square.offset(0, 1).unwrap();
                        if let Some(piece) = board.get_piece(square_offset) {
                            if piece.color != self.color {
                                moves.push(Move::fromNormal(square, square_offset));
                            }
                        }
                        if let Some(square_offset) = square.offset(1, 1) {
                            if let Some(piece) = board.get_piece(square_offset) {
                                if piece.color != self.color {
                                    moves.push(Move::fromCapture(square, square_offset));
                                }
                            }
                        }
                    }
                }
                Color::Black => todo!(),
            },
            PieceType::Knight => match self.color {
                Color::White => todo!(),
                Color::Black => todo!(),
            },
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
    }
}
