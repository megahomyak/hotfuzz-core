use std::{collections::HashMap, rc::Rc};

use crate::name::Name;

pub(crate) struct Hot<T> {
    subtrees: HashMap<char, Hot<T>>,
    value: Option<(Rc<Name>, Rc<T>)>,
}

pub(crate) enum HotResult<T> {
    Exact(T),
    Prefixed(Vec<T>),
}

impl<T> Default for Hot<T> {
    fn default() -> Self {
        Self {
            subtrees: HashMap::new(),
            value: None,
        }
    }
}

pub(crate) enum InsertionError {
    NoHotCharactersInInput,
    HotCollision,
}

impl<T> Hot<T> {
    pub fn get(&self, prompt: &str) -> Option<HotResult<(&Name, &T)>> {

    }

    pub fn get_mut(&self, prompt: &str) -> Option<HotResult<(&Name, &mut T)>> {}

    pub fn insert(
        &self,
        name: Rc<Name>,
        value: Rc<T>,
    ) -> Result<Option<(Rc<Name>, Rc<T>)>, InsertionError> {
        let mut current_node = self;
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
