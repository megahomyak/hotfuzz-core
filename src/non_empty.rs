pub struct Grapheme<'a>(&'a str);
pub struct Graphemes<'a>(&'a str);
pub struct GraphemesIterator<'a> {

}

fn graphemes(s: &str) -> unicode_segmentation::Graphemes {
    use unicode_segmentation::UnicodeSegmentation;

    s.graphemes(true) // Extended graphemes, not legacy graphemes
}

impl<'a> Grapheme<'a> {
    pub fn content(&self) -> &'a str {
        &self.0
    }
}

impl<'a> Graphemes<'a> {
    pub fn content(&self) -> unicode_segmentation::Graphemes {
        graphemes(&self.0)
    }
}

pub struct NonEmptyStr<'a> {
    pub first: Grapheme<'a>,
    pub rest: Graphemes<'a>,
}

pub enum CreationError {}

impl<'a> NonEmptyStr<'a> {
    pub fn new(s: &'a str) {
        let mut graphemes = graphemes(s);

        let first = graphemes.next();
        let rest = graphemes.as_str();

        Self { first, rest }
    }
}
