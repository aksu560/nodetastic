use super::*;

#[test]
pub fn graph_new() {
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
pub fn remove_node() {
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
pub fn remove_edge() {
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

pub fn remove_edge_between() {
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
pub fn get_node() {
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
pub fn get_edge() {
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
pub fn get_edge_between() {
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