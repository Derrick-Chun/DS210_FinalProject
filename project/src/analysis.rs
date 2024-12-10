use std::collections::HashMap;
use crate::graph::Graph;
extern crate plotters;
use plotters::prelude::*;

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();
    for edges in graph.adjacency_list.values() {
        let degree = edges.len();
        *distribution.entry(degree).or_insert(0) += 1;
    }
    distribution
}

pub fn plot_degree_distribution(distribution: &HashMap<usize, usize>) {
    println!("Degree Distribution:");
    for (degree, count) in distribution {
        println!("Degree {}: {}", degree, count);
    }
    let root_area = BitMapBackend::new("degree_distribution_histogram.png", (640, 480))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let max_count = distribution.values().max().unwrap_or(&0);
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Degree Distribution Histogram", ("Arial", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(1..120, 0..(*max_count as i32 + 1))
        .unwrap();
    chart.configure_mesh()
        .x_desc("Degree")
        .y_desc("Count")
        .draw()
        .unwrap();
    chart.draw_series(
        distribution.iter().filter_map(|(&degree, &count)| {
            if degree >= 1 && degree < 120 {
                Some(Rectangle::new(
                    [(degree as i32, 0), (degree as i32 + 1, count as i32)],
                    BLUE.filled(),
                ))
            } else {
                None
            }
        })
    ).unwrap();
}

pub fn trust_correlation_data(edges: &[(i32, i32, i32)]) -> Vec<(i32, usize, f64)> {
    let mut trust_to_neighbors = HashMap::new();
    // Collecting trust ratings for each vertex based on source
    for &(source, _target, weight) in edges {
        trust_to_neighbors.entry(source).or_insert_with(Vec::new).push(weight);
    }
    // Computing degree and average trust
    let mut results = Vec::new();
    for (vertex, trust_ratings) in trust_to_neighbors {
        let degree = trust_ratings.len();
        let avg_trust: f64 = trust_ratings.iter().map(|&x| x as f64).sum::<f64>() / degree as f64;
        results.push((vertex, degree, avg_trust));
    }
    results
}

pub fn trust_correlation(edges: &[(i32, i32, i32)]) {
    let data = trust_correlation_data(edges);
    // Setting points as (avg_trust, degree) to swap the axes for more neat looking graph
    let points: Vec<_> = data.iter().map(|(_, degree, avg_trust)| (*avg_trust, *degree as f64)).collect();
    println!("Trust Rating vs Number of Connections:");
    for (vertex, degree, avg_trust) in &data {
        println!("Vertex {}: Degree = {}, Average Trust = {:.2}", vertex, degree, avg_trust);
    }
    let root_area = BitMapBackend::new("trust_vs_degree.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let max_avg_trust = points.iter().map(|(x, _)| *x).fold(f64::NAN, f64::max);
    let max_degree = points.iter().map(|(_, y)| *y).fold(f64::NAN, f64::max);

    let max_avg_trust = if max_avg_trust.is_nan() { 0.0 } else { max_avg_trust };
    let max_degree = if max_degree.is_nan() { 0.0 } else { max_degree };

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Degree vs Average Trust", ("Arial", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        // X-axis: avg_trust & Y-axis: degree
        .build_cartesian_2d(0f64..(max_avg_trust + 1.0), 0f64..(max_degree + 1.0))
        .unwrap();
    chart.configure_mesh()
        .x_desc("Average Trust")
        .y_desc("Degree")
        .draw()
        .unwrap();
    chart.draw_series(
        points.iter().map(|(x, y)| Circle::new((*x, *y), 3, GREEN.filled()))
    ).unwrap();
}
