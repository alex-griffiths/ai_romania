mod graph;

use graph::{Successors, Graph, NodeIndex};

fn main() {
    let mut graph = Graph::new();
    
    // Define each node. Each of these values is actually an index
    let oradea = graph.add_node("Oradea");
    let zerind = graph.add_node("Zerind");
    let arad = graph.add_node("Arad");
    let timisoara = graph.add_node("Timisoara");
    let lugoj = graph.add_node("Lugoj");
    let mehadia = graph.add_node("Mehadia");
    let dobreta = graph.add_node("Dobreta");
    let sibiu = graph.add_node("Sibiu");
    let rimnicu_vilcea = graph.add_node("Rimnicu Vilcea");
    let craiova = graph.add_node("Craiova");
    let fagaras = graph.add_node("Fagaras");
    let pitesti = graph.add_node("Pitesti");
    let bucharest = graph.add_node("Bucharest");
    let giurgiu = graph.add_node("Giurgiu");
    let neamt = graph.add_node("Neamt");
    let iasi = graph.add_node("Iasi");
    let vaslui = graph.add_node("Vaslui");
    let urziceni = graph.add_node("Urziceni");
    let hirsova = graph.add_node("Hirsova");
    let eforie = graph.add_node("Eforie");

    // Define edges between nodes and the weights of those edges
    graph.add_edge(oradea, zerind, 71);
    graph.add_edge(zerind, arad, 75);
    graph.add_edge(arad, timisoara, 118);
    graph.add_edge(timisoara, lugoj, 111);
    graph.add_edge(lugoj, mehadia, 70);
    graph.add_edge(mehadia, dobreta, 75);
    graph.add_edge(dobreta, craiova, 120);
    graph.add_edge(oradea, sibiu, 151);
    graph.add_edge(arad, sibiu, 140);
    graph.add_edge(sibiu, fagaras, 99);
    graph.add_edge(sibiu, rimnicu_vilcea, 80);
    graph.add_edge(rimnicu_vilcea, craiova, 146);
    graph.add_edge(rimnicu_vilcea, pitesti, 97);
    graph.add_edge(craiova, pitesti, 138);
    graph.add_edge(pitesti, bucharest, 101);
    graph.add_edge(fagaras, bucharest, 211);
    graph.add_edge(bucharest, giurgiu, 90);
    graph.add_edge(bucharest, urziceni, 85);
    graph.add_edge(urziceni, hirsova, 98);
    graph.add_edge(hirsova, eforie, 86);
    graph.add_edge(urziceni, vaslui, 142);
    graph.add_edge(vaslui, iasi, 92);
    graph.add_edge(iasi, neamt, 87);

    let mut start_node: NodeIndex;
    let mut end_node: NodeIndex;

    for node in &graph.nodes {
        print!("{} | ", node.node_name);

        for edge_idx in &node.outgoing_edges {
            // Find all the connected nodes via the edges
            let edge = graph.edges[*edge_idx];

            if edge.source != node.node_index {
                print!("{}({}) ", &graph.nodes[edge.source].node_name, edge.edge_weight);
            }
            if edge.target != node.node_index {
                print!("{}({}) ", &graph.nodes[edge.target].node_name, edge.edge_weight);
            }
        }

        println!("");
    }
}
