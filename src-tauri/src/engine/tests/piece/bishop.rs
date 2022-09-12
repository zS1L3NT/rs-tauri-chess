use crate::{
    bishop,
    engine::{board::Board, r#move::Move},
    square,
};

#[test]
fn normal_capture_no_team_capture() {
    let mut board = Board::new();
    board.pieces.insert(square!(H _4), bishop!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(H _4), square!(G _3)),
            Move::from_normal(square!(H _4), square!(G _5)),
            Move::from_normal(square!(H _4), square!(F _6)),
            Move::from_capture(square!(H _4), square!(E _7)),
        ],
        board.get_piece(square!(H _4)).unwrap().get_moves(&board)
    );
}
