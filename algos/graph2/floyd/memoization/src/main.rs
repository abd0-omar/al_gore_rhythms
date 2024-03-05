fn main() {
    println!("Hello, world!");
    let mut adj_list = vec![
        vec![Some(0), Some(3), None, Some(7)],
        vec![Some(8), Some(0), Some(2), None],
        vec![Some(5), None, Some(0), Some(1)],
        vec![Some(2), None, None, Some(0)],
    ];
    floyd(&mut adj_list);

    for row in adj_list {
        for cell in row {
            let value = cell;
            print!(
                "{} ",
                match value {
                    Some(v) => v.to_string(),
                    None => "X".to_string(),
                }
            );
        }
        println!();
    }
}

fn floyd(adj_list: &mut Vec<Vec<Option<i32>>>) {
    let n = adj_list.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(a), Some(b)) = (adj_list[i][k], adj_list[k][j]) {
                    let relax = a + b;
                    if let Some(existing) = adj_list[i][j] {
                        adj_list[i][j] = Some(existing.min(relax));
                    } else {
                        adj_list[i][j] = Some(relax);
                    }
                }
            }
        }
    }
}
