use std::collections::HashMap;

// Undirected graph structure using an adjacency list
pub struct Graph {
    pub adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    // To construct graph with slice of edges (source, target, weight)
    pub fn new(edges: &[(i32, i32, i32)]) -> Self {
        let mut adjacency_list = HashMap::new();
        for &(source, target, _weight) in edges {
            adjacency_list.entry(source).or_insert_with(Vec::new).push(target);
            adjacency_list.entry(target).or_insert_with(Vec::new).push(source);
        }
        Graph { adjacency_list }
    }
}
