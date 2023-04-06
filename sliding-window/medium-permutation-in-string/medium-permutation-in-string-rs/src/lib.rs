mod sol2;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    // build char map - O(len(s1))
    let mut char_map = std::collections::HashMap::<char, i32>::new();
    for c in s1.chars() {
        match char_map.get_mut(&c) {
            None => {
                char_map.insert(c, 1);
            }
            Some(count) => *count += 1,
        }
    }

    // check
    let s1_len = s1.len() as i32;
    let data = s2.chars().collect::<Vec<char>>();
    let (mut r, mut l): (usize, usize) = (0, 0);
    let mut matches: i32 = 0;
    let mut check_map = char_map.clone();

    // O(len(s2) * len(s1))
    while r < s2.len() {
        let char_counter = match check_map.get_mut(&data[r]) {
            None => {
                l += 1;
                r = l;
                matches = 0;
                check_map = char_map.clone();
                continue;
            }
            Some(c) => c,
        };

        if *char_counter == 0 {
            l += 1;
            r = l;
            matches = 0;
            check_map = char_map.clone();
            continue;
        }

        matches += 1;
        *char_counter -= 1;
        // instead of checking the whole char map; O(len(s1) * len(s2)) --> O(len(s2))
        if matches == s1_len {
            return true;
        }

        r += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(check_inclusion("ab".into(), "eidbaooo".into()))
    }

    #[test]
    fn test_2() {
        assert!(!check_inclusion("ab".into(), "eidboaoo".into()))
    }

    #[test]
    fn test_3() {
        assert!(check_inclusion("adc".into(), "dcda".into()))
    }
}
