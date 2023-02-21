// 3 ms - 2.7MB
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut nums = Vec::<i32>::new();

    for token in tokens {
        let str_token = token.as_str();
        match str_token {
            "*" | "/" | "-" | "+" => {
                let o1 = nums.pop().unwrap();
                let o2 = nums.pop().unwrap();
                match str_token {
                    "*" => nums.push(o2 * o1),
                    "/" => nums.push(o2 / o1),
                    "-" => nums.push(o2 - o1),
                    "+" => nums.push(o2 + o1),
                    _ => {}
                }
            }
            _ => {
                let num: i32 = token.parse().unwrap();
                nums.push(num);
            }
        }
    }

    nums.pop().unwrap()
}
