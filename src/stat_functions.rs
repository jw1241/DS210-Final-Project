use std::collections::HashMap;

pub fn compute_mean_min(distance: &HashMap<usize, Vec<usize>>) -> (usize, f64) {
    let mut mean_all: HashMap<usize, f64> = HashMap::new();

    for (v, d) in distance.iter() {
        let sum: usize = d.iter().sum();
        let mean = sum as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    let mut min = f64::MAX;
    let mut key = 0 as usize;
    for (&v, &m) in mean_all.iter() {
        if m < min {
            min = m;
            key = v;
        } 
    }
    return (key, min);
}

pub fn compute_std_min(distance: &HashMap<usize, Vec<usize>>) -> (usize, f64) {
    let mut std: HashMap<usize, f64> = HashMap::new();
    let mut mean_all: HashMap<usize, f64> = HashMap::new();

    for (v, d) in distance.iter() {
        let sum: usize = d.iter().sum();
        let mean = sum as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    
    for (v, d) in distance.iter() {
        let mut sum_diff = 0.0;
        for &i in d {
            let diff = i as f64 - mean_all[v];
            sum_diff += diff * diff;
        }
        let mut variance= 0.0;
        if d.len() == 0 {
            variance = 0.0;   
        } else {
            variance = sum_diff / d.len() as f64
        }
        let stdeviation = variance.sqrt();
        std.insert(*v, stdeviation);
    }
    let mut min = f64::MAX;
    let mut key = 0 as usize;
    for (&v, &s) in std.iter() {
        if s < min {
            min = s;
            key = v;
        }
    }
    return (key, min);
}

fn compute_min_max() {}