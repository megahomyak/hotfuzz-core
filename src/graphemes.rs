pub struct Grapheme {
    content: String,
}

impl Grapheme {
    pub fn content(&self) -> &str {
        &self.0[..]
    }
}

pub struct GraphemeIter<'a> {
    inner: unicode_segmentation::Graphemes<'a>,
}

impl<'a> GraphemeIter<'a> {
    pub fn new(s: &'a str) -> Self {
        use unicode_segmentation::UnicodeSegmentation;

        Self {
            inner: s.graphemes(true), // Extended graphemes
        }
    }

    pub fn as_str(&'a self) -> &'a str {
        self.inner.as_str()
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = Grapheme;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| Grapheme { content: s.to_owned() })
    }
}
