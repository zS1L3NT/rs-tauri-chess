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
        write!(f, "{:?}", Into::<i8>::into(*self) + 1)
    }
}

impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8((*self).into())
    }
}

impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = i8::deserialize(deserializer)?;
        match Rank::try_from(index) {
            Ok(rank) => Ok(rank),
            Err(_) => Err(serde::de::Error::custom("invalid rank")),
        }
    }
}

impl Into<i8> for Rank {
    fn into(self) -> i8 {
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

impl TryFrom<i8> for Rank {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::_1),
            1 => Ok(Rank::_2),
            2 => Ok(Rank::_3),
            3 => Ok(Rank::_4),
            4 => Ok(Rank::_5),
            5 => Ok(Rank::_6),
            6 => Ok(Rank::_7),
            7 => Ok(Rank::_8),
            _ => Err(()),
        }
    }
}
