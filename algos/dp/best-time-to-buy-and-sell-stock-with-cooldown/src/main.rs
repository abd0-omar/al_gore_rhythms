fn main() {
    println!("Hello, world!");
    let prices = vec![1, 2, 3, 0, 2];
    // let prices = vec![1];
    println!("prices={:?}", max_profit(prices));
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut memory = [[[None; 3]; 2]; 5000];
    _max_profit(&prices, 0, false, &LastTransaction::None, &mut memory)
}

#[derive(PartialEq, Debug)]
enum LastTransaction {
    None,
    Sell,
    Buy,
}

impl From<&LastTransaction> for usize {
    fn from(transaction: &LastTransaction) -> Self {
        match transaction {
            LastTransaction::None => 0,
            LastTransaction::Sell => 1,
            LastTransaction::Buy => 2,
        }
    }
}

fn _max_profit(
    prices: &Vec<i32>,
    idx: usize,
    has_stock: bool,
    last_transaction: &LastTransaction,
    memory: &mut [[[Option<i32>; 3]; 2]; 5000],
) -> i32 {
    if idx == prices.len() {
        return 0;
    }

    if let Some(ret) = memory[idx][has_stock as usize][usize::from(last_transaction)] {
        return ret;
    }

    // leave aka do nothing
    let choice1 = _max_profit(prices, idx + 1, has_stock, &LastTransaction::None, memory);
    let mut choice2 = 0;
    let mut choice3 = 0;

    if has_stock {
        choice2 = _max_profit(prices, idx + 1, false, &LastTransaction::Sell, memory) + prices[idx];
    } else if *last_transaction != LastTransaction::Sell {
        choice3 = _max_profit(prices, idx + 1, true, &LastTransaction::Buy, memory) - prices[idx];
    }

    memory[idx][has_stock as usize][usize::from(last_transaction)] =
        Some(choice1.max(choice2.max(choice3)));

    choice1.max(choice2.max(choice3))
}
