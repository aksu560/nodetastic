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
pub fn test_directed_graph_creation() {
    let mut graph = Graph::new();
    for _ in 0..500 {
        graph.add_node();
    }
    for i in 0..500 {
        graph.add_directed_edge(i, (i + 1) % 500);
    }
    assert_eq!(graph.nodes.len(), 500);
    assert_eq!(graph.edges.len(), 500);
    for i in 0..500 {
        assert_eq!(graph.nodes[i].edges.len(), 1);
        assert_eq!(graph.edges[i].edge_type, EdgeType::Directed);
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

#[test]
pub fn test_dicrected_node_removal() {
    let mut graph = Graph::new();
    let mut target_graph = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_directed_edge(0, 1);
    graph.add_directed_edge(1, 2);
    graph.add_directed_edge(2, 0);

    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_directed_edge(0, 1);

    graph.remove_node(2);

    assert_eq!(graph, target_graph);
}

#[test]
pub fn test_directed_edge_removal() {
    let mut graph = Graph::new();
    let mut target_graph = Graph::new();

    graph.add_node();
    graph.add_node();
    graph.add_node();
    graph.add_directed_edge(0, 1);
    graph.add_directed_edge(1, 2);
    graph.add_directed_edge(2, 0);

    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_node();
    target_graph.add_directed_edge(0, 1);
    target_graph.add_directed_edge(1, 2);

    graph.remove_edge(2);

    assert_eq!(graph, target_graph);
}