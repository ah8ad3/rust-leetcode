
fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 5, 4, 3]), 0);
    assert_eq!(max_profit(vec![1]), 0);
    assert_eq!(max_profit(vec![]), 0);
    assert_eq!(max_profit(vec![1, 2]), 1);
}

fn max_profit(mut prices: Vec<i32>) -> i32 {
    if prices.len() <= 1{
        return 0;
    }

    // some trick to pass leetcode
    prices.dedup();
    let max_number = prices.iter().max().unwrap_or(&0);
    let min_number = prices.iter().min().unwrap_or(&0);
    let best_amount = max_number - min_number;

    let mut max_profit = 0;
    for (index, price) in prices.iter().enumerate() {
        let max_vec_price = prices[index+1..].iter().max().unwrap_or(&0);
        if max_vec_price - price > max_profit {
            max_profit = max_vec_price - price;
        }
        if max_profit == best_amount {
            return max_profit;
        }
    }
    max_profit
}