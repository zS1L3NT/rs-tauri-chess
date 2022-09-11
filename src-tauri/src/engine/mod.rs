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
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::Pawn, Color::$team)
    };
}

#[macro_export]
macro_rules! knight {
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::Knight, Color::$team)
    };
}

#[macro_export]
macro_rules! bishop {
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::Bishop, Color::$team)
    };
}

#[macro_export]
macro_rules! rook {
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::Rook, Color::$team)
    };
}

#[macro_export]
macro_rules! queen {
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::Queen, Color::$team)
    };
}

#[macro_export]
macro_rules! king {
    ($id:tt, $team:tt) => {
        Piece::new($id, PieceType::King, Color::$team)
    };
}
