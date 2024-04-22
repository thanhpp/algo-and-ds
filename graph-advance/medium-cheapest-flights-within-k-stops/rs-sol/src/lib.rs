pub struct Solution {}

impl Solution {
    // TIME: must tries all edges -> O(E)
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj_list = vec![Vec::<Vec<i32>>::new(); (n + 1) as usize];
        for f in flights {
            let (from, to, price) = (f[0], f[1], f[2]);
            adj_list[from as usize].push(vec![to, price]);
        }

        let mut dest = vec![vec![i32::MAX; (k + 1) as usize]; (n + 1) as usize]; // node - hop - cheapest
        let mut min_heap = std::collections::BinaryHeap::<std::cmp::Reverse<Node>>::new();
        dest[src as usize][0] = 0;

        min_heap.push(std::cmp::Reverse(Node {
            to: src,
            price: 0,
            stops: 0,
        }));

        while let Some(n) = min_heap.pop() {
            let node = n.0;
            let neighbors = &adj_list[node.to as usize];
            for nb in neighbors {
                if node.stops <= k && node.price + nb[1] < dest[nb[0] as usize][node.stops as usize]
                {
                    dest[nb[0] as usize][node.stops as usize] = node.price + nb[1];
                    min_heap.push(std::cmp::Reverse(Node {
                        to: nb[0],
                        price: nb[1] + node.price,
                        stops: node.stops + 1,
                    }))
                }
                // println!("node: {:?}; neighbor: {:?}", node, nb);
            }
        }

        let mut min_price = i32::MAX;
        for v in dest[dst as usize].iter() {
            min_price = min_price.min(*v);
        }

        println!("{:?}", dest);

        if min_price == i32::MAX {
            return -1;
        }

        min_price
    }
}

#[derive(Debug)]
pub struct Node {
    pub to: i32,
    pub price: i32,
    pub stops: i32,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.price.cmp(&other.price))
    }
}

mod test {
    #[test]
    fn test_1() {
        println!(
            "{}",
            crate::Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            )
        )
    }

    #[test]
    fn test_2() {
        println!(
            "{}",
            crate::Solution::find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        )
    }
}
