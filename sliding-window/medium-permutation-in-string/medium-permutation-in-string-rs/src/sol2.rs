// pub fn check_inclusion(s1: String, s2: String) -> bool {
//     // s1 & s2 consist of a-z

//     // sanity check
//     if s1.len() > s2.len() {
//         return false; // it's impossible for s2 to contains a s1's permutation
//     }

//     // count the character's occurence in s1
//     let mut s1_count = vec![0; 26];
//     for c in s1.chars() {
//         // ascii(a) = 97
//         s1_count[c as usize - 97] += 1;
//     }

//     // the current window contains <matches> matches with the s1_count
//     let mut matches = 26; // we want to match the count of 26 characters between s1 and s2

//     // use a fixed sdbize window, because the window length need to = s1.len() to contains a s1's permutation.
//     let mut start = 0;
//     let mut end = start + s1.len() - 1;
//     let data = s2.chars().collect::<Vec<char>>();
//     let mut window_count = vec![0; 26];

//     // O(len(s1))
//     for i in start..=end {
//         let idx = data[i] as usize - 97;
//         window_count[idx] += 1;
//     }

//     // account all mismatches
//     for i in 0..window_count.len() {
//         if window_count[i] != s1_count[i] {
//             println!("{i}, {}, {}", window_count[i], s1_count[i]);
//             matches -= 1;
//         }
//     }

//     let ASCII_LOWER: [char; 26] = [
//         'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
//         's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
//     ];

//     println!(
//         "1 - start: {start}, end: {end}, matches {matches},\n window_count: \n{:?}\n{:?}\n",
//         window_count
//             .iter()
//             .map(|x| char::from_u32(x + 48 as u32).unwrap())
//             .collect::<Vec<char>>(),
//         ASCII_LOWER
//     );

//     // O(n)
//     while end < s2.len() {
//         if matches == 26 {
//             return true;
//         }

//         // println!(
//         //     "0 - start: {start}, end: {end}, matches {matches},\n window_count: \n{:?}\n{:?}",
//         //     window_count
//         //         .iter()
//         //         .map(|x| char::from_u32(x + 48 as u32).unwrap())
//         //         .collect::<Vec<char>>(),
//         //     ASCII_LOWER
//         // );

//         start += 1;
//         end = start + s1.len() - 1;
//         if end >= s2.len() {
//             break;
//         }

//         // decrease the start char occurence
//         let idx = data[start - 1] as usize - 97;
//         window_count[idx] -= 1;
//         if window_count[idx] == s1_count[idx] {
//             matches += 1;
//         }

//         let idx = data[end] as usize - 97;
//         window_count[idx] += 1;
//         if window_count[idx] != s1_count[idx] {
//             matches -= 1;
//         } else {
//             matches += 1;
//         }

//         println!(
//             "1 - start: {start}, end: {end}, matches {matches},\n window_count: \n{:?}\n{:?}\n",
//             window_count
//                 .iter()
//                 .map(|x| char::from_u32(x + 48 as u32).unwrap())
//                 .collect::<Vec<char>>(),
//             ASCII_LOWER
//         );
//     }

//     false
// }

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    // turn s1, s2 to char slice
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();

    // use to count the occurence of 26 lowercase character
    let (mut s1_count, mut window_count): (Vec<i32>, Vec<i32>) = (vec![0; 26], vec![0; 26]);
    for i in 0..s1.len() {
        s1_count[s1[i] as usize - 'a' as usize] += 1; // count the char occurences in s1
        window_count[s2[i] as usize - 'a' as usize] += 1; //  count the char occurences in the first window in s2
    }

    let mut matches = 0; // shows the number of characters that its occurence in the window matches the occurence in s1
    for i in 0..26 {
        if s1_count[i] == window_count[i] {
            matches += 1;
        }
    }

    // use a fixed size window to iterate through s2
    let mut l = 0;
    for r in s1.len()..s2.len() {
        if matches == 26 {
            return true;
        }

        // extend the window to the right
        let idx = s2[r] as usize - 'a' as usize;
        window_count[idx] += 1;
        // 1 more character match
        if s1_count[idx] == window_count[idx] {
            matches += 1;
        } else if s1_count[idx] + 1 == window_count[idx] {
            // they are equal in the old window, but in the new window they are no longer equal
            matches -= 1;
        }

        let idx = s2[l] as usize - 'a' as usize;
        window_count[idx] -= 1; // remove the left most count of the old window
        if s1_count[idx] == window_count[idx] {
            matches += 1;
        } else if s1_count[idx] - 1 == window_count[idx] {
            matches -= 1;
        }

        l += 1;
    }

    // println!("{matches}\n{:?}\n{:?}", s1_count, window_count);

    // if only 1 window exist (s1.len() == s2.len())
    matches == 26
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

    #[test]
    fn test_4() {
        assert!(check_inclusion("abc".into(), "bbbca".into()))
    }

    #[test]
    fn test_5() {
        assert!(check_inclusion("abc".into(), "cccccbabbbaaaa".into()))
    }
}
