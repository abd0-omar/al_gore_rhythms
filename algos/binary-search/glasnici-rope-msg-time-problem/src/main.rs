fn possible(positions: &[f64], msg_dist: f64, can_move_dist: f64) -> bool {
    let mut last_position = positions[0] + can_move_dist;
    for &pos in &positions[1..] {
        let last_possible_msg_position = last_position + msg_dist;
        if pos - can_move_dist > last_possible_msg_position {
            return false;
        }
        last_position = last_possible_msg_position.min(pos + can_move_dist);
    }
    true
}

fn min_time(positions: &Vec<f64>, msg_dist: f64) -> f64 {
    let mut start = 0.0;
    let mut end = 1e9;

    while end - start > 1e-9 {
        let mid = start + (end - start) / 2.0;
        if possible(positions, msg_dist, mid) {
            end = mid;
        } else {
            start = mid;
        }
    }
    start
}

fn main() {
    let positions = vec![0.0, 4.0, 4.0, 8.0];
    let msg_dist = 2.0;

    let time = min_time(&positions, msg_dist);
    println!("{:.2}", time);
}
