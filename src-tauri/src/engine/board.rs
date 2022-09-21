use std::collections::HashMap;

use crate::{bishop, king, knight, pawn, queen, rook, square};

use super::{
    attack_lines::AttackLines,
    client::{ClientBoard, ClientPiece},
    color::Color,
    piece::{Directions, Piece, PieceType},
    r#move::{Move, MoveType},
    square::{Rank, Square},
};

#[derive(Debug)]
pub struct Board {
    pub history: Vec<Move>,
    pub pieces: HashMap<Square, Piece>,
    pub attack_lines: HashMap<Square, AttackLines>,
    pub enpassent_square: Option<Square>,
    pub white_king: Square,
    pub black_king: Square,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            history: vec![],
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
            attack_lines: HashMap::new(),
            enpassent_square: None,
            white_king: square!(E _1),
            black_king: square!(E _8),
        };

        board.attack_lines = board
            .pieces
            .iter()
            .map(|(s, p)| (*s, p.get_attack_lines(&board, *s)))
            .collect::<HashMap<_, _>>();

        board
    }

    pub fn get_square(&self, piece: &Piece) -> Option<Square> {
        self.pieces
            .iter()
            .find_map(|(s, p)| if p.id == piece.id { Some(*s) } else { None })
    }

    pub fn to_client_board(&self) -> ClientBoard {
        ClientBoard::new(
            self.pieces
                .iter()
                .map(|(s, p)| ClientPiece::from(p, *s))
                .collect::<Vec<_>>(),
            self.get_moves(),
        )
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let color = if self.history.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };

        let mut moves = vec![];
        let mut king_move_indexes = vec![];

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
                PieceType::Bishop => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::BISHOP)
                }
                PieceType::Rook => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::ROOK)
                }
                PieceType::Queen => {
                    self.get_straight_moves(&piece, color, &mut moves, &Directions::QUEEN)
                }
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
                                    king_move_indexes.push(moves.len() - 1);
                                }
                            } else {
                                moves.push(Move::from_normal(square, target_square));
                                king_move_indexes.push(moves.len() - 1);
                            }
                        }
                    }
                }
            }
        }

        let king_square = if color == Color::White {
            self.white_king
        } else {
            self.black_king
        };

        for square in Square::ALL {
            if let Some(piece) = self.pieces.get(&square) {
                if piece.color != color {
                    let attack_lines = piece.get_attack_lines(&self, square);
                    if let Some(index) = attack_lines.lines_with_king {
                        let line = attack_lines
                            .lines
                            .get(index)
                            .expect("Invalid lines_with_king index");
                        self.filter_line(&mut moves, attack_lines.origin, line, king_square);
                    }

                    for line in &attack_lines.lines {
                        let mut index = 0;
                        moves.retain(|r#move| {
                            if king_move_indexes.contains(&index) {
                                if self.is_clear_line(line, r#move.to) {
                                    index += 1;
                                    return false;
                                }
                            }

                            index += 1;
                            true
                        });
                    }
                }
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

    fn filter_line(
        &self,
        moves: &mut Vec<Move>,
        origin: Square,
        line: &Vec<Square>,
        king_square: Square,
    ) {
        // The number of pieces between the origin and the opponent King
        let mut blocking_pieces = 0;

        // Squares that resolve the check if a piece moves to them
        let mut resolving_squares = vec![origin];

        for line_square in line {
            let line_square = *line_square;

            if let Some(_) = self.pieces.get(&line_square) {
                if line_square == king_square {
                    match blocking_pieces {
                        0 => {
                            // Move that checks the King
                            moves.retain(|r#move| {
                                if r#move.from == king_square {
                                    return true;
                                }

                                if resolving_squares.contains(&r#move.to) {
                                    return true;
                                }

                                return false;
                            });
                        }
                        1 => {
                            // Move that pins another piece
                            moves.retain(|r#move| {
                                if r#move.from != line_square {
                                    return true;
                                }

                                if resolving_squares.contains(&r#move.to) {
                                    return true;
                                }

                                return false;
                            });
                        }
                        _ => {}
                    }

                    break;
                } else {
                    blocking_pieces += 1;
                }
            }

            resolving_squares.push(line_square);
        }
    }

    fn is_clear_line(&self, line: &Vec<Square>, king_square: Square) -> bool {
        for square in line.iter() {
            if let Some(_) = self.pieces.get(&square) {
                return *square == king_square;
            }
        }
        return true;
    }

    pub fn execute(&mut self, r#move: Move) {
        match r#move.r#type {
            MoveType::Normal => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Capture => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.pieces.remove(&r#move.to);
                self.attack_lines.remove(&r#move.from);
                self.attack_lines.remove(&r#move.to);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Promotion => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::PromotionCapture => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();

                self.pieces.remove(&r#move.to);
                self.attack_lines.remove(&r#move.from);
                self.attack_lines.remove(&r#move.to);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::PawnJump => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);

                self.enpassent_square = Some(r#move.to);
            }
            MoveType::Enpassant => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                let target_square = &r#move
                    .from
                    .offset(r#move.to.file.to_index() - r#move.from.file.to_index(), 0)
                    .unwrap();
                self.pieces.remove(target_square);
                self.attack_lines.remove(&r#move.from);
                self.attack_lines.remove(target_square);

                let attack_lines = piece.get_attack_lines(&self, r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Castle => todo!(),
        }

        if let PieceType::King = self.pieces.get(&r#move.to).unwrap().r#type {
            let color = if self.history.len() % 2 == 0 {
                self.white_king = r#move.to;
                Color::White
            } else {
                self.black_king = r#move.to;
                Color::Black
            };

            for (square, attack_lines) in self.attack_lines.iter_mut() {
                if self.pieces.get(square).unwrap().color != color {
                    for attack_line in &attack_lines.lines {
                        if let Some(index) = attack_line.iter().position(|s| s == &r#move.to) {
                            attack_lines.lines_with_king = Some(index);
                        } else {
                            attack_lines.lines_with_king = None;
                        }
                    }
                }
            }
        }

        self.history.push(r#move);
    }
}
