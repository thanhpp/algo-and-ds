pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut can_break_from_idx = vec![false; s.len() + 1];
        can_break_from_idx[s.len()] = true;

        for i in (0..s.len()).rev() {
            for w in word_dict.iter() {
                if (i + w.len()) > s.len() || !s[i..i + w.len()].eq(w) {
                    continue;
                }

                if can_break_from_idx[i + w.len()] {
                    can_break_from_idx[i] = true;
                    // if the code doesn't break here, it will try a new word
                    // which can be equal but can not break s using that word
                    break;
                }
            }
        }

        println!("{:#?}", can_break_from_idx);

        can_break_from_idx[0]
    }
}
