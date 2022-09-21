mod file;
mod rank;
#[cfg(test)]
mod tests;

pub use file::File;
pub use rank::Rank;

use serde::{Deserialize, Serialize};

use crate::square;

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
        square!(B _8),
        square!(C _8),
        square!(D _8),
        square!(E _8),
        square!(F _8),
        square!(G _8),
        square!(H _8),
        square!(A _7),
        square!(B _7),
        square!(C _7),
        square!(D _7),
        square!(E _7),
        square!(F _7),
        square!(G _7),
        square!(H _7),
        square!(A _6),
        square!(B _6),
        square!(C _6),
        square!(D _6),
        square!(E _6),
        square!(F _6),
        square!(G _6),
        square!(H _6),
        square!(A _5),
        square!(B _5),
        square!(C _5),
        square!(D _5),
        square!(E _5),
        square!(F _5),
        square!(G _5),
        square!(H _5),
        square!(A _4),
        square!(B _4),
        square!(C _4),
        square!(D _4),
        square!(E _4),
        square!(F _4),
        square!(G _4),
        square!(H _4),
        square!(A _3),
        square!(B _3),
        square!(C _3),
        square!(D _3),
        square!(E _3),
        square!(F _3),
        square!(G _3),
        square!(H _3),
        square!(A _2),
        square!(B _2),
        square!(C _2),
        square!(D _2),
        square!(E _2),
        square!(F _2),
        square!(G _2),
        square!(H _2),
        square!(A _1),
        square!(B _1),
        square!(C _1),
        square!(D _1),
        square!(E _1),
        square!(F _1),
        square!(G _1),
        square!(H _1),
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
