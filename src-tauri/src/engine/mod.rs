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
            crate::engine::piece::Pawn,
            crate::engine::color::$color,
        )
    };
}

#[macro_export]
macro_rules! knight {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::Knight,
            crate::engine::color::$color,
        )
    };
}

#[macro_export]
macro_rules! bishop {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::Bishop,
            crate::engine::color::$color,
        )
    };
}

#[macro_export]
macro_rules! rook {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::Rook,
            crate::engine::color::$color,
        )
    };
}

#[macro_export]
macro_rules! queen {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::Queen,
            crate::engine::color::$color,
        )
    };
}

#[macro_export]
macro_rules! king {
    ($id:tt, $color:tt) => {
        crate::engine::piece::Piece::new(
            $id,
            crate::engine::piece::King,
            crate::engine::color::$color,
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
