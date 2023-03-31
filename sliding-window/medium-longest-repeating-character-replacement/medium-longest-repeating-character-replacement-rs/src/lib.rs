// 3ms - 2.1MB
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut freqs = std::collections::HashMap::<char, i32>::new();
    let (mut l, mut r): (usize, usize) = (0, 0);
    let mut max_length = 0;
    let chars = s.chars().collect::<Vec<char>>();

    while r < chars.len() {
        // increse the frequency count
        match freqs.get_mut(&chars[r]) {
            Some(f) => *f += 1,
            None => {
                freqs.insert(chars[r], 1);
            }
        };

        // get max freq
        let mut max_freq = 0;
        freqs.iter().for_each(|f| {
            if *f.1 > max_freq {
                max_freq = *f.1;
            }
        });

        // if a length is valid <= the possible replacement <= k
        let length = ((r - l) + 1) as i32;
        let possible_replace = length - max_freq;
        if possible_replace <= k {
            if length > max_length {
                max_length = length;
            }
        } else {
            // shift the left pointer and decrease the frequency
            match freqs.get_mut(&chars[l]) {
                Some(f) => *f -= 1,
                None => {}
            };
            l += 1;
        }

        r += 1;
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(character_replacement("ABAB".into(), 2), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(character_replacement("AABABBA".into(), 1), 4)
    }

    #[test]
    fn test_3() {
        assert_eq!(character_replacement("ABAA".into(), 0), 2)
    }
}
