use indexmap::{indexmap, IndexMap};
use rs_tauri_chess::square;

use crate::{
    bishop,
    engine::{
        color::Color,
        piece::{Piece, PieceType},
        square::{File, Rank, Square},
    },
    king, knight, pawn, queen, rook,
};

use super::{castling_rights::CastlingRights, Board};

#[derive(Debug)]
pub enum FenError {
    InvalidFen,
    PiecePlacements,
    ActiveColor,
    CastlingRights,
    EnPassantSquare,
    HalfMoveClock,
    FullMoveNumber,
}

impl Board {
    pub fn from_fen(fen: String) -> Result<Board, FenError> {
        let mut fen = fen.split_whitespace();
        let piece_placements = fen.next().ok_or(FenError::InvalidFen)?;
        let active_color = fen.next().ok_or(FenError::InvalidFen)?;
        let castling_rights = fen.next().ok_or(FenError::InvalidFen)?;
        let enpassant_square = fen.next().ok_or(FenError::InvalidFen)?;
        let halfmove_clock = fen.next().ok_or(FenError::InvalidFen)?;
        let fullmove_number = fen.next().ok_or(FenError::InvalidFen)?;
        if fen.next().is_some() {
            return Err(FenError::InvalidFen);
        }

        let pieces = Board::parse_piece_placements(piece_placements)?;
        let active_color = Board::parse_active_color(active_color)?;
        let castling_rights = Board::parse_castling_rights(castling_rights)?;
        let enpassant_square = Board::parse_enpassant_square(enpassant_square)?;
        let halfmove_clock = Board::parse_halfmove_clock(halfmove_clock)?;
        let fullmove_number = Board::parse_fullmove_number(fullmove_number)?;

        for (color, castling_rights) in &castling_rights {
            let king = pieces.get(
                &(if *color == Color::White {
                    square!(E1)
                } else {
                    square!(E8)
                }),
            );

            let rook = pieces.get(
                &(if *color == Color::White {
                    square!(H1)
                } else {
                    square!(H8)
                }),
            );
            if castling_rights.kingside {
                if king.is_some() && rook.is_some() {
                    let king = king.unwrap();
                    let rook = rook.unwrap();

                    if king.r#type == PieceType::King
                        || rook.r#type == PieceType::Rook
                        || king.color == *color
                        || rook.color == *color
                    {
                        continue;
                    }
                }

                return Err(FenError::CastlingRights);
            }

			let rook = pieces.get(
                &(if *color == Color::White {
                    square!(A1)
                } else {
                    square!(A8)
                }),
            );
            if castling_rights.queenside {
                if king.is_some() && rook.is_some() {
                    let king = king.unwrap();
                    let rook = rook.unwrap();

                    if king.r#type == PieceType::King
                        || rook.r#type == PieceType::Rook
                        || king.color == *color
                        || rook.color == *color
                    {
                        continue;
                    }
                }

                return Err(FenError::CastlingRights);
            }
		}

        if let Some(enpassant_square) = enpassant_square {
            if enpassant_square.rank != Rank::_3 && enpassant_square.rank != Rank::_6 {
                return Err(FenError::EnPassantSquare);
            }
        }

        if !(halfmove_clock
            <= ((fullmove_number - 1) * 2) + (if active_color == Color::Black { 1 } else { 0 }))
        {
            return Err(FenError::HalfMoveClock);
        }

        let kings = indexmap! {
            Color::White => pieces.iter().find_map(|(s, p)| if p.r#type == PieceType::King && p.color == Color::White { Some(*s) } else {None} ).unwrap(),
            Color::Black => pieces.iter().find_map(|(s, p)| if p.r#type == PieceType::King && p.color == Color::Black { Some(*s) } else {None} ).unwrap(),
        };

        let mut board = Board {
            history: vec![],
            pieces,
            attack_lines: indexmap! {},
            kings,

            turn: active_color,
            castling_rights,
            enpassant_square,
            halfmove_clock,
            fullmove_number,
        };

        board.attack_lines = board
            .pieces
            .iter()
            .map(|(s, p)| (*s, p.get_attack_lines(*s)))
            .collect::<IndexMap<_, _>>();

        Ok(board)
    }

    fn parse_piece_placements(text: &str) -> Result<IndexMap<Square, Piece>, FenError> {
        let mut pieces = indexmap! {};

        let mut rank_index: i8 = 7;
        let mut piece_id = 0;

        for part in text.split("/") {
            let mut file_index = 0;
            for char in part.chars().into_iter() {
                if char.is_numeric() {
                    file_index += char.to_digit(10).unwrap() as i8;
                } else {
                    let file = File::from_index(file_index).unwrap();
                    let rank = Rank::from_index(rank_index).unwrap();
                    let square = square!(file rank);

                    match char {
                        'p' => pieces.insert(square, pawn!(piece_id, Black)),
                        'P' => pieces.insert(square, pawn!(piece_id, White)),
                        'n' => pieces.insert(square, knight!(piece_id, Black)),
                        'N' => pieces.insert(square, knight!(piece_id, White)),
                        'b' => pieces.insert(square, bishop!(piece_id, Black)),
                        'B' => pieces.insert(square, bishop!(piece_id, White)),
                        'r' => pieces.insert(square, rook!(piece_id, Black)),
                        'R' => pieces.insert(square, rook!(piece_id, White)),
                        'q' => pieces.insert(square, queen!(piece_id, Black)),
                        'Q' => pieces.insert(square, queen!(piece_id, White)),
                        'k' => pieces.insert(square, king!(piece_id, Black)),
                        'K' => pieces.insert(square, king!(piece_id, White)),
                        _ => return Err(FenError::PiecePlacements),
                    };
                    piece_id += 1;
                    file_index += 1;
                }
            }

            rank_index -= 1;
            if file_index != 8 {
                return Err(FenError::PiecePlacements);
            }
        }

        if rank_index != -1 {
            return Err(FenError::PiecePlacements);
        }

        if pieces
            .values()
            .filter(|p| p.r#type == PieceType::King && p.color == Color::White)
            .count()
            != 1
        {
            return Err(FenError::PiecePlacements);
        }

        if pieces
            .values()
            .filter(|p| p.r#type == PieceType::King && p.color == Color::Black)
            .count()
            != 1
        {
            return Err(FenError::PiecePlacements);
        }

        Ok(pieces)
    }

    fn parse_active_color(text: &str) -> Result<Color, FenError> {
        match text {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => return Err(FenError::ActiveColor),
        }
    }

    fn parse_castling_rights(text: &str) -> Result<IndexMap<Color, CastlingRights>, FenError> {
        let mut castling_rights = indexmap! {};
        let mut white_kingside = false;
        let mut white_queenside = false;
        let mut black_kingside = false;
        let mut black_queenside = false;

        for char in text.chars().into_iter() {
            match char {
                'K' => white_kingside = true,
                'Q' => white_queenside = true,
                'k' => black_kingside = true,
                'q' => black_queenside = true,
                '-' => (),
                _ => return Err(FenError::CastlingRights),
            }
        }

        castling_rights.insert(
            Color::White,
            CastlingRights::new(white_kingside, white_queenside),
        );
        castling_rights.insert(
            Color::Black,
            CastlingRights::new(black_kingside, black_queenside),
        );

        Ok(castling_rights)
    }

    fn parse_enpassant_square(text: &str) -> Result<Option<Square>, FenError> {
        match text {
            "-" => Ok(None),
            _ => {
                let mut chars = text.chars();

                let file = match chars.next() {
                    Some('a') => File::A,
                    Some('b') => File::B,
                    Some('c') => File::C,
                    Some('d') => File::D,
                    Some('e') => File::E,
                    Some('f') => File::F,
                    Some('g') => File::G,
                    Some('h') => File::H,
                    None | Some(_) => return Err(FenError::EnPassantSquare),
                };

                let rank = match chars.next() {
                    Some('1') => Rank::_1,
                    Some('2') => Rank::_2,
                    Some('3') => Rank::_3,
                    Some('4') => Rank::_4,
                    Some('5') => Rank::_5,
                    Some('6') => Rank::_6,
                    Some('7') => Rank::_7,
                    Some('8') => Rank::_8,
                    None | Some(_) => return Err(FenError::EnPassantSquare),
                };

                if chars.next().is_some() {
                    return Err(FenError::EnPassantSquare);
                }

                Ok(Some(square!(file rank)))
            }
        }
    }

    fn parse_halfmove_clock(text: &str) -> Result<u32, FenError> {
        match text.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(FenError::HalfMoveClock),
        }
    }

    fn parse_fullmove_number(text: &str) -> Result<u32, FenError> {
        match text.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(FenError::FullMoveNumber),
        }
    }
}
