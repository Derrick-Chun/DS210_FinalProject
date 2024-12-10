use crate::graph::Graph;
use std::collections::VecDeque;
use std::collections::HashMap;

pub fn bfs(graph: &Graph, start: i32, end: i32) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((start, 0));
    visited.insert(start, true);
    while let Some((node, distance)) = queue.pop_front() {
        if node == end {
            return Some(distance);
        }
        if let Some(neighbors) = graph.adjacency_list.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, true);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
    }
    None
}
