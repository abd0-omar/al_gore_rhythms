use std::f64::consts::PI;

fn circle_area(r: f64) -> f64 {
    PI * r * r
}

fn total_can_eat(area: &[f64], x: f64) -> usize {
    area.iter().map(|&a| (a / x) as usize).sum()
}

fn largest_area(pie_radius: &[f64], people: usize) -> f64 {
    let mut start = 0.0;
    let mut end: f64 = 0.0;
    let mut area = vec![0.0; pie_radius.len()];

    for (i, &radius) in pie_radius.iter().enumerate() {
        area[i] = circle_area(radius);
        end = end.max(area[i]);
    }

    for _ in 0..100 {
        let mid = (start + end) / 2.0;
        if total_can_eat(&area, mid) >= people {
            start = mid;
        } else {
            end = mid;
        }
    }
    start
}

fn main() {
    let pie_radius = vec![4.0, 3.0, 3.0];
    let f = 3;
    let area = largest_area(&pie_radius, f + 1); // +1 for the host person

    println!("{:.4}", area);
}
