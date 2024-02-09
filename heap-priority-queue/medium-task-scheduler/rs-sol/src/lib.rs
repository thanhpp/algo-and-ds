pub struct Solution {}

#[derive(Debug)]
pub struct Task {
    pub name: char,
    pub count: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = std::collections::HashMap::<char, i32>::new();
        for c in tasks.iter() {
            match count.get_mut(c) {
                None => {
                    count.insert(*c, 1);
                }
                Some(v) => {
                    *v += 1;
                }
            }
        }
        println!("{:?}", count);
        let mut remain_tasks = std::collections::BinaryHeap::<Task>::new();
        for (k, v) in count {
            remain_tasks.push(Task { name: k, count: v })
        }

        // wait_tasks: (task, count, available at)
        let mut wait_tasks = std::collections::VecDeque::<(char, i32, i32)>::new();

        let mut time = 0;
        loop {
            if remain_tasks.is_empty() && wait_tasks.is_empty() {
                break;
            }
            // println!("{} | {} | {}", remain_tasks.len(), wait_tasks.len(), time);

            loop {
                // push wait task back if the time condition is passed
                if let Some(t) = wait_tasks.front() {
                    if t.2 > time {
                        break;
                    }
                    let (name, count, _) = wait_tasks.pop_front().unwrap();
                    remain_tasks.push(Task { name, count });
                    // println!("remain pushed {} {}", name, count);
                    continue;
                }
                break;
            }

            time += 1;
            if !remain_tasks.is_empty() {
                let next_task = remain_tasks.pop().unwrap();
                // println!("next {:?}", next_task);
                if next_task.count > 1 {
                    wait_tasks.push_back((next_task.name, next_task.count - 1, time + n));
                    // println!(
                    //     "wait pused {} {} {}",
                    //     next_task.name,
                    //     next_task.count - 1,
                    //     time + n
                    // )
                }
            }
        }

        time
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_solution() {
        println!(
            "{}",
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );
    }
}
