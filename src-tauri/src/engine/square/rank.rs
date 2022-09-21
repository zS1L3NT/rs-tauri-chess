use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_index() + 1)
    }
}

impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(self.to_index())
    }
}

impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = i8::deserialize(deserializer)?;
        match Rank::from_index(index) {
            Some(rank) => Ok(rank),
            None => Err(serde::de::Error::custom("invalid rank")),
        }
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
