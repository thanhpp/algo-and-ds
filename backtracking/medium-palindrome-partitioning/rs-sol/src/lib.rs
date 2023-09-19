pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::<Vec<String>>::new();
        let mut path = Vec::<String>::new();

        Self::backtrack(&s, &mut path, &mut res);

        res
    }

    pub fn backtrack(s: &str, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if s.is_empty() {
            res.push(path.clone());
            return;
        }

        for i in 1..=s.len() {
            let (curr, remain) = s.split_at(i);
            if !Self::is_palidrome(curr) {
                continue;
            }
            path.push(curr.to_string());
            Self::backtrack(remain, path, res);
            path.pop();
        }
    }

    pub fn is_palidrome(s: &str) -> bool {
        // this is much faster
        s.chars().eq(s.chars().rev())

        // let b = s.as_bytes();
        // let mut i = 0_usize;
        // let mut j = b.len() - 1;

        // while i < j {
        //     if b[i].ne(&b[j]) {
        //         return false;
        //     }
        //     i += 1;
        //     j -= 1;
        // }

        // true
    }
}

mod test {
    use crate::*;

    #[test]
    fn test_1() {
        let res = Solution::partition("aab".to_string());
        assert_eq!(res, vec![vec!["a", "a", "b"], vec!["aa", "b"]])
    }

    #[test]
    fn test_2() {
        let res = Solution::partition("bb".to_string());
        assert_eq!(res, vec![vec!["b", "b"], vec!["bb"]])
    }

    #[test]
    fn test_split() {
        println!("{:#?}", "".split_at(0))
    }
}
