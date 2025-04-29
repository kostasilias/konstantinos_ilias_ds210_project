// This is the main program to compute centrality measures on the Enron email network dataset and conduct clustering.
// It loads the dataset, calculates degree, closeness, and betweenness centralities,
// It also performs clustering (connected components + k-means), and generates plots.

mod graph;
mod cluster;
mod plot;

use graph::*;
use plot::*;
use std::collections::HashMap;

fn main() {
    // Load the edge list and email mapping
    let edges = read_file("email-Enron (1).txt");
    let email_map = load_email_mapping("email_to_node.csv");

    // Compute degree centrality
    let degree = compute_degree(&edges);
    let mut deg_sorted: Vec<_> = degree.clone().into_iter().collect();
    deg_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // Print Top 10 nodes by Degree Centrality
    println!("\n🏆 Top 10 by Degree Centrality:");
    for (i, (node, deg)) in deg_sorted.iter().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {} connections", i + 1, node, email, folder, deg);
        }
    }

    // Select top 1000 nodes for more computationally expensive centralities
    let top_nodes: std::collections::HashSet<usize> = deg_sorted.iter().take(1000).map(|(n, _)| *n).collect();

    // Compute closeness centrality for top nodes and print Top 10
    println!("\n🏆 Top 10 by Closeness Centrality:");
    let closeness = compute_closeness(&edges, &top_nodes);
    let mut close_sorted: Vec<_> = closeness.clone().into_iter().collect();
    close_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (i, (node, score)) in close_sorted.iter().rev().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {:.5}", i + 1, node, email, folder, score);
        }
    }

    // Compute betweenness centrality for top nodes and print Top 10
    println!("\n🏆 Top 10 by Betweenness Centrality (top 1000 nodes only):");
    let betweenness = compute_betweenness(&edges, &top_nodes);
    let mut between_sorted: Vec<_> = betweenness.clone().into_iter().collect();
    between_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (i, (node, score)) in between_sorted.iter().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {:.5}", i + 1, node, email, folder, score);
        }
    }

    // Find clusters and print leaders
    println!("\n🏆 Cluster Leaders by Degree:");
    let clusters = find_clusters(&edges);
    for (i, cluster) in clusters.iter().enumerate().take(10) {
        let leader = cluster
            .iter()
            .max_by_key(|&&n| degree.get(&n).unwrap_or(&0))
            .unwrap();
        let degree_score = degree.get(leader).unwrap_or(&0);
        if let Some((email, folder)) = email_map.get(leader) {
            println!(
                "🧩 Cluster {} ({} nodes) → Node {} ({}) [{}], Degree: {}",
                i + 1, cluster.len(), leader, email, folder, degree_score
            );
        }
    }

    use cluster::{find_clusters, kmeans, normalize_features}; 

    // Prepare feature vectors for K-Means clustering: (degree, closeness, betweenness)
    let mut features = HashMap::new();
    for &node in top_nodes.iter() {
        let deg = *degree.get(&node).unwrap_or(&0) as f64;
        let close = *closeness.get(&node).unwrap_or(&0.0);
        let between = *betweenness.get(&node).unwrap_or(&0.0);
        features.insert(node, (deg, close, between));
    }

    // Normalize features to avoid scaling bias
    normalize_features(&mut features);

    // Apply k-means clustering for k = 5 groups
    let assignments = kmeans(&features, 5, 100);

    println!("\n🕸️ K-Means Clustering (5 clusters):");
    for i in 0..5 {
        println!("Cluster {}:", i);
        for (&node, &cluster_id) in &assignments {
            if cluster_id == i {
                if let Some((email, folder)) = email_map.get(&node) {
                    println!("  Node {} ({}) [{}]", node, email, folder);
                }
            }
        }
    }

    // Generate plots 
    plot_degree_histogram(&degree).unwrap();
    plot_closeness_vs_degree(&degree, &closeness).unwrap();
    plot_betweenness_histogram(&betweenness).unwrap();
    plot_clusters(&features, &assignments).unwrap();
}
// Intermediate commit: updated main.rs
