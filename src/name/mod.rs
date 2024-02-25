pub mod char;

use char::NameGrapheme;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    pub chars: Vec<NameGrapheme>,
}
