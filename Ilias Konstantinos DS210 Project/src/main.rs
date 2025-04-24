mod graph;
mod cluster;
mod plot;

use graph::*;
use cluster::*;
use plot::*;

fn main() {
    let edges = read_file("email-Enron (1).txt");
    let email_map = load_email_mapping("email_to_node.csv");

    let degree = compute_degree(&edges);
    let mut deg_sorted: Vec<_> = degree.clone().into_iter().collect();
    deg_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n🏆 Top 10 by Degree Centrality:");
    for (i, (node, deg)) in deg_sorted.iter().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {} connections", i + 1, node, email, folder, deg);
        }
    }

    let top_nodes: std::collections::HashSet<usize> = deg_sorted.iter().take(1000).map(|(n, _)| *n).collect();

    println!("\n🏆 Top 10 by Closeness Centrality:");
    let closeness = compute_closeness(&edges, &top_nodes);
    let mut close_sorted: Vec<_> = closeness.clone().into_iter().collect();
    close_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (i, (node, score)) in close_sorted.iter().rev().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {:.5}", i + 1, node, email, folder, score);
        }
    }

    println!("\n🏆 Top 10 by Betweenness Centrality (top 1000 nodes only):");
    let betweenness = compute_betweenness(&edges, &top_nodes);
    let mut between_sorted: Vec<_> = betweenness.clone().into_iter().collect();
    between_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (i, (node, score)) in between_sorted.iter().take(10).enumerate() {
        if let Some((email, folder)) = email_map.get(node) {
            println!("{:>2}. Node {} ({}) [{}]: {:.5}", i + 1, node, email, folder, score);
        }
    }

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

    
    plot_degree_histogram(&degree).unwrap();
    plot_closeness_vs_degree(&degree, &closeness).unwrap();
    plot_betweenness_histogram(&betweenness).unwrap();
}
