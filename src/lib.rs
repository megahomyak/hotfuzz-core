pub enum NameChar {
    Hot(char),
    Regular(char),
}

trait Item {
    type Name: Iterator<Item = NameChar>;

    fn name_chars(&self) -> Self::Name;
}

pub struct HotFuzz<Item> {
    tree: preftree::PrefixTree<char, Item>,
    items: Vec<(String, Item)>,
}

pub enum HotResponse<'a, Item> {
    Exact(&'a Item),
    Prefixed(Vec<&'a Item>),
}

pub struct FuzzResponse(pub Vec<dumbfuzz::Difference>);

pub enum AddingError {
    HotCollision,
}

impl<I: Clone + Item> HotFuzz<I> {
    pub fn new() -> Self {
        Self {
            tree: preftree::PrefixTree::new(),
            items: Vec::new(),
        }
    }

    pub fn add(&self, item: I) -> Result<(), AddingError> {
        let mut full_name = String::new();
        let mut hot_name = String::new();
        for char in item.name_chars() {
            match char {
                NameChar::Regular(char) => full_name.push(char),
                NameChar::Hot(char) => {
                    hot_name.push(char);
                    full_name.push(char);
                }
            }
        }
        if !hot_name.is_empty() {
            // Now, I could've made two different methods for adding non-hot things and
            // definitely-hot things (which would be respresented with a special type), but it'll
            // take too much time, and I don't have it now with school and other things
            if let Some(_) = self.tree.get_by_shortest_prefix(hot_name.chars()) {
                return Err(AddingError::HotCollision);
            }
            self.tree.insert(hot_name.chars(), item.clone());
        }
        self.items.push((full_name, item));
        Ok(())
    }

    pub fn hot(&self, prompt: &str) -> &I {
        self.tree.value
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
