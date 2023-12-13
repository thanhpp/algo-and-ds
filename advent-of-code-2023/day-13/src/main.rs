use std::{fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";
    let patterns = p1_parse_input(INPUT);

    println!("{:?}", patterns.last().unwrap().map.last().unwrap());

    println!("p1 {}", solve(&patterns));
}

fn solve(patterns: &Vec<Pattern>) -> usize {
    let mut sum = 0;
    for p in patterns {
        let h_old = find_horizontal_reflect_idx(p, usize::MAX, false)
            .unwrap_or((usize::MAX, false))
            .0;
        let v_old = find_vertical_reflect_idx(p, usize::MAX, false)
            .unwrap_or((usize::MAX, false))
            .0;
        let h = find_horizontal_reflect_idx(p, h_old, true);
        let v = find_vertical_reflect_idx(p, v_old, true);
        println!("v {:?} | h {:?} | v_old {} | h_old {}", v, h, v_old, h_old);
        if h.is_some() && v.is_some() {
            if let Some(v) = v {
                if v.1 {
                    sum += v.0 + 1;
                }
            };
            if let Some(h) = h {
                if h.1 {
                    sum += (h.0 + 1) * 100
                }
            }
            continue;
        }

        if let Some(v) = v {
            sum += v.0 + 1;
        };
        if let Some(h) = h {
            sum += (h.0 + 1) * 100
        }
    }

    sum
}

#[derive(Debug)]
struct Pattern {
    map: Vec<Vec<char>>,
}

fn find_horizontal_reflect_idx(
    p: &Pattern,
    old_mirror: usize,
    fix_smudge: bool,
) -> Option<(usize, bool)> {
    for r in 0..p.map.len() {
        let mut i = r;
        let mut j = r + 1;
        let mut fixed = false;
        while j < p.map.len() {
            if p.map[i].ne(&p.map[j]) {
                if !fix_smudge || fixed {
                    break;
                }
                let mut not_eq_count = 0;
                for k in 0..p.map[i].len() {
                    if p.map[i][k] == p.map[j][k] {
                        continue;
                    }
                    not_eq_count += 1;
                }
                if not_eq_count != 1 {
                    break;
                }
                fixed = true;
            }
            if i == 0 || j == p.map.len() - 1 {
                if r == old_mirror {
                    break;
                }
                return Some((r, fixed));
            }
            i -= 1;
            j += 1;
        }
    }

    None
}

fn find_vertical_reflect_idx(
    p: &Pattern,
    old_mirror: usize,
    fix_smudge: bool,
) -> Option<(usize, bool)> {
    for c in 0..p.map[0].len() {
        let mut i = c;
        let mut j = c + 1;
        let mut fixed = false;
        while j < p.map[0].len() {
            let col_1: Vec<char> = p.map.iter().map(|r| r[i]).collect();
            let col_2: Vec<char> = p.map.iter().map(|r| r[j]).collect();
            if col_1.ne(&col_2) {
                if !fix_smudge || fixed {
                    break;
                }
                let mut not_eq_count = 0;
                for k in 0..col_1.len() {
                    if col_1[k] == col_2[k] {
                        continue;
                    }
                    not_eq_count += 1;
                }
                if not_eq_count != 1 {
                    break;
                }
                fixed = true;
            }
            if i == 0 || j == p.map[0].len() - 1 {
                if c == old_mirror {
                    break;
                }
                return Some((c, fixed));
            }
            i -= 1;
            j += 1;
        }
    }

    None
}

fn p1_parse_input(file: &str) -> Vec<Pattern> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().collect();
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut data: Vec<Vec<char>> = Vec::new();

    for l in lines {
        if l.is_empty() {
            if data.is_empty() {
                continue;
            }
            patterns.push(Pattern { map: data.clone() });
            data = vec![];
            continue;
        }

        data.push(l.chars().collect());
    }
    if !data.is_empty() {
        patterns.push(Pattern { map: data })
    }

    patterns
}
