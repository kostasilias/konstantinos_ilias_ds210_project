// Graph has functions that are used for loading the Enron dataset and computing centrality measures.
// Specifically it provides functions that read graph data, map emails to nodes, and compute degree, closeness, and betweenness.


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet, VecDeque};

// Reads an edge list file and returns a vector of (node1, node2) tuples.
// Its inputs are - `path`: path to the edge list file
// Its outputs are - `Vec<(usize, usize)>`: list of undirected edges
pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();

    for line in buf_reader.skip(4) {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split('\t').collect();
        if v.len() == 2 {
            if let (Ok(x), Ok(y)) = (v[0].parse(), v[1].parse()) {
                result.push((x, y));
            }
        }
    }
    result
}

// Loads a CSV file mapping node IDs to email addresses and folders.
// Its inputs are - `path`: path to the CSV mapping file
// Its outputs are - `HashMap<usize, (String, String)>`: node ID -> (email, folder)
pub fn load_email_mapping(path: &str) -> HashMap<usize, (String, String)> {
    let mut map = HashMap::new();
    let file = File::open(path).expect("Could not open mapping file");
    for (i, line) in BufReader::new(file).lines().enumerate() {
        if i == 0 { continue; }
        if let Ok(row) = line {
            let parts: Vec<&str> = row.trim().split(',').collect();
            if parts.len() >= 3 {
                if let Ok(id) = parts[0].parse() {
                    map.insert(id, (parts[1].to_string(), parts[2].to_string()));
                }
            }
        }
    }
    map
}

/// Computes the degree centrality (number of connections) for each node.
// Its inputs are - `edges`: list of graph edges
// Its outputs are - `HashMap<usize, usize>`: node ID -> degree
pub fn compute_degree(edges: &[(usize, usize)]) -> HashMap<usize, usize> {
    let mut degrees = HashMap::new();
    for &(u, v) in edges {
        *degrees.entry(u).or_insert(0) += 1;
        *degrees.entry(v).or_insert(0) += 1;
    }
    degrees
}

// Computes closeness centrality for a set of nodes.
// Its Inputs are - `edges`: list of edges - `nodes`: node IDs to compute closeness for each of them
// Its outputs are - `HashMap<usize, f64>`: node ID -> closeness centrality
pub fn compute_closeness(edges: &[(usize, usize)], nodes: &HashSet<usize>) -> HashMap<usize, f64> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(u, v) in edges {
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }

    let mut closeness = HashMap::new();

    for &start in nodes {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        visited.insert(start, 0);
        queue.push_back(start);

        // BFS to calculate shortest paths from start node
        while let Some(node) = queue.pop_front() {
            let dist = visited[&node];
            for &nbr in graph.get(&node).unwrap_or(&vec![]) {
                if !visited.contains_key(&nbr) {
                    visited.insert(nbr, dist + 1);
                    queue.push_back(nbr);
                }
            }
        }

        // Closeness = (number of reachable nodes) / (sum of distances)
        let total_distance: usize = visited.values().sum();
        let score = if total_distance > 0 {
            (visited.len() - 1) as f64 / total_distance as f64
        } else {
            0.0
        };
        closeness.insert(start, score);
    }

    closeness
}

// Computes betweenness centrality for a set of nodes using Brandes' algorithm.
// Its inputs are - `edges`: list of edges - `nodes`: node IDs to compute betweenness for
// Its outputs are - `HashMap<usize, f64>`: node ID -> normalized betweenness score
pub fn compute_betweenness(edges: &[(usize, usize)], nodes: &HashSet<usize>) -> HashMap<usize, f64> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(u, v) in edges {
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }

    let mut centrality = HashMap::new();
    for &s in nodes {
        let mut stack = Vec::new();
        let mut pred: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut sigma = HashMap::new();
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();

        for &v in graph.keys() {
            pred.insert(v, Vec::new());
            sigma.insert(v, 0.0);
        }

        // Initializing
        sigma.insert(s, 1.0);
        dist.insert(s, 0);
        queue.push_back(s);

        // BFS to find shortest paths
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let d = dist[&v];
            for &w in &graph[&v] {
                if !dist.contains_key(&w) {
                    dist.insert(w, d + 1);
                    queue.push_back(w);
                }
                if dist[&w] == d + 1 {
                    sigma.insert(w, sigma[&w] + sigma[&v]);
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        // Accumulating the dependencies
        let mut delta = HashMap::new();
        for &v in stack.iter().rev() {
            let coeff = (1.0 + *delta.get(&v).unwrap_or(&0.0)) / sigma[&v];
            for &p in &pred[&v] {
                let contrib = sigma[&p] * coeff;
                *delta.entry(p).or_insert(0.0) += contrib;
            }
            if v != s {
                *centrality.entry(v).or_insert(0.0) += delta.get(&v).unwrap_or(&0.0);
            }
        }
    }

    // Normalizing betweenness scores
    let max_val = centrality.values().cloned().fold(0.0, f64::max);
    if max_val > 0.0 {
        for val in centrality.values_mut() {
            *val /= max_val;
        }
    }

    centrality
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Tests that degree centrality is computed correctly.
    // It checks that each node's degree matches the expected number of connections.
    #[test]
    fn test_compute_degree() {
        let edges = vec![(1, 2), (2, 3), (3, 4)];
        let degree = compute_degree(&edges);

        assert_eq!(degree.len(), 4); 
        assert_eq!(degree[&1], 1);
        assert_eq!(degree[&2], 2);
        assert_eq!(degree[&3], 2);
        assert_eq!(degree[&4], 1);
    }

    // Tests that closeness centrality finds more central nodes correctly.
    // Node 2 (middle of chain) should have higher closeness centrality than node 1 (end of chain).
    #[test]
    fn test_compute_closeness() {
        let edges = vec![(1, 2), (2, 3), (3, 4)];
        let nodes: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
        let closeness = compute_closeness(&edges, &nodes);

        assert_eq!(closeness.len(), 4); 
        assert!(closeness[&2] > closeness[&1]);
        assert!(closeness[&3] > closeness[&4]);
    }

    // Tests that betweenness centrality correctly identifies bridge nodes.
    // Nodes in the middle (2, 3) should have higher betweenness than nodes in the edges (1, 4).
    #[test]
    fn test_compute_betweenness() {
        let edges = vec![(1, 2), (2, 3), (3, 4)];
        let nodes: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
        let betweenness = compute_betweenness(&edges, &nodes);

        assert_eq!(betweenness.len(), 4); 
        assert!(betweenness[&2] > betweenness[&1]);
        assert!(betweenness[&3] > betweenness[&4]);
    }
}
// Intermediate commit: updated graph.rs
// Intermediate commit: modified graph.rs
