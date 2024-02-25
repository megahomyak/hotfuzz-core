use crate::Grapheme;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameGrapheme {
    Hot(Grapheme),
    Regular(Grapheme),
}

impl NameGrapheme {
    pub fn hot(&self) -> Option<char> {
        match self {
            Self::Regular(c) => None,
            Self::Hot(c) => Some(*c),
        }
    }

    pub fn regular(&self) -> Option<char> {
        match self {
            Self::Hot(c) => None,
            Self::Regular(c) => Some(*c),
        }
    }

    pub fn content(&self) -> char {
        match self {
            Self::Hot(c) | Self::Regular(c) => *c,
        }
    }
}
