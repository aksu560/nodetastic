use std::collections::HashMap;

pub enum MapType {
    Node,
    Edge,
}

pub struct Map<T> {
    map: HashMap<T, usize>,
    map_type: MapType,
}

impl<T> Map<T> where T: Eq + std::hash::Hash{
    pub fn new(map_type: MapType) -> Self {
        Self {
            map: HashMap::new(),
            map_type,
        }
    }

    pub fn insert(&mut self, key: T, id: usize) {
        self.map.insert(key, id);
    }

    pub fn get(&self, key: T) -> Option<&usize> {
        self.map.get(&key)
    }

    pub fn remove(&mut self, key: T) {
        self.map.remove(&key);
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T, &usize) -> bool,
    {
        self.map.retain(|key, value| f(key, value));
    }
    
}

// map tests
#[cfg(test)]
mod test;