fn main() {
    println!("Hello, world!");

    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 1;

    println!("{}", min_days(bloom_day, m, k));

    // Output: 3
    // Explanation: Let us see what happened in the first three days. x means flower bloomed and _ means flower did not bloom in the garden.
    // We need 3 bouquets each should contain 1 flower.
    // After day 1: [x, _, _, _, _]   // we can only make one bouquet.
    // After day 2: [x, _, _, _, x]   // we can only make two bouquets.
    // After day 3: [x, _, x, _, x]   // we can make 3 bouquets. The answer is 3.
    // Example 2:
    //
    // Input: bloomDay = [1,10,3,10,2], m = 3, k = 2
    // Output: -1
    // Explanation: We need 3 bouquets each has 2 flowers, that means we need 6 flowers. We only
    // have 5 flowers so it is impossible to get the needed bouquets and we return -1.
    // Example 3:
    //
    // Input: bloomDay = [7,7,7,7,12,7,7], m = 2, k = 3
    // Output: 12
    // Explanation: We need 2 bouquets each should have 3 flowers.
    // Here is the garden after the 7 and 12 days:
    // After day 7: [x, x, x, x, _, x, x]
    // We can make one bouquet of the first three flowers that bloomed. We cannot make another
    // bouquet from the last three flowers that bloomed because they are not adjacent.
    // After day 12: [x, x, x, x, x, x, x]
    // It is obvious that we can make two bouquets in different ways.
}

//1st time
pub fn possible(mid: i32, m: i32, k: i32, bloom_day: &Vec<i32>) -> bool {
    let mut counter = 0;
    let mut big_counter = 0;
    for &day in bloom_day {
        if day <= mid {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == k {
            counter = 0;
            big_counter += 1;
        }
    }
    big_counter >= m
}

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let mut l = 0;
    let max = *bloom_day.iter().max().unwrap();
    let mut r = max;
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        match possible(mid, m, k, &bloom_day) {
            true => {
                r = mid - 1;
                ans = mid;
            }
            false => l = mid + 1,
        }
    }

    ans
}
