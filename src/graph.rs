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

        self.nodes.push(
            NodeData {
                node_name: node_name.to_string(), 
                node_index: index,
                outgoing_edges: HashSet::new()
            }
        );

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

    pub fn successors(&self, source: NodeIndex) -> Successors {
        unimplemented!{}
        // let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        // Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                Some(edge.target)
            }
        }
    }
}
