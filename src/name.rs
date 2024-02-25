use crate::Grapheme;

pub trait Name {
    type Iter<'a>: Iterator<Item = NameGrapheme> + 'a;

    fn graphemes<'a>(&'a self) -> Self::Iter<'a>;
}

impl<T> Name for T
where
    for<'a> &'a T: IntoIterator<Item = NameGrapheme>,
{
    type Iter<'a> = <&'a T as IntoIterator>::IntoIter;

    fn graphemes<'a>(&'a self) -> Self::Iter<'a> {
        self.into_iter()
    }
}

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
