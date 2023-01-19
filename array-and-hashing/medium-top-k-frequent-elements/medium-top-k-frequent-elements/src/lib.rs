// 4 ms - 2.6 MB
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::<i32, i32>::new();
    // count elements O(n)
    for v in nums {
        match hm.get_mut(&v) {
            None => {
                hm.insert(v, 1);
            }
            Some(c) => {
                *c += 1;
            }
        }
    }

    // collect k & v to vec O(n)
    let mut sort_kv: Vec<(&i32, &i32)> = hm.iter().collect();

    // sort by value O(nlogn)
    sort_kv.sort_by(|a, b| b.1.cmp(a.1));

    // create answer
    let mut resp = Vec::<i32>::new();
    for i in 0..(k as usize) {
        match sort_kv.get(i) {
            None => {
                continue;
            }
            Some(x) => {
                resp.push(x.0.clone());
            }
        }
    }

    resp
}
