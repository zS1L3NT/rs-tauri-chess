use indexmap::IndexMap;

use crate::engine::{
    piece::Piece,
    square::{Rank, Square},
};

use super::Board;

#[derive(Debug)]
pub enum FenError {
    InvalidFen,
    PiecePlacements,
    ActiveColor,
    CastlingAvailability,
    EnPassantTargetSquare,
    HalfMoveClock,
    FullMoveNumber,
}

impl Board {
    pub fn from_fen(fen: String) -> Result<Board, FenError> {
        let mut fen = fen.split_whitespace();
        let piece_placements = fen.next().ok_or(FenError::InvalidFen)?;
        let active_color = fen.next().ok_or(FenError::InvalidFen)?;
        let castling_rights = fen.next().ok_or(FenError::InvalidFen)?;
        let enpassant_target_square = fen.next().ok_or(FenError::InvalidFen)?;
        let halfmove_clock = fen.next().ok_or(FenError::InvalidFen)?;
        let fullmove_number = fen.next().ok_or(FenError::InvalidFen)?;
        if fen.next().is_some() {
            return Err(FenError::InvalidFen);
        }

        println!("{}", piece_placements);
        println!("{}", active_color);
        println!("{}", castling_rights);
        println!("{}", enpassant_target_square);
        println!("{}", halfmove_clock);
        println!("{}", fullmove_number);

        Ok(Board::new())
    }

    fn parse_piece_placement_rank(text: String, rank: Rank) -> IndexMap<Square, Piece> {
		todo!()
	}
}

#[test]
fn test() {
    Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".into()).unwrap();
    Board::from_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50".into()).unwrap();
}
