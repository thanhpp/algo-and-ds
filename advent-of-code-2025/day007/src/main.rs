fn main() {
    let input: Vec<Vec<char>> = include_str!("../input.1")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    println!("{}", solve_1(&mut (input.clone())));

    let mut m = Map {
        m: input.clone(),
        max_r: input.len(),
        max_c: input[0].len(),
    };
    println!("{}", m.solve_2())
}

pub fn solve_1(m: &mut [Vec<char>]) -> i64 {
    let mut count = 0;
    let (max_r, max_c) = (m.len(), m[0].len());

    for r in 1..max_r {
        for c in 0..max_c {
            if m[r][c] == '|' {
                continue;
            }
            if m[r - 1][c] == 'S' || m[r - 1][c] == '|' {
                if m[r][c] == '^' {
                    count += 1;
                    if c > 0 {
                        m[r][c - 1] = '|'
                    }
                    if c < max_c - 1 {
                        m[r][c + 1] = '|'
                    }
                    continue;
                }

                m[r][c] = '|'
            }
        }
    }

    count
}

pub struct Map {
    max_r: usize,
    max_c: usize,
    m: Vec<Vec<char>>,
}

impl Map {
    pub fn solve_2(&mut self) -> i64 {
        let mut count_map = Vec::<Vec<i64>>::new();
        for r in 0..self.max_r {
            count_map.push(vec![0; self.max_c]);
        }

        for c in 0..self.max_c {
            if self.m[0][c] == 'S' {
                count_map[0][c] = 1;
                break;
            }
        }
        for r in 1..self.max_r {
            for c in 0..self.max_c {
                if self.m[r][c] == '.' {
                    count_map[r][c] += count_map[r - 1][c];
                    continue;
                }
                if self.m[r][c] == '^' {
                    // count_map[r][c] += count_map[r - 1][c];
                    if c > 0 {
                        count_map[r][c - 1] += count_map[r - 1][c];
                    }
                    if c < self.max_c - 1 {
                        count_map[r][c + 1] += count_map[r - 1][c];
                    }
                }
            }
        }

        count_map.last().unwrap().iter().sum()
    }
}
