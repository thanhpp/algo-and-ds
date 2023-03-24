// 115 ms, 89.5 MB

#[derive(Debug)]
struct TimeMap {
    m: std::collections::HashMap<String, Vec<(i32, String)>>,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            m: std::collections::HashMap::new(),
            min: i32::MAX,
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.m.get_mut(&key) {
            None => {
                self.m.insert(key, vec![(timestamp, value)]);
            }
            Some(v) => v.push((timestamp, value)),
        }

        if timestamp < self.min {
            self.min = timestamp
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if timestamp < self.min {
            return String::new();
        }

        let v = match self.m.get(&key) {
            None => {
                return String::new();
            }
            Some(v) => v,
        };

        let (mut l, mut r): (usize, usize) = (0, v.len() - 1);
        let mut res = String::new();
        while l <= r {
            let m = l + (r - l) / 2;
            // println!("{} {} {} {:?}", l, r, m, v[m]);
            if v[m].0 == timestamp {
                return v[m].1.clone();
            }

            if v[m].0 > timestamp {
                if m == 0 {
                    break;
                }
                r = m - 1;
            } else {
                res = v[m].1.clone();
                l = m + 1;
            }
        }

        res
    }
}

/*
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        // [[],["love","high",10],["love","low",20],["love",5],["love",10],["love",15],["love",20],["love",25]]
        // [null,null,null,"","high","high","low","low"]
        let mut tm = super::TimeMap::new();
        tm.set("love".into(), "high".into(), 10);
        tm.set("love".into(), "low".into(), 20);
        println!("{:?}", tm);
        println!("get 1");
        assert_eq!(tm.get("love".into(), 5), "".to_string());
        println!("get 2");
        assert_eq!(tm.get("love".into(), 10), "high".to_string());
        println!("get 3");
        assert_eq!(tm.get("love".into(), 15), "high".to_string());
        println!("get 4");
        assert_eq!(tm.get("love".into(), 20), "low".to_string());
        println!("get 5 ");
        assert_eq!(tm.get("love".into(), 25), "low".to_string());
    }
}
