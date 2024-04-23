use std::collections::{VecDeque, HashMap};

pub fn compute_min_distance(graph: &Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
    let mut distance_all: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for i in 0..graph.len() {
        let mut distance: Vec<Option<usize>> = vec![None; graph.len()];
        let mut min_distance = Vec::new();

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(i);
        distance[i] = Some(0);

        while let Some(v) = queue.pop_front() { // new unprocessed vertex
            for &u in &graph[v] {
                if let None = distance[u] { // consider all unprocessed neighbors of v
                    distance[u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(u);
                }
            }
        }

        for dist in distance.iter() {
            if let Some(d) = dist {
                min_distance.push(*d);
            } else {
                min_distance.push(0);
            }
        }
        distance_all.insert(i, min_distance);
    }
    return distance_all;
}

pub fn uniform(distance: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let mut mean_all: HashMap<usize, f64> = HashMap::new();

    for (v, d) in distance.iter() {
        let sum: usize = d.iter().sum();
        let mean = sum as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    return mean_all;
}