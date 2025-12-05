fn main() {
    let input = include_str!("../input.1");

    let mut m = Map::new(input);
    println!("sol_1: {}", m.solve_1());

    println!("sol_2: {}", m.solve_2());
}

#[derive(Debug)]
pub struct Map {
    max_r: i64,
    max_c: i64,
    m: Vec<Vec<char>>,
}

impl Map {
    pub fn new(p: &str) -> Map {
        let mut m = vec![];

        for (r, l) in p.lines().filter(|l| !l.is_empty()).enumerate() {
            m.push(vec!['.'; l.len()]);
            for (c, cr) in l.chars().enumerate() {
                m[r][c] = cr
            }
        }

        Map {
            max_r: m.len() as i64,
            max_c: m[0].len() as i64,
            m,
        }
    }

    pub fn count_rolls(&self, (r, c): (i64, i64)) -> i64 {
        let mut count = 0;
        for delta_r in -1..=1 {
            for delta_c in -1..=1 {
                let (new_r, new_c) = (r + delta_r, c + delta_c);
                if new_r < 0 || new_c < 0 || new_r >= self.max_r || new_c >= self.max_c {
                    continue;
                }

                if self.m[new_r as usize][new_c as usize] == '@' {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn solve_1(&self) -> i64 {
        // The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions
        let mut sol_1 = 0;
        for r in 0..self.max_r {
            for c in 0..self.max_c {
                if self.m[r as usize][c as usize] != '@' {
                    continue;
                }

                let count = self.count_rolls((r, c));
                // println!("{}, {} = {}", r, c, count);
                if count > 4 {
                    continue;
                }

                sol_1 += 1;
            }
        }

        sol_1
    }

    pub fn solve_2(&mut self) -> i64 {
        // The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions
        let mut sol_2 = 0;

        loop {
            let mut to_remove = Vec::<(usize, usize)>::new();
            for r in 0..self.max_r {
                for c in 0..self.max_c {
                    if self.m[r as usize][c as usize] != '@' {
                        continue;
                    }

                    let count = self.count_rolls((r, c));
                    // println!("{}, {} = {}", r, c, count);
                    if count > 4 {
                        continue;
                    }
                    to_remove.push((r as usize, c as usize));
                    sol_2 += 1;
                }
            }

            if to_remove.is_empty() {
                break;
            }

            for (r, c) in to_remove.iter().cloned() {
                self.m[r][c] = '.'
            }
        }

        sol_2
    }
}
