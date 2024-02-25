mod iters;

pub use iters::{Iter, IterMut};

use std::collections::HashMap;

use crate::{name::Name, NonEmptyStr, hot::{self, HotResult}};

pub struct HotFuzz<T> {
    items: HashMap<Name, T>,
}

pub enum InsertionError {

}

impl<T> HotFuzz<T> {
    pub fn fuzz_get(&self, prompt: NonEmptyStr) -> Option<Vec<(&Name, &T)>> {

    }

    pub fn fuzz_get_mut(&mut self, prompt: NonEmptyStr) -> Option<Vec<(&Name, &mut T)>> {

    }

    pub fn hot_get(&self, prompt: &str) -> Option<HotResult<T, hot::Iter<T>>> {

    }

    pub fn hot_get_mut(&self, prompt: &str) -> Option<&T> {

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
