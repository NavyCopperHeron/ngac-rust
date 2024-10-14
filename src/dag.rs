use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct DAG<T> {
    adjacency_list: HashMap<T, Vec<T>>, // A map from each node to its neighbors
}

impl<T: Eq + std::hash::Hash + Clone + std::fmt::Debug> DAG<T> {
    // Creates a new, empty DAG
    pub fn new() -> Self {
        DAG {
            adjacency_list: HashMap::new(),
        }
    }

    // Add a new edge from `from` to `to`
    pub fn add_edge(&mut self, from: T, to: T) {
        self.adjacency_list
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
        self.adjacency_list.entry(to).or_insert_with(Vec::new);
    }

    // BFS implementation
    pub fn bfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        result
    }

    // DFS implementation (recursive)
    pub fn dfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_helper(start, &mut visited, &mut result);
        result
    }

    // Helper function for DFS
    fn dfs_helper(&self, node: &T, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        if visited.contains(node) {
            return;
        }

        visited.insert(node.clone());
        result.push(node.clone());

        if let Some(neighbors) = self.adjacency_list.get(node) {
            for neighbor in neighbors {
                self.dfs_helper(neighbor, visited, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dag() {
        let mut dag = DAG::new();

        // Add edges to the DAG
        dag.add_edge(1, 2);
        dag.add_edge(1, 3);
        dag.add_edge(3, 4);
        dag.add_edge(2, 4);
        dag.add_edge(4, 5);

        println!("BFS starting from 1: {:?}", dag.bfs(&1));
        println!("DFS starting from 1: {:?}", dag.dfs(&1));
    }
}
