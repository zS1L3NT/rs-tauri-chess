use std::fmt::{Debug, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => "W",
                Color::Black => "B",
            }
        )
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
