use crate::hotfuzz::HotFuzz;

pub(crate) struct Iter<T>(pub HotFuzz<T>);
pub(crate) struct IterMut<T>(pub HotFuzz<T>);
