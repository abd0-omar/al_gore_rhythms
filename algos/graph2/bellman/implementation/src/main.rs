fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

#[allow(unused)]
fn bellman(dist: &mut Vec<i32>, edge_list: &Vec<Edge>, n: usize) {
    for it in 0..n {
        println!("it={:?}", it);
        for i in 0..edge_list.len() {
            println!("i={:?}", i);
            println!("dist={:?}", dist);
        }
        for j in 0..edge_list.len() {
            let ne = &edge_list[j];
            if dist[ne.to] > dist[ne.from] + ne.weight {
                dist[ne.to] = dist[ne.from] + ne.weight;
            }
        }
    }
}
