use crate::{
    engine::{board::Board, r#move::Move},
    knight, pawn, square,
};

#[test]
fn a1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _1), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(A _1), square!(B _3)),
            Move::fromNormal(square!(A _1), square!(C _2)),
        ],
        board.get_piece(square!(A _1)).unwrap().get_moves(&board)
    );
}

#[test]
fn a2_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _2), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(A _2), square!(B _4)),
            Move::fromNormal(square!(A _2), square!(C _3)),
            Move::fromNormal(square!(A _2), square!(C _1)),
        ],
        board.get_piece(square!(A _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn b1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B _1), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(B _1), square!(C _3)),
            Move::fromNormal(square!(B _1), square!(D _2)),
            Move::fromNormal(square!(B _1), square!(A _3)),
        ],
        board.get_piece(square!(B _1)).unwrap().get_moves(&board)
    );
}

#[test]
fn b2_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B _2), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(B _2), square!(C _4)),
            Move::fromNormal(square!(B _2), square!(D _3)),
            Move::fromNormal(square!(B _2), square!(D _1)),
            Move::fromNormal(square!(B _2), square!(A _4)),
        ],
        board.get_piece(square!(B _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn a3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _3), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(A _3), square!(B _5)),
            Move::fromNormal(square!(A _3), square!(C _4)),
            Move::fromNormal(square!(A _3), square!(C _2)),
            Move::fromNormal(square!(A _3), square!(B _1)),
        ],
        board.get_piece(square!(A _3)).unwrap().get_moves(&board)
    );
}

#[test]
fn b3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B _3), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(B _3), square!(C _5)),
            Move::fromNormal(square!(B _3), square!(D _4)),
            Move::fromNormal(square!(B _3), square!(D _2)),
            Move::fromNormal(square!(B _3), square!(C _1)),
            Move::fromNormal(square!(B _3), square!(A _1)),
            Move::fromNormal(square!(B _3), square!(A _5)),
        ],
        board.get_piece(square!(B _3)).unwrap().get_moves(&board)
    );
}

#[test]
fn c3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(C _3), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(C _3), square!(D _5)),
            Move::fromNormal(square!(C _3), square!(E _4)),
            Move::fromNormal(square!(C _3), square!(E _2)),
            Move::fromNormal(square!(C _3), square!(D _1)),
            Move::fromNormal(square!(C _3), square!(B _1)),
            Move::fromNormal(square!(C _3), square!(A _2)),
            Move::fromNormal(square!(C _3), square!(A _4)),
            Move::fromNormal(square!(C _3), square!(B _5)),
        ],
        board.get_piece(square!(C _3)).unwrap().get_moves(&board)
    );
}

#[test]
fn c2_corner() {
    let mut board = Board::empty();
    let king = board.pieces.remove(&square!(E _1)).unwrap();
    board.pieces.insert(square!(F _1), king);
    board.pieces.insert(square!(C _2), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(C _2), square!(D _4)),
            Move::fromNormal(square!(C _2), square!(E _3)),
            Move::fromNormal(square!(C _2), square!(E _1)),
            Move::fromNormal(square!(C _2), square!(A _1)),
            Move::fromNormal(square!(C _2), square!(A _3)),
            Move::fromNormal(square!(C _2), square!(B _4)),
        ],
        board.get_piece(square!(C _2)).unwrap().get_moves(&board)
    );
}

#[test]
fn c1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(C _1), knight!(32, White));

    assert_eq!(
        vec![
            Move::fromNormal(square!(C _1), square!(D _3)),
            Move::fromNormal(square!(C _1), square!(E _2)),
            Move::fromNormal(square!(C _1), square!(A _2)),
            Move::fromNormal(square!(C _1), square!(B _3)),
        ],
        board.get_piece(square!(C _1)).unwrap().get_moves(&board)
    );
}

#[test]
fn captures() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _1), knight!(32, White));
    board.pieces.insert(square!(B _3), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::fromCapture(square!(A _1), square!(B _3)),
            Move::fromNormal(square!(A _1), square!(C _2)),
        ],
        board.get_piece(square!(A _1)).unwrap().get_moves(&board)
    );
}

#[test]
fn no_team_captures() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A _1), knight!(32, White));
    board.pieces.insert(square!(B _3), pawn!(33, White));

    assert_eq!(
        vec![Move::fromNormal(square!(A _1), square!(C _2)),],
        board.get_piece(square!(A _1)).unwrap().get_moves(&board)
    );
}
