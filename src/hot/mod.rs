mod iters;
pub use iters::{Iter, IterMut};

use std::collections::HashMap;

use crate::{r#box::BoxPtr, Grapheme, Name};

pub enum HotResult<T, Iter> {
    Exact(T),
    Prefixed(Iter),
}

pub(self) enum Node<T> {
    Free(BoxPtr<Name>, BoxPtr<T>),
    Occupied(HashMap<Grapheme, Node<T>>),
}

pub(crate) struct Hot<T> {
    root: Node<T>,
}

impl<T> Hot<T> {
    pub fn get(&self, prompt: impl Iterator<Item = Grapheme>) -> HotResult<T, Iter<T>> {}
}
