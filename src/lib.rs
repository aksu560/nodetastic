#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    edges: Vec<usize>,
    id: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EdgeType {
    Directed,
    Undirected,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq)]
pub struct Edge {
    nodes: [usize; 2],
    id: usize,
    edge_type: EdgeType,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        if self.nodes[0] == other.nodes[0] && self.nodes[1] == other.nodes[1] {
            true
        } else if self.edge_type == EdgeType::Undirected {
            self.nodes[0] == other.nodes[1] && self.nodes[1] == other.nodes[0]
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self) -> usize {
        
        let mut id = self.nodes.len();

        for (i, node) in self.nodes.iter().enumerate() {
            if node.id != i {
                id = i;
                break;
            }
        }

        self.nodes.push(Node { edges: Vec::new(), id });
        id
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(Edge {
            nodes: [node1, node2],
            id,
            edge_type: EdgeType::Undirected,
        });
        self.nodes[node1].edges.push(id);
        self.nodes[node2].edges.push(id);
        id
    }

    pub fn add_directed_edge(&mut self, node1: usize, node2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(Edge {
            nodes: [node1, node2],
            id,
            edge_type: EdgeType::Directed,
        });
        self.nodes[node1].edges.push(id);
        id
    }

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_edge(&self, id: usize) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn get_edge_between(&self, node1: usize, node2: usize) -> Option<&Edge> {
        self.nodes[node1]
            .edges
            .iter()
            .map(|&edge_id| &self.edges[edge_id])
            .find(|edge| edge.nodes.iter().any(|&node_id| node_id == node2))
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(id);
        for node in &mut self.nodes {
            node.edges.retain(|&edge_id| {
                let edge = &self.edges[edge_id];
                edge.nodes.iter().all(|&node_id| node_id != id)
            });
        }
        self.edges.retain(|edge| edge.nodes.iter().all(|&node_id| node_id != id));
    }

    pub fn remove_edge(&mut self, id: usize) {
        let edge = self.edges[id];
        self.nodes[edge.nodes[0]].edges.retain(|&edge_id| edge_id != id);
        self.nodes[edge.nodes[1]].edges.retain(|&edge_id| edge_id != id);
        self.edges.remove(id);
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test;