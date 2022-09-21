use serde::Serialize;

use crate::square;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        })
    }
}

impl File {
    pub fn from_index(index: i8) -> Option<File> {
        match index {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }

    pub fn to_index(&self) -> i8 {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Rank {
    _1 = 0,
    _2 = 1,
    _3 = 2,
    _4 = 3,
    _5 = 4,
    _6 = 5,
    _7 = 6,
    _8 = 7,
}

impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            Rank::_1 => 0,
            Rank::_2 => 1,
            Rank::_3 => 2,
            Rank::_4 => 3,
            Rank::_5 => 4,
            Rank::_6 => 5,
            Rank::_7 => 6,
            Rank::_8 => 7,
        })
    }
}

impl Rank {
    pub fn from_index(index: i8) -> Option<Rank> {
        match index {
            0 => Some(Rank::_1),
            1 => Some(Rank::_2),
            2 => Some(Rank::_3),
            3 => Some(Rank::_4),
            4 => Some(Rank::_5),
            5 => Some(Rank::_6),
            6 => Some(Rank::_7),
            7 => Some(Rank::_8),
            _ => None,
        }
    }

    pub fn to_index(&self) -> i8 {
        match self {
            Rank::_1 => 0,
            Rank::_2 => 1,
            Rank::_3 => 2,
            Rank::_4 => 3,
            Rank::_5 => 4,
            Rank::_6 => 5,
            Rank::_7 => 6,
            Rank::_8 => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
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
