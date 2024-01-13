use crate::engine::board::Board;

#[test]
fn from_and_to() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = Board::from_fen(fen.into());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().to_fen(), fen);

    let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
    let board = Board::from_fen(fen.into());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().to_fen(), fen);

    let fen = "rnb2bnr/ppp1pppp/4k3/3q4/8/8/PPPP1PPP/RNBQKBNR w KQ - 0 1";
    let board = Board::from_fen(fen.into());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().to_fen(), fen);

    let fen = "rnb2bnr/ppp1pppp/4k3/3q4/8/7K/PPPP1PPP/RNBQ1BNR b - - 0 1";
    let board = Board::from_fen(fen.into());
    assert!(board.is_ok());
    assert_eq!(board.unwrap().to_fen(), fen);
}
