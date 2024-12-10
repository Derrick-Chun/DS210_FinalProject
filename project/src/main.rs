mod graph;
mod bfs;
mod analysis;

use graph::Graph;
use bfs::bfs;
use analysis::{degree_distribution, plot_degree_distribution, trust_correlation_data, trust_correlation};
use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::SliceRandom;
use std::collections::HashMap;

// Reading the csv file and returns the vector of edges: source, target, weight
fn read_file(file_path: &str) -> Vec<(i32, i32, i32)> {
    let file = File::open(file_path).expect("File not found");
    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let source = parts[0].parse::<i32>().unwrap();
            let target = parts[1].parse::<i32>().unwrap();
            let weight = parts[2].parse::<i32>().unwrap();
            edges.push((source, target, weight));
        }
    }
    edges
}

// Calculating the average shortest path of the distance between random vertex pairs
fn average_distance(graph: &Graph, samples: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let vertices: Vec<&i32> = graph.adjacency_list.keys().collect();
    if vertices.len() < 2 {
        return 0.0;
    }
    let mut total_distance = 0;
    let mut valid_pairs = 0;
    for _ in 0..samples {
        if let Some(&start) = vertices.choose(&mut rng) {
            if let Some(&end) = vertices.choose(&mut rng) {
                if start != end {
                    if let Some(distance) = bfs(graph, *start, *end) {
                        total_distance += distance;
                        valid_pairs += 1;
                    }
                }
            }
        }
    }
    if valid_pairs > 0 {
        total_distance as f64 / valid_pairs as f64
    } else {
        0.0
    }
}

fn main() {
    let file_path = "../soc-sign-bitcoinalpha_backup.csv";
    let edges = read_file(file_path);
    println!("Loaded {} edges", edges.len());
    let graph = Graph::new(&edges);
    // Computing the average distance between vertices
    let avg_distance = average_distance(&graph, 10); // reduced sample size for simplicity
    println!("Average distance between two random vertices: {:.2}", avg_distance);
    // Computing the degree distribution
    let distribution = degree_distribution(&graph);
    plot_degree_distribution(&distribution);
    // Computing the trust rating correlation
    trust_correlation(&edges);
}

#[test]
fn test_trust_correlation() {
    // Creating a small set of edges:
    // Vertex 1: connected to 3 others with trust values 10, 20, 30
    // Vertex 2: connected to 2 others with trust values 40, 50
    let edges = vec![
        (1, 2, 10),
        (1, 3, 20),
        (1, 4, 30),
        (2, 3, 40),
        (2, 4, 50),
    ];
    // Calling the helper function to get the trust correlation data
    let results = trust_correlation_data(&edges);
    // Converting the results into a map for easier lookup
    let mut vertex_map = HashMap::new();
    for (vertex, degree, avg_trust) in results {
        vertex_map.insert(vertex, (degree, avg_trust));
    }
    // What is expected:
    // Vertex 1: Degree = 3, Average Trust = (10+20+30)/3 = 20.0
    // Vertex 2: Degree = 2, Average Trust = (40+50)/2 = 45.0
    let (d1, a1) = vertex_map.get(&1).expect("Vertex 1 not found");
    assert_eq!(*d1, 3, "Expected Vertex 1 degree to be 3");
    assert!((a1 - 20.0).abs() < f64::EPSILON, "Expected Vertex 1 avg trust to be 20.0, got {}", a1);
    let (d2, a2) = vertex_map.get(&2).expect("Vertex 2 not found");
    assert_eq!(*d2, 2, "Expected Vertex 2 degree to be 2");
    assert!((a2 - 45.0).abs() < f64::EPSILON, "Expected Vertex 2 avg trust to be 45.0, got {}", a2);
}

#[test]
fn second_test_trust_correlation() {
    // Creating another set of edges:
    // Vertex 10 is connected to 5 different vertices, while the verticies all have the same trust: 5
    let edges = vec![
        (10, 11, 5),
        (10, 12, 5),
        (10, 13, 5),
        (10, 14, 5),
        (10, 15, 5),
    ];
    // Vertex 10 should have degree = 5, and average trust = (5+5+5+5+5)/5 = 5.0
    let results = trust_correlation_data(&edges);
    let mut vertex_map = HashMap::new();
    for (vertex, degree, avg_trust) in results {
        vertex_map.insert(vertex, (degree, avg_trust));
    }
    let (d10, a10) = vertex_map.get(&10).expect("Vertex 10 not found");
    assert_eq!(*d10, 5, "Expected Vertex 10 degree to be 5");
    assert!((a10 - 5.0).abs() < f64::EPSILON, "Expected Vertex 10 avg trust to be 5.0, got {}", a10);
}
