#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn from(file: File, rank: Rank) -> Square {
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
