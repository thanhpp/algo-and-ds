// 6ms, 2.2 MB
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_idx = std::collections::HashMap::<char, usize>::new();
    let (mut l, mut r): (usize, usize) = (0, 0);
    let mut idx: usize = 0;
    let mut max_length: usize = 0;

    for c in s.chars().into_iter() {
        match char_idx.get_mut(&c) {
            None => {
                char_idx.insert(c, idx);
            }

            Some(c_idx) => {
                if *c_idx >= l {
                    l = *c_idx + 1;
                }
                *c_idx = idx;
            }
        }

        r += 1;
        idx += 1;

        let length = r - l;
        // println!("l {}, r {}, length {length},char_idx {:?}", l, r, char_idx);
        if length > max_length {
            max_length = length;
        }
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3)
    }
    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3)
    }
}
