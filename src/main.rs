mod bfs;
mod stat_functions;
use std::fs::File;
use std::io::prelude::*;
use crate::bfs::compute_min_distance;
use crate::stat_functions::{compute_mean_min, compute_std_min, compute_min_max, compute_mean_deviation};

fn read_file(path: &str) -> Vec<Vec<usize>> { //Read the file and returning a vector & # of vertices
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    let mut max_index = 0;
    for i in buf_reader.lines() {
        let line = i.expect("Error reading");
        let v: Vec<&str> = line.trim().split(',').collect();
        let x = v[0].parse::<usize>().expect("Failed to get x");
        let y = v[1].parse::<usize>().expect("Failed to get y");
        if x > max_index {
            max_index = x;
        }
        if y > max_index {
            max_index = y;
        }
        result.push((x, y));
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![];max_index + 1]; //Adjacency list
    for (i,j) in result.iter() {
        graph[*i].push(*j);
        graph[*j].push(*i);
    }    
    return graph;
}
fn main() {
    let graph = read_file("lastfm_asia_edges.txt");
    let a = compute_mean_deviation(&compute_min_distance(&graph));
    println!("{:?}", a)
}