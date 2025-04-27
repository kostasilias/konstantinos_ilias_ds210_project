use plotters::prelude::*;
use std::collections::HashMap;
use plotters::style::Color;

pub fn plot_degree_histogram(degree: &HashMap<usize, usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("degree_histogram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut data: Vec<usize> = degree.values().copied().collect();
    data.sort_unstable();

    let max = *data.iter().max().unwrap_or(&1);
    let bins = 50;
    let bin_width = (max / bins).max(1);
    let mut counts = vec![0; bins];
    for val in &data {
        let idx = (*val / bin_width).min(bins - 1);
        counts[idx] += 1;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Centrality Distribution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..bins, 0..*counts.iter().max().unwrap_or(&1))?;

    chart
        .configure_mesh()
        .x_desc("Degree Bin")
        .y_desc("Count")
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data((0..counts.len()).map(|i| (i, counts[i]))),
    )?;

    Ok(())
}

pub fn plot_betweenness_histogram(between: &HashMap<usize, f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("betweenness_histogram.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut values: Vec<f64> = between.values().copied().collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let bins = 50;
    let max = values.iter().copied().fold(0.0, f64::max);
    let bin_width = max / bins as f64;
    let mut counts = vec![0; bins];
    for v in &values {
        let idx = (*v / bin_width).floor() as usize;
        let i = idx.min(bins - 1);
        counts[i] += 1;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Betweenness Centrality Distribution", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..bins, 0..*counts.iter().max().unwrap_or(&1))?;

    chart
        .configure_mesh()
        .x_desc("Betweenness Bin")
        .y_desc("Count")
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data((0..counts.len()).map(|i| (i, counts[i]))),
    )?;

    Ok(())
}

pub fn plot_closeness_vs_degree(
    degree: &HashMap<usize, usize>,
    closeness: &HashMap<usize, f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("closeness_vs_degree.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut points: Vec<(usize, f64)> = Vec::new();
    for (&node, &deg) in degree {
        if let Some(&close) = closeness.get(&node) {
            points.push((deg, close));
        }
    }

    let max_deg = points.iter().map(|x| x.0).max().unwrap_or(10);
    let max_closeness = points.iter().map(|x| x.1).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Closeness vs Degree", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_deg, 0.0..max_closeness)?;

    chart
        .configure_mesh()
        .x_desc("Degree")
        .y_desc("Closeness Centrality")
        .draw()?;

    chart.draw_series(points.iter().map(|(d, c)| Circle::new((*d, *c), 3, GREEN.filled())))?;

    Ok(())
}

pub fn plot_clusters(
    features: &HashMap<usize, (f64, f64, f64)>,
    assignments: &HashMap<usize, usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("clusters.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_deg = features.values().map(|x| x.0).fold(0.0, f64::max);
    let max_closeness = features.values().map(|x| x.1).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("K-Means Clusters: Closeness vs Degree", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..max_deg, 0.0..max_closeness)?;

    chart
        .configure_mesh()
        .x_desc("Degree")
        .y_desc("Closeness Centrality")
        .draw()?;

    let colors = [RED, BLUE, GREEN, BLACK, CYAN];

    for (&node, &(deg, close, _)) in features.iter() {
        if let Some(&cluster_id) = assignments.get(&node) {
            let color = colors[cluster_id % colors.len()];
            chart.draw_series(std::iter::once(Circle::new((deg, close), 3, color.filled())))?;
        }
    }

    Ok(())
}