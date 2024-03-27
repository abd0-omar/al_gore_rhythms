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

pub fn is_all_heated(radius: i32, heaters: &Vec<i32>, houses: &Vec<i32>) -> bool {
    let mut j = 0;
    for &heater in heaters.iter() {
        let l = heater - radius;
        let r = heater + radius;
        while j < houses.len() && l <= houses[j] && r >= houses[j] {
            j += 1;
        }
    }
    j == houses.len()
}
pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    // search for radius
    let mut l = 1;
    let mut r = i32::MAX;
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        match is_all_heated(mid, &heaters, &houses) {
            true => {
                r = mid - 1;
                ans = mid;
            }
            false => l = mid + 1,
        }
    }
    ans
}
