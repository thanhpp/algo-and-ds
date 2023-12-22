use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::Read,
};

use graphviz_rust::{
    cmd::{CommandArg, Format},
    dot_generator::*,
    dot_structures::*,
    exec,
    printer::PrinterContext,
};

fn main() {
    const INPUT: &str = "input.txt";
    let cubes = p1_parse_input(INPUT);
    let nodes = p1_solve(&cubes);
    p2_solve(nodes);
}

// transform cube -> node
#[derive(Debug)]
struct P1Node {
    cube: Cube,
    parents_ids: HashSet<usize>,
    children_ids: HashSet<usize>,
}

fn p2_solve(nodes: HashMap<usize, P1Node>) {
    let mut total_sum = 0;
    for n in nodes.keys() {
        let mut queue = VecDeque::<usize>::new();
        let mut disintergrated = HashSet::<usize>::new();
        let s = nodes.get(n).unwrap();
        if s.children_ids.is_empty() {
            continue;
        }
        disintergrated.insert(s.cube.id);
        for c in s.children_ids.iter() {
            queue.push_front(*c);
        }

        while !queue.is_empty() {
            let l = queue.len();
            let before = disintergrated.len();
            for _ in 0..l {
                let node = nodes.get(&queue.pop_back().unwrap()).unwrap();
                // check if all of its parent is disintegrated
                let mut all_disintegrated = true;
                for p in node.parents_ids.iter() {
                    if !disintergrated.contains(p) {
                        all_disintegrated = false;
                        break;
                    }
                }
                if !all_disintegrated {
                    queue.push_front(node.cube.id);
                    continue;
                }
                disintergrated.insert(node.cube.id);
                for c in node.children_ids.iter() {
                    queue.push_front(*c);
                }
            }
            if disintergrated.len() == before {
                break;
            }
        }

        if disintergrated.len() == 1 {
            continue;
        }

        total_sum += disintergrated.len() - 1;
        println!("n {n}, disintergrated {}", disintergrated.len());
    }

    println!("p2 total: {}", total_sum)
}

fn p1_solve(cubes: &[Cube]) -> HashMap<usize, P1Node> {
    let mut cubes = cubes.to_vec();
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    for c in cubes.iter() {
        max_x = max_x.max(c.end_1.x).max(c.end_2.x);
        max_y = max_y.max(c.end_1.y).max(c.end_2.y);
        max_z = max_z.max(c.end_1.z).max(c.end_2.z);
    }

    // sort by z low -> high
    cubes.sort_unstable_by_key(|c| c.end_1.z.min(c.end_2.z));

    let mut positions = vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1]; // -> positions[x][y][z] = cube number
    for c in cubes.iter() {
        for p in c.points() {
            positions[p.x][p.y][p.z] = c.id;
        }
    }
    loop {
        let mut no_move_down = true;
        for c in cubes.iter_mut() {
            while c.end_1.z.min(c.end_2.z) > 1 {
                let mut can_move_down = true;
                // check
                for p in c.points() {
                    if positions[p.x][p.y][p.z - 1] != 0 && positions[p.x][p.y][p.z - 1] != c.id {
                        println!(
                            "can't move down, current {}, under cube {}, positions: {:?}",
                            c.id,
                            positions[p.x][p.y][p.z - 1],
                            c.points()
                        );
                        can_move_down = false;
                        break;
                    }
                }

                if !can_move_down {
                    break;
                }

                no_move_down = false;
                // println!("moving down cube: [{}]", c.id);
                // move down 1
                // clear current position
                for p in c.points() {
                    if positions[p.x][p.y][p.z] != c.id {
                        panic!("invalid positions, {:?}", p)
                    }
                    positions[p.x][p.y][p.z] = 0;
                }
                // update cube
                c.end_1.z -= 1;
                c.end_2.z -= 1;
                // update positions
                for p in c.points() {
                    positions[p.x][p.y][p.z] = c.id
                }
            }
        }
        if no_move_down {
            break;
        }
        cubes.sort_unstable_by_key(|c| c.end_1.z.min(c.end_2.z));
    }

    let mut nodes: HashMap<usize, P1Node> = HashMap::new();
    for c in cubes.iter() {
        nodes.insert(
            c.id,
            P1Node {
                cube: *c,
                parents_ids: HashSet::new(),
                children_ids: HashSet::new(),
            },
        );
    }

    // build parents and children
    let mut g1 = graph!(di id!("p1_graph_parent_to_child"));

    for (_, n) in nodes.iter_mut() {
        if n.cube.id == 250 {
            g1.add_stmt(Stmt::Node(node!(n.cube.id; attr!("shape", "square"))));
        } else {
            g1.add_stmt(Stmt::Node(node!(n.cube.id)));
        }

        for p in n.cube.points() {
            // find parents (support cubes)
            if p.z > 0
                && positions[p.x][p.y][p.z - 1] != 0
                && positions[p.x][p.y][p.z - 1] != n.cube.id
            {
                let parent_id = positions[p.x][p.y][p.z - 1];
                if n.parents_ids.insert(parent_id) {
                    g1.add_stmt(Stmt::Edge(
                        edge!(node_id!(parent_id) => node_id!(n.cube.id)),
                    ));
                }
            }
            // find children (supporting cubs)
            if p.z < max_z - 1
                && positions[p.x][p.y][p.z + 1] != 0
                && positions[p.x][p.y][p.z + 1] != n.cube.id
            {
                let child_id: usize = positions[p.x][p.y][p.z + 1];
                n.children_ids.insert(child_id);
            }
        }
    }

    exec(
        g1,
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("./p1_graphviz_from_parent.png".to_string()),
        ],
    )
    .unwrap();

    for (_, n) in nodes.iter() {
        println!("{:?}", n);
    }

    let mut count_disintegrated = 0;
    for id in nodes.keys() {
        let node = nodes.get(id).unwrap();
        let mut can_disintegrated = true;

        for child_id in node.children_ids.iter() {
            let child = nodes.get(child_id).unwrap();
            if child.parents_ids.len() == 1 {
                can_disintegrated = false;
                println!("can not disintegrated {:?}, because of {:?}", node, child);
                break;
            }
        }

        if can_disintegrated {
            println!("can_disintegrated {:?}", node);
            count_disintegrated += 1;
            continue;
        }
    }

    println!("p1 {}", count_disintegrated);

    nodes
}
fn p1_parse_input(file: &str) -> Vec<Cube> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().filter(|l| !l.is_empty()).collect();
    let mut cubes: Vec<Cube> = Vec::with_capacity(lines.len());
    for c in cubes.iter() {
        println!("{:?}", c);
    }

    for (id, l) in lines.iter().enumerate() {
        let (end_1, end_2) = l.split_once('~').unwrap();
        let (end_1, end_2) = (Point::from_str(end_1), Point::from_str(end_2));
        cubes.push(Cube {
            id: id + 1,
            end_1,
            end_2,
        })
    }

    cubes
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    id: usize,
    end_1: Point,
    end_2: Point,
}

impl Cube {
    fn points(&self) -> Vec<Point> {
        // THIS TOOK ME SEVERAL HOURS TO ADD THIS CONDITION
        // ALWAYS, ALWAYS ADD VALIDATIONS
        if (self.end_1.x == self.end_2.x)
            && (self.end_1.y == self.end_2.y)
            && (self.end_1.z == self.end_2.z)
        {
            return vec![self.end_1];
        }

        let mut points: Vec<Point> = Vec::new();
        if self.end_1.x != self.end_2.x {
            for x in self.end_1.x.min(self.end_2.x)..=self.end_1.x.max(self.end_2.x) {
                points.push(Point {
                    x,
                    y: self.end_1.y,
                    z: self.end_2.z,
                })
            }
        }
        if self.end_1.y != self.end_2.y {
            for y in self.end_1.y.min(self.end_2.y)..=self.end_1.y.max(self.end_2.y) {
                points.push(Point {
                    x: self.end_1.x,
                    y,
                    z: self.end_2.z,
                })
            }
        }
        if self.end_1.z != self.end_2.z {
            for z in self.end_1.z.min(self.end_2.z)..=self.end_1.z.max(self.end_2.z) {
                points.push(Point {
                    x: self.end_1.x,
                    y: self.end_2.y,
                    z,
                })
            }
        }

        // validate
        if points.len()
            != isize::abs(
                self.end_1.x as isize - self.end_2.x as isize + self.end_1.y as isize
                    - self.end_2.y as isize
                    + self.end_1.z as isize
                    - self.end_2.z as isize,
            ) as usize
                + 1
        {
            panic!("invalid points {:?} {:?}", self, points)
        }

        points
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let cord: Vec<&str> = s.split(',').collect();
        if cord.len() != 3 {
            panic!("invalid coordinates {}", s)
        }
        Point {
            x: cord[0].parse().unwrap(),
            y: cord[1].parse().unwrap(),
            z: cord[2].parse().unwrap(),
        }
    }
}
