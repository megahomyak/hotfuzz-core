use std::{collections::HashMap, rc::Rc};

use crate::name::Name;

enum Node<T> {
    Occupied((Rc<Name>, Rc<T>)),
    Free(HashMap<char, Node<T>>),
}

pub(crate) struct Hot<T> {
    root: Node<T>,
}

pub(crate) enum HotResult<T> {
    Exact(T),
    Prefixed(crate::hot_prefixed_entries_iterator::HotPrefixedEntriesIterator<T>),
}

pub(crate) enum InsertionError {
    NoHotCharactersInInput,
    HotCollision,
}

impl<T> Hot<T> {
    pub fn get(&self, prompt: &str) -> Option<HotResult<(&Name, &T)>> {

    }

    pub fn get_mut(&self, prompt: &str) -> Option<HotResult<(&Name, &mut T)>> {

    }

    pub fn remove(&self, name: &Name) -> Option<(&Name, Rc<T>)> {

    }

    pub fn insert(
        &mut self,
        name: Rc<Name>,
        value: Rc<T>,
    ) -> Result<Option<(Rc<Name>, Rc<T>)>, InsertionError> {
        let mut current_node = &mut self.root;
        let hot_chars = name.chars.iter().filter_map(|char| char.hot());
        for char in hot_chars {
            if current_node.value.is_some() {
                return Err(InsertionError::HotCollision);
            }
            current_node = current_node.subtrees.entry(char).or_insert(Hot::default());
        }
        if std::ptr::eq(&current_node, &self) {
            return Err(InsertionError::NoHotCharactersInInput);
        }
        Ok(current_node.value.replace((name, value)))
    }
}
