use std::{fs, io::Read};

mod p2_solve;

fn main() {
    const INPUT: &str = "test_1.txt";
    const MIN_BOUND: f64 = 200000000000000.0;
    const MAX_BOUND: f64 = 400000000000000.0;

    let stones = p1_parse_input(INPUT);
    for s in stones.iter() {
        println!("{:?}", s);
    }

    // p1_solve(&stones, MIN_BOUND, MAX_BOUND);

    // p2_solve(&stones)
    p2_solve::p2_solve(&p2_solve::p2_parse_input(INPUT))
}

fn get_time(stone: &HailStone, pos_x: f64, pos_y: f64) -> Option<f64> {
    if pos_x == 0.0 && pos_y == 0.0 {
        return None;
    }
    if pos_x != 0.0 {
        return Some((pos_x - stone.px) / stone.vx);
    }
    Some((pos_y - stone.py) / stone.vy)
}

fn p1_solve(stones: &[HailStone], min_bound: f64, max_bound: f64) {
    let mut collide_count = 0;
    for i in 0..stones.len() {
        for j in (i + 1)..stones.len() {
            if let Some((t1, t2, _)) = p1_collide(&stones[i], &stones[j], min_bound, max_bound) {
                println!("i: {} | j: {} | t1: {} | t2: {}", i, j, t1, t2);

                collide_count += 1;
            }
        }
    }

    println!("p1_solve: {}", collide_count);
}

fn p1_collide(
    h1: &HailStone,
    h2: &HailStone,
    min_bound: f64,
    max_bound: f64,
) -> Option<(f64, f64, (f64, f64))> {
    if h1.px == h2.px && h1.py == h2.py && h1.pz == h2.pz {
        return Some((0.0, 0.0, (0.0, 0.0)));
    }

    /*
        {
            x1 = px1 + vx1 * t1; y1 = py1 + vy1 * t1
            x2 = px2 + vx2 * t2; y2 = py2 + vy2 * t2
        }
        => t2 = ((px2 - px1)*vy1 - (py2 - py1)*vx1)/(-vx2*vy1 + vy2*vx1)
    */
    let div_2 = -h2.vx * h1.vy + h2.vy * h1.vx;
    if div_2 == 0.0 {
        return None;
    }
    let t2 = ((h2.px - h1.px) * h1.vy - (h2.py - h1.py) * h1.vx) / div_2;

    /*
       with t2 & x1 = x2
       => px1 + vx1 * t1 = px2 + vx2 * t2
       <=> vx1 * t1 = px2 - px1 + vx2 * t2
       <=> t1 = (px2 - px1 + vx2 * t2)/vx1
    */
    let t1 = (h2.px - h1.px + h2.vx * t2) / h1.vx;

    if t1 < 0.0 || t2 < 0.0 {
        return None;
    }

    let (collide_x, collide_y) = (h1.px + h1.vx * t1, h1.py + h1.vy * t1);
    if collide_x < min_bound
        || collide_x > max_bound
        || collide_y < min_bound
        || collide_y > max_bound
    {
        return None;
    }

    Some((t1, t2, (collide_x, collide_y)))
}

fn p1_parse_input(file: &str) -> Vec<HailStone> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let lines: Vec<&str> = buffer.lines().filter(|l| !l.is_empty()).collect();

    let mut hail_stones = Vec::with_capacity(lines.len());
    for l in lines {
        let (positions, velocities) = l.split_once(" @ ").unwrap();
        let pos: Vec<f64> = positions
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect();
        let vel: Vec<f64> = velocities
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect();
        if pos.len() != 3 || vel.len() != 3 {
            panic!("invalid parse {:?} {:?}", pos, vel)
        }
        hail_stones.push(HailStone {
            px: pos[0],
            py: pos[1],
            pz: pos[2],
            vx: vel[0],
            vy: vel[1],
            vz: vel[2],
        })
    }

    hail_stones
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct HailStone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

mod test {
    #[test]
    fn test_p1_collide() {
        let h1 = crate::HailStone {
            px: 19.0,
            py: 13.0,
            pz: 30.0,
            vx: -2.0,
            vy: 1.0,
            vz: -2.0,
        };

        let h2 = crate::HailStone {
            px: 18.0,
            py: 19.0,
            pz: 22.0,
            vx: -1.0,
            vy: -1.0,
            vz: -2.0,
        };

        println!("{:?}", crate::p1_collide(&h1, &h2, 7.0, 27.0));
    }
}
