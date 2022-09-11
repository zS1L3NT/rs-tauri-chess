use crate::{
    engine::{board::Board, piece::PieceType, r#move::Move},
    pawn, square,
};

#[test]
fn white_normal() {
    let mut board = Board::new();
    board.pieces.insert(square!(E _4), pawn!(32, Black));

    assert_eq!(
        vec![Move::fromNormal(square!(E _2), square!(E _3))],
        board.get_piece(square!(E _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_normal() {
    let mut board = Board::new();
    board.pieces.insert(square!(E _5), pawn!(32, White));

    assert_eq!(
        vec![Move::fromNormal(square!(E _7), square!(E _6))],
        board.get_piece(square!(E _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_jump() {
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
fn black_normal_jump() {
    let board = Board::new();

    assert_eq!(
        vec![
            Move::fromPawnJump(square!(E _7), square!(E _5)),
            Move::fromNormal(square!(E _7), square!(E _6)),
        ],
        board.get_piece(square!(E _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_no_team_captures() {
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
fn black_no_team_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _6), pawn!(32, Black));
    board.pieces.insert(square!(E _6), pawn!(33, Black));
    board.pieces.insert(square!(F _6), pawn!(34, Black));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_captures() {
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
fn black_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _6), pawn!(32, White));
    board.pieces.insert(square!(E _6), pawn!(33, White));
    board.pieces.insert(square!(F _6), pawn!(34, White));

    assert_eq!(
        vec![
            Move::fromCapture(square!(E _7), square!(D _6)),
            Move::fromCapture(square!(E _7), square!(F _6)),
        ],
        board.get_piece(square!(E _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_jump_captures() {
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
fn black_normal_jump_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _6), pawn!(32, White));
    board.pieces.insert(square!(F _6), pawn!(33, White));

    assert_eq!(
        vec![
            Move::fromPawnJump(square!(E _7), square!(E _5)),
            Move::fromCapture(square!(E _7), square!(D _6)),
            Move::fromNormal(square!(E _7), square!(E _6)),
            Move::fromCapture(square!(E _7), square!(F _6)),
        ],
        board.get_piece(square!(E _7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_enpassent() {
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
fn black_normal_enpassent() {
    let mut board = Board::new();
    board.enpassent_square = Some(square!(D _4));
    board.pieces.insert(square!(D _4), pawn!(32, White));
    board.pieces.insert(square!(E _4), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::fromEnpassent(square!(E _4), square!(D _3)),
            Move::fromNormal(square!(E _4), square!(E _3)),
        ],
        board.get_piece(square!(E _4)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_fake_enpassent() {
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
fn black_fake_enpassent() {
    let mut board = Board::new();
    board.pieces.insert(square!(D _4), pawn!(32, White));
    board.pieces.insert(square!(E _4), pawn!(33, Black));
    board.pieces.insert(square!(E _3), pawn!(34, White));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E _4)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_tiple_promotion() {
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
fn black_tiple_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _1), pawn!(32, White));
    board.pieces.insert(square!(C _1), pawn!(33, White));
    board.pieces.insert(square!(B _2), pawn!(34, Black));

    assert_eq!(
        vec![
            Move::fromPromotionCapture(square!(B _2), square!(A _1), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(B _2), square!(A _1), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(B _2), square!(A _1), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(B _2), square!(A _1), PieceType::Knight, false),
            Move::fromPromotion(square!(B _2), square!(B _1), PieceType::Queen, true),
            Move::fromPromotion(square!(B _2), square!(B _1), PieceType::Rook, true),
            Move::fromPromotion(square!(B _2), square!(B _1), PieceType::Bishop, true),
            Move::fromPromotion(square!(B _2), square!(B _1), PieceType::Knight, false),
            Move::fromPromotionCapture(square!(B _2), square!(C _1), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(B _2), square!(C _1), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(B _2), square!(C _1), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(B _2), square!(C _1), PieceType::Knight, false),
        ],
        board.get_piece(square!(B _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_side_promotion() {
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

#[test]
fn black_side_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _2), pawn!(32, Black));
    board.pieces.insert(square!(B _1), pawn!(33, White));

    assert_eq!(
        vec![
            Move::fromPromotion(square!(A _2), square!(A _1), PieceType::Queen, true),
            Move::fromPromotion(square!(A _2), square!(A _1), PieceType::Rook, true),
            Move::fromPromotion(square!(A _2), square!(A _1), PieceType::Bishop, true),
            Move::fromPromotion(square!(A _2), square!(A _1), PieceType::Knight, false),
            Move::fromPromotionCapture(square!(A _2), square!(B _1), PieceType::Queen, true),
            Move::fromPromotionCapture(square!(A _2), square!(B _1), PieceType::Rook, true),
            Move::fromPromotionCapture(square!(A _2), square!(B _1), PieceType::Bishop, true),
            Move::fromPromotionCapture(square!(A _2), square!(B _1), PieceType::Knight, false),
        ],
        board.get_piece(square!(A _2)).unwrap().get_moves(&board)
    );
}
