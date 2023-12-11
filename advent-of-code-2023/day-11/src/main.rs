use std::{fs, io::Read};

mod a_star;

fn main() {
    const INPUT: &str = "input.txt";

    println!(
        "factor 1: {}",
        sum_manhattan_distance(&parse_input(INPUT, 2))
    );

    println!(
        "factor 1,000,000: {}",
        sum_manhattan_distance(&parse_input(INPUT, 1_000_000))
    );
}

/*
Consider a map and a path
- src, dst: (1, 2), (6, 8)
- shortest path: P = 11
    - (8 - 2) + (6 - 1) = 11 (manhattan distance)

0 1 2 3 4 5 6 7 8 9 10
0 . . . . . . . . . .
1 . # . . . . . . . .
2 . P P . . . . . . .
3 . . P P . . . . . .
4 . . . P P . . . . .
5 . . . . P P . . . .
6 . . . . . P P # . .
7 . . . . . . . . . .
*/

fn sum_manhattan_distance(galaxies: &Vec<Galaxy>) -> usize {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += manhattan_distance(&galaxies[i], &galaxies[j])
        }
    }

    sum
}

fn manhattan_distance(start: &Galaxy, end: &Galaxy) -> usize {
    (isize::abs(end.row as isize - start.row as isize)
        + isize::abs(end.col as isize - start.col as isize)) as usize
}

struct Galaxy {
    row: usize,
    col: usize,
}

fn parse_input(file: &str, factor: usize) -> Vec<Galaxy> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let char_map: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();

    let mut empty_rows = Vec::<usize>::new();
    for (r, row) in char_map.iter().enumerate() {
        let mut is_empty = true;
        for c in row {
            if *c == '.' {
                continue;
            }
            is_empty = false;
            break;
        }
        if is_empty {
            empty_rows.push(r);
        }
    }

    let mut empty_cols = Vec::<usize>::new();
    for c in 0..char_map[0].len() {
        let mut is_empty = true;
        for r in char_map.iter() {
            if r[c] == '.' {
                continue;
            }
            is_empty = false;
            break;
        }
        if is_empty {
            empty_cols.push(c);
        }
    }

    let mut galaxies = Vec::new();
    for (row, r) in char_map.iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            let mut empty_row_count = 0;
            for er in empty_rows.iter() {
                if *er < row {
                    empty_row_count += 1;
                    continue;
                }
                break;
            }
            let mut empty_col_count = 0;
            for ec in empty_cols.iter() {
                if *ec < col {
                    empty_col_count += 1;
                    continue;
                }
                break;
            }

            // factor - 1: the current row & count already have the empty row & empty count of the original map
            galaxies.push(Galaxy {
                row: row + empty_row_count * (factor - 1),
                col: col + empty_col_count * (factor - 1),
            })
        }
    }

    galaxies
}
