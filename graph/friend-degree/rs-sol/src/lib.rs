use std::collections::{HashMap, HashSet, VecDeque};

fn degree(connections: Vec<String>, name_1: String, name_2: String) -> i32 {
    let mut adj = HashMap::<String, Vec<String>>::new(); // friend a -> b, c, d

    for c in connections.iter() {
        let names: Vec<String> = c.split(':').map(|x| x.to_string()).collect();
        if names.len() != 2 {
            panic!("invalid name length")
        };

        if let Some(v) = adj.get_mut(&names[0]) {
            v.push(names[1].clone());
        } else {
            adj.insert(names[0].clone(), vec![names[1].clone()]);
        };
    }

    let mut visited = HashSet::<String>::new();
    let mut queue = VecDeque::<&String>::new();
    let mut count = 1;
    visited.insert(name_1.clone());
    match adj.get(&name_1) {
        None => panic!("idk"),
        Some(friends) => {
            for friend in friends {
                queue.push_back(friend)
            }
        }
    }

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        if visited.contains(next) {
            continue;
        }

        count += 1;
        if next.eq(&name_2) {
            return count;
        }

        match adj.get(&name_1) {
            None => panic!("idk"),
            Some(friends) => {
                for friend in friends {
                    if visited.contains(friend) {
                        continue;
                    }
                    queue.push_back(friend)
                }
            }
        }
    }

    -1
}
