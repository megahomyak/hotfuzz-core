use crate::{Grapheme, graphemes::GraphemeIter};

pub struct NonEmptyStr<'a> {
    pub first: Grapheme,
    pub rest: &'a str,
}

pub enum CreationError {
    CollectionIsEmpty,
}

impl<'a> NonEmptyStr<'a> {
    pub fn new(s: &str) -> Result<Self, CreationError> {
        let mut graphemes = GraphemeIter::new(s);
        if let Some(first) = graphemes.next() {
            Ok(Self { first, rest: graphemes.as_str() })
        } else {
            Err(CreationError::CollectionIsEmpty)
        }
    }
}
