mod bfs;
use std::fs::File;
use std::io::prelude::*;
use crate::bfs::{compute_min_distance, uniform};

fn read_file(path: &str) -> Vec<Vec<usize>> { //Read the file and returning a vector & # of vertices
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    let mut max_index = 0;
    for i in buf_reader.lines() {
        let line = i.expect("Error reading");
        let v: Vec<&str> = line.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to get x");
        let y = v[1].parse::<usize>().expect("Failed to get y");
        if x > max_index {
            max_index = x;
        }
        result.push((x, y));
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![];max_index + 1]; //Adjacency list
    for (i,j) in result.iter() {
        graph[*i].push(*j);
    }    
    return graph;
}

fn main() {
    let graph = read_file("pagerank_data.txt");
    let a = uniform(&compute_min_distance(&graph));
    print!("{:?}", a);
}

