pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        if s[0] == '0' {
            return 0;
        }

        let (mut count_0, mut count_1) = (1, 0);

        for i in (0..s.len()).rev() {
            if s[i] == '0' {
                count_1 = count_0;
                count_0 = 0; // can not decode 0
                continue;
            }

            if (i + 1 < s.len())
                && (s[i] == '1' || (s[i] == '2' && s[i + 1] >= '0' && s[i + 1] <= '6'))
            {
                let tmp = count_0 + count_1;
                count_1 = count_0;
                count_0 = tmp;
                continue;
            }

            count_1 = count_0;
        }

        count_0
    }
}
