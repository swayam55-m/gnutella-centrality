use crate::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs, EdgeRef};
use std::collections::{HashMap, VecDeque};

pub fn compute_closeness(graph: &Graph) -> HashMap<NodeIndex, f64> {
    let n = graph.node_count();
    let mut scores = HashMap::new();

    for node in graph.node_indices() {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(node, 0);
        queue.push_back(node);

        while let Some(current) = queue.pop_front() {
            let dist = visited[&current];
            for neighbor in graph.neighbors(current) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        let reachable = visited.len();
        let total_distance: usize = visited.values().sum();

        let closeness = if reachable > 1 && total_distance > 0 {
            (reachable as f64 - 1.0) / total_distance as f64
        } else {
            0.0
        };

        println!(
            "Node {}: reachable = {}, total_dist = {}, closeness = {}",
            node.index(),
            reachable,
            total_distance,
            closeness
        );

        scores.insert(node, closeness);
    }

    scores
}

pub fn compute_betweenness(graph: &Graph) -> HashMap<NodeIndex, f64> {
    let mut betweenness = HashMap::new();

    for s in graph.node_indices() {
        let mut stack = Vec::new();
        let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
        let mut sigma: HashMap<NodeIndex, usize> = HashMap::new();
        let mut dist: HashMap<NodeIndex, isize> = HashMap::new();

        for v in graph.node_indices() {
            pred.insert(v, Vec::new());
            sigma.insert(v, 0);
            dist.insert(v, -1);
        }

        sigma.insert(s, 1);
        dist.insert(s, 0);

        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for w in graph.neighbors(v) {
                if dist[&w] < 0 {
                    queue.push_back(w);
                    dist.insert(w, dist[&v] + 1);
                }
                if dist[&w] == dist[&v] + 1 {
                    sigma.insert(w, sigma[&w] + sigma[&v]);
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        let mut delta: HashMap<NodeIndex, f64> = HashMap::new();
        for v in graph.node_indices() {
            delta.insert(v, 0.0);
        }

        while let Some(w) = stack.pop() {
            for &v in &pred[&w] {
                let ratio = (sigma[&v] as f64) / (sigma[&w] as f64);
                delta.insert(v, delta[&v] + ratio * (1.0 + delta[&w]));
            }
            if w != s {
                *betweenness.entry(w).or_insert(0.0) += delta[&w];
            }
        }
    }

    for val in betweenness.values_mut() {
        *val /= 2.0; 
    }

    betweenness
}
