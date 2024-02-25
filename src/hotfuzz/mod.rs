mod iters;

pub use iters::{Iter, IterMut};

use crate::{name::Name, NonEmptyStr, hot::{self, HotResult}, fuzz};

pub struct HotFuzz<T> {
    hot: hot::Hot<T>,
    fuzz: fuzz::Fuzz<T>,
}

pub enum InsertionError {

}

impl<T> HotFuzz<T> {
    pub fn fuzz_get(&self, prompt: NonEmptyStr) -> Option<Vec<(&Name, &T)>> {
        self.fuzz.get(prompt)
    }

    pub fn fuzz_get_mut(&mut self, prompt: NonEmptyStr) -> Option<Vec<(&Name, &mut T)>> {
        self.fuzz.get_mut(prompt)
    }

    pub fn hot_get(&self, prompt: &str) -> Option<HotResult<T, hot::Iter<T>>> {
        self.hot.get(prompt)
    }

    pub fn hot_get_mut(&self, prompt: &str) -> Option<&T> {
        self.hot.get_mut(prompt)
    }

    pub fn insert(&mut self, name: Name, value: T) -> Result<(), InsertionError> {

    }

    pub fn remove(&mut self, name: &Name, value: T) -> Option<(Name, T)> {

    }

    pub fn iter(&self) -> iters::Iter<T> {

    }

    pub fn iter_mut(&mut self) -> iters::IterMut<T> {

    }
}
