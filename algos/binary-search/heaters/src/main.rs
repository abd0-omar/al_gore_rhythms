use std::collections::HashMap;

fn main() {
    // let houses = vec![1, 2, 3, 4];
    // let heaters = vec![1, 4];
    //
    // println!("{}", find_radius(houses, heaters));
    //
    // let houses = vec![1, 2, 3];
    // let heaters = vec![2];
    //
    // println!("{}", find_radius(houses, heaters));
    //
    // let houses = vec![1, 5];
    // let heaters = vec![2];
    //
    // println!("{}", find_radius(houses, heaters));

    let houses = vec![1, 5];
    let heaters = vec![2];

    println!("{}", find_radius(houses, heaters));
}

pub fn is_all_heated(
    mid: i32,
    heaters: &Vec<i32>,
    max: i32,
    hs: &HashMap<&i32, usize>,
    min: i32,
) -> bool {
    let mut boolean_vec: Vec<bool> = vec![false; hs.len()];
    for &heater in heaters.iter() {
        let mut l = heater - mid;
        if l < min {
            l = min;
        }
        let mut r = heater + mid;
        if r > max {
            r = max;
        }
        for j in l..=r {
            if hs.contains_key(&j) {
                let house_index = *hs.get(&j).unwrap();
                // Skip checking if the house is already heated by this heater.
                if heaters.contains(&j) {
                    continue;
                }
                boolean_vec[house_index] = true;
            }
        }
    }
    for boolean in boolean_vec {
        if !boolean {
            return false;
        }
    }
    true
}
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    if houses.len() == 1 {
        // If there's only one house, the radius is the minimum distance to the nearest heater.
        let mut min_distance = i32::MAX;
        for &heater in heaters.iter() {
            let distance = (houses[0] - heater).abs();
            if distance < min_distance {
                min_distance = distance;
            }
        }
        return min_distance;
    }
    let mut hs = HashMap::new();
    for (i, h) in houses.iter().enumerate() {
        hs.insert(h, i);
    }
    println!("DEBUGPRINT[1]: main.rs:30: hs={:#?}", hs);
    let max = *houses.iter().max().unwrap();
    let min = *houses.iter().min().unwrap();
    let mut l = 1;
    let mut r = heaters[heaters.len() - 1] + 1;
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        match is_all_heated(mid, &heaters, max, &hs, min) {
            true => {
                r = mid - 1;
                ans = mid;
                println!("DEBUGPRINT[5]: main.rs:62: ans={:#?}", ans);
            }
            false => l = mid + 1,
        }
    }
    ans
}
