fn main() {
    println!("Hello, world!");
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    println!("{}", count_components_dfs(n, edges));
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    println!("{}", count_components_dfs(n, edges));
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    println!("{}", count_components_dsu(n, edges));
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    println!("{}", count_components_dsu(n, edges));
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    forests: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).into_iter().collect();
        let rank = vec![1; n as usize];

        Self {
            forests: n,
            parent,
            rank,
        }
    }

    fn find_parent_and_link(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.parent[x] = self.find_parent_and_link(self.parent[x]);
        self.parent[x]
    }

    fn union_set(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_parent_and_link(x);
        let y = self.find_parent_and_link(y);

        // already in the same component
        if x == y {
            return false;
        }

        // link
        if self.rank[x] > self.rank[y] {
            self.parent[x] = y;
        } else if self.rank[x] < self.rank[y] {
            self.parent[y] = x;
        } else {
            self.parent[y] = x;
            self.rank[y] += 1;
        }
        self.forests -= 1;

        true
    }
}

fn count_components_dsu(n: usize, edges: Vec<Vec<i32>>) -> i32 {
    _count_components_dsu(n, &edges)
}

fn _count_components_dsu(n: usize, edges: &Vec<Vec<i32>>) -> i32 {
    let mut union_set = UnionFind::new(n);
    for edge in edges {
        let _ = union_set.union_set(edge[0] as usize, edge[1] as usize);
    }
    union_set.forests as i32
}

fn count_components_dfs(n: usize, edges: Vec<Vec<i32>>) -> i32 {
    _count_components_dfs(n, &edges)
}

fn _count_components_dfs(n: usize, edges: &Vec<Vec<i32>>) -> i32 {
    let mut graph = vec![vec![]; n];

    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        graph[from].push(to);
        graph[to].push(from);
    }

    println!("graph={:?}", graph);
    let mut visited = vec![false; n];

    let mut rezult = 0;
    for i in 0..n {
        let mut first_time_reach = false;
        dfs(&graph, &mut visited, i, &mut first_time_reach);
        if first_time_reach {
            rezult += 1;
        }
    }

    rezult
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    visited: &mut [bool],
    curr_node: usize,
    first_time_reach: &mut bool,
) {
    visited[curr_node] = true;

    for &neighbor in graph[curr_node].iter() {
        if !visited[neighbor] {
            *first_time_reach = true;
            dfs(graph, visited, neighbor, first_time_reach)
        }
    }
}
