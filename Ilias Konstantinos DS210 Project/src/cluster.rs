use std::collections::{HashMap, HashSet, VecDeque};

pub fn find_clusters(edges: &[(usize, usize)]) -> Vec<HashSet<usize>> {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    for &(u, v) in edges {
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }

    let mut visited = HashSet::new();
    let mut clusters = Vec::new();

    for &node in graph.keys() {
        if visited.contains(&node) {
            continue;
        }

        let mut cluster = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);
        visited.insert(node);

        while let Some(curr) = queue.pop_front() {
            cluster.insert(curr);
            for &nbr in graph.get(&curr).unwrap_or(&vec![]) {
                if !visited.contains(&nbr) {
                    visited.insert(nbr);
                    queue.push_back(nbr);
                }
            }
        }

        clusters.push(cluster);
    }

    clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_clusters() {
        let edges = vec![
            (1, 2), (2, 3), 
            (10, 11),       
        ];
        let clusters = find_clusters(&edges);

        let sizes: Vec<_> = clusters.iter().map(|c| c.len()).collect();
        assert_eq!(sizes.len(), 2);
        assert!(sizes.contains(&3));
        assert!(sizes.contains(&2));
    }
}