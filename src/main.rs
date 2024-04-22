use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read_file(path: &str) -> Vec<Vec<usize>> { //Read the file and returning a vector & # of vertices
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    let n = 262111;
    for i in buf_reader.lines() {
        let line = i.expect("Error reading");
        let v: Vec<&str> = line.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to get x");
        let y = v[1].parse::<usize>().expect("Failed to get y");
        result.push((x, y));
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![];n]; //Adjacency list
    for (i,j) in result.iter() {
        graph[*i].push(*j);
    }    
    return graph;
}

fn compute_min_distance(start: usize, graph: &[Vec<usize>]) -> Vec<usize> {
    let mut distance: Vec<Option<usize>> = vec![None; 262111];
    let mut min_distance = Vec::new();
    distance[start] = Some(0); 
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
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
    return min_distance;
}

fn uniform(mut d: Vec<usize>) -> Vec<usize> {
    let mut a = Vec::new();
    for _ in 0..26111 {
        let mut sum = 0;
        for i in 0..262111 {
            sum += d[i];
        }
        a.push(sum);   
        let b = d.split_off(26112);
        d = b;
    }
return a;
}



fn main() {
    let graph = read_file("amazon0302.txt");
    let a = compute_min_distance(2, &graph).len();
    print!("{:?}", a)
    
}

