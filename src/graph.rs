// graph.rs
// Purpose: Load and represent the Gnutella graph using petgraph::Graph.

// This module handles reading the Gnutella dataset and constructing a directed graph.
// Each unique node ID is mapped to an internal petgraph NodeIndex.

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Graph = DiGraph<u32, ()>;

/// Loads the Gnutella P2P graph from a file.
/// Input: path to dataset (TSV with header lines and FromNodeId -> ToNodeId)
/// Output: Directed graph and mapping from raw node IDs to NodeIndex
pub fn load_graph(path: &str) -> (Graph, HashMap<u32, NodeIndex>) {
    let file = File::open(path).expect("Failed to open graph file");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    let mut node_map: HashMap<u32, NodeIndex> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.starts_with('#') || line.trim().is_empty() {
            continue; // Skip comments and blank lines
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue; // Malformed line
        }

        let from_id: u32 = parts[0].parse().unwrap();
        let to_id: u32 = parts[1].parse().unwrap();

        let from_idx = *node_map.entry(from_id).or_insert_with(|| graph.add_node(from_id));
        let to_idx = *node_map.entry(to_id).or_insert_with(|| graph.add_node(to_id));

        graph.add_edge(from_idx, to_idx, ());
    }

    (graph, node_map)
}
