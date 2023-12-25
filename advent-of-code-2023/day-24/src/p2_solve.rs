use std::{collections::HashSet, fs, io::Read};

use num_bigfloat::{BigFloat, ZERO};
use z3::ast::Ast;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HailStone {
    px: BigFloat,
    py: BigFloat,
    pz: BigFloat,
    vx: BigFloat,
    vy: BigFloat,
    vz: BigFloat,
}

// z3
pub fn p2_solve_z3(stones: &[HailStone]) {
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in stones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.px.to_i64().unwrap());
        let va = z3::ast::Int::from_i64(&context, hs.vx.to_i64().unwrap());
        let b = z3::ast::Int::from_i64(&context, hs.py.to_i64().unwrap());
        let vb = z3::ast::Int::from_i64(&context, hs.vy.to_i64().unwrap());
        let c = z3::ast::Int::from_i64(&context, hs.pz.to_i64().unwrap());
        let vc = z3::ast::Int::from_i64(&context, hs.vz.to_i64().unwrap());

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return;
        };
        println!("{}", m.eval(&(x + y + z), true).unwrap());
    }
}

// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keq7g67
pub fn p2_solve(stones: &[HailStone]) {
    // consider the rock standing still
    // -> each stone velocity will be adjusted to v = v - v_rock (if 2 cars in opposite directions v1/v2 = -1. If v1 = 0 => v2 += |v1|)
    let sign: Vec<isize> = vec![-1, 1];

    // asumption: x, y, z are integers
    let mut n = 0;
    let mut checked = HashSet::<(usize, usize, isize, isize)>::new();

    for x in 0..=1000 {
        // let y = n - x; // this is briliant (instead of for y in 0..max ~ very aggressive, try y in a given range)
        for y in 0..=1000 {
            for sig_x in sign.iter() {
                for sig_y in sign.iter() {
                    if x % 100 == 0 && y % 100 == 0 {
                        println!("{} {} {} {}", x, y, *sig_x, *sig_y)
                    }

                    if checked.contains(&(x, y, *sig_x, *sig_y)) {
                        continue;
                    }
                    checked.insert((x, y, *sig_x, *sig_y));

                    let mut stones = stones.to_vec();
                    let length = stones.len();
                    let vr_x = BigFloat::from_f64(*sig_x as f64 * x as f64);
                    let vr_y = BigFloat::from_f64(*sig_y as f64 * y as f64);

                    // adjust hailstone
                    stones[0].vx -= vr_x;
                    stones[0].vy -= vr_y;

                    let mut intersect: Option<(BigFloat, BigFloat)> = None; // all adjusted must collide in 1 point (the rock point). (first, consider only x & y)
                    let mut found_common = true;
                    for i in 1..length {
                        stones[i].vx -= vr_x;
                        stones[i].vy -= vr_y;
                        let (cx, cy) = match collide(&stones[0], &stones[i]) {
                            None => break,
                            Some((cx, cy)) => {
                                if cx.is_inf() || cx.is_nan() || cy.is_inf() || cy.is_nan() {
                                    found_common = false;
                                    break;
                                }
                                // if cx.frac() != ZERO || cy.frac() != ZERO {
                                //     found_common = false;
                                //     break;
                                // }

                                // println!(
                                //     "h0: {:?} | hn: {:?} | cx: {cx}, cy: {cy}",
                                //     stones[0], stones[i]
                                // );
                                (cx, cy)
                            }
                        };
                        match intersect {
                            None => {
                                intersect = Some((cx, cy));
                                continue;
                            }
                            Some((i_cx, i_cy)) => {
                                if i_cx != cx || i_cy != cy {
                                    found_common = false;
                                    break;
                                }
                            }
                        }
                    }
                    if intersect.is_none() {
                        continue;
                    }
                    if !found_common {
                        continue;
                    }
                    let (ix, iy) = intersect.unwrap();
                    // if ix.frac() != ZERO || iy.frac() != ZERO {
                    //     continue;
                    // }

                    println!(
                        "common intersec x, y found: {:?} | {vr_x} {vr_y}",
                        intersect
                    );

                    // find a suitable z
                    let mut vr_z: Option<BigFloat> = None;
                    let mut found_common = true;
                    for i in 1..length {
                        // find z between first and i
                        /*
                           z = h1.pz + (h1.vz - vr_z) * t1
                           z = h2.pz + (h2.vz - vr_z) * t2
                           => h1.pz + (h1.vz - vr_z) * t1 = h2.pz + (h2.vz - vr_z) * t2
                           <=> h1.pz + h1.vz*t1 - vr_z*t1 = h2.pz + h2.vz * t2 - vr_z*t2
                           <=> vr_z(t2 - t1) = h2.pz + h2.vz * t2 - h1.pz - h1.vz*t1
                           <=> vr_z = (h2.pz + h2.vz*t2 - h1.pz - h1.vz*t1)/(t2-t1)
                        */
                        let t1 = get_time(&stones[0], ix, iy);
                        let t2 = get_time(&stones[i], ix, iy);
                        if t1.is_none() || t2.is_none() {
                            break;
                        }
                        let (t1, t2) = (t1.unwrap(), t2.unwrap());
                        if t1 == t2 {
                            found_common = false;
                            break;
                        }
                        let vr_z_n =
                            (stones[i].pz + stones[i].vz * t2 - stones[0].pz - stones[0].vz * t1)
                                / (t2 - t1);
                        if vr_z.is_none() {
                            vr_z = Some(vr_z_n);
                            continue;
                        }
                        if let Some(vr_z) = vr_z {
                            if vr_z == vr_z_n {
                                continue;
                            }
                            println!("invalid vr_z {} {:?}", vr_z.to_f64(), stones[i]);
                            found_common = false;
                            break;
                        }
                    }
                    if vr_z.is_none() {
                        continue;
                    }
                    if !found_common {
                        continue;
                    }
                    let vr_z = vr_z.unwrap();
                    let z = stones[0].pz
                        + get_time(&stones[0], ix, iy).unwrap() * (stones[0].vz - vr_z);

                    for s in stones.iter() {
                        println!("{:?}", s);
                    }

                    println!(
                        "found {} {} {} | {} {} {} | {}",
                        ix.to_f64(),
                        iy.to_f64(),
                        z.to_f64(),
                        vr_x.to_f64(),
                        vr_y.to_f64(),
                        vr_z.to_f64(),
                        get_time(&stones[0], ix, iy).unwrap().to_f64()
                    );
                    return;
                }
            }
        }
    }
}

fn get_time(stone: &HailStone, pos_x: BigFloat, pos_y: BigFloat) -> Option<BigFloat> {
    if pos_x == ZERO && pos_y == ZERO {
        return None;
    }
    if pos_x != ZERO {
        return Some((pos_x - stone.px) / stone.vx);
    }
    Some((pos_y - stone.py) / stone.vy)
}

pub fn p2_parse_input(file: &str) -> Vec<HailStone> {
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
            px: BigFloat::from_f64(pos[0]),
            py: BigFloat::from_f64(pos[1]),
            pz: BigFloat::from_f64(pos[2]),
            vx: BigFloat::from_f64(vel[0]),
            vy: BigFloat::from_f64(vel[1]),
            vz: BigFloat::from_f64(vel[2]),
        })
    }

    hail_stones
}

fn collide(h1: &HailStone, h2: &HailStone) -> Option<(BigFloat, BigFloat)> {
    if h1.px == h2.px && h1.py == h2.py && h1.pz == h2.pz {
        return Some((ZERO, ZERO));
    }

    /*
        {
            x1 = px1 + vx1 * t1; y1 = py1 + vy1 * t1
            x2 = px2 + vx2 * t2; y2 = py2 + vy2 * t2
        }
        => t2 = ((px2 - px1)*vy1 - (py2 - py1)*vx1)/(-vx2*vy1 + vy2*vx1)
    */
    let div_2 = -h2.vx * h1.vy + h2.vy * h1.vx;
    if div_2 == ZERO {
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

    if t1 < ZERO || t2 < ZERO {
        return None;
    }

    let (collide_x, collide_y) = (h1.px + h1.vx * t1, h1.py + h1.vy * t1);

    Some((collide_x, collide_y))
}
