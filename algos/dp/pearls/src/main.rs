fn main() {
    println!("Hello, world!");
    let quantity = [1, 1, 100];
    let price = [10, 11, 12];
    println!("{}", pearls(&quantity, &price));
}

fn pearls(quantity: &[i32], price: &[i32]) -> i32 {
    _pearls(quantity, price, 0)
}

fn _pearls(quantity: &[i32], price: &[i32], start: usize) -> i32 {
    if start == quantity.len() {
        return 0;
    }

    let mut res = i32::MAX;
    let mut sum = 0;
    // do the operations on the first idx and let recursion do the rest
    for end in start..quantity.len() {
        sum += quantity[end];
        let block = price[end] * (sum + 10);
        let total_block = block + _pearls(quantity, price, end + 1);
        res = res.min(total_block);
    }

    res
}
