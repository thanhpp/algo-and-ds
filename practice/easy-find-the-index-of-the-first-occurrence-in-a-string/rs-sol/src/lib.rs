pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() == needle.len() && haystack.eq(&needle) {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }

        let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());

        // println!("{}", haystack.len() - needle.len() + 1);

        for i in 0..(haystack.len() - needle.len() + 1) {
            // println!("i: {}", i);

            if haystack[i] != needle[0] {
                continue;
            }

            let mut is_found = true;
            for j in 0..needle.len() {
                // println!("checking {} {}", i + j, j);
                if haystack[i + j] == needle[j] {
                    continue;
                }
                // println!("not found {i} {j}");
                is_found = false;
                break;
            }

            if is_found {
                return i as i32;
            }
        }

        -1
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_str_str() {
        println!(
            "{}",
            Solution::str_str("hello".to_string(), "ll".to_string())
        )
    }
}
