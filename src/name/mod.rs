pub mod char;

use char::Char;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    pub chars: Vec<Char>,
}
