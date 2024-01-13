use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct IndexedMap<T> {
    vector: Vec<T>,
    map: HashMap<T, usize>,
}

impl<T: Eq + Hash + Clone> IndexedMap<T> {
    pub fn new() -> Self {
        IndexedMap {
            vector: Vec::new(),
            map: HashMap::new(),
        }
    }
    pub fn from_vector(vec: Vec<T>) -> Self {
        let mut map = HashMap::new();
        for (index, item) in vec.iter().enumerate() {
            map.insert(item.clone(), index);
        }

        IndexedMap { vector: vec, map }
    }

    pub fn add(&mut self, item: T) {
        if !self.map.contains_key(&item) {
            let index = self.vector.len();
            self.vector.push(item.clone());
            self.map.insert(item, index);
        }
    }

    pub fn get_index(&self, item: &T) -> Option<usize> {
        self.map.get(item).cloned()
    }

    pub fn get_item(&self, index: usize) -> Option<&T> {
        self.vector.get(index)
    }

    pub fn into_vector(self) -> Vec<T> {
        self.vector
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }
}
