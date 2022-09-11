use std::collections::HashMap;

use super::{
    color::Color,
    piece::{Piece, PieceType},
    r#move::Move,
    square::{File, Rank, Square},
};

pub struct Board {
    pieces: HashMap<Square, Piece>,
    threats: HashMap<Color, Vec<Square>>,
	pub enpassent_square: Option<Square>
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: HashMap::from([
                (
                    Square::from(File::A, Rank::_8),
                    Piece::new(0, PieceType::Rook, Color::Black),
                ),
                (
                    Square::from(File::B, Rank::_8),
                    Piece::new(1, PieceType::Knight, Color::Black),
                ),
                (
                    Square::from(File::C, Rank::_8),
                    Piece::new(2, PieceType::Bishop, Color::Black),
                ),
                (
                    Square::from(File::D, Rank::_8),
                    Piece::new(3, PieceType::Queen, Color::Black),
                ),
                (
                    Square::from(File::E, Rank::_8),
                    Piece::new(4, PieceType::King, Color::Black),
                ),
                (
                    Square::from(File::F, Rank::_8),
                    Piece::new(5, PieceType::Bishop, Color::Black),
                ),
                (
                    Square::from(File::G, Rank::_8),
                    Piece::new(6, PieceType::Knight, Color::Black),
                ),
                (
                    Square::from(File::H, Rank::_8),
                    Piece::new(7, PieceType::Rook, Color::Black),
                ),
                (
                    Square::from(File::A, Rank::_7),
                    Piece::new(8, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::B, Rank::_7),
                    Piece::new(9, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::C, Rank::_7),
                    Piece::new(10, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::D, Rank::_7),
                    Piece::new(11, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::E, Rank::_7),
                    Piece::new(12, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::F, Rank::_7),
                    Piece::new(13, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::G, Rank::_7),
                    Piece::new(14, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::H, Rank::_7),
                    Piece::new(15, PieceType::Pawn, Color::Black),
                ),
                (
                    Square::from(File::A, Rank::_2),
                    Piece::new(16, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::B, Rank::_2),
                    Piece::new(17, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::C, Rank::_2),
                    Piece::new(18, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::D, Rank::_2),
                    Piece::new(19, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::E, Rank::_2),
                    Piece::new(20, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::F, Rank::_2),
                    Piece::new(21, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::G, Rank::_2),
                    Piece::new(22, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::H, Rank::_2),
                    Piece::new(23, PieceType::Pawn, Color::White),
                ),
                (
                    Square::from(File::A, Rank::_1),
                    Piece::new(24, PieceType::Rook, Color::White),
                ),
                (
                    Square::from(File::B, Rank::_1),
                    Piece::new(25, PieceType::Knight, Color::White),
                ),
                (
                    Square::from(File::C, Rank::_1),
                    Piece::new(26, PieceType::Bishop, Color::White),
                ),
                (
                    Square::from(File::D, Rank::_1),
                    Piece::new(27, PieceType::Queen, Color::White),
                ),
                (
                    Square::from(File::E, Rank::_1),
                    Piece::new(28, PieceType::King, Color::White),
                ),
                (
                    Square::from(File::F, Rank::_1),
                    Piece::new(29, PieceType::Bishop, Color::White),
                ),
                (
                    Square::from(File::G, Rank::_1),
                    Piece::new(30, PieceType::Knight, Color::White),
                ),
                (
                    Square::from(File::H, Rank::_1),
                    Piece::new(31, PieceType::Rook, Color::White),
                ),
            ]),
            threats: HashMap::from([(Color::White, vec![]), (Color::Black, vec![])]),
			enpassent_square: None
        }
    }

    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        self.pieces.get(&square)
    }

    pub fn get_square(&self, piece: &Piece) -> Square {
        self.pieces
            .iter()
            .find(|(_, p)| p.id == piece.id)
            .map(|(s, _)| *s)
            .expect("Could not find piece on board")
    }

    pub fn execute(r#move: Move) {}

	pub fn determine_threat(&self, piece: &Piece) -> bool {
		todo!()
	}
}
