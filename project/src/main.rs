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

// Reads a CSV file, then each line represents an edge (source, target, weight) and returns a vector of (source, target, weight) tuples
// Please ensure the CSV file is accessible before running the code
fn read_file(file_path: &str) -> Vec<(i32, i32, i32)> {
    let file = File::open(file_path).expect("File not found");
    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();
    // Lines that are not matching the "source,target,weight" format is ignored
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

// Approximate average shortest path length is calculated through sampling random vertex pairs and BFS (for shortest path)
// Average their distances of those randomly chosen pairs and chose a small sample size (10) for simplicity
fn average_distance(graph: &Graph, samples: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let vertices: Vec<&i32> = graph.adjacency_list.keys().collect();
    if vertices.len() < 2 {
        // If graph has fewer than 2 vertices, the average distance is zero
        return 0.0;
    }
    let mut total_distance = 0;
    let mut valid_pairs = 0;
    // Shortest path distance is measured through randomly select pairs and BFS
    // If returns None, it means no path was found and that pair is not cosidered
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
    // Constructing the graph from the edges
    let graph = Graph::new(&edges);
    // Computing an approximate average shortest path
    // This is not a complete metric, instead only an approximation
    let avg_distance = average_distance(&graph, 10);
    println!("Average distance between two random vertices: {:.2}", avg_distance);
    // Computing the degree distribution of the graph and then plotting it
    let distribution = degree_distribution(&graph);
    plot_degree_distribution(&distribution);
    // Computing and plotting the correlation between vertex degree and trust rating
    trust_correlation(&edges);
}

// First test for test correlation
// Vertex 1 connects to 3 neighbors with trust ratings 10,20,30
#[test]
fn test_trust_correlation() {
    let edges = vec![
        (1, 2, 10),
        (1, 3, 20),
        (1, 4, 30),
        (2, 3, 40),
        (2, 4, 50),
    ];
    let results = trust_correlation_data(&edges);
    let mut vertex_map = HashMap::new();
    for (vertex, degree, avg_trust) in results {
        vertex_map.insert(vertex, (degree, avg_trust));
    }
    // To check the expected values:
    let (d1, a1) = vertex_map.get(&1).expect("Vertex 1 not found");
    assert_eq!(*d1, 3, "Expected Vertex 1 degree to be 3");
    assert!((a1 - 20.0).abs() < f64::EPSILON, "Expected Vertex 1 avg trust to be 20.0, got {}", a1);
    let (d2, a2) = vertex_map.get(&2).expect("Vertex 2 not found");
    assert_eq!(*d2, 2, "Expected Vertex 2 degree to be 2");
    assert!((a2 - 45.0).abs() < f64::EPSILON, "Expected Vertex 2 avg trust to be 45.0, got {}", a2);
}

// Second test for trust correlation 
// Vertex 10: 5 edges, all the edges with trust of 5, so the average trust should be exactly 5.0
#[test]
fn second_test_trust_correlation() {
    let edges = vec![
        (10, 11, 5),
        (10, 12, 5),
        (10, 13, 5),
        (10, 14, 5),
        (10, 15, 5),
    ];
    let results = trust_correlation_data(&edges);
    let mut vertex_map = HashMap::new();
    for (vertex, degree, avg_trust) in results {
        vertex_map.insert(vertex, (degree, avg_trust));
    }
    let (d10, a10) = vertex_map.get(&10).expect("Vertex 10 not found");
    assert_eq!(*d10, 5, "Expected Vertex 10 degree to be 5");
    assert!((a10 - 5.0).abs() < f64::EPSILON, "Expected Vertex 10 avg trust to be 5.0, got {}", a10);
}
