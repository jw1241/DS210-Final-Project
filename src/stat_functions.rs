use std::collections::HashMap;
use round::round;

pub fn compute_mean(distance: &HashMap<usize, Vec<usize>>) -> ((usize, f64), (usize, f64)) {
    let mut mean_all: HashMap<usize, f64> = HashMap::new();
    //Iterate all the distances to compute the mean for each node
    for (v, d) in distance.iter() {
        let sum1: usize = d.iter().sum();
        let mean = sum1 as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut min_key = 0; //FInd the minimum & maximum mean and the associated node
    let mut max_key = 0;
    for (&v, &m) in mean_all.iter() {
        if m < min && m != 0.0 {
            min = m;
            min_key = v;
        } if m > max {
            max = m;
            max_key = v;
        }
    }
    return ((min_key, round(min, 3)), (max_key, round(max,3)));
}

pub fn compute_std(distance: &HashMap<usize, Vec<usize>>) -> ((usize, f64), (usize, f64)) {
    let mut std: HashMap<usize, f64> = HashMap::new();
    let mut mean_all: HashMap<usize, f64> = HashMap::new();
    //Find the mean of the Nodes
    for (v, d) in distance.iter() {
        let sum1: usize = d.iter().sum();
        let mean = sum1 as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    //Find the variance, then the standard deviation
    for (v, d) in distance.iter() {
        let mut sum_diff = 0.0;
        for &i in d {
            let diff = i as f64 - mean_all[v];
            sum_diff += diff * diff;
        }
        let variance = if d.len() == 0 {
            0.0
        } else {
            sum_diff / d.len() as f64
        };
        let stdeviation = variance.sqrt();
        std.insert(*v, stdeviation);
    } //Add the nodes & their associated standard deviation
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut min_key = 0; //FInd the minimum & maximum standard deviation and the associated node
    let mut max_key = 0;
    for (&v, &s) in std.iter() { 
        if s < min && s != 0.0 { //Find the minimum standard deviation & node
            min = s;
            min_key = v;
        } if s> max {
            max = s;
            max_key = v;
        }
    }
    return ((min_key, round(min, 3)), (max_key, round(max, 3)));
}

pub fn compute_min_max(distance: &HashMap<usize, Vec<usize>>) -> ((usize, usize), (usize, usize)) { 
    let mut min = usize::MAX; //Output the min & max distances > 0 & the node
    let mut max = usize::MIN;
    let mut min_key = 0 as usize;
    let mut max_key = 0 as usize;
    for (v, d) in distance.iter() {
        for &i in d { //Find max & min distances & their nodes
            if i < min && i != 0 {
                min = i;
                min_key = *v; 
            } if i > max {
                max = i;
                max_key = *v;
            }
        }
    }
    return ((min_key, min),(max_key, max));
}

pub fn compute_mean_deviation(distance: &HashMap<usize, Vec<usize>>) -> ((usize, f64), (usize, f64)) {
    let mut mean_deviation: HashMap<usize, f64> = HashMap::new();
    let mut mean_all: HashMap<usize, f64> = HashMap::new();
    //Find the mean of the distances for every node
    for (v, d) in distance.iter() {
        let sum1: usize = d.iter().sum();
        let mean = sum1 as f64 / d.len() as f64;
        mean_all.insert(*v, mean);
    }
    //Compute the mean deviation distances of every node
    for (v, d) in distance.iter() {
        let mean = mean_all[v];
        let mut sum_diff = 0.0;
        for &i in d {
            let diff = (i as f64 - mean).abs();
            sum_diff += diff;
        }
        
        let mean_dev = if d.len() != 0 {sum_diff / d.len() as f64} else {0.0};
        mean_deviation.insert(*v, mean_dev);
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut min_key = 0 as usize;
    let mut max_key = 0 as usize;
    for (&v, &s) in mean_deviation.iter() {
        if s < min && s != 0.0 { //Find the min & max mean_deviation & the node
            min = s;
            min_key = v;
        } if s > max {
            max = s;
            max_key = v;
        }
    }
    return ((min_key, round(min, 3)), (max_key, round(max, 3)));
}

