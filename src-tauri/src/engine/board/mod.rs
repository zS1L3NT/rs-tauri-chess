mod _execute;
mod _from_fen;
mod _get_moves;
mod _to_fen;
mod _undo;
mod castling_rights;

use {
    crate::{
        bishop,
        engine::{
            client::{ClientBoard, ClientPiece},
            color::*,
            piece::*,
            r#move::*,
            square::Square,
        },
        king, knight, pawn, queen, rook,
    },
    castling_rights::CastlingRights,
    indexmap::{indexmap, IndexMap},
    rs_tauri_chess::square,
};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub history: Vec<Move>,
    pub pieces: IndexMap<Square, Piece>,
    pub attack_lines: IndexMap<Square, Vec<Vec<Square>>>,
    pub kings: IndexMap<Color, Square>,

    pub turn: Color,
    pub castling_rights: IndexMap<Color, CastlingRights>,
    pub enpassant_square: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            history: vec![],
            pieces: indexmap! {
                square!(A8) => rook!(0, Black),
                square!(B8) => knight!(1, Black),
                square!(C8) => bishop!(2, Black),
                square!(D8) => queen!(3, Black),
                square!(E8) => king!(4, Black),
                square!(F8) => bishop!(5, Black),
                square!(G8) => knight!(6, Black),
                square!(H8) => rook!(7, Black),
                square!(A7) => pawn!(8, Black),
                square!(B7) => pawn!(9, Black),
                square!(C7) => pawn!(10, Black),
                square!(D7) => pawn!(11, Black),
                square!(E7) => pawn!(12, Black),
                square!(F7) => pawn!(13, Black),
                square!(G7) => pawn!(14, Black),
                square!(H7) => pawn!(15, Black),
                square!(A2) => pawn!(16, White),
                square!(B2) => pawn!(17, White),
                square!(C2) => pawn!(18, White),
                square!(D2) => pawn!(19, White),
                square!(E2) => pawn!(20, White),
                square!(F2) => pawn!(21, White),
                square!(G2) => pawn!(22, White),
                square!(H2) => pawn!(23, White),
                square!(A1) => rook!(24, White),
                square!(B1) => knight!(25, White),
                square!(C1) => bishop!(26, White),
                square!(D1) => queen!(27, White),
                square!(E1) => king!(28, White),
                square!(F1) => bishop!(29, White),
                square!(G1) => knight!(30, White),
                square!(H1) => rook!(31, White),
            },
            attack_lines: indexmap! {},
            kings: indexmap! {
                White => square!(E1),
                Black => square!(E8)
            },

            turn: White,
            castling_rights: indexmap! {
                White => CastlingRights::new(true, true),
                Black => CastlingRights::new(true, true),
            },
            enpassant_square: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        };

        board.attack_lines = board
            .pieces
            .iter()
            .map(|(s, p)| (*s, p.get_attack_lines(*s)))
            .collect::<IndexMap<_, _>>();

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
}
