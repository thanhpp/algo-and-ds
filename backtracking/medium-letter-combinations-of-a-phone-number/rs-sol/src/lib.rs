pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut res = Vec::<String>::new();

        let mapping = std::collections::HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        fn backtrack(
            mapping: &std::collections::HashMap<char, Vec<char>>,
            next: &[char],
            comb: &mut String,
            res: &mut Vec<String>,
        ) {
            if next.is_empty() {
                res.push(comb.clone());
                return;
            }

            let chars = mapping.get(&next[0]).unwrap();
            for c in chars {
                comb.push(*c);
                backtrack(mapping, &next[1..], comb, res);
                comb.pop();
            }
        }

        let chars: Vec<char> = digits.chars().collect();
        backtrack(&mapping, &chars, &mut String::from(""), &mut res);

        res
    }
}
