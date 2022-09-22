use crate::{
    bishop,
    engine::{board::Board, r#move::Move},
    square,
};

#[test]
fn normal_capture_no_team_capture() {
    let mut board = Board::new();
    board.pieces.insert(square!(H4), bishop!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(H4), square!(G3)),
            Move::from_normal(square!(H4), square!(G5)),
            Move::from_normal(square!(H4), square!(F6)),
            Move::from_capture(square!(H4), square!(E7)),
        ],
        board.get_piece(square!(H4)).unwrap().get_moves(&board)
    );
}
