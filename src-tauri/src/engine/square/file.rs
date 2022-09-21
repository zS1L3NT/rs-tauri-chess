use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ["A", "B", "C", "D", "E", "F", "G", "H"][self.to_index() as usize]
        )
    }
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(self.to_index())
    }
}

impl<'de> Deserialize<'de> for File {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let index = i8::deserialize(deserializer)?;
        match File::from_index(index) {
            Some(file) => Ok(file),
            None => Err(serde::de::Error::custom("invalid file")),
        }
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
