use std::{fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";
    let mut input = Input::from_file(INPUT);
    input.print_info();

    let (path, val) = part_1(&input);
    println!("part 1 = {}", (val + 1) / 2);

    println!("part 2 = {}", part_2(&mut input, &path));
}

fn part_2(input: &mut Input, path: &[(usize, usize)]) -> usize {
    // update the start title (because the start is not a pipe)
    let start_title = path[1];
    let last_title = path.last().unwrap();
    if start_title.0 == last_title.0 {
        input.grid[input.start.0][input.start.1] = Tile::Vertical;
    } else if start_title.1 == last_title.1 {
        input.grid[input.start.0][input.start.1] = Tile::Horizontal;
    } else if start_title.1 > last_title.1 {
        if start_title.0 > last_title.0 {
            // from north
            input.grid[input.start.0][input.start.1] = Tile::NorthEast;
        } else {
            input.grid[input.start.0][input.start.1] = Tile::SouthEast;
        }
    } else if start_title.0 > last_title.0 {
        // from north
        input.grid[input.start.0][input.start.1] = Tile::NorthWest;
    } else {
        input.grid[input.start.0][input.start.1] = Tile::SouthWest;
    }
    println!(
        "start: {:?} | end: {:?} | start {:?}",
        start_title, last_title, input.grid[input.start.0][input.start.1]
    );

    let mut mark_map = vec![vec![0; input.grid[0].len()]; input.grid.len()];
    for (id, pos) in path.iter().enumerate() {
        mark_map[pos.0][pos.1] = id as isize + 1;
    }

    // for r in mark_map.iter() {
    //     println!("{:?}", r);
    // }

    /*
    This rule determines the "insideness" of a point on the canvas by drawing a ray
    from that point to infinity in any direction and counting the number of path segments from the given shape that the ray crosses.
    If this number is odd, the point is inside; if even, the point is outside.

    https://www.reddit.com/r/adventofcode/comments/18evyu9/comment/kcqtow6
    interact badly if your ray and one of the edges of the shape is collinear, so you have to be clever about it for this problem.
    ? consider a square, going diagonal (x + 1, y + 1) hits one of these corners [top right & bottom left] -> odd intersect -> but still outside
    ** Going diagonal: easily hit vertical lines & horizontal lines once
    */

    let mut inside_point = 0;

    for (r, row) in mark_map.iter().enumerate() {
        for (col, char) in row.iter().enumerate() {
            if char.ne(&0) {
                continue;
            }
            // start drawing diagonal ray
            let mut intersect_count = 0;
            let (mut ray_row, mut ray_col) = (r, col);
            while ray_row < mark_map.len() && ray_col < row.len() {
                let title = &input.grid[ray_row][ray_col];
                if mark_map[ray_row][ray_col] != 0
                    && title.ne(&Tile::SouthWest)
                    // && title.ne(&Tile::SouthEast)
                    // && title.ne(&Tile::NorthWest)
                    && title.ne(&Tile::NorthEast)
                {
                    intersect_count += 1;
                }
                ray_row += 1;
                ray_col += 1;
            }
            if intersect_count % 2 != 0 {
                inside_point += 1
            }
        }
    }

    // println!("\n");
    // for r in mark_map.iter() {
    //     println!("{:?}", r);
    // }

    inside_point
}

fn part_1(input: &Input) -> (Vec<(usize, usize)>, isize) {
    let mut res = 0;
    let mut max_path: Vec<(usize, usize)> = Vec::new();

    let mut start_pos = Vec::new();
    if input.start.0 > 0 {
        start_pos.push((input.start.0 - 1, input.start.1))
    }
    if input.start.1 > 0 {
        start_pos.push((input.start.0, input.start.1 - 1))
    }
    start_pos.push((input.start.0 + 1, input.start.1));
    start_pos.push((input.start.0, input.start.1 + 1));

    for pos in start_pos {
        let mut distance = vec![vec![-1; input.grid[0].len()]; input.grid.len()];

        let mut prev_pos = input.start;
        let mut cur_pos = pos;
        let mut length = 0;
        let mut path: Vec<(usize, usize)> = Vec::new();
        path.push(prev_pos);

        loop {
            let next_pos = match part_1_next_pos(&input.grid, prev_pos, cur_pos) {
                None => {
                    break;
                }
                Some(p) => p,
            };
            if next_pos.0 >= input.grid.len() {
                println!("out of bound 0");
                break;
            }
            if next_pos.1 >= input.grid[0].len() {
                println!("out of bound 1");
                break;
            }
            // a loop from start
            if input.grid[next_pos.0][next_pos.1] == Tile::Start {
                length += 1;
                path.push(cur_pos);
                println!("found loop");
                break;
            }
            if distance[next_pos.0][next_pos.1] != -1 {
                println!("loop - exit: {:?}", next_pos);
                break;
            }
            distance[next_pos.0][next_pos.1] = length;

            path.push(cur_pos);
            prev_pos = cur_pos;
            cur_pos = next_pos;
            length += 1;
        }

        if length > res {
            res = length;
            max_path = path;
        }
    }

    (max_path, res)
}

fn part_1_next_pos(
    grid: &[Vec<Tile>],
    prev_position: (usize, usize),
    position: (usize, usize),
) -> Option<(usize, usize)> {
    // // out of bound
    // if position.0 >= grid.len() {
    //     println!("out of bound 0");
    //     return None;
    // }
    // if position.1 >= grid[0].len() {
    //     println!("out of bound 1");
    //     return None;
    // }
    // // a loop from start
    // if grid[position.0][position.1] == Tile::Start {
    //     println!("found loop");
    //     return length;
    // }
    // a loop
    // if distance[position.0][position.1] != -1 {
    //     println!("loop - exit: {:?}", position);
    //     return 0;
    // }

    // distance[position.0][position.1] = length as isize;

    // println!(
    //     "visiting {} {} {:?}",
    //     position.0, position.1, grid[position.0][position.1]
    // );

    match grid[position.0][position.1] {
        Tile::Ground => {
            println!("ground - exit, {:?}", position);
            None
        }
        Tile::Start => {
            println!("start - exit");
            None
        }
        Tile::Vertical => {
            if prev_position.0 == position.0 - 1 {
                // going down
                return Some((position.0 + 1, position.1));
            }
            // going up
            Some((position.0 - 1, position.1))
        }
        Tile::Horizontal => {
            // going right
            if prev_position.1 == position.1 - 1 {
                return Some((position.0, position.1 + 1));
            }

            // going left
            Some((position.0, position.1 - 1))
        }
        Tile::NorthEast => {
            // north to east
            if prev_position.0 == position.0 - 1 {
                return Some((position.0, position.1 + 1));
            }
            // east to north

            Some((position.0 - 1, position.1))
        }
        Tile::NorthWest => {
            // north to west
            if prev_position.0 == position.0 - 1 {
                return Some((position.0, position.1 - 1));
            }
            // west to north
            Some((position.0 - 1, position.1))
        }
        Tile::SouthEast => {
            // south to east
            if prev_position.0 == position.0 + 1 {
                return Some((position.0, position.1 + 1));
            }
            // east to south

            Some((position.0 + 1, position.1))
        }
        Tile::SouthWest => {
            // south to west
            if prev_position.0 == position.0 + 1 {
                return Some((position.0, position.1 - 1));
            }
            // west to south
            Some((position.0 + 1, position.1))
        }
    }
}

struct Input {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Input {
    fn from_file(file: &str) -> Input {
        let mut buffer = String::new();
        fs::File::open(file)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        let lines: Vec<&str> = buffer.lines().filter(|l| !l.is_empty()).collect();

        let mut grid = vec![vec![Tile::Ground; lines[0].len()]; lines.len()];
        let mut start = (0, 0);

        for (row, l) in lines.iter().enumerate() {
            for (col, c) in l.chars().enumerate() {
                let t = Tile::from_char(&c).unwrap();
                if let Tile::Start = t {
                    start = (row, col)
                }
                grid[row][col] = t;
            }
        }

        Input { grid, start }
    }

    fn print_info(&self) {
        for l in self.grid.iter() {
            println!("{:?}", l)
        }
        println!("start: {:?}", self.start)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '|' => Some(Tile::Vertical),
            '-' => Some(Tile::Horizontal),
            'L' => Some(Tile::NorthEast),
            'J' => Some(Tile::NorthWest),
            '7' => Some(Tile::SouthWest),
            'F' => Some(Tile::SouthEast),
            '.' => Some(Tile::Ground),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
}
