pub struct Solution {}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() as i32 + 1;
        let mut parents: Vec<i32> = (0..n).collect();
        let mut rank = vec![1; n as usize];

        println!("{:#?} | {:#?}", parents, rank);

        for e in edges {
            if !Self::union(&mut parents, &mut rank, e[0], e[1]) {
                return e;
            }
        }

        vec![]
    }

    pub fn find_parent(parents: &mut [i32], node: i32) -> i32 {
        let mut parent = parents[node as usize];

        while parent != parents[parent as usize] {
            parents[parent as usize] = parents[parents[parent as usize] as usize];
            parent = parents[parent as usize];
        }

        parent
    }

    pub fn union(parents: &mut [i32], rank: &mut [i32], a: i32, b: i32) -> bool {
        let (par_a, par_b) = (Self::find_parent(parents, a), Self::find_parent(parents, b));

        if par_a == par_b {
            return false;
        }

        if rank[par_a as usize] > rank[par_b as usize] {
            parents[par_b as usize] = par_a;
            rank[par_a as usize] += rank[par_b as usize];
            return true;
        }

        parents[par_a as usize] = par_b;
        rank[par_b as usize] += rank[par_a as usize];

        true
    }
}
