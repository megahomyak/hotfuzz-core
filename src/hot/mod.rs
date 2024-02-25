mod iter;
mod iter_mut;
pub use iter::Iter;
pub use iter_mut::IterMut;

use std::collections::HashMap;

use crate::{Name, Grapheme, r#box::BoxPtr};

pub enum HotResult<T> {
    Exact(T),
    Prefixed(Iter<T>),
}

pub(self) enum Node<T> {
    Free(BoxPtr<Name>, BoxPtr<T>),
    Occupied(HashMap<Grapheme, Node<T>>),
}

pub(crate) struct Hot<T> {
    root: Node<T>,
}

impl<T> Hot<T> {
    pub fn get(&self, prompt: impl Iterator<Item = Grapheme>) -> HotResult<> {

    }
}
