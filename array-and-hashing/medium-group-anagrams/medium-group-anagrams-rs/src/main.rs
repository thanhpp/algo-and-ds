fn main() {
    println!("Hello, world!");
}

// 10 ms - 5.3 MB
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hm = std::collections::HashMap::<String, Vec<String>>::new();

    for i in 0..strs.len() {
        let mut chars = strs[i].chars().collect::<Vec<char>>();
        chars.sort();
        let sorted_str = chars.iter().collect::<String>();

        match hm.get_mut(&sorted_str) {
            Some(v) => {
                v.push(strs[i].clone());
            }
            None => {
                hm.insert(sorted_str.clone(), vec![strs[i].clone()]);
            }
        };
    }

    let mut resp: Vec<Vec<String>> = Vec::new();
    for (_, v) in hm.iter() {
        resp.push(v.to_owned());
    }

    resp
}
