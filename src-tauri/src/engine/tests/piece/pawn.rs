use crate::{
    engine::{
        board::Board,
        color::Color,
        piece::{Piece, PieceType},
        r#move::Move,
    },
    pawn, square,
};

#[test]
fn normal() {
    let mut board = Board::new();
    board.pieces.insert(square!(E _4), pawn!(32, Black));

    assert_eq!(
        vec![Move::fromNormal(square!(E _2), square!(E _3))],
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn normal_jump() {
    let board = Board::new();

    assert_eq!(
        vec![
            Move::fromPawnJump(square!(E _2), square!(E _4)),
            Move::fromNormal(square!(E _2), square!(E _3)),
        ],
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn no_team_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _3), pawn!(32, White));
    board.pieces.insert(square!(E _3), pawn!(33, White));
    board.pieces.insert(square!(F _3), pawn!(34, White));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _3), pawn!(32, Black));
    board.pieces.insert(square!(E _3), pawn!(33, Black));
    board.pieces.insert(square!(F _3), pawn!(34, Black));

    assert_eq!(
        vec![
            Move::fromCapture(square!(E _2), square!(D _3)),
            Move::fromCapture(square!(E _2), square!(F _3)),
        ],
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn normal_jump_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _3), pawn!(32, Black));
    board.pieces.insert(square!(F _3), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::fromPawnJump(square!(E _2), square!(E _4)),
            Move::fromCapture(square!(E _2), square!(D _3)),
            Move::fromNormal(square!(E _2), square!(E _3)),
            Move::fromCapture(square!(E _2), square!(F _3)),
        ],
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn normal_enpassent() {
    let mut board = Board::new();
    board.enpassent_square = Some(square!(D _5));
    board.pieces.insert(square!(D _5), pawn!(32, Black));
    board.pieces.insert(square!(E _5), pawn!(33, White));

    assert_eq!(
        vec![
            Move::fromEnpassent(square!(E _5), square!(D _6)),
            Move::fromNormal(square!(E _5), square!(E _6)),
        ],
        board.get_piece(square!(E _5)).unwrap().get_moves(&board)
    );
}

#[test]
fn fake_enpassent() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _5), pawn!(32, Black));
    board.pieces.insert(square!(E _5), pawn!(33, White));
    board.pieces.insert(square!(E _6), pawn!(34, Black));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E _5)).unwrap().get_moves(&board)
    );
}

#[test]
fn tiple_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _8), pawn!(32, Black));
    board.pieces.insert(square!(C _8), pawn!(33, Black));
    board.pieces.insert(square!(B _7), pawn!(34, White));

    assert_eq!(
        vec![
            Move::fromPromotionCapture(square!(B _7), square!(A _8), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(B _7), square!(A _8), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(B _7), square!(A _8), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(B _7), square!(A _8), PieceType::Knight, false),
            Move::fromPromotion(square!(B _7), square!(B _8), PieceType::Queen, true),
            Move::fromPromotion(square!(B _7), square!(B _8), PieceType::Rook, true),
            Move::fromPromotion(square!(B _7), square!(B _8), PieceType::Bishop, true),
            Move::fromPromotion(square!(B _7), square!(B _8), PieceType::Knight, false),
            Move::fromPromotionCapture(square!(B _7), square!(C _8), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(B _7), square!(C _8), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(B _7), square!(C _8), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(B _7), square!(C _8), PieceType::Knight, false),
        ],
        board.get_piece(square!(B _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn side_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _7), pawn!(32, White));
    board.pieces.insert(square!(B _8), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::fromPromotion(square!(A _7), square!(A _8), PieceType::Queen, true),
            Move::fromPromotion(square!(A _7), square!(A _8), PieceType::Rook, true),
            Move::fromPromotion(square!(A _7), square!(A _8), PieceType::Bishop, true),
            Move::fromPromotion(square!(A _7), square!(A _8), PieceType::Knight, false),
            Move::fromPromotionCapture(square!(A _7), square!(B _8), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(A _7), square!(B _8), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(A _7), square!(B _8), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(A _7), square!(B _8), PieceType::Knight, false),
        ],
        board.get_piece(square!(A _7)).unwrap().get_moves(&board)
    );
}
