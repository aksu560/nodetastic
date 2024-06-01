use super::*;

#[test]
pub fn insert_and_get() {
    let mut map = Map::new(MapType::Node);
    map.insert(0, 42);
    assert_eq!(map.get(0), Some(&42));
}

#[test]
pub fn remove() {
    let mut map = Map::new(MapType::Node);
    map.insert(0, 42);
    map.remove(0);
    assert_eq!(map.get(0), None);
}

#[test]
pub fn retain() {
    let mut map = Map::new(MapType::Node);
    map.insert(0, 42);
    map.insert(1, 43);
    map.retain(|&id, _| id == 0);
    assert_eq!(map.get(0), Some(&42));
    assert_eq!(map.get(1), None);
}