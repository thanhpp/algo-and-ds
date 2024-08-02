pub struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase(mut purchase_amount: i32) -> i32 {
        let init_amount = 100;

        let last_digit = purchase_amount % 10;
        if last_digit >= 5 {
            purchase_amount += 10 - last_digit
        } else {
            purchase_amount -= last_digit
        }
        println!("{}", purchase_amount);

        init_amount - purchase_amount
    }
}
