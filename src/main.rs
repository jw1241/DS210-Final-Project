use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read_file(path: &str) -> (Vec<Vec<usize>>, usize) { //Read the file and returning a vector & # of vertices
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    let mut max_index = 0;
    for i in buf_reader.lines() {
        let line = i.expect("Error reading");
        let v: Vec<&str> = line.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to get x");
        let y = v[1].parse::<usize>().expect("Failed to get y");
        result.push((x, y));
        if x > max_index {
            max_index = x;
        }
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![];max_index + 1]; //Adjacency list
    for (i,j) in result.iter() {
        graph[*i].push(*j);
    }    
    return (graph, max_index);
}

fn main() {
    let graph = read_file("amazon0302.txt");
    //let a = compute_min_distance(2, &graph).len();
    print!("{:?}", graph)
    
}

