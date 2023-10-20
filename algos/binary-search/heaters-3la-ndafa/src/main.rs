fn main() {
    println!("Hello, world!");
}

fn possible(mid: i32, houses: &Vec<i32>, heaters: &Vec<i32>) -> bool {
    let mut j = 0;

    for heater in heaters {
        let l = heater - mid;
        let r = heater + mid;
        while j < houses.len() && houses[j] >= l && houses[j] <= r {
            j += 1;
        }
    }

    j == houses.len()
}
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let mut houses = houses;
    let mut heaters = heaters;

    houses.sort(); // Sort the houses and heaters
    heaters.sort();

    let mut l = 0;
    let mut r: i32 = std::i32::MAX; // Set r to the maximum possible value
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        // Check if the current mid value is valid
        if possible(mid, &houses, &heaters) {
            ans = mid;
            r = mid - 1; // Update the upper boundary
        } else {
            l = mid + 1; // Update the lower boundary
        }
    }

    ans
}
