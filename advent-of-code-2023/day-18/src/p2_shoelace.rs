/*
https://artofproblemsolving.com/wiki/index.php/Shoelace_Theorem
https://www.theoremoftheday.org/GeometryAndTrigonometry/Shoelace/TotDShoelace.pdf

NOTE: The order only need to be listed in clockwise or anti-clockwise order
*/

use std::{fs, io::Read, usize};

pub fn solve(file: &str) {
    let plans = p1_parse_input(file);
    let (vertices, total_dist) = find_all_vertices(&plans);
    println!("area {}", area_by_shoelace(&vertices, total_dist));
}

// https://gist.github.com/icub3d/2c8775d11a29d6a481c75c0adf9fcfbe
fn area_by_shoelace(vertices: &[(usize, usize)], total_dist: usize) -> usize {
    // sum(x[0]*y[1] + ... + x[n-1] * y[0])
    let mut sum_1 = 0;
    for i in 0..(vertices.len() - 1) {
        sum_1 += vertices[i].1 * vertices[i + 1].0 // sum(x[0]*y[1] + ... + x[n-2]*y[n-1])
    }
    sum_1 += vertices[vertices.len() - 1].1 * vertices[0].0; // x[n-1] * y[0]

    // sum(x[1]*y[0] + ... + x[0] * y[n-1])
    let mut sum_2 = 0;
    for i in 0..(vertices.len() - 1) {
        sum_2 += vertices[i + 1].1 * vertices[i].0 // sum(x[1]*y[0] + ... + x[n-1]*y[n-2])
    }
    sum_2 += vertices[0].1 * vertices[vertices.len() - 1].0; // x[0] * y[n-1]

    println!("{sum_1} {sum_2}");

    /* https://youtu.be/dqwQyrjaHuA?si=UfbAeS8SW9TjMFBH&t=2100
    Consider a vertical is the center of each tile (position)
    -> the area calculated by the shoelace formula only account the area inside (i)
    But this problem needs the outside area (o) of the title to be calculated (the trench edge)
    o o o o o
    o o o o o
    o o ┌ - -
    o o | i i
    o o | i i
    */

    if sum_1 > sum_2 {
        println!("shoelace {}", (sum_1 - sum_2) / 2);
        return (sum_1 - sum_2) / 2 + total_dist / 2 + 1;
    }
    println!("shoelace {}", (sum_2 - sum_1) / 2);
    (sum_2 - sum_1) / 2 + total_dist / 2 + 1
}

fn find_all_vertices(plans: &[Plan]) -> (Vec<(usize, usize)>, usize) {
    // calculate size
    let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
    let mut total_dist = 0;
    for p in plans {
        let (dir, length) = p.parse_color();
        match dir {
            Direction::Up => up += length,
            Direction::Down => down += length,
            Direction::Left => left += length,
            Direction::Right => right += length,
        }
        total_dist += length;
    }
    println!(
        "up: [{up}], down: [{down}], left: [{left}], right: [{right}] total_dist: {}",
        total_dist
    );

    let (start_r, start_c) = ((up + down) / 2, (left + right) / 2);
    let mut map = Vec::<(usize, usize)>::new(); // row_number -> digged collumns

    // digging
    let (mut pos_r, mut pos_c) = (start_r, start_c);
    let (mut min_r, mut max_r, mut min_c, mut max_c) = (usize::MAX, 0, usize::MAX, 0);
    let mut area: isize = 0;
    for p in plans {
        let (dir, length) = p.parse_color();
        let (mut next_r, mut next_c) = (pos_r, pos_c);
        match dir {
            Direction::Up => next_r -= length,
            Direction::Down => next_r += length,
            Direction::Left => next_c -= length,
            Direction::Right => next_c += length,
        }
        println!("{}", (pos_c as isize + next_c as isize));
        println!("{}", (pos_r as isize - next_r as isize));
        println!("{length}");
        area += ((pos_c as isize + next_c as isize) * (pos_r as isize - next_r as isize))
            + length as isize;

        pos_r = next_r;
        pos_c = next_c;
        map.push((pos_r, pos_c));
        // println!("added {} {}", pos_r, pos_c);
        min_r = min_r.min(pos_r);
        max_r = max_r.max(pos_r);
        min_c = min_c.min(pos_c);
        max_c = max_c.max(pos_c);
    }
    println!("digged {min_r} {max_r} {min_c} {max_c} | {}", area / 2 + 1);

    for (r, c) in map.iter_mut() {
        *r -= min_r;
        *c -= min_c;
    }

    println!("{:?}", map);

    // // let mut start_from_left_most = map[left_most_pos..].to_vec();
    // // let mut end_before_left_most = map[..left_most_pos].to_vec();

    // // start_from_left_most.append(&mut end_before_left_most);

    // println!("{:?}", start_from_left_most);

    (map, total_dist)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Plan {
    direction: Direction,
    length: usize,
    color: String,
}

impl Plan {
    fn parse_color(&self) -> (Direction, usize) {
        let dir = match self.color.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("invalid direction char"),
        };

        let length = usize::from_str_radix(
            &self
                .color
                .chars()
                .take(self.color.len() - 1)
                .skip(1)
                .collect::<String>(),
            16,
        )
        .unwrap();

        (self.direction, self.length)
        // (dir, length)
    }
}

fn p1_parse_input(file: &str) -> Vec<Plan> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut plans = Vec::new();

    for l in buffer.lines().filter(|l| !l.is_empty()) {
        let data: Vec<&str> = l.split(' ').collect();
        if data.len() != 3 {
            panic!("invalid data: {}", l);
        }

        let dir = match data[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("invalid dir {}", data[0])
            }
        };

        plans.push(Plan {
            direction: dir,
            length: data[1].parse().unwrap(),
            color: data[2]
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string(),
        })
    }

    plans
}
