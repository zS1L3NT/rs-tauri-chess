mod file;
mod rank;
#[cfg(test)]
mod tests;

pub use file::File;
pub use rank::Rank;

use rs_tauri_chess::square;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.file, self.rank)
    }
}

impl Square {
    pub const ALL: [Square; 64] = [
        Square::from(File::A, Rank::_8),
        square!(B8),
        square!(C8),
        square!(D8),
        square!(E8),
        square!(F8),
        square!(G8),
        square!(H8),
        square!(A7),
        square!(B7),
        square!(C7),
        square!(D7),
        square!(E7),
        square!(F7),
        square!(G7),
        square!(H7),
        square!(A6),
        square!(B6),
        square!(C6),
        square!(D6),
        square!(E6),
        square!(F6),
        square!(G6),
        square!(H6),
        square!(A5),
        square!(B5),
        square!(C5),
        square!(D5),
        square!(E5),
        square!(F5),
        square!(G5),
        square!(H5),
        square!(A4),
        square!(B4),
        square!(C4),
        square!(D4),
        square!(E4),
        square!(F4),
        square!(G4),
        square!(H4),
        square!(A3),
        square!(B3),
        square!(C3),
        square!(D3),
        square!(E3),
        square!(F3),
        square!(G3),
        square!(H3),
        square!(A2),
        square!(B2),
        square!(C2),
        square!(D2),
        square!(E2),
        square!(F2),
        square!(G2),
        square!(H2),
        square!(A1),
        square!(B1),
        square!(C1),
        square!(D1),
        square!(E1),
        square!(F1),
        square!(G1),
        square!(H1),
    ];

    pub const fn from(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }

    pub fn offset(&self, file_offset: i8, rank_offset: i8) -> Option<Square> {
        let file = File::from_index(self.file.to_index() + file_offset);
        let rank = Rank::from_index(self.rank.to_index() + rank_offset);

        if let Some(file) = file {
            if let Some(rank) = rank {
                return Some(Square::from(file, rank));
            }
        }

        None
    }
}
