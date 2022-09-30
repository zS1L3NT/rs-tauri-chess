macro_rules! perft {
    ($fen:tt, $depth:tt, $nodes:tt) => {
        assert_eq!(
            crate::engine::Board::from_fen($fen.into())
                .unwrap()
                .perft($depth, 0),
            $nodes
        );
    };
    ($fen:tt, $depth:tt!!, $nodes:tt) => {
        assert_eq!(
            crate::engine::Board::from_fen($fen.into())
                .unwrap()
                .perft($depth, $depth),
            $nodes
        );
    };
}

mod initial_position {
    #[test]
    fn depth_1() {
        perft!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            1,
            20
        );
    }

    #[test]
    fn depth_2() {
        perft!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            2,
            400
        );
    }

    #[test]
    fn depth_3() {
        perft!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            3,
            8902
        );
    }

    #[test]
    fn depth_4() {
        perft!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            4,
            197281
        );
    }

    #[test]
    fn depth_5() {
        perft!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            5,
            4865609
        );
    }
}

mod position_2 {
    #[test]
    fn depth_1() {
        perft!(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            1,
            48
        );
    }

    #[test]
    fn depth_2() {
        perft!(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            2,
            2039
        );
    }

    #[test]
    fn depth_3() {
        perft!(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            3,
            97862
        );
    }

    #[test]
    fn depth_4() {
        perft!(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            4,
            4085603
        );
    }
}

mod position_3 {
    #[test]
    fn depth_1() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 1, 14);
    }

    #[test]
    fn depth_2() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 2, 191);
    }

    #[test]
    fn depth_3() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 3, 2812);
    }

    #[test]
    fn depth_4() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 4, 43238);
    }

    #[test]
    fn depth_5() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 5, 674624);
    }

    #[test]
    fn depth_6() {
        perft!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 6, 11030083);
    }
}

mod position_4 {
    #[test]
    fn depth_1() {
        perft!(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            1,
            6
        );
    }

    #[test]
    fn depth_2() {
        perft!(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            2,
            264
        );
    }

    #[test]
    fn depth_3() {
        perft!(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            3,
            9467
        );
    }

    #[test]
    fn depth_4() {
        perft!(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            4,
            422333
        );
    }

    #[test]
    fn depth_5() {
        perft!(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            5,
            15833292
        );
    }
}

mod position_5 {
    #[test]
    fn depth_1() {
        perft!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            1,
            44
        );
    }

    #[test]
    fn depth_2() {
        perft!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            2,
            1486
        );
    }

    #[test]
    fn depth_3() {
        perft!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            3,
            62379
        );
    }

    #[test]
    fn depth_4() {
        perft!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            4,
            2103487
        );
    }
}

mod position_6 {
    #[test]
    fn depth_1() {
        perft!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            1,
            46
        );
    }

    #[test]
    fn depth_2() {
        perft!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            2,
            2079
        );
    }

    #[test]
    fn depth_3() {
        perft!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            3,
            89890
        );
    }

    #[test]
    fn depth_4() {
        perft!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            4,
            3894594
        );
    }
}
