use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

fn main() {
    let points = read(include_str!("../input.1"));
    println!("{}", solve_1(&points, 1000));
    println!("{}", solve_2(&points));
}

fn solve_1(points: &[Point], joints: i64) -> i64 {
    let mut distance: BinaryHeap<Reverse<Distance>> = BinaryHeap::new(); // min heap
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            distance.push(Reverse(Distance::new(&points[i], &points[j])));
        }
    }

    let mut edges = HashMap::<Point, Vec<Point>>::new();
    for _ in 0..joints {
        let dis = distance.pop().unwrap().0;
        match edges.get_mut(&dis.a) {
            None => {
                edges.insert(dis.a, vec![dis.b]);
            }
            Some(v) => v.push(dis.b),
        }

        match edges.get_mut(&dis.b) {
            None => {
                edges.insert(dis.b, vec![dis.a]);
            }
            Some(v) => v.push(dis.a),
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let points: Vec<Point> = edges.keys().cloned().collect();
    let mut sizes = vec![];
    for p in points.iter() {
        if visited.contains(p) {
            continue;
        }

        let mut size = 0;
        let mut bfs: VecDeque<Point> = VecDeque::new();
        bfs.push_back(*p);
        while let Some(pop) = bfs.pop_back() {
            if visited.contains(&pop) {
                continue;
            }
            visited.insert(pop);
            size += 1;
            if let Some(to) = edges.get(&pop) {
                for t in to.iter() {
                    if visited.contains(t) {
                        continue;
                    }

                    bfs.push_back(*t);
                }
            }
        }
        // println!("size {}", size);
        sizes.push(size);
    }

    sizes.sort_by(|a, b| a.cmp(b).reverse());
    sizes.iter().take(3).product()
}

fn solve_2(points: &[Point]) -> i64 {
    let mut distance: BinaryHeap<Reverse<Distance>> = BinaryHeap::new(); // min heap
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            distance.push(Reverse(Distance::new(&points[i], &points[j])));
        }
    }

    let mut parents: HashMap<Point, Point> = HashMap::new();
    for p in points.iter() {
        parents.insert(*p, *p);
    }

    let mut last_dis = Distance::default();
    while let Some(Reverse(dis)) = distance.pop() {
        // check all parents
        let keys: Vec<Point> = parents.keys().cloned().collect();
        let mut set = HashSet::<Point>::new();
        for k in keys {
            set.insert(find_parents(&k, &mut parents));
        }

        // println!("dis {:?} -> {}", dis, set.len());
        if set.len() == 1 {
            break;
        }
        last_dis = dis;
        union(&dis.a, &dis.b, &mut parents);
    }
    println!("{:?}", last_dis);
    last_dis.a.x * last_dis.b.x
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Default)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Eq for Point {}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Distance {
    pub a: Point,
    pub b: Point,
    pub dis: f64,
}

impl Distance {
    pub fn new(a: &Point, b: &Point) -> Distance {
        Distance {
            a: *a,
            b: *b,
            dis: distance(a, b),
        }
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dis.total_cmp(&other.dis)
    }
}

fn read(s: &str) -> Vec<Point> {
    s.lines()
        .map(|l| {
            let split: Vec<&str> = l.split(',').collect();
            Point::new(
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
            )
        })
        .collect()
}

fn distance(a: &Point, b: &Point) -> f64 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
}

fn find_parents(p: &Point, parents: &mut HashMap<Point, Point>) -> Point {
    let par = *parents.get(p).unwrap();
    if p.eq(&par) {
        return par;
    }

    let par1 = find_parents(&par, parents);
    parents.insert(*p, par1);
    // println!("set new parents {:?} {:?}", p, par);

    par1
}

fn union(a: &Point, b: &Point, parents: &mut HashMap<Point, Point>) {
    // println!("a {:?} b {:?}", a, b);
    let par_a = find_parents(a, parents);
    let par_b = find_parents(b, parents);
    if par_a.ne(&par_b) {
        parents.insert(par_a, par_b);

        // for (k, v) in parents.iter() {
        //     println!("{:?}: {:?}", k, v)
        // }
        // println!("----");

        // println!("set new parents {:?} {:?} - 2", par_a, par_b);
    }

    // for (k, v) in parents.iter() {
    //     println!("{:?}: {:?}", k, v)
    // }
    // println!("----");
}
