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
    fn new(map_type: MapType) -> Self {
        Self {
            map: HashMap::new(),
            map_type,
        }
    }

    fn insert(&mut self, id: usize, value: T) {
        self.map.insert(id, value);
    }

    fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }

    fn retain<F>(&mut self,  f: F)
    where F: FnMut(&usize, &mut T) -> bool, {
        self.map.retain(f);
    }

    fn iter(&self) -> std::collections::hash_map::Iter<usize, T> {
        self.map.iter()
    }

    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<usize, T> {
        self.map.iter_mut()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn keys(&self) -> std::collections::hash_map::Keys<usize, T> {
        self.map.keys()
    }

    fn values(&self) -> std::collections::hash_map::Values<usize, T> {
        self.map.values()
    }

    fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<usize, T> {
        self.map.values_mut()
    }

    fn get_map_type(&self) -> &MapType {
        &self.map_type
    }

    fn get_map(&self) -> &HashMap<usize, T> {
        &self.map
    }

    fn get_map_mut(&mut self) -> &mut HashMap<usize, T> {
        &mut self.map
    }
    
}

// map tests
#[cfg(test)]
mod test;