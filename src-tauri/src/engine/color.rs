use serde::Serialize;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
