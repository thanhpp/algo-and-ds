use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

fn main() {
    const INPUT: &str = "input.txt";
    let map = p1_parse_input(INPUT);
    for r in map.tiles.iter() {
        println!("{:?}", r);
    }
    println!("{:?} | {:?}", map.start, map.end);

    p1_solve(&map);

    p2_solve(&map);
}

// idea: https://www.reddit.com/r/adventofcode/comments/18oy4pc/comment/kekk8l
// short: get all intersection, find the distance from one to its adjacent intersections -> DFS
// This works because we don't step on a tile twice -> never cross an intersection twice
fn p2_solve(map: &Map) {
    // find intersections (>= 3 neighbors)
    let mut intersections: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..map.tiles.len() {
        for c in 0..map.tiles[0].len() {
            if map.neighbors_p2(r, c).len() < 3 {
                continue;
            }
            intersections.insert((r, c));
        }
    }

    // build a weighted map between intersections. (itersection -> adj intersections + distance)
    type AdjWeightedMap = HashMap<(usize, usize), HashMap<(usize, usize), usize>>;
    fn dfs_intersection_distance(
        map: &Map,
        intersections: &HashSet<(usize, usize)>,
        w_map: &mut AdjWeightedMap,
        visited: &mut HashSet<(usize, usize)>,
        start_intersection: (usize, usize),
        r: usize,
        c: usize,
        distance: usize,
    ) {
        println!("dfs_intersection_distance {} {}", r, c);
        if (r, c) != start_intersection && intersections.contains(&(r, c)) {
            match w_map.get_mut(&start_intersection) {
                None => {
                    let mut adj = HashMap::<(usize, usize), usize>::new();
                    adj.insert((r, c), distance);
                    w_map.insert(start_intersection, adj);
                    return;
                }
                Some(adj) => {
                    adj.insert((r, c), distance);
                    return;
                }
            }
        }
        visited.insert((r, c));

        let neighbors = map.neighbors_p2(r, c);
        for (nr, nc) in neighbors {
            if visited.contains(&(nr, nc)) {
                continue;
            }

            visited.insert((nr, nc));
            dfs_intersection_distance(
                map,
                intersections,
                w_map,
                visited,
                start_intersection,
                nr,
                nc,
                distance + 1,
            );
            visited.remove(&(nr, nc));
        }
        visited.remove(&(r, c));
    }

    // let's build
    // start & end are not intersections but we do need to find the distance from them to their adj intersection
    intersections.insert(map.start);
    intersections.insert(map.end);
    println!("intersections {:?}", intersections);

    let mut weighted_map: AdjWeightedMap = HashMap::new();
    for it_sec in intersections.iter() {
        dfs_intersection_distance(
            map,
            &intersections,
            &mut weighted_map,
            &mut HashSet::new(),
            *it_sec,
            it_sec.0,
            it_sec.1,
            0,
        );
    }

    println!("weighted map {:?}", weighted_map);

    // kinda same with p1
    fn dfs_weighted_map(
        map: &Map,
        weighted_map: &AdjWeightedMap,
        visited: &mut HashSet<(usize, usize)>,
        (r, c): (usize, usize),
        distance: usize,
    ) -> usize {
        if (r, c) == map.end {
            println!("dfs_weighted_map {r} {c} {}", distance);
            return distance;
        }
        visited.insert((r, c));
        let adjs = weighted_map.get(&(r, c)).unwrap();
        let mut max_dist: usize = 0;
        for (k, v) in adjs.iter() {
            if visited.contains(k) {
                continue;
            }
            visited.insert(*k);
            max_dist = max_dist.max(dfs_weighted_map(
                map,
                weighted_map,
                visited,
                *k,
                distance + v,
            ));

            visited.remove(k);
        }

        visited.remove(&(r, c));
        max_dist
    }

    println!(
        "p2_solve {}",
        dfs_weighted_map(map, &weighted_map, &mut HashSet::new(), map.start, 0)
    )
}

fn p1_solve(map: &Map) {
    println!(
        "p1_solve: {}",
        p1_dfs(map, &mut HashSet::new(), map.start.0, map.start.1, 0)
    );
}

fn p1_dfs(
    map: &Map,
    visited: &mut HashSet<(usize, usize)>,
    r: usize,
    c: usize,
    distance: usize,
) -> usize {
    // println!("dfs {} {}", r, c);
    if (r, c) == map.end {
        return distance;
    }
    visited.insert((r, c));

    let mut max_length = 0;
    for n in map.neighbors_p1(r, c) {
        if visited.contains(&n) {
            continue;
        }
        visited.insert(n);

        max_length = max_length.max(p1_dfs(map, visited, n.0, n.1, distance + 1));

        visited.remove(&n);
    }

    visited.remove(&(r, c));

    max_length
}

fn p1_parse_input(file: &str) -> Map {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let tiles = buffer
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '>' => Tile::SlopeRight,
                    '<' => Tile::SlopeLeft,
                    '^' => Tile::SlopeUp,
                    'v' => Tile::SlopeDown,
                    _ => panic!("invalid char {}", c),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let (start_r, mut start_c, end_r, mut end_c) = (0, 0, tiles.len() - 1, 0);
    for c in 0..tiles[0].len() {
        if tiles[start_r][c] == Tile::Path {
            start_c = c;
        }
        if tiles[end_r][c] == Tile::Path {
            end_c = c;
        }
    }

    Map {
        tiles,
        start: (start_r, start_c),
        end: (end_r, end_c),
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn neighbors_p1(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let (up, down, left, right) = (
            r > 0 && self.tiles[r - 1][c] != Tile::Forest,
            r < self.tiles.len() - 1 && self.tiles[r + 1][c] != Tile::Forest,
            c > 0 && self.tiles[r][c - 1] != Tile::Forest,
            c < self.tiles[0].len() - 1 && self.tiles[r][c + 1] != Tile::Forest,
        );

        match self.tiles[r][c] {
            Tile::Forest => {
                vec![]
            }
            Tile::SlopeUp => {
                if !up {
                    return vec![];
                }
                vec![(r - 1, c)]
            }
            Tile::SlopeDown => {
                if !down {
                    return vec![];
                }
                vec![(r + 1, c)]
            }
            Tile::SlopeLeft => {
                if !left {
                    return vec![];
                }
                vec![(r, c - 1)]
            }
            Tile::SlopeRight => {
                if !right {
                    return vec![];
                }
                vec![(r, c + 1)]
            }
            Tile::Path => {
                let mut v = Vec::new();
                if up {
                    v.push((r - 1, c))
                }
                if down {
                    v.push((r + 1, c))
                }
                if left {
                    v.push((r, c - 1))
                }
                if right {
                    v.push((r, c + 1))
                }
                v
            }
        }
    }

    fn neighbors_p2(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let (up, down, left, right) = (
            r > 0 && self.tiles[r - 1][c] != Tile::Forest,
            r < self.tiles.len() - 1 && self.tiles[r + 1][c] != Tile::Forest,
            c > 0 && self.tiles[r][c - 1] != Tile::Forest,
            c < self.tiles[0].len() - 1 && self.tiles[r][c + 1] != Tile::Forest,
        );

        match self.tiles[r][c] {
            Tile::Forest => {
                vec![]
            }
            _ => {
                let mut v = Vec::new();
                if up {
                    v.push((r - 1, c))
                }
                if down {
                    v.push((r + 1, c))
                }
                if left {
                    v.push((r, c - 1))
                }
                if right {
                    v.push((r, c + 1))
                }
                v
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}
