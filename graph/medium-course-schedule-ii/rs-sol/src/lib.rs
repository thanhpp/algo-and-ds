use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_list = HashMap::<i32, Vec<i32>>::new();
        for pre in prerequisites {
            let course = pre[0];
            let course_pre = pre[1];
            match adj_list.get_mut(&course) {
                None => {
                    adj_list.insert(course, vec![course_pre]);
                }
                Some(p) => {
                    p.push(course_pre);
                }
            }
        }

        let mut path = Vec::<i32>::new();
        let mut visiting = HashSet::<i32>::new();
        let mut visited = HashSet::<i32>::new();

        for i in 0..num_courses {
            if !Self::dfs(i, &adj_list, &mut visiting, &mut visited, &mut path) {
                return vec![];
            }
        }

        path
    }

    pub fn dfs(
        course: i32,
        adj_list: &HashMap<i32, Vec<i32>>,
        visiting: &mut HashSet<i32>,
        visited: &mut HashSet<i32>,
        path: &mut Vec<i32>,
    ) -> bool {
        if visited.contains(&course) {
            return true;
        }

        if visiting.contains(&course) {
            return false;
        }

        visiting.insert(course);

        if let Some(pres) = adj_list.get(&course) {
            for pre in pres {
                if !Self::dfs(*pre, adj_list, visiting, visited, path) {
                    return false;
                }
            }
        }

        visiting.remove(&course);

        visited.insert(course);
        path.push(course);

        true
    }
}
