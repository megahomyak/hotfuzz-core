mod iters;
pub use iters::{Iter, IterMut};

use std::collections::HashMap;

use crate::{r#box::BoxPtr, Grapheme, Name, NameGrapheme};

pub enum HotResult<T, Iter> {
    Exact(T),
    Prefixed(Iter),
}

pub(self) enum Node<T, C> {
    Free(BoxPtr<Name<C>>, BoxPtr<T>),
    Occupied(HashMap<Grapheme, Node<T>>),
}

pub(crate) struct Hot<T, C> {
    root: Node<T, C>,
}

pub enum InsertionError {
    HotCollision,
}

impl<T, C> Hot<T, C> where for<'a> &'a C: IntoIterator<Item = NameGrapheme> {
    pub fn get(&self, prompt: impl Iterator<Item = Grapheme>) -> HotResult<T, Iter<T>> {

    }

    pub fn get_mut(&mut self, prompt: impl Iterator<Item = Grapheme>) -> HotResult<T, Iter<T>> {

    }

    pub fn insert(&mut self, name: Name<C>) -> 
}
