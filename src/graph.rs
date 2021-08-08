use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;
pub type EdgeWeight = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeData {
    pub node_name: String,
    pub node_index: NodeIndex,
    pub outgoing_edges: HashSet<EdgeIndex>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EdgeData {
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub edge_weight: EdgeWeight,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(NodeData {
            node_name: node_name.to_string(),
            node_index: index,
            outgoing_edges: HashSet::new(),
        });

        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, edge_weight: EdgeWeight) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            source: source,
            target: target,
            edge_weight: edge_weight,
        });

        node_data.outgoing_edges.insert(edge_index);
        self.nodes[target].outgoing_edges.insert(edge_index);
    }

    /// Accept a city name as an input and find the node that matches that name and then return
    /// the NodeIndex of that node.
    pub fn find_node(&mut self, node_name: &String) -> Option<NodeIndex> {
        let mut node_index: Option<NodeIndex> = None;
        for node in &self.nodes {
            if node_name.to_uppercase() == node.node_name.to_uppercase() {
                node_index = Some(node.node_index);
            }
        }

        node_index
    }

    /// Create a walker that can traverse the graph bi-directionally
    pub fn walker(&self, source: NodeIndex) -> Walker {
        Walker {
            graph: self,
            current_node_index: Some(source),
        }
    }
}

/// A walker that can traverse a graph bi-directionally from a
/// defined starting node index.
///
/// @TODO: Make this actually work.
pub struct Walker<'graph> {
    graph: &'graph Graph,
    current_node_index: Option<NodeIndex>,
}

impl<'graph> Walker<'graph> {
    pub fn new(start_idx: NodeIndex, graph: &Graph) -> Walker {
        Walker {
            graph: graph,
            current_node_index: Some(start_idx),
        }
    }
}

impl<'graph> Iterator for Walker<'graph> {
    type Item = &'graph NodeData;

    /// Iterate over the destination nodes and move to the one
    /// that is closest
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node_index {
            None => None,
            Some(node_idx) => {

                let current_node = &self.graph.nodes[node_idx];

                let mut closest_node: Option<(&NodeData, EdgeWeight)> = None;

                for edge_idx in &current_node.outgoing_edges {
                    // Find all the connected nodes via the edges
                    let edge = self.graph.edges[*edge_idx];

                    if edge.source != current_node.node_index {

                        closest_node = match closest_node {
                            None => {
                                Some((&self.graph.nodes[edge.source], edge.edge_weight))
                            },
                            Some(c_node) => {
                                if c_node.1 > edge.edge_weight {
                                    Some((&self.graph.nodes[edge.source], edge.edge_weight))
                                } else {
                                    Some(c_node)
                                }
                            }
                        }

                    }

                    // if edge.target != node.node_index {
                    //     print!(
                    //         "{}({}) ",
                    //         &graph.nodes[edge.target].node_name, edge.edge_weight
                    //     );
                    // }
                }


                Some(closest_node.unwrap().0)
            }
        }
    }
}
