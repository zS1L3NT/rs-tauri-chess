use crate::{
    engine::{board::Board, r#move::Move},
    knight, pawn, square,
};

#[test]
fn a1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A1), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(A1), square!(B3)),
            Move::from_normal(square!(A1), square!(C2)),
        ],
        board.get_piece(square!(A1)).unwrap().get_moves(&board)
    );
}

#[test]
fn a2_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A2), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(A2), square!(B4)),
            Move::from_normal(square!(A2), square!(C3)),
            Move::from_normal(square!(A2), square!(C1)),
        ],
        board.get_piece(square!(A2)).unwrap().get_moves(&board)
    );
}

#[test]
fn b1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B1), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(B1), square!(C3)),
            Move::from_normal(square!(B1), square!(D2)),
            Move::from_normal(square!(B1), square!(A3)),
        ],
        board.get_piece(square!(B1)).unwrap().get_moves(&board)
    );
}

#[test]
fn b2_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B2), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(B2), square!(C4)),
            Move::from_normal(square!(B2), square!(D3)),
            Move::from_normal(square!(B2), square!(D1)),
            Move::from_normal(square!(B2), square!(A4)),
        ],
        board.get_piece(square!(B2)).unwrap().get_moves(&board)
    );
}

#[test]
fn a3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A3), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(A3), square!(B5)),
            Move::from_normal(square!(A3), square!(C4)),
            Move::from_normal(square!(A3), square!(C2)),
            Move::from_normal(square!(A3), square!(B1)),
        ],
        board.get_piece(square!(A3)).unwrap().get_moves(&board)
    );
}

#[test]
fn b3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(B3), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(B3), square!(C5)),
            Move::from_normal(square!(B3), square!(D4)),
            Move::from_normal(square!(B3), square!(D2)),
            Move::from_normal(square!(B3), square!(C1)),
            Move::from_normal(square!(B3), square!(A1)),
            Move::from_normal(square!(B3), square!(A5)),
        ],
        board.get_piece(square!(B3)).unwrap().get_moves(&board)
    );
}

#[test]
fn c3_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(C3), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(C3), square!(D5)),
            Move::from_normal(square!(C3), square!(E4)),
            Move::from_normal(square!(C3), square!(E2)),
            Move::from_normal(square!(C3), square!(D1)),
            Move::from_normal(square!(C3), square!(B1)),
            Move::from_normal(square!(C3), square!(A2)),
            Move::from_normal(square!(C3), square!(A4)),
            Move::from_normal(square!(C3), square!(B5)),
        ],
        board.get_piece(square!(C3)).unwrap().get_moves(&board)
    );
}

#[test]
fn c2_corner() {
    let mut board = Board::empty();
    let king = board.pieces.remove(&square!(E1)).unwrap();
    board.pieces.insert(square!(F1), king);
    board.pieces.insert(square!(C2), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(C2), square!(D4)),
            Move::from_normal(square!(C2), square!(E3)),
            Move::from_normal(square!(C2), square!(E1)),
            Move::from_normal(square!(C2), square!(A1)),
            Move::from_normal(square!(C2), square!(A3)),
            Move::from_normal(square!(C2), square!(B4)),
        ],
        board.get_piece(square!(C2)).unwrap().get_moves(&board)
    );
}

#[test]
fn c1_corner() {
    let mut board = Board::empty();
    board.pieces.insert(square!(C1), knight!(32, White));

    assert_eq!(
        vec![
            Move::from_normal(square!(C1), square!(D3)),
            Move::from_normal(square!(C1), square!(E2)),
            Move::from_normal(square!(C1), square!(A2)),
            Move::from_normal(square!(C1), square!(B3)),
        ],
        board.get_piece(square!(C1)).unwrap().get_moves(&board)
    );
}

#[test]
fn captures() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A1), knight!(32, White));
    board.pieces.insert(square!(B3), pawn!(33, Black));

    assert_eq!(
        vec![
            Move::from_capture(square!(A1), square!(B3)),
            Move::from_normal(square!(A1), square!(C2)),
        ],
        board.get_piece(square!(A1)).unwrap().get_moves(&board)
    );
}

#[test]
fn no_team_captures() {
    let mut board = Board::empty();
    board.pieces.insert(square!(A1), knight!(32, White));
    board.pieces.insert(square!(B3), pawn!(33, White));

    assert_eq!(
        vec![Move::from_normal(square!(A1), square!(C2)),],
        board.get_piece(square!(A1)).unwrap().get_moves(&board)
    );
}
