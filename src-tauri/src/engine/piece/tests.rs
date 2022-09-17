mod pawn {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        square,
    };

    #[test]
    fn white_normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![square!(E _2), square!(D _3)],
                    vec![square!(E _2), square!(F _3)]
                ],
                None
            ),
            board
                .pieces
                .get(&square!(E _2))
                .unwrap()
                .get_attack_lines(&board, square!(E _2))
        );
    }

    #[test]
    fn white_left() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(vec![vec![square!(A _2), square!(B _3)]], None),
            board
                .pieces
                .get(&square!(A _2))
                .unwrap()
                .get_attack_lines(&board, square!(A _2))
        );
    }

    #[test]
    fn white_right() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(vec![vec![square!(H _2), square!(G _3)]], None),
            board
                .pieces
                .get(&square!(H _2))
                .unwrap()
                .get_attack_lines(&board, square!(H _2))
        );
    }

    #[test]
    fn black_normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![square!(E _7), square!(D _6)],
                    vec![square!(E _7), square!(F _6)]
                ],
                None
            ),
            board
                .pieces
                .get(&square!(E _7))
                .unwrap()
                .get_attack_lines(&board, square!(E _7))
        );
    }

    #[test]
    fn black_left() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(vec![vec![square!(A _7), square!(B _6)]], None),
            board
                .pieces
                .get(&square!(A _7))
                .unwrap()
                .get_attack_lines(&board, square!(A _7))
        );
    }

    #[test]
    fn black_right() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(vec![vec![square!(H _7), square!(G _6)]], None),
            board
                .pieces
                .get(&square!(H _7))
                .unwrap()
                .get_attack_lines(&board, square!(H _7))
        );
    }
}

mod knight {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        square,
    };

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![square!(B _1), square!(C _3)],
                    vec![square!(B _1), square!(D _2)],
                    vec![square!(B _1), square!(A _3)]
                ],
                None
            ),
            board
                .pieces
                .get(&square!(B _1))
                .unwrap()
                .get_attack_lines(&board, square!(B _1))
        );
    }
}

mod bishop {
    use crate::{
        bishop,
        engine::{attack_lines::AttackLines, board::Board},
        square,
    };

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(C _1),
                        square!(D _2),
                        square!(E _3),
                        square!(F _4),
                        square!(G _5),
                        square!(H _6)
                    ],
                    vec![square!(C _1), square!(B _2), square!(A _3)],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(C _1))
                .unwrap()
                .get_attack_lines(&board, square!(C _1))
        );
    }

    #[test]
    fn lines_with_king() {
        let mut board = Board::new();
        board.pieces.insert(square!(H _5), bishop!(32, White));

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(H _5),
                        square!(G _4),
                        square!(F _3),
                        square!(E _2),
                        square!(D _1),
                    ],
                    vec![square!(H _5), square!(G _6), square!(F _7), square!(E _8)],
                ],
                Some(1)
            ),
            board
                .pieces
                .get(&square!(H _5))
                .unwrap()
                .get_attack_lines(&board, square!(H _5))
        );
    }
}

mod rook {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        rook, square,
    };

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(A _1),
                        square!(A _2),
                        square!(A _3),
                        square!(A _4),
                        square!(A _5),
                        square!(A _6),
                        square!(A _7),
                        square!(A _8)
                    ],
                    vec![
                        square!(A _1),
                        square!(B _1),
                        square!(C _1),
                        square!(D _1),
                        square!(E _1),
                        square!(F _1),
                        square!(G _1),
                        square!(H _1)
                    ],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(A _1))
                .unwrap()
                .get_attack_lines(&board, square!(A _1))
        );
    }

    #[test]
    fn lines_with_king() {
        let mut board = Board::new();
        board.pieces.insert(square!(E _3), rook!(32, White));

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(E _3),
                        square!(E _4),
                        square!(E _5),
                        square!(E _6),
                        square!(E _7),
                        square!(E _8),
                    ],
                    vec![square!(E _3), square!(F _3), square!(G _3), square!(H _3),],
                    vec![square!(E _3), square!(E _2), square!(E _1),],
                    vec![
                        square!(E _3),
                        square!(D _3),
                        square!(C _3),
                        square!(B _3),
                        square!(A _3),
                    ],
                ],
                Some(0)
            ),
            board
                .pieces
                .get(&square!(E _3))
                .unwrap()
                .get_attack_lines(&board, square!(E _3))
        );
    }
}

mod queen {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        queen, square,
    };

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(D _1),
                        square!(D _2),
                        square!(D _3),
                        square!(D _4),
                        square!(D _5),
                        square!(D _6),
                        square!(D _7),
                        square!(D _8)
                    ],
                    vec![
                        square!(D _1),
                        square!(E _2),
                        square!(F _3),
                        square!(G _4),
                        square!(H _5)
                    ],
                    vec![
                        square!(D _1),
                        square!(E _1),
                        square!(F _1),
                        square!(G _1),
                        square!(H _1)
                    ],
                    vec![square!(D _1), square!(C _1), square!(B _1), square!(A _1),],
                    vec![square!(D _1), square!(C _2), square!(B _3), square!(A _4),],
                ],
                None,
            ),
            board
                .pieces
                .get(&square!(D _1))
                .unwrap()
                .get_attack_lines(&board, square!(D _1))
        );
    }

    #[test]
    fn lines_with_king_straight() {
        let mut board = Board::new();
        board.pieces.insert(square!(E _3), queen!(32, White));

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![
                        square!(E _3),
                        square!(E _4),
                        square!(E _5),
                        square!(E _6),
                        square!(E _7),
                        square!(E _8),
                    ],
                    vec![square!(E _3), square!(F _4), square!(G _5), square!(H _6),],
                    vec![square!(E _3), square!(F _3), square!(G _3), square!(H _3),],
                    vec![square!(E _3), square!(F _2), square!(G _1),],
                    vec![square!(E _3), square!(E _2), square!(E _1),],
                    vec![square!(E _3), square!(D _2), square!(C _1),],
                    vec![
                        square!(E _3),
                        square!(D _3),
                        square!(C _3),
                        square!(B _3),
                        square!(A _3),
                    ],
                    vec![
                        square!(E _3),
                        square!(D _4),
                        square!(C _5),
                        square!(B _6),
                        square!(A _7),
                    ],
                ],
                Some(0)
            ),
            board
                .pieces
                .get(&square!(E _3))
                .unwrap()
                .get_attack_lines(&board, square!(E _3))
        );
    }

    #[test]
    fn lines_with_king_diagonal() {
        let mut board = Board::new();
        board.pieces.insert(square!(H _5), queen!(32, White));

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![square!(H _5), square!(H _6), square!(H _7), square!(H _8),],
                    vec![
                        square!(H _5),
                        square!(H _4),
                        square!(H _3),
                        square!(H _2),
                        square!(H _1)
                    ],
                    vec![
                        square!(H _5),
                        square!(G _4),
                        square!(F _3),
                        square!(E _2),
                        square!(D _1),
                    ],
                    vec![
                        square!(H _5),
                        square!(G _5),
                        square!(F _5),
                        square!(E _5),
                        square!(D _5),
                        square!(C _5),
                        square!(B _5),
                        square!(A _5),
                    ],
                    vec![square!(H _5), square!(G _6), square!(F _7), square!(E _8),],
                ],
                Some(4)
            ),
            board
                .pieces
                .get(&square!(H _5))
                .unwrap()
                .get_attack_lines(&board, square!(H _5))
        );
    }
}

mod king {
    use crate::{
        engine::{attack_lines::AttackLines, board::Board},
        square,
    };

    #[test]
    fn normal() {
        let board = Board::new();

        assert_eq!(
            AttackLines::new(
                vec![
                    vec![square!(E _1), square!(E _2)],
                    vec![square!(E _1), square!(F _2)],
                    vec![square!(E _1), square!(F _1)],
                    vec![square!(E _1), square!(D _1)],
                    vec![square!(E _1), square!(D _2)],
                ],
                None
            ),
            board
                .pieces
                .get(&square!(E _1))
                .unwrap()
                .get_attack_lines(&board, square!(E _1))
        );
    }
}
