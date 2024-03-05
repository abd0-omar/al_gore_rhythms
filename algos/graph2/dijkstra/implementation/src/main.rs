use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
    let adj_list = vec![
        vec![Edge { to: 1, weight: 2 }, Edge { to: 3, weight: 1 }],
        vec![Edge { to: 4, weight: 10 }, Edge { to: 3, weight: 3 }],
        vec![Edge { to: 0, weight: 4 }, Edge { to: 5, weight: 5 }],
        vec![
            Edge { to: 4, weight: 2 },
            Edge { to: 6, weight: 4 },
            Edge { to: 2, weight: 2 },
            Edge { to: 5, weight: 8 },
        ],
        vec![Edge { to: 6, weight: 6 }],
        vec![Edge { to: 5, weight: 1 }],
        vec![],
    ];
    let src = 1;
    let dist = dijkstra(adj_list, src);
    println!(
        "dist={:?}",
        dist.iter().flatten().cloned().collect::<Vec<usize>>()
    );
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Edge {
    to: usize,
    weight: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.to.partial_cmp(&self.to) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.to.cmp(&other.to))
    }
}

// adj_list:  2 -> 0
// [[], [], [0]]
fn dijkstra(adj_list: Vec<Vec<Edge>>, src: usize) -> Vec<Option<usize>> {
    let n = adj_list.len();
    let mut dist: Vec<Option<usize>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut bh = BinaryHeap::new();
    bh.push(Edge { to: src, weight: 0 });
    visited[src] = true;
    dist[src] = Some(0);

    while let Some(edge) = bh.pop() {
        for neighbor_edge in adj_list[edge.to].iter() {
            if visited[neighbor_edge.to] {
                continue;
            }

            // if el neighbor el gded as8r yb2a el dist tt8yer
            let accumlated_dist = neighbor_edge.weight + dist[edge.to].unwrap();
            if let Some(old_dist) = dist[neighbor_edge.to] {
                if accumlated_dist < old_dist {
                    // relaxing
                    dist[neighbor_edge.to] = Some(accumlated_dist);
                    // add to heap
                    bh.push(Edge {
                        to: neighbor_edge.to,
                        weight: accumlated_dist,
                    })
                }
            } else {
                // first time assigning to the node's distance
                dist[neighbor_edge.to] = Some(accumlated_dist);
                bh.push(Edge {
                    to: neighbor_edge.to,
                    weight: accumlated_dist,
                })
            }
        }
        println!("bh={:?}", bh);
        visited[edge.to] = true;
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_basic() {
        let adj_list = vec![
            vec![Edge { to: 1, weight: 2 }, Edge { to: 2, weight: 4 }],
            vec![Edge { to: 2, weight: 1 }],
            vec![],
        ];
        /*
        0 -(2)->1
        |       |
        (4)     |
        |       |
        2 <-(1)-
        */
        // won't draw test cases again as I can't find down arrow
        let src = 0;
        let dist = dijkstra(adj_list, src);
        println!("dist={:?}", dist);
        assert_eq!(dist[0], Some(0));
        assert_eq!(dist[1], Some(2));
        assert_eq!(dist[2], Some(4));
    }

    #[test]
    fn test_shortest_path_lecture_example() {
        let adj_list = vec![
            vec![Edge { to: 1, weight: 2 }, Edge { to: 3, weight: 1 }],
            vec![Edge { to: 4, weight: 10 }, Edge { to: 3, weight: 3 }],
            vec![Edge { to: 0, weight: 4 }, Edge { to: 5, weight: 5 }],
            vec![
                Edge { to: 4, weight: 2 },
                Edge { to: 6, weight: 4 },
                Edge { to: 2, weight: 2 },
                Edge { to: 5, weight: 8 },
            ],
            vec![Edge { to: 6, weight: 6 }],
            vec![Edge { to: 5, weight: 1 }],
            vec![],
        ];
        let src = 1;
        let dist = dijkstra(adj_list, src);
        println!("dist={:?}", dist);
    }
}
