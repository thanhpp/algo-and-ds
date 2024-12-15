use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{Read, Write},
};

fn main() {
    const CAP: (i64, i64) = (101, 103);
    let robots = read("input_1.txt");
    // for r in robots.iter() {
    //     println!("r {:?}", r)
    // }
    // println!("--");

    solve1(CAP, robots.clone(), 100);

    solve2(CAP, robots.clone(), 10_000);
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pub p: (i64, i64),
    pub v: (i64, i64),
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let (p_raw, v_raw) = s.split_once(' ').unwrap();
        let (p_split, v_split) = (
            p_raw.split(',').collect::<Vec<&str>>(),
            v_raw.split(',').collect::<Vec<&str>>(),
        );

        Robot {
            p: (
                p_split[0].trim_start_matches("p=").parse().unwrap(),
                p_split[1].parse().unwrap(),
            ),
            v: (
                v_split[0].trim_start_matches("v=").parse().unwrap(),
                v_split[1].parse().unwrap(),
            ),
        }
    }

    fn move_by_secs(&mut self, cap: (i64, i64), secs: i64) {
        let mut delta = (secs * self.v.0, secs * self.v.1);
        // eliminate loop
        delta.0 = delta.0.signum() * (delta.0.abs() % cap.0);
        delta.1 = delta.1.signum() * (delta.1.abs() % cap.1);
        // println!("delta: {:?}", delta);

        (self.p.0, self.p.1) = (self.p.0 + delta.0, self.p.1 + delta.1);
        // println!("tmp Self {:?}", self);

        if self.p.0 < 0 {
            self.p.0 = cap.0 + self.p.0
        }
        if self.p.0 >= cap.0 {
            self.p.0 = self.p.0 - cap.0
        }

        if self.p.1 < 0 {
            self.p.1 = cap.1 + self.p.1
        }
        if self.p.1 >= cap.1 {
            self.p.1 = self.p.1 - cap.1
        }
    }
}

fn read(p: &str) -> Vec<Robot> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines().map(Robot::from_str).collect()
}

fn solve1(cap: (i64, i64), mut robots: Vec<Robot>, secs: i64) {
    for r in robots.iter_mut() {
        r.move_by_secs(cap, secs);
    }

    let remove = (cap.0 / 2, cap.1 / 2);
    let mut quad_count = (0, 0, 0, 0);
    for r in robots {
        // println!("{:?}", r);
        if r.p.0 == remove.0 || r.p.1 == remove.1 {
            continue;
        }
        if r.p.0 < remove.0 {
            if r.p.1 < remove.1 {
                quad_count.0 += 1
            } else {
                quad_count.3 += 1
            }
        } else if r.p.1 < remove.1 {
            quad_count.1 += 1
        } else {
            quad_count.2 += 1
        }
    }

    println!(
        "quad: {:?} | solve1 {}",
        quad_count,
        quad_count.0 * quad_count.1 * quad_count.2 * quad_count.3
    )
}

fn solve2(cap: (i64, i64), robots: Vec<Robot>, secs: i64) {
    let step = 1000;
    let mut idx = 1000;
    let mut f = fs::File::create_new(format!("{}.txt", idx)).unwrap();
    println!("new f {:?}", f.metadata().unwrap());

    for i in 1..=secs {
        if i != 0 && i % step == 0 {
            idx += step;
            f.flush().unwrap();
            f = fs::File::create_new(format!("{}.txt", idx)).unwrap();
            println!("new f {:?}", f.metadata().unwrap());
        }
        let mut content = String::new();
        content += format!("secs: {}\n", i).as_str();
        let mut robots = robots.clone();

        // r -> c
        // 1 -> 0
        let mut locations: HashMap<i64, HashSet<i64>> = HashMap::new();
        for r in robots.iter_mut() {
            r.move_by_secs(cap, i);
            match locations.get_mut(&r.p.1) {
                None => {
                    locations.insert(r.p.1, HashSet::from([r.p.0]));
                }
                Some(set) => {
                    set.insert(r.p.0);
                }
            }
        }

        for r in 0..cap.1 {
            for c in 0..cap.0 {
                if let Some(set) = locations.get(&r) {
                    if set.contains(&c) {
                        content += "X";
                        continue;
                    }
                }
                content += " ";
            }
            content += "\n";
        }

        f.write_all(content.as_bytes()).unwrap();
    }
}
