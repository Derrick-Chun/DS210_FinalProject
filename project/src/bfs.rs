use crate::graph::Graph;
use std::collections::{VecDeque, HashMap};

// BFS to used to return the shortest path distance (in edge count)
// None is returned if the end is not reachable
pub fn bfs(graph: &Graph, start: i32, end: i32) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((start, 0));
    visited.insert(start, true);
    while let Some((node, distance)) = queue.pop_front() {
        if node == end {
            return Some(distance);
        }
        // Checks for neighbors in the current node
        if let Some(neighbors) = graph.adjacency_list.get(&node) {
            for &neighbor in neighbors {
                // If the neighbor not visited, it adds to queue with distance + 1
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, true);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
    }
    None
}
