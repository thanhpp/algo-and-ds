// 10ms - 3.1MB
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r, mut max): (usize, usize, i32) = (0, 0, 0);
    while r < prices.len() {
        let diff = prices[r] - prices[l];
        if diff > max {
            max = diff
        }
        // if we see a lower price, then we should buy the stock in that day
        if prices[r] < prices[l] {
            l = r
        }
        r += 1;
    }

    max
}
