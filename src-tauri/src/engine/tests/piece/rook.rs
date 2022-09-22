use crate::{
    engine::{board::Board, r#move::Move},
    rook, square,
};

#[test]
fn normal_capture_no_team_capture() {
    let mut board = Board::new();
    board.pieces.insert(square!(H4), rook!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(H4), square!(H5)),
            Move::from_normal(square!(H4), square!(H6)),
            Move::from_capture(square!(H4), square!(H7)),
            Move::from_normal(square!(H4), square!(H3)),
            Move::from_normal(square!(H4), square!(G4)),
            Move::from_normal(square!(H4), square!(F4)),
            Move::from_normal(square!(H4), square!(E4)),
            Move::from_normal(square!(H4), square!(D4)),
            Move::from_normal(square!(H4), square!(C4)),
            Move::from_normal(square!(H4), square!(B4)),
            Move::from_normal(square!(H4), square!(A4)),
        ],
        board.get_piece(square!(H4)).unwrap().get_moves(&board)
    );
}
