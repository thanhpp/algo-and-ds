use std::collections::HashMap;

pub struct Solution {}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Action {
    Buy,
    Sell,
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Self::dfs_with_cache(&prices, &mut cache, 0, Action::Buy)
    }

    fn dfs_with_cache(
        prices: &[i32],
        cache: &mut HashMap<(usize, Action), i32>, // (day, Action) -> profit
        day: usize,
        action: Action,
    ) -> i32 {
        if day >= prices.len() {
            return 0; // can not buy or sell
        }

        if let Some(&p) = cache.get(&(day, action)) {
            return p;
        }

        let p: i32 = match action {
            Action::Buy => {
                // buy at this day -> need to sell in the following days
                let buy_profit =
                    Self::dfs_with_cache(prices, cache, day + 1, Action::Sell) - prices[day];
                // not buy at this day, continue to buy at the following days
                let cooldown = Self::dfs_with_cache(prices, cache, day + 1, Action::Buy);
                buy_profit.max(cooldown)
            }
            Action::Sell => {
                // sell at this day -> cooldown 1 day then continue to buy
                let sell_profit =
                    Self::dfs_with_cache(prices, cache, day + 2, Action::Buy) + prices[day];
                // do nothing, can sell next day
                let cooldown = Self::dfs_with_cache(prices, cache, day + 1, Action::Sell);
                sell_profit.max(cooldown)
            }
        };

        cache.insert((day, action), p);

        p
    }
}
