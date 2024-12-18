use std::{collections::HashMap, fs, io::Read};

mod z3_sol;

#[tokio::main]
async fn main() {
    let prob = read("input_1.txt");
    println!("{:?}", prob);

    solve1(&prob.0, &prob.1);

    // const CONC: i64 = 100;
    // const STEP: i64 = 10_000_000;
    // let mut idx = 8_i64.pow(15);
    // loop {
    //     let mut futures = vec![];
    //     for c in 0..CONC {
    //         let start = idx + c * STEP;
    //         let end: i64 = start + STEP;
    //         let ins = prob.1.clone();
    //         let f = tokio::spawn(async move { solve2((start, end), &ins) });
    //         futures.push(f);
    //     }
    //     for f in futures {
    //         let found = f.await.unwrap();
    //         if found {
    //             return;
    //         }
    //     }
    //     println!("from {}, to {}", idx, idx + CONC * STEP);
    //     idx += CONC * STEP;
    // }

    let expect = prob
        .1
        .iter()
        .flat_map(|v| vec![v.0, v.1])
        .collect::<Vec<i64>>();
    z3_sol::solve_by_z3(&expect);
}

fn read(p: &str) -> (HashMap<String, i64>, Vec<(i64, i64)>) {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    let lines = s.lines().collect::<Vec<&str>>();
    let mut idx = 0;
    let mut regs = HashMap::<String, i64>::new();
    for l in lines.iter() {
        if l.is_empty() {
            break;
        }
        idx += 1;

        let split = l.split(' ').collect::<Vec<&str>>();
        let name = split[1].trim_end_matches(':').to_string();
        let value = split[2].parse().unwrap();

        regs.insert(name, value);
    }
    idx += 1;

    let ops = lines[idx].split(' ').collect::<Vec<&str>>()[1]
        .split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|c| (c[0].parse::<i64>().unwrap(), c[1].parse::<i64>().unwrap()))
        .collect();

    (regs, ops)
}

fn literal_operand(operand: i64) -> i64 {
    operand
}

fn combo_operand(register: &HashMap<String, i64>, operand: i64) -> i64 {
    match operand {
        0..=3 => literal_operand(operand),
        4 => register.get("A").cloned().unwrap(),
        5 => register.get("B").cloned().unwrap(),
        6 => register.get("C").cloned().unwrap(),
        _ => {
            panic!("invalid opcode: {}", operand)
        }
    }
}

// The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
fn adv_0(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    should_store: bool,
) -> (usize, i64) {
    let numerator = register.get("A").unwrap();
    let res = numerator / 2_i64.pow(combo_operand(register, operand) as u32);

    if should_store {
        register.insert("A".to_string(), res);
    }

    (ins_ptr + 1, res)
}
// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
fn bxl_1(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    should_store: bool,
) -> (usize, i64) {
    let val = register.get("B").cloned().unwrap();

    let res = val ^ literal_operand(operand);

    if should_store {
        register.insert("B".to_string(), res);
    }

    (ins_ptr + 1, res)
}
// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
fn bst_2(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    should_store: bool,
) -> (usize, i64) {
    let combo = combo_operand(register, operand);
    let res = combo % 8;

    if should_store {
        register.insert("B".to_string(), res);
    }

    (ins_ptr + 1, res)
}
// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
fn jnz_3(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    _: bool,
) -> (usize, i64) {
    let a = register.get("A").unwrap();
    if *a == 0 {
        return (ins_ptr + 1, 0);
    }

    (literal_operand(operand) as usize / 2, 0)
}
// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc_4(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    _: i64,
    should_store: bool,
) -> (usize, i64) {
    let b = register.get("B").unwrap();
    let c = register.get("C").unwrap();
    let res = *b ^ *c;

    if should_store {
        register.insert("B".to_string(), res);
    }

    (ins_ptr + 1, res)
}
// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
fn out_5(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    _: bool,
) -> (usize, i64) {
    let res = combo_operand(register, operand) % 8;

    (ins_ptr + 1, res)
}
// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
fn bdv_6(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    should_store: bool,
) -> (usize, i64) {
    let (_, adv) = adv_0(register, ins_ptr, operand, false);

    if should_store {
        register.insert("B".to_string(), adv);
    }

    (ins_ptr + 1, adv)
}
// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
fn cdv_7(
    register: &mut HashMap<String, i64>,
    ins_ptr: usize,
    operand: i64,
    should_store: bool,
) -> (usize, i64) {
    // println!(
    //     "register: {:?} | ins_ptr: {} | operand {}",
    //     register, ins_ptr, operand
    // );

    let (_, adv) = adv_0(register, ins_ptr, operand, false);

    if should_store {
        register.insert("C".to_string(), adv);
    }

    (ins_ptr + 1, adv)
}

fn solve1(register: &HashMap<String, i64>, ins: &[(i64, i64)]) {
    let mut register = register.clone();
    let mut res = vec![];

    let mut ins_ptr = 0;
    while ins_ptr < ins.len() {
        // println!("\nins_ptr: {}", ins_ptr);
        let (opcode, operand) = ins[ins_ptr];
        // println!("opcode: {}, operand: {}", opcode, operand);

        let (next_ins_ptr, val) = match opcode {
            0 => adv_0(&mut register, ins_ptr, operand, true),
            1 => bxl_1(&mut register, ins_ptr, operand, true),
            2 => bst_2(&mut register, ins_ptr, operand, true),
            3 => jnz_3(&mut register, ins_ptr, operand, true),
            4 => bxc_4(&mut register, ins_ptr, operand, true),
            5 => out_5(&mut register, ins_ptr, operand, true),
            6 => bdv_6(&mut register, ins_ptr, operand, true),
            7 => cdv_7(&mut register, ins_ptr, operand, true),
            _ => {
                panic!("invalid exec: {}", opcode)
            }
        };
        if opcode == 5 {
            // println!("out: {}", val);
            res.push(val);
        }
        ins_ptr = next_ins_ptr;
        // println!("next_ins_ptr {}, val: {}", next_ins_ptr, val);
    }

    println!(
        "solve1: {}",
        res.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
// pub struct State {
//     pub a: i64,
//     pub b: i64,
//     pub c: i64,
//     pub opcode: i64,
//     pub operand: i64,
//     pub current_matching_len: usize,
// }

// fn solve2(ins: &[(i64, i64)]) {
//     let expect = ins
//         .iter()
//         .flat_map(|v| vec![v.0, v.1])
//         .collect::<Vec<i64>>();

//     println!("expect {:?}", expect);

//     let mut a_val = 0;
//     let mut impossible = HashSet::new();
//     loop {
//         if a_val % 100_000 == 0 {
//             println!("visiting {}, impossible {}", a_val, impossible.len());
//         }
//         if try_with_a(&mut impossible, ins, a_val, &expect).is_none() {
//             a_val += 1;
//             continue;
//         }
//         break;
//     }
//     println!("solve2: {}", a_val)
// }

// fn try_with_a(
//     impossible: &mut HashSet<State>,
//     ins: &[(i64, i64)],
//     a: i64,
//     expect: &[i64],
// ) -> Option<()> {
//     let mut register = HashMap::new();
//     register.insert("A".to_string(), a);
//     register.insert("B".to_string(), 0);
//     register.insert("C".to_string(), 0);

//     let mut current_out = vec![];
//     let mut visited = vec![];

//     let mut ins_ptr = 0;
//     while ins_ptr < ins.len() {
//         // println!("\nins_ptr: {}", ins_ptr);
//         let (opcode, operand) = ins[ins_ptr];
//         // println!("opcode: {}, operand: {}", opcode, operand);

//         let current_state = State {
//             a: *register.get("A").unwrap(),
//             b: *register.get("B").unwrap(),
//             c: *register.get("C").unwrap(),
//             opcode,
//             operand,
//             current_matching_len: current_out.len(),
//         };
//         if impossible.contains(&current_state) {
//             break;
//         }

//         visited.push(current_state);

//         let (next_ins_ptr, val) = match opcode {
//             0 => adv_0(&mut register, ins_ptr, operand, true),
//             1 => bxl_1(&mut register, ins_ptr, operand, true),
//             2 => bst_2(&mut register, ins_ptr, operand, true),
//             3 => jnz_3(&mut register, ins_ptr, operand, true),
//             4 => bxc_4(&mut register, ins_ptr, operand, true),
//             5 => out_5(&mut register, ins_ptr, operand, true),
//             6 => bdv_6(&mut register, ins_ptr, operand, true),
//             7 => cdv_7(&mut register, ins_ptr, operand, true),
//             _ => {
//                 panic!("invalid exec: {}", opcode)
//             }
//         };
//         if opcode == 5 {
//             // println!("out: {}", val);
//             current_out.push(val);
//             // compare
//             if current_out[current_out.len() - 1] != expect[current_out.len() - 1] {
//                 break;
//             }
//             if current_out.len() == expect.len() {
//                 return Some(());
//             }
//         }
//         ins_ptr = next_ins_ptr;
//         // println!("next_ins_ptr {}, val: {}", next_ins_ptr, val);
//     }

//     for v in visited {
//         impossible.insert(v);
//     }

//     None
// }

fn solve2((start_a, end_a): (i64, i64), ins: &[(i64, i64)]) -> bool {
    // println!("solving from: {} to: {}", start_a, end_a);

    let expect = ins
        .iter()
        .flat_map(|v| vec![v.0, v.1])
        .collect::<Vec<i64>>();

    let mut found = false;
    for a in start_a..=end_a {
        if found {
            return found;
        }
        let mut res = vec![];
        let mut tmp_a = a;
        loop {
            if tmp_a == 0 {
                break;
            }
            let mut b = (tmp_a % 8) ^ 1;
            let c = tmp_a / 2_i64.pow(b as u32);
            b ^= 5;
            b ^= c;
            res.push(b % 8);
            if res[res.len() - 1] != expect[res.len() - 1] {
                break;
            }
            if res.len() == expect.len() {
                found = true;
                println!("{}", a);
                break;
            }
            tmp_a /= 8;
        }
    }

    found
}
