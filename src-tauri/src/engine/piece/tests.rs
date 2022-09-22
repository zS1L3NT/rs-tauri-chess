mod pawn {
    use crate::engine::{attack_lines::AttackLines, board::Board};
    use rs_tauri_chess::square;

    #[test]
    fn white_normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(E2),
                vec![vec![square!(D3)], vec![square!(F3)]],
                None
            ),
            board
                .pieces
                .get(&square!(E2))
                .unwrap()
                .get_attack_lines(&board, square!(E2))
        );
    }

    #[test]
    fn white_left() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(square!(A2), vec![vec![square!(B3)]], None),
            board
                .pieces
                .get(&square!(A2))
                .unwrap()
                .get_attack_lines(&board, square!(A2))
        );
    }

    #[test]
    fn white_right() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(square!(H2), vec![vec![square!(G3)]], None),
            board
                .pieces
                .get(&square!(H2))
                .unwrap()
                .get_attack_lines(&board, square!(H2))
        );
    }

    #[test]
    fn black_normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(E7),
                vec![vec![square!(D6)], vec![square!(F6)]],
                None
            ),
            board
                .pieces
                .get(&square!(E7))
                .unwrap()
                .get_attack_lines(&board, square!(E7))
        );
    }

    #[test]
    fn black_left() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(square!(A7), vec![vec![square!(B6)]], None),
            board
                .pieces
                .get(&square!(A7))
                .unwrap()
                .get_attack_lines(&board, square!(A7))
        );
    }

    #[test]
    fn black_right() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(square!(H7), vec![vec![square!(G6)]], None),
            board
                .pieces
                .get(&square!(H7))
                .unwrap()
                .get_attack_lines(&board, square!(H7))
        );
    }
}

mod knight {
    use crate::engine::{attack_lines::AttackLines, board::Board};
    use rs_tauri_chess::square;

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(B1),
                vec![vec![square!(C3)], vec![square!(D2)], vec![square!(A3)]],
                None
            ),
            board
                .pieces
                .get(&square!(B1))
                .unwrap()
                .get_attack_lines(&board, square!(B1))
        );
    }
}

mod bishop {
    use crate::{
        bishop,
        engine::{attack_lines::AttackLines, board::Board},
    };
    use rs_tauri_chess::square;

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(C1),
                vec![
                    vec![
                        square!(D2),
                        square!(E3),
                        square!(F4),
                        square!(G5),
                        square!(H6)
                    ],
                    vec![square!(B2), square!(A3)],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(C1))
                .unwrap()
                .get_attack_lines(&board, square!(C1))
        );
    }

    #[test]
    fn lines_with_king() {
        let mut board = Board::new();
        board.pieces.insert(square!(H5), bishop!(32, White));

        assert_eq!(
            AttackLines::new(
                square!(H5),
                vec![
                    vec![square!(G4), square!(F3), square!(E2), square!(D1),],
                    vec![square!(G6), square!(F7), square!(E8)],
                ],
                Some(1)
            ),
            board
                .pieces
                .get(&square!(H5))
                .unwrap()
                .get_attack_lines(&board, square!(H5))
        );
    }
}

mod rook {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        rook,
    };
    use rs_tauri_chess::square;

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(A1),
                vec![
                    vec![
                        square!(A2),
                        square!(A3),
                        square!(A4),
                        square!(A5),
                        square!(A6),
                        square!(A7),
                        square!(A8)
                    ],
                    vec![
                        square!(B1),
                        square!(C1),
                        square!(D1),
                        square!(E1),
                        square!(F1),
                        square!(G1),
                        square!(H1)
                    ],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(A1))
                .unwrap()
                .get_attack_lines(&board, square!(A1))
        );
    }

    #[test]
    fn lines_with_king() {
        let mut board = Board::new();
        board.pieces.insert(square!(E3), rook!(32, White));

        assert_eq!(
            AttackLines::new(
                square!(E3),
                vec![
                    vec![
                        square!(E4),
                        square!(E5),
                        square!(E6),
                        square!(E7),
                        square!(E8),
                    ],
                    vec![square!(F3), square!(G3), square!(H3),],
                    vec![square!(E2), square!(E1),],
                    vec![square!(D3), square!(C3), square!(B3), square!(A3),],
                ],
                Some(0)
            ),
            board
                .pieces
                .get(&square!(E3))
                .unwrap()
                .get_attack_lines(&board, square!(E3))
        );
    }
}

mod queen {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        queen,
    };
    use rs_tauri_chess::square;

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(D1),
                vec![
                    vec![
                        square!(D2),
                        square!(D3),
                        square!(D4),
                        square!(D5),
                        square!(D6),
                        square!(D7),
                        square!(D8)
                    ],
                    vec![square!(E2), square!(F3), square!(G4), square!(H5)],
                    vec![square!(E1), square!(F1), square!(G1), square!(H1)],
                    vec![square!(C1), square!(B1), square!(A1),],
                    vec![square!(C2), square!(B3), square!(A4),],
                ],
                None,
            ),
            board
                .pieces
                .get(&square!(D1))
                .unwrap()
                .get_attack_lines(&board, square!(D1))
        );
    }

    #[test]
    fn lines_with_king_straight() {
        let mut board = Board::new();
        board.pieces.insert(square!(E3), queen!(32, White));

        assert_eq!(
            AttackLines::new(
                square!(E3),
                vec![
                    vec![
                        square!(E4),
                        square!(E5),
                        square!(E6),
                        square!(E7),
                        square!(E8),
                    ],
                    vec![square!(F4), square!(G5), square!(H6),],
                    vec![square!(F3), square!(G3), square!(H3),],
                    vec![square!(F2), square!(G1),],
                    vec![square!(E2), square!(E1),],
                    vec![square!(D2), square!(C1),],
                    vec![square!(D3), square!(C3), square!(B3), square!(A3),],
                    vec![square!(D4), square!(C5), square!(B6), square!(A7),],
                ],
                Some(0)
            ),
            board
                .pieces
                .get(&square!(E3))
                .unwrap()
                .get_attack_lines(&board, square!(E3))
        );
    }

    #[test]
    fn lines_with_king_diagonal() {
        let mut board = Board::new();
        board.pieces.insert(square!(H5), queen!(32, White));

        assert_eq!(
            AttackLines::new(
                square!(H5),
                vec![
                    vec![square!(H6), square!(H7), square!(H8),],
                    vec![square!(H4), square!(H3), square!(H2), square!(H1)],
                    vec![square!(G4), square!(F3), square!(E2), square!(D1),],
                    vec![
                        square!(G5),
                        square!(F5),
                        square!(E5),
                        square!(D5),
                        square!(C5),
                        square!(B5),
                        square!(A5),
                    ],
                    vec![square!(G6), square!(F7), square!(E8),],
                ],
                Some(4)
            ),
            board
                .pieces
                .get(&square!(H5))
                .unwrap()
                .get_attack_lines(&board, square!(H5))
        );
    }
}

mod king {
    use crate::engine::{attack_lines::AttackLines, board::Board};
    use rs_tauri_chess::square;

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                square!(E1),
                vec![
                    vec![square!(E2)],
                    vec![square!(F2)],
                    vec![square!(F1)],
                    vec![square!(D1)],
                    vec![square!(D2)],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(E1))
                .unwrap()
                .get_attack_lines(&board, square!(E1))
        );
    }
}
