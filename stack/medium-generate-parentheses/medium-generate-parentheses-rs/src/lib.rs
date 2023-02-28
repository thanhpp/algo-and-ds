// 1 ms, 2.3 MB
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack: Vec<char> = Vec::new();
    let mut resp: Vec<String> = Vec::new();
    backtrack(n, 0, 0, &mut stack, &mut resp);

    resp
}

pub fn backtrack(n: i32, open: i32, close: i32, stack: &mut Vec<char>, resp: &mut Vec<String>) {
    if open == close && open == n {
        let ans: String = stack.iter().collect();
        resp.push(ans);
        return;
    }

    if open < n {
        stack.push('(');
        backtrack(n, open + 1, close, stack, resp);
        stack.pop();
    }

    if close < open {
        stack.push(')');
        backtrack(n, open, close + 1, stack, resp);
        stack.pop();
    }
}
