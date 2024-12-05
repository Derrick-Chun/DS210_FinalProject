mod graph;
mod bfs;
mod analysis;

use graph::Graph;
use bfs::bfs;
use analysis::{degree_distribution, plot_degree_distribution, trust_correlation};
use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::SliceRandom;

// reads the CSV file and returns a vector of edges: (source, target, weight).
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

// computes the average shortest-path distance between random vertex pairs.
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

    // average distance between vertices
    let avg_distance = average_distance(&graph, 10); // reduced sample size for simplicity
    println!("Average distance between two random vertices: {:.2}", avg_distance);

    // degree distribution
    let distribution = degree_distribution(&graph);
    plot_degree_distribution(&distribution);

    // trust rating correlation
    trust_correlation(&edges);
}
