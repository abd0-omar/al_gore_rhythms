fn main() {
    println!("Hello, world!");
}

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![vec![false; image[0].len()]; image.len()];
    let old_color = image[sr as usize][sc as usize];
    dfs(&mut image, sr, sc, color, &mut visited, old_color);
    image
}

pub fn isvalid(image: &Vec<Vec<i32>>, sr: i32, sc: i32) -> bool {
    if sr >= image.len() as i32 || sr < 0 || sc >= image[0].len() as i32 || sc < 0 {
        return false;
    }
    return true;
}

pub fn dfs(
    image: &mut Vec<Vec<i32>>,
    sr: i32,
    sc: i32,
    color: i32,
    visited: &mut Vec<Vec<bool>>,
    old_color: i32,
) {
    if !isvalid(image, sr, sc)
        || visited[sr as usize][sc as usize] == true
        || image[sr as usize][sc as usize] != old_color
    {
        return;
    }

    visited[sr as usize][sc as usize] = true;
    image[sr as usize][sc as usize] = color;

    let dr = [-1, 0, 1, 0];
    let dc = [0, -1, 0, 1];

    for i in 0..4 {
        dfs(image, sr + dr[i], sc + dc[i], color, visited, old_color)
    }

    // dfs(image, sr - 1, sc, color, visited, old_color);
    // dfs(image, sr + 1, sc, color, visited, old_color);
    // dfs(image, sr, sc - 1, color, visited, old_color);
    // dfs(image, sr, sc + 1, color, visited, old_color);
}
