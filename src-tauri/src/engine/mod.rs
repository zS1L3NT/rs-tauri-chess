pub mod board;
pub mod client;
pub mod color;
pub mod r#move;
pub mod piece;
pub mod square;

#[macro_export]
macro_rules! pawn {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::Pawn,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
macro_rules! knight {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::Knight,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
macro_rules! bishop {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::Bishop,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
macro_rules! rook {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::Rook,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
macro_rules! queen {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::Queen,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
macro_rules! king {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::PieceType::King,
            crate::engine::color::Color::$color,
        )
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! execute {
    ($board:tt $square_1:tt $square_2:tt) => {
        $board.execute(
            $board
                .get_moves()
                .iter()
                .filter(|m| {
                    m.from == rs_tauri_chess::square!($square_1)
                        && m.to == rs_tauri_chess::square!($square_2)
                })
                .next()
                .unwrap()
                .clone(),
        );
    };
}
