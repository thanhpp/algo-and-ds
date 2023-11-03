pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let (mut start, mut end): (usize, usize) = (0, 0);

        for i in 0..s.len() {
            let (mut l, mut r) = (i, i);
            while (r < s.len()) && s[l] == s[r] {
                if (end - start) < (r - l + 1) {
                    start = l;
                    end = r + 1
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }

            let (mut l, mut r) = (i, i + 1);
            while (r < s.len()) && s[l] == s[r] {
                if (end - start) < (r - l + 1) {
                    start = l;
                    end = r + 1
                }

                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }

        s[start..end].iter().collect()
    }
}
