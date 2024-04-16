pub struct Solution {}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut adj_map =
            std::collections::HashMap::<i32, std::collections::HashMap<i32, i32>>::new(); // from -> to: weight

        for dataset in times {
            let (from, to, weight) = (dataset[0], dataset[1], dataset[2]);
            match adj_map.get_mut(&from) {
                Some(m) => {
                    m.insert(to, weight);
                }
                None => {
                    adj_map.insert(from, std::collections::HashMap::from([(to, weight)]));
                }
            }
        }

        println!("adj_list: {:#?}", adj_map);

        let mut min_heap = std::collections::BinaryHeap::<std::cmp::Reverse<Node>>::new();
        let mut dest = std::collections::HashMap::<i32, i32>::new();
        let mut is_optimized = std::collections::HashMap::<i32, bool>::new();
        min_heap.push(std::cmp::Reverse(Node { v: k, min_dest: 0 }));
        dest.insert(k, 0);

        while !min_heap.is_empty() {
            let node = match min_heap.pop() {
                None => {
                    panic!("node not found")
                }
                Some(n) => n.0,
            };

            match is_optimized.get_mut(&node.v) {
                Some(b) => {
                    if *b {
                        continue;
                    }
                }
                None => {
                    is_optimized.insert(node.v, true);
                }
            }

            let edges = match adj_map.get(&node.v) {
                None => {
                    continue;
                }
                Some(v) => v,
            };

            for (&v, &w) in edges.iter() {
                let dist_v = *dest.get(&v).unwrap_or(&i32::MAX);
                let dist_u = *dest.get(&node.v).unwrap_or(&i32::MAX);
                if dist_u + w < dist_v {
                    dest.insert(v, dist_u + w);
                    min_heap.push(std::cmp::Reverse(Node {
                        v,
                        min_dest: dist_u + w,
                    }))
                } else {
                    // println!("{dist_u} | {dist_v} | {w}")
                }
            }
        }

        if dest.len() != n as usize {
            println!("{:?} | n: {}", dest, n);
            return -1;
        }

        dest.iter().fold(i32::MIN, |acc, (_, &v)| acc.max(v))
    }
}

pub struct Node {
    pub v: i32,
    pub min_dest: i32,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.min_dest == other.min_dest
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_dest.cmp(&other.min_dest)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.min_dest.cmp(&other.min_dest))
    }
}

mod test {
    #[test]
    fn test_1() {
        println!(
            "{}",
            crate::Solution::network_delay_time(
                vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]],
                4,
                2
            ),
        )
    }
}
