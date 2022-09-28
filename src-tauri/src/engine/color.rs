pub use Color::*;

use {
    serde::{Deserialize, Serialize},
    std::fmt::{Debug, Formatter, Result},
};

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
                White => "W",
                Black => "B",
            }
        )
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}
