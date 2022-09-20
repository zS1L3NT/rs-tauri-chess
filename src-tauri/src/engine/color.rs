use serde::Serialize;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => "white".into(),
            Color::Black => "black".into(),
        }
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
