pub struct Solution {}

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj_list = HashMap::<i32, Vec<i32>>::new();
        for p in prerequisites {
            match adj_list.get_mut(&p[0]) {
                None => {
                    adj_list.insert(p[0], vec![p[1]]);
                }
                Some(v) => {
                    v.push(p[1]);
                }
            }
        }

        let mut visited = HashSet::<i32>::new();
        for i in 0..num_courses {
            if !Self::dfs(i, &mut visited, &mut adj_list) {
                return false;
            }
        }

        true
    }

    fn dfs(course: i32, visited: &mut HashSet<i32>, adj_list: &mut HashMap<i32, Vec<i32>>) -> bool {
        if visited.contains(&course) {
            return false;
        }

        if !adj_list.contains_key(&course) {
            return true;
        }

        visited.insert(course);

        match adj_list.clone().get_mut(&course) {
            None => return true,
            Some(pres) => {
                for p in pres {
                    if !Self::dfs(*p, visited, adj_list) {
                        return false;
                    }
                    adj_list.remove(&course);
                }
            }
        }

        visited.remove(&course);

        true
    }
}
