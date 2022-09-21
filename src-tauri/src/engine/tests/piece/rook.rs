use crate::{
    engine::{board::Board, r#move::Move},
    rook, square,
};

#[test]
fn normal_capture_no_team_capture() {
    let mut board = Board::new();
    board.pieces.insert(square!(H _4), rook!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(H _4), square!(H _5)),
            Move::from_normal(square!(H _4), square!(H _6)),
            Move::from_capture(square!(H _4), square!(H _7)),
            Move::from_normal(square!(H _4), square!(H _3)),
            Move::from_normal(square!(H _4), square!(G _4)),
            Move::from_normal(square!(H _4), square!(F _4)),
            Move::from_normal(square!(H _4), square!(E _4)),
            Move::from_normal(square!(H _4), square!(D _4)),
            Move::from_normal(square!(H _4), square!(C _4)),
            Move::from_normal(square!(H _4), square!(B _4)),
            Move::from_normal(square!(H _4), square!(A _4)),
        ],
        board.get_piece(square!(H _4)).unwrap().get_moves(&board)
    );
}
