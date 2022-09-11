pub mod board;
pub mod color;
pub mod r#move;
pub mod piece;
pub mod square;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! square {
    ($file:tt $rank:tt) => {
        crate::engine::square::Square::from(
            crate::engine::square::File::$file,
            crate::engine::square::Rank::$rank,
        )
    };
}

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
