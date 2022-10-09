mod board;
mod client;
mod color;
mod r#move;
mod piece;
mod square;

pub use {board::*, client::*, color::*, piece::*, r#move::*, square::*};

#[macro_export]
macro_rules! pawn {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::Pawn, $crate::engine::$color)
    };
}

#[macro_export]
macro_rules! knight {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::Knight, $crate::engine::$color)
    };
}

#[macro_export]
macro_rules! bishop {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::Bishop, $crate::engine::$color)
    };
}

#[macro_export]
macro_rules! rook {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::Rook, $crate::engine::$color)
    };
}

#[macro_export]
macro_rules! queen {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::Queen, $crate::engine::$color)
    };
}

#[macro_export]
macro_rules! king {
    ($id:tt, $color:tt) => {
        $crate::engine::Piece::new($id, $crate::engine::King, $crate::engine::$color)
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
