// 1 ms - 2.4 MB

use std::collections::VecDeque;

pub fn is_valid(s: String) -> bool {
    let mut stack: VecDeque<char> = VecDeque::new();

    for c in s.chars() {
        match c {
            '{' | '(' | '[' => stack.push_back(c),
            '}' | ')' | ']' => {
                let open = stack.pop_back();
                match open {
                    None => {
                        return false;
                    }
                    Some(o) => match c {
                        '}' => {
                            if !o.eq(&'{') {
                                return false;
                            }
                        }
                        ')' => {
                            if !o.eq(&'(') {
                                return false;
                            }
                        }
                        ']' => {
                            if !o.eq(&'[') {
                                return false;
                            }
                        }
                        _ => {}
                    },
                }
            }
            _ => {}
        }
    }

    stack.len() == 0
}
