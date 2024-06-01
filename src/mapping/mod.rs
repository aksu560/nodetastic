use std::collections::HashMap;

pub enum MapType {
    Node,
    Edge,
}

pub struct Map<T> {
    map: HashMap<usize, T>,
    map_type: MapType,
}

impl<T> Map<T> {
    pub fn new(map_type: MapType) -> Self {
        Self {
            map: HashMap::new(),
            map_type,
        }
    }

    pub fn insert(&mut self, id: usize, value: T) {
        self.map.insert(id, value);
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&usize, &T) -> bool,
    {
        self.map.retain(|id, value| f(id, value));
    }
}

// map tests
#[cfg(test)]
mod test;