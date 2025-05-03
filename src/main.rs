// main.rs
// Main orchestrator of the program.
// Loads the graph, computes centrality, and prints results.
mod graph;
mod centrality;
mod visualize;
use visualize::draw_bar_chart;

use graph::load_graph;
use centrality::{compute_betweenness, compute_closeness};

fn display_top(title: &str, scores: &std::collections::HashMap<petgraph::graph::NodeIndex, f64>) {
    println!("\n--- {} ---", title);

    let mut sorted: Vec<_> = scores.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (node, score) in sorted.iter().take(10) {
        println!("Node {:?}: {:.4}", node.index(), score);
    }
}

fn main() {
    let (graph, id_map) = load_graph("data/p2p-Gnutella08.txt");
    println!("Graph loaded with {} nodes and {} edges", graph.node_count(), graph.edge_count());

    let closeness = compute_closeness(&graph);
    let betweenness = compute_betweenness(&graph);

    display_top("Closeness Centrality", &closeness);
    display_top("Betweenness Centrality", &betweenness);
    draw_bar_chart(
        "Top 10 Nodes by Closeness Centrality",
        "closeness_chart.png",
        &closeness,
        10,
    ).unwrap();

    draw_bar_chart(
        "Top 10 Nodes by Betweenness Centrality",
        "betweenness_chart.png",
        &betweenness,
        10,
    ).unwrap();

    println!("Charts saved as closeness_chart.png and betweenness_chart.png");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::load_graph;
    use crate::centrality::{compute_closeness, compute_betweenness};

    #[test]
    fn test_closeness_nonnegative() {
        let (graph, _) = load_graph("data/p2p-Gnutella08.txt");
        let scores = compute_closeness(&graph);
        for (&node, &score) in &scores {
            assert!(score >= 0.0, "Node {:?} has negative closeness: {}", node.index(), score);
        }
    }

    #[test]
    fn test_betweenness_nonnegative() {
        let (graph, _) = load_graph("data/p2p-Gnutella08.txt");
        let scores = compute_betweenness(&graph);
        for (&node, &score) in &scores {
            assert!(score >= 0.0, "Node {:?} has negative betweenness: {}", node.index(), score);
        }
    }
}

