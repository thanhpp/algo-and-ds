use std::{collections::HashMap, fs, io::Read};

fn main() {
    let data = read("input_1.txt");
    solve2(&data, 75);
    solve1(&data, 25);
}

fn read(p: &str) -> Vec<i64> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

fn blink_solve1(v: i64) -> Vec<i64> {
    if v == 0 {
        return vec![1];
    }
    let s = v.to_string();
    let size = s.len();
    if size % 2 == 0 {
        let split = s.split_at(size / 2);
        return vec![
            split.0.parse::<i64>().unwrap(),
            split.1.parse::<i64>().unwrap(),
        ];
    }

    vec![v * 2024]
}

#[derive(Clone, Debug)]
pub struct StartStone {
    pub value: i64,
    pub next: Option<Box<StartStone>>,
    pub blink_count: i64,
}

impl StartStone {
    fn split(&mut self, blink_count: i64) {
        let mut current = Some(self);
        while let Some(c) = current {
            if c.blink_count >= blink_count {
                current = c.next.as_deref_mut();
                continue;
            }

            // println!("current: {:?}", c);
            c.blink_count = blink_count;

            let splited = blink_solve1(c.value);
            // println!("split {} -> {:?}", c.value, splited);
            if splited.len() == 1 {
                c.value = splited[0];
                // println!("after: {:?}", c);
                current = c.next.as_deref_mut();
                continue;
            }

            c.value = splited[0];
            let new_stone = Some(Box::new(StartStone {
                value: splited[1],
                next: c.next.take(),
                blink_count,
            }));

            c.next = new_stone;
            // println!("after: {:?}", c);

            current = c.next.as_deref_mut()
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current = Some(self);

        while let Some(node) = current {
            // print!("{} | ", node.value);
            length += 1;
            current = node.next.as_deref();
        }
        // println!();

        length
    }
}

fn solve1(data: &[i64], blink: i64) {
    let mut stones = Vec::<StartStone>::new();
    for v in data {
        let s = StartStone {
            blink_count: 0,
            next: None,
            value: *v,
        };

        stones.push(s);
    }
    // println!("stones {:?}", stones);

    for i in 1..=blink {
        // println!("blink: {}", i);
        for s in stones.iter_mut() {
            s.split(i);
        }
    }

    let mut res: usize = 0;
    for s in stones {
        res += s.len();
    }

    println!("solve1 {}", res)
}

fn solve2(data: &[i64], blink: i64) {
    let mut stones = HashMap::<i64, i64>::new();
    for v in data.iter() {
        stones.insert(*v, 1);
    }

    for i in 1..=blink {
        // println!("blink {}", i);
        let keys = stones.keys().cloned().collect::<Vec<i64>>();
        let mut actions = vec![];
        for k in keys {
            let count = match stones.get_mut(&k) {
                None => {
                    continue;
                }
                Some(v) => {
                    if *v == 0 {
                        continue;
                    }
                    let tmp = *v;
                    *v = 0;
                    tmp
                }
            };

            let v = blink_solve1(k);
            // println!("k {} | count {} => {:?}", k, count, v);
            for tmp in v {
                actions.push((tmp, count));
            }
        }

        for (v, count) in actions {
            match stones.get_mut(&v) {
                None => {
                    stones.insert(v, count);
                }
                Some(c) => {
                    *c += count;
                }
            };
        }
    }

    // println!("---");
    // for (k, v) in stones.iter().filter(|(_, &v)| v != 0) {
    //     println!("{}: {}", k, v);
    // }

    println!("solve2 {}", stones.values().sum::<i64>())
}
