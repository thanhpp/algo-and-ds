pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut curr = 0;
        let mut last = 0;

        for c in s.chars() {
            if c == ' ' {
                if curr != 0 {
                    last = curr;
                    curr = 0;
                }
                continue;
            }

            curr += 1;
        }

        if curr != 0 {
            last = curr
        }

        last
    }
}
