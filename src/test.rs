use std::collections::HashMap;

use super::*;

#[test]
pub fn test_graph_creation() {
    let mut graph = Graph::new();
    for _ in 0..500 {
        graph.add_node();
    }
    for i in 0..500 {
        graph.add_edge(i, (i + 1) % 500);
    }
    assert_eq!(graph.nodes.len(), 500);
    assert_eq!(graph.edges.len(), 500);
    for i in 0..500 {
        assert_eq!(graph.nodes[i].edges.len(), 2);
    }
}

#[test]
pub fn test_node_removal() {
    let mut graph = Graph::new();
    let mut target_graph = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);

    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_edge(0, 1);

    graph.remove_node(2);

    assert_eq!(graph, target_graph);
}

#[test]
pub fn test_edge_removal() {
    let mut graph = Graph::new();
    let mut target_graph = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);

    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_edge(0, 1);
    target_graph.add_edge(1, 2);

    graph.remove_edge(2);

    assert_eq!(graph, target_graph);
}

pub fn test_remove_edge_between() {
    let mut graph = Graph::new();
    let mut target_graph = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);

    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_edge(0, 1);
    target_graph.add_edge(2, 0);

    graph.remove_edge_between(1, 2);

    assert_eq!(graph, target_graph);
}

#[test]
pub fn test_get_node() {
    let mut graph = Graph::new();
    graph.add_node();
    graph.add_node();
    graph.add_node();
    assert_eq!(graph.get_node(0).unwrap().id, 0);
    assert_eq!(graph.get_node(1).unwrap().id, 1);
    assert_eq!(graph.get_node(2).unwrap().id, 2);
    assert_eq!(graph.get_node(3), None);
}

#[test]
pub fn test_get_edge() {
    let mut graph = Graph::new();
    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    assert_eq!(graph.get_edge(0).unwrap().id, 0);
    assert_eq!(graph.get_edge(1).unwrap().id, 1);
    assert_eq!(graph.get_edge(2).unwrap().id, 2);
    assert_eq!(graph.get_edge(3), None);
}

#[test]
pub fn test_get_edge_between() {
    let mut graph = Graph::new();
    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    assert_eq!(graph.get_edge_between(0, 1).unwrap().id, 0);
    assert_eq!(graph.get_edge_between(1, 2).unwrap().id, 1);
    assert_eq!(graph.get_edge_between(2, 0), None);
}


#[test]
pub fn test_merge() {
    let mut graph = Graph::new();
    let mut other = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);

    other.add_node();
    other.add_node();
    other.add_edge(0, 1);

    let mut target_graph = Graph::new();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_edge(0, 1);
    target_graph.add_edge(1, 2);
    target_graph.add_edge(3, 4);

    merge_graphs(&mut graph, &other);

    assert_eq!(graph, target_graph);
}

pub fn test_merge_map() {
    let mut graph = Graph::new();
    let mut other = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);

    other.add_node();
    other.add_node();
    other.add_edge(0, 1);

    let target_map = MergeMap {
        nodes: vec![[0, 3], [1, 4]],
        edges: vec![[0, 2]],
    };

    let map = merge_graphs(&mut graph, &other);

    assert_eq!(map, target_map);
}

#[test]
pub fn test_merge_map_update() {
    let mut graph = Graph::new();
    let mut other = Graph::new();

    let mut names = HashMap::new();
    names.insert(graph.add_node(), "Alice");
    names.insert(graph.add_node(), "Bob");
    names.insert(graph.add_node(), "Charlie");

    let mut relationships = HashMap::new();
    relationships.insert(graph.add_edge(0, 1), "friends");
    relationships.insert(graph.add_edge(1, 2), "friends");
    
    let mut other_names = HashMap::new();
    other_names.insert(other.add_node(), "David");
    other_names.insert(other.add_node(), "Eve");

    let mut other_relationships = HashMap::new();
    other_relationships.insert(other.add_edge(0, 1), "enemies");

    let mut target_names = HashMap::new();
    target_names.insert(0, "Alice");
    target_names.insert(1, "Bob");
    target_names.insert(2, "Charlie");
    target_names.insert(3, "David");
    target_names.insert(4, "Eve");

    let mut target_relationships = HashMap::new();
    target_relationships.insert(0, "friends");
    target_relationships.insert(1, "friends");
    target_relationships.insert(2, "enemies");

    let map = merge_graphs(&mut graph, &other);

    for (a) in map.nodes {
        names.insert(a[1], other_names[&a[0]]);

    }

    for (a) in map.edges {
        relationships.insert(a[1], other_relationships[&a[0]]);
    }

    assert_eq!(names, target_names);
    assert_eq!(relationships, target_relationships);
}