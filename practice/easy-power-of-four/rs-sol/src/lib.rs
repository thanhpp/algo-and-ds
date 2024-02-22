pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let n = n as f64;

        let res = n.log(4.0);

        res.fract() == 0.0
    }
}
