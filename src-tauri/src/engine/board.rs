use std::collections::HashMap;

use crate::{bishop, king, knight, pawn, queen, rook};
use rs_tauri_chess::square;

use super::{
    client::{ClientBoard, ClientPiece},
    color::Color,
    piece::{Directions, Piece, PieceType},
    r#move::{Move, MoveType},
    square::{File, Rank, Square},
};

#[derive(Debug)]
pub struct Board {
    pub history: Vec<Move>,
    pub pieces: HashMap<Square, Piece>,
    pub attack_lines: HashMap<Square, Vec<Vec<Square>>>,
    pub kings: HashMap<Color, Square>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            history: vec![],
            pieces: HashMap::from([
                (square!(A8), rook!(0, Black)),
                (square!(B8), knight!(1, Black)),
                (square!(C8), bishop!(2, Black)),
                (square!(D8), queen!(3, Black)),
                (square!(E8), king!(4, Black)),
                (square!(F8), bishop!(5, Black)),
                (square!(G8), knight!(6, Black)),
                (square!(H8), rook!(7, Black)),
                (square!(A7), pawn!(8, Black)),
                (square!(B7), pawn!(9, Black)),
                (square!(C7), pawn!(10, Black)),
                (square!(D7), pawn!(11, Black)),
                (square!(E7), pawn!(12, Black)),
                (square!(F7), pawn!(13, Black)),
                (square!(G7), pawn!(14, Black)),
                (square!(H7), pawn!(15, Black)),
                (square!(A2), pawn!(16, White)),
                (square!(B2), pawn!(17, White)),
                (square!(C2), pawn!(18, White)),
                (square!(D2), pawn!(19, White)),
                (square!(E2), pawn!(20, White)),
                (square!(F2), pawn!(21, White)),
                (square!(G2), pawn!(22, White)),
                (square!(H2), pawn!(23, White)),
                (square!(A1), rook!(24, White)),
                (square!(B1), knight!(25, White)),
                (square!(C1), bishop!(26, White)),
                (square!(D1), queen!(27, White)),
                (square!(E1), king!(28, White)),
                (square!(F1), bishop!(29, White)),
                (square!(G1), knight!(30, White)),
                (square!(H1), rook!(31, White)),
            ]),
            attack_lines: HashMap::new(),
            kings: HashMap::from([(Color::White, square!(E1)), (Color::Black, square!(E8))]),
        };

        board.attack_lines = board
            .pieces
            .iter()
            .map(|(s, p)| (*s, p.get_attack_lines(*s)))
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
                    }
                }
            }
        }

        let initial_king_rank = if color == Color::White {
            Rank::_1
        } else {
            Rank::_8
        };

        if self
            .history
            .iter()
            .find(|m| m.from == Square::from(File::E, initial_king_rank))
            .is_none()
        {
            for castle_squares in vec![
                vec![
                    Square::from(File::A, initial_king_rank),
                    Square::from(File::B, initial_king_rank),
                    Square::from(File::C, initial_king_rank),
                    Square::from(File::D, initial_king_rank),
                ],
                vec![
                    Square::from(File::H, initial_king_rank),
                    Square::from(File::G, initial_king_rank),
                    Square::from(File::F, initial_king_rank),
                ],
            ] {
                if castle_squares[1..]
                    .iter()
                    .all(|s| self.pieces.get(s).is_none())
                    && self
                        .history
                        .iter()
                        .find(|m| m.from == *castle_squares.first().unwrap())
                        .is_none()
                {
                    let mut can_castle = true;
                    'square_loop: for square in Square::ALL {
                        if let Some(piece) = self.pieces.get(&square) {
                            if piece.color != color {
                                let attack_lines = piece.get_attack_lines(square);
                                for attack_line in attack_lines {
                                    if attack_line.iter().any(|s| castle_squares[1..].contains(s)) {
                                        for square in attack_line {
                                            if let Some(_) = self.pieces.get(&square) {
                                                can_castle = false;
                                                break 'square_loop;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if can_castle {
                        moves.push(Move::from_castle(
                            Square::from(File::E, initial_king_rank),
                            *castle_squares.get(castle_squares.len() - 2).unwrap(),
                        ))
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

    pub fn execute(&mut self, r#move: Move) {
        match r#move.r#type {
            MoveType::Normal => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Capture => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.pieces.remove(&r#move.to);
                self.attack_lines.remove(&r#move.from);
                self.attack_lines.remove(&r#move.to);

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Promotion => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::PromotionCapture => {
                let mut piece = self.pieces.remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();

                self.pieces.remove(&r#move.to);
                self.attack_lines.remove(&r#move.from);
                self.attack_lines.remove(&r#move.to);

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::PawnJump => {
                let piece = self.pieces.remove(&r#move.from).unwrap();

                self.attack_lines.remove(&r#move.from);

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
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

                let attack_lines = piece.get_attack_lines(r#move.to);
                self.attack_lines.insert(r#move.to, attack_lines);
                self.pieces.insert(r#move.to, piece);
            }
            MoveType::Castle => todo!(),
        }

        if let PieceType::King = self.pieces.get(&r#move.to).unwrap().r#type {
            if self.history.len() % 2 == 0 {
                *self.kings.get_mut(&Color::White).unwrap() = r#move.to;
            } else {
                *self.kings.get_mut(&Color::Black).unwrap() = r#move.to;
            }
        }

        self.history.push(r#move);
    }
}
