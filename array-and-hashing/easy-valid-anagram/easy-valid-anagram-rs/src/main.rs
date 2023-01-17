fn main() {
    println!("Hello, world!");
}

// 4 ms - 2.2 MB
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut hm = std::collections::HashMap::<char, i32>::new();
    for c in s.chars() {
        match hm.get_mut(&c) {
            None => {
                hm.insert(c, 1);
            }
            Some(v) => *v = *v + 1,
        };
    }

    for c in t.chars() {
        match hm.get_mut(&c) {
            None => return false,
            Some(v) => {
                if *v == 0 {
                    return false;
                }
                *v = *v - 1
            }
        }
    }

    true
}

// 12 ms - 2.9 MB
pub fn is_anagram_v2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut chars_s: Vec<char> = s.chars().collect();
    let mut chars_t: Vec<char> = t.chars().collect();
    chars_s.sort();
    chars_t.sort();

    for i in 0..chars_s.len() {
        if chars_s[i] != chars_t[i] {
            return false;
        }
    }

    true
}
