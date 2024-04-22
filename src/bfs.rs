fn compute_min_distance(graph: &[Vec<usize>]) -> Vec<usize> {
    let mut distance: Vec<Option<usize>> = vec![None; graph.len()];
    let mut min_distance = Vec::new();
    
    for i in 0..graph.len()
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        for &u in &graph[v] {
            //if let None = distance[u] { // consider all unprocessed neighbors of v
                //distance[u] = Some(distance[v].unwrap() + 1);
                //queue.push_back(u);
            //}
        //}
    //}
    //for dist in distance.iter() {
        //if let Some(d) = dist {
            //min_distance.push(*d);
        //} else {
            //min_distance.push(0);

        //}
    //}
    //return min_distance;
//}

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