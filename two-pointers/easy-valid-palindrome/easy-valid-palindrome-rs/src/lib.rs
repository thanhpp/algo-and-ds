// 5 ms - 2.7 MB
pub fn is_palindrome(s: String) -> bool {
    // -----------------------------------
    // 5 ms - 2.7 MB
    // let s = s.to_lowercase(); // lowercases
    // let mut s1: Vec<char> = Vec::new();
    // for c in s.chars() {
    //     if c.is_alphanumeric() {
    //         // take only alphanumeric
    //         s1.push(c);
    //     }
    // }
    // -----------------------------------
    let s1: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if s1.len() == 0 {
        return true;
    }

    let mut i: usize = 0;
    let mut j = s1.len() - 1;

    loop {
        if i >= j {
            // if s1.len() is odd => i can not equals j
            break;
        }

        if s1[i] != s1[j] {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}
