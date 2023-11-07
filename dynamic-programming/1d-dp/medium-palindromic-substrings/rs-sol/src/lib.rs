pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut count = 0;
        let (mut l, mut r): (usize, usize);

        for i in 0..s.len() {
            l = i;
            r = i;
            while r < s.len() && s[l] == s[r] {
                count += 1;
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }

            l = i;
            r = i + 1;
            while r < s.len() && s[l] == s[r] {
                count += 1;
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }

        count
    }
}
