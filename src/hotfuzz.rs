use std::rc::Rc;

use crate::{hot::{Hot, HotResult}, fuzz::Fuzz, name::Name, char::Char};

pub struct HotFuzz<T> {
    hot: Hot<Rc<T>>,
    fuzz: Fuzz<Rc<T>>,
}

impl<T> HotFuzz<T> {
    pub fn fuzz_get(&self, prompt: &str) -> Option<Vec<(&Name, &T)>> {
        self.fuzz.get(prompt)
    }

    pub fn fuzz_get_mut(&mut self, prompt: &str) -> Option<Vec<(&Name, &mut T)>> {
        self.fuzz.get_mut(prompt)
    }

    pub fn hot_get(&self, prompt: &str) -> Option<HotResult<(&Name, &T)>> {
        self.hot.get(prompt)
    }

    pub fn hot_get_mut(&self, prompt: &str) -> Option<&T> {
        self.hot.get_mut(prompt)
    }

    pub fn insert(&mut self, name: Name, value: T) -> Result<Option<(Name, T)>, InsertionError> {}

    pub fn remove(&mut self, name: &Name, value: T) -> Option<(Name, T)> {}

    pub fn iter(&self) -> iters::Iter<T> {}

    pub fn iter_mut(&mut self) -> iters::IterMut<T> {}
}
