mod bfs;
mod stat_functions;
use std::fs::File;
use std::io::prelude::*;
use crate::bfs::compute_min_distance;
use crate::stat_functions::{compute_mean, compute_std, compute_min_max, compute_mean_deviation};
use std::collections::HashSet;

fn read_file(path: &str) -> Vec<Vec<usize>> { //Read the file and returning a vector with node & # of vertices
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    let mut max_index = 0; //Find the highest node/vertex value
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for i in buf_reader.lines() {
        let line = i.expect("Error reading");
        let v: Vec<&str> = line.trim().split(',').collect();
        let x = v[0].parse::<usize>().expect("Failed to get x");
        let y = v[1].parse::<usize>().expect("Failed to get y");
        if x > max_index { //Check if x and y are bigger than current max_index
            max_index = x;
        }
        if y > max_index {
            max_index = y;
        }
        result.push((x, y)); //Add to the result vector
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![];max_index + 1]; 
    //Adjacency list built with the highest node/vertext to ensure all nodes are captured
    for &(i,j) in result.iter() {
        if visited.contains(&(i, j)) || visited.contains(&(j, i)) {
            continue;
        } else {
            graph[i].push(j); //Iterate through the vector and push into a vec of vec with vec<node<vec<vertices>>
            graph[j].push(i); //Push twice with the vertex as the node, undirected graph
            visited.insert((i,j));
            visited.insert((j,i));
        }
    }    
    return graph;
}
fn main() {
    let graph = read_file("test_set.txt");
    let z = compute_min_distance(&graph);
    let a = compute_mean(&z);
    let b = compute_std(&z);
    let c = compute_min_max(&z);
    let d = compute_mean_deviation(&z);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", graph);
    println!("{:?}", z);
}

#[test]
fn check_read() { //Check if the read function connects the nodes & edges properly
    let adjaceny_list = read_file("test_set.txt");
    let correct: Vec<Vec<usize>> = vec![vec![5, 6, 4], vec![7, 5], vec![6, 8], vec![6], vec![7, 0], 
                                        vec![0, 1], vec![0, 2, 3], vec![1, 4], vec![2, 9], vec![8]];
    assert_eq!(adjaceny_list, correct);
}

#[test]
fn check_bfs() { //Check if BFS runs correctly, check on correct adjacency list
    let adjaceny_list = read_file("test_set.txt");
    let distance = compute_min_distance(&adjaceny_list);
    let correct: Vec<Vec<usize>> = vec![vec![0, 2, 2, 2, 1, 1, 1, 2, 3, 4], vec![2, 0, 4, 4, 2, 1, 3, 1, 5, 6], vec![2, 4, 0, 2, 3, 3, 1, 4, 1, 2],
    vec![2, 4, 2, 0, 3, 3, 1, 4, 3, 4], vec![1, 2, 3, 3, 0, 2, 2, 1, 4, 5], vec![1, 1, 3, 3, 2, 0, 2, 2, 4, 5], vec![1, 3, 1, 1, 2, 2, 0, 3, 2, 3],
    vec![2, 1, 4, 4, 1, 2, 3, 0, 5, 6], vec![3, 5, 1, 3, 4, 4, 2, 5, 0, 1], vec![4, 6, 2, 4, 5, 5, 3, 6, 1, 0]];
    
    for (v, d) in distance.iter() {
        for i in 0..distance.len() {
            if i == *v {
                assert_eq!(*d, correct[i]);
            }
        }
    }
}

#[test]
fn check_stat_func() { //Compare the values to check and the correct values
    let adjaceny_list = read_file("test_set.txt");
    let distance = compute_min_distance(&adjaceny_list);
    let mean = compute_mean(&distance);
    let std = compute_std(&distance);
    let min_max = compute_min_max(&distance);
    let deviation = compute_mean_deviation(&distance);
    let check = vec![mean.0.1, mean.1.1, std.0.1, std.1.1, min_max.0.1 as f64, min_max.1.1 as f64, deviation.0.1, deviation.1.1];
    let correct = vec![1.8, 3.6, 0.98, 1.96, 1.0, 6.0, 0.84, 1.68];

    for i in 0..correct.len() {
        assert_eq!(check[i], correct[i]);
    }
}

