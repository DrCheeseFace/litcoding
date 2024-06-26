pub fn max_profit(prices: Vec<i32>) -> i32 {
    // let mut profits: Vec<i32> = Vec::new();
    // let mut prices = prices;
    // prices.dedup();
    //
    // for (index, i) in prices.iter().enumerate() {
    //     let mut max_diff: i32 = 0;
    //     for j in index..prices.len() {
    //         if prices[j] - i > max_diff {
    //             max_diff = prices[j] - i;
    //         }
    //     }
    //     profits.push(max_diff);
    // }
    // *profits.iter().max().unwrap()

    let mut buy = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        } else if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }
    profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_worke() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5)
    }

    #[test]
    fn no_profit() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0)
    }
}
