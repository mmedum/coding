pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min = i32::MAX;

    for price in prices {
        if price < min {
            min = price;
        }

        let profit = price - min;

        if profit > max_profit {
            max_profit = profit
        }
    }

    return max_profit;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(input), 5);

        input = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(input), 0);
    }
}
