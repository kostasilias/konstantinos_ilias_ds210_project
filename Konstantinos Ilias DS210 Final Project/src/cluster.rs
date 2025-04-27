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

use rand::seq::SliceRandom;
use rand::thread_rng;

pub type Features = (f64, f64, f64);

pub fn kmeans(
    features: &std::collections::HashMap<usize, Features>,
    k: usize,
    max_iters: usize,
) -> std::collections::HashMap<usize, usize> {
    let mut rng = thread_rng();
    let node_ids: Vec<_> = features.keys().cloned().collect();

    let mut centroids: Vec<Features> = node_ids
        .choose_multiple(&mut rng, k)
        .map(|id| features[id])
        .collect();

    let mut assignments = std::collections::HashMap::new();

    for _ in 0..max_iters {
        for (&node, &feat) in features {
            let mut best = 0;
            let mut best_dist = euclidean_distance(feat, centroids[0]);
            for (i, &centroid) in centroids.iter().enumerate().skip(1) {
                let dist = euclidean_distance(feat, centroid);
                if dist < best_dist {
                    best = i;
                    best_dist = dist;
                }
            }
            assignments.insert(node, best);
        }

        let mut counts = vec![0; k];
        let mut sums = vec![(0.0, 0.0, 0.0); k];

        for (&node, &cluster) in &assignments {
            let (d, c, b) = features[&node];
            sums[cluster].0 += d;
            sums[cluster].1 += c;
            sums[cluster].2 += b;
            counts[cluster] += 1;
        }

        for i in 0..k {
            if counts[i] > 0 {
                centroids[i] = (
                    sums[i].0 / counts[i] as f64,
                    sums[i].1 / counts[i] as f64,
                    sums[i].2 / counts[i] as f64,
                );
            }
        }
    }

    assignments
}

fn euclidean_distance(a: Features, b: Features) -> f64 {
    let (dx, dy, dz) = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn normalize_features(features: &mut std::collections::HashMap<usize, Features>) {
    let mut max_d = 0.0;
    let mut max_c = 0.0;
    let mut max_b = 0.0;

    for (_, (d, c, b)) in features.iter() {
        if *d > max_d { max_d = *d; }
        if *c > max_c { max_c = *c; }
        if *b > max_b { max_b = *b; }
    }

    for val in features.values_mut() {
        if max_d > 0.0 { val.0 /= max_d; }
        if max_c > 0.0 { val.1 /= max_c; }
        if max_b > 0.0 { val.2 /= max_b; }
    }
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

        assert_eq!(clusters.len(), 2); 
        let sizes: Vec<_> = clusters.iter().map(|c| c.len()).collect();
        assert!(sizes.contains(&3)); 
        assert!(sizes.contains(&2)); 
    }

    #[test]
    fn test_normalize_features() {
        let mut features = HashMap::new();
        features.insert(1, (2.0, 4.0, 8.0));
        features.insert(2, (1.0, 2.0, 6.0));
        
        normalize_features(&mut features);

        assert_eq!(features.get(&1), Some(&(1.0, 1.0, 1.0))); 
        assert_eq!(features.get(&2), Some(&(0.5, 0.5, 0.75)));
    }

    #[test]
    fn test_kmeans() {
        let mut features = HashMap::new();
        features.insert(1, (0.0, 0.0, 0.0));
        features.insert(2, (1.0, 1.0, 1.0));
        features.insert(3, (0.9, 1.0, 0.95));
        features.insert(4, (0.0, 0.1, 0.0));

        let assignments = kmeans(&features, 2, 10);

        assert_eq!(assignments.len(), 4);
        assert!(assignments.values().all(|&c| c == 0 || c == 1)); 
    }
}