use crate::{
    engine::{board::Board, piece::PieceType, r#move::Move},
    pawn, square,
};

#[test]
fn white_normal() {
    let mut board = Board::new();
    board.pieces.insert(square!(E4), pawn!(32, Black));

    assert_eq!(
        vec![Move::from_normal(square!(E2), square!(E3))],
        board.get_piece(square!(E2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_normal() {
    let mut board = Board::new();
    board.pieces.insert(square!(E5), pawn!(32, White));

    assert_eq!(
        vec![Move::from_normal(square!(E7), square!(E6))],
        board.get_piece(square!(E7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_jump() {
    let board = Board::new();

    assert_eq!(
        vec![
            Move::from_pawn_jump(square!(E2), square!(E4)),
            Move::from_normal(square!(E2), square!(E3)),
        ],
        board.get_piece(square!(E2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_normal_jump() {
    let board = Board::new();

    assert_eq!(
        vec![
            Move::from_pawn_jump(square!(E7), square!(E5)),
            Move::from_normal(square!(E7), square!(E6)),
        ],
        board.get_piece(square!(E7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_no_team_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D3), pawn!(32, White));
    board.pieces.insert(square!(E3), pawn!(33, White));
    board.pieces.insert(square!(F3), pawn!(34, White));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_no_team_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D6), pawn!(32, Black));
    board.pieces.insert(square!(E6), pawn!(33, Black));
    board.pieces.insert(square!(F6), pawn!(34, Black));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D3), pawn!(32, Black));
    board.pieces.insert(square!(E3), pawn!(33, Black));
    board.pieces.insert(square!(F3), pawn!(34, Black));

    assert_eq!(
        vec![
            Move::from_capture(square!(E2), square!(D3)),
            Move::from_capture(square!(E2), square!(F3)),
        ],
        board.get_piece(square!(E2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D6), pawn!(32, White));
    board.pieces.insert(square!(E6), pawn!(33, White));
    board.pieces.insert(square!(F6), pawn!(34, White));

    assert_eq!(
        vec![
            Move::from_capture(square!(E7), square!(D6)),
            Move::from_capture(square!(E7), square!(F6)),
        ],
        board.get_piece(square!(E7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_jump_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D3), pawn!(32, Black));
    board.pieces.insert(square!(F3), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::from_pawn_jump(square!(E2), square!(E4)),
            Move::from_capture(square!(E2), square!(D3)),
            Move::from_normal(square!(E2), square!(E3)),
            Move::from_capture(square!(E2), square!(F3)),
        ],
        board.get_piece(square!(E2)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_normal_jump_captures() {
    let mut board = Board::new();
    board.pieces.insert(square!(D6), pawn!(32, White));
    board.pieces.insert(square!(F6), pawn!(33, White));

    assert_eq!(
        vec![
            Move::from_pawn_jump(square!(E7), square!(E5)),
            Move::from_capture(square!(E7), square!(D6)),
            Move::from_normal(square!(E7), square!(E6)),
            Move::from_capture(square!(E7), square!(F6)),
        ],
        board.get_piece(square!(E7)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_normal_enpassent() {
    let mut board = Board::new();
    board.enpassent_square = Some(square!(D5));
    board.pieces.insert(square!(D5), pawn!(32, Black));
    board.pieces.insert(square!(E5), pawn!(33, White));

    assert_eq!(
        vec![
            Move::from_enpassant(square!(E5), square!(D6)),
            Move::from_normal(square!(E5), square!(E6)),
        ],
        board.get_piece(square!(E5)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_normal_enpassent() {
    let mut board = Board::new();
    board.enpassent_square = Some(square!(D4));
    board.pieces.insert(square!(D4), pawn!(32, White));
    board.pieces.insert(square!(E4), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::from_enpassant(square!(E4), square!(D3)),
            Move::from_normal(square!(E4), square!(E3)),
        ],
        board.get_piece(square!(E4)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_fake_enpassent() {
    let mut board = Board::new();
    board.pieces.insert(square!(D5), pawn!(32, Black));
    board.pieces.insert(square!(E5), pawn!(33, White));
    board.pieces.insert(square!(E6), pawn!(34, Black));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E5)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_fake_enpassent() {
    let mut board = Board::new();
    board.pieces.insert(square!(D4), pawn!(32, White));
    board.pieces.insert(square!(E4), pawn!(33, Black));
    board.pieces.insert(square!(E3), pawn!(34, White));

    assert_eq!(
        vec![] as Vec<Move>,
        board.get_piece(square!(E4)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_tiple_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A8), pawn!(32, Black));
    board.pieces.insert(square!(C8), pawn!(33, Black));
    board.pieces.insert(square!(B7), pawn!(34, White));

    assert_eq!(
        vec![
            Move::from_promotion_capture(square!(B7), square!(A8), PieceType::Queen, true),
            Move::from_promotion_capture(square!(B7), square!(A8), PieceType::Rook, true),
            Move::from_promotion_capture(square!(B7), square!(A8), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(B7), square!(A8), PieceType::Knight, false),
            Move::from_promotion(square!(B7), square!(B8), PieceType::Queen, true),
            Move::from_promotion(square!(B7), square!(B8), PieceType::Rook, true),
            Move::from_promotion(square!(B7), square!(B8), PieceType::Bishop, true),
            Move::from_promotion(square!(B7), square!(B8), PieceType::Knight, false),
            Move::from_promotion_capture(square!(B7), square!(C8), PieceType::Queen, true),
            Move::from_promotion_capture(square!(B7), square!(C8), PieceType::Rook, true),
            Move::from_promotion_capture(square!(B7), square!(C8), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(B7), square!(C8), PieceType::Knight, false),
        ],
        board.get_piece(square!(B7)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_tiple_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A1), pawn!(32, White));
    board.pieces.insert(square!(C1), pawn!(33, White));
    board.pieces.insert(square!(B2), pawn!(34, Black));

    assert_eq!(
        vec![
            Move::from_promotion_capture(square!(B2), square!(A1), PieceType::Queen, true),
            Move::from_promotion_capture(square!(B2), square!(A1), PieceType::Rook, true),
            Move::from_promotion_capture(square!(B2), square!(A1), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(B2), square!(A1), PieceType::Knight, false),
            Move::from_promotion(square!(B2), square!(B1), PieceType::Queen, true),
            Move::from_promotion(square!(B2), square!(B1), PieceType::Rook, true),
            Move::from_promotion(square!(B2), square!(B1), PieceType::Bishop, true),
            Move::from_promotion(square!(B2), square!(B1), PieceType::Knight, false),
            Move::from_promotion_capture(square!(B2), square!(C1), PieceType::Queen, true),
            Move::from_promotion_capture(square!(B2), square!(C1), PieceType::Rook, true),
            Move::from_promotion_capture(square!(B2), square!(C1), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(B2), square!(C1), PieceType::Knight, false),
        ],
        board.get_piece(square!(B2)).unwrap().get_moves(&board)
    );
}

#[test]
fn white_side_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A7), pawn!(32, White));
    board.pieces.insert(square!(B8), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::from_promotion(square!(A7), square!(A8), PieceType::Queen, true),
            Move::from_promotion(square!(A7), square!(A8), PieceType::Rook, true),
            Move::from_promotion(square!(A7), square!(A8), PieceType::Bishop, true),
            Move::from_promotion(square!(A7), square!(A8), PieceType::Knight, false),
            Move::from_promotion_capture(square!(A7), square!(B8), PieceType::Queen, true),
            Move::from_promotion_capture(square!(A7), square!(B8), PieceType::Rook, true),
            Move::from_promotion_capture(square!(A7), square!(B8), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(A7), square!(B8), PieceType::Knight, false),
        ],
        board.get_piece(square!(A7)).unwrap().get_moves(&board)
    );
}

#[test]
fn black_side_promotion() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A2), pawn!(32, Black));
    board.pieces.insert(square!(B1), pawn!(33, White));

    assert_eq!(
        vec![
            Move::from_promotion(square!(A2), square!(A1), PieceType::Queen, true),
            Move::from_promotion(square!(A2), square!(A1), PieceType::Rook, true),
            Move::from_promotion(square!(A2), square!(A1), PieceType::Bishop, true),
            Move::from_promotion(square!(A2), square!(A1), PieceType::Knight, false),
            Move::from_promotion_capture(square!(A2), square!(B1), PieceType::Queen, true),
            Move::from_promotion_capture(square!(A2), square!(B1), PieceType::Rook, true),
            Move::from_promotion_capture(square!(A2), square!(B1), PieceType::Bishop, true),
            Move::from_promotion_capture(square!(A2), square!(B1), PieceType::Knight, false),
        ],
        board.get_piece(square!(A2)).unwrap().get_moves(&board)
    );
}
