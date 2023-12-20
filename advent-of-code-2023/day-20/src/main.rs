use graphviz_rust::{
    attributes::{arrowhead, EdgeAttributes},
    cmd::{CommandArg, Format},
    dot_generator::*,
    dot_structures::*,
    exec,
    printer::PrinterContext,
};
use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::Read,
    time::SystemTime,
};

fn main() {
    const INPUT: &str = "input.txt";
    println!("start p1");
    let mods = p1_parse(INPUT);
    for m in mods.iter() {
        println!("{:?}", m)
    }
    p1_solve(&mods);
    println!("end p1");

    println!("start p2");
    p2_graphviz(&mods);

    /*
        for rx to receive a low pulse
        -> qb must remember all recent pulse = high
        -> kv, jg, rz, mr must send high (each is an output of a subgraph (all flipflop -> conj)) -> record all lows
        -> qz, sk, sv, dr must record all high -> to send low
    */
    for n in ["qz", "sk", "sv", "dr"] {
        println!("p2 {} {}", n, p2_find_push(&mods, n));
    }

    /*
    minimum push to cause these to send a low pulse (records all high)
    p2 qz 3739
    p2 sk 3911
    p2 sv 4073
    p2 dr 4003
    => LCM of those above
    */

    // 238,420,328,103,151

    println!("end p2");
}

fn p2_find_push(mods: &HashMap<String, Module>, node: &str) -> usize {
    let mut mods = mods.clone();
    // println!("original state [{}]", orginal_state);

    for push in 1..=usize::MAX {
        if push % 1_000_000 == 0 {
            println!("{:#?} {}", SystemTime::now(), push)
        }
        // println!("start cycle {}", cycle);
        let mut queue = VecDeque::<(String /*from*/, String /*to*/, Pulse)>::new();
        let broadcast_action = mods.get_mut("broadcaster").unwrap().next("", Pulse::Low);
        for act in broadcast_action {
            // println!("pushed {:?}", act);
            queue.push_front(("broadcaster".to_string(), act.0, act.1))
        }
        while !queue.is_empty() {
            let act = queue.pop_back().unwrap();
            // println!("--> {:?}", act);
            let module = match mods.get_mut(&act.1) {
                None => continue,
                Some(m) => m,
            };
            for next_act in module.next(&act.0, act.2) {
                // println!("pushed {:?}", next_act);
                queue.push_front((act.1.clone(), next_act.0, next_act.1))
            }

            let n = mods.get(node).unwrap();
            // println!("{:?}", n);
            let mut is_all_high = true;
            for (_, v) in n.most_recent.iter() {
                if v.eq(&Pulse::Low) {
                    is_all_high = false;
                    break;
                }
            }
            if is_all_high {
                return push;
            }
        }
    }

    0
}

fn p2_graphviz(mods: &HashMap<String, Module>) {
    let mut g = graph!(di id!("p2_graph"));
    for (k, v) in mods.iter() {
        match v.mod_type {
            ModuleType::Broadcaster => {
                g.add_stmt(Stmt::Node(node!(k.clone(); attr!("color", "pink"))));
            }
            ModuleType::FlipFlop => {
                g.add_stmt(Stmt::Node(node!(k.clone(); attr!("color", "green"))));
            }
            ModuleType::Conjunction => {
                g.add_stmt(Stmt::Node(node!(k.clone(); attr!("color", "blue"))));
            }
        }

        for d in v.dest.iter() {
            g.add_stmt(Stmt::Edge(
                edge!(node_id!(k.to_string()) => node_id!(d.clone()); attr!("arrowhead", "normal"), attr!("arrowtail", "normal")),
            ));
        }
    }

    exec(
        g,
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("./p2_graphviz.png".to_string()),
        ],
    )
    .unwrap();
}

fn p1_solve(mods: &HashMap<String, Module>) {
    let (push, low, high) = p1_send_pulse(mods, 1000);
    println!("{} {} {}", push, low, high);

    let (_, left_low, left_high) = match 1000 % push {
        0 => (0, 0, 0),
        _ => p1_send_pulse(mods, 1000 % push),
    };
    println!("{} {} {}", 1000 % push, left_low, left_low);

    let ans = (low * 1000 / push + left_low) * (high * 1000 / push + left_high);

    println!("p1_solve: {}", ans);
}

fn p1_send_pulse(mods: &HashMap<String, Module>, limit: usize) -> (usize, usize, usize) {
    let mut mods = mods.clone();
    let orginal_state = p1_hash_mods(&mods);
    // println!("original state [{}]", orginal_state);

    let (mut cycle_low, mut cycle_high) = (0, 0);
    for push in 1..=limit {
        if push % 1_000_000 == 0 {
            println!("{:#?} {}", SystemTime::now(), push)
        }
        // println!("start cycle {}", cycle);
        let mut queue = VecDeque::<(String /*from*/, String /*to*/, Pulse)>::new();
        let broadcast_action = mods.get_mut("broadcaster").unwrap().next("", Pulse::Low);
        for act in broadcast_action {
            // println!("pushed {:?}", act);
            queue.push_front(("broadcaster".to_string(), act.0, act.1))
        }
        while !queue.is_empty() {
            let act = queue.pop_back().unwrap();
            // println!("--> {:?}", act);
            if act.2 == Pulse::High {
                cycle_high += 1
            } else {
                cycle_low += 1
            }

            let module = match mods.get_mut(&act.1) {
                None => continue,
                Some(m) => m,
            };
            for next_act in module.next(&act.0, act.2) {
                // println!("pushed {:?}", next_act);
                queue.push_front((act.1.clone(), next_act.0, next_act.1))
            }
        }

        // println!(
        //     "cycle state [{}] [{}]",
        //     p1_hash_mods(&mods),
        //     p1_hash_mods(&mods) == orginal_state
        // );

        if p1_hash_mods(&mods) == orginal_state {
            // println!("cycle end {}, {} {}", push, cycle_low, cycle_high);
            return (push, cycle_low + push, cycle_high);
        }
    }

    (limit, cycle_low + limit, cycle_high)
}

fn p1_hash_mods(mods: &HashMap<String, Module>) -> String {
    let mut buffer = String::new();
    let mut v = mods
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<(String, Module)>>();
    v.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, v) in v.iter() {
        buffer.push_str(&v.hash())
    }

    buffer
}

fn p1_parse(file: &str) -> HashMap<String, Module> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut mods: HashMap<String, Module> = HashMap::new();
    for l in buffer.lines().filter(|l| !l.is_empty()) {
        let (src, dst) = l.split_once(" -> ").unwrap();
        let dst: Vec<String> = dst.split(',').map(|s| s.trim().to_string()).collect();
        let (mod_type, mod_name) = ModuleType::from_str(src);

        mods.insert(
            mod_name.clone(),
            Module {
                name: mod_name,
                mod_type,
                is_on: false,
                most_recent: HashMap::new(),
                dest: dst,
            },
        );
    }

    // build most recent
    let mut src_to_dst: Vec<(String, Vec<String>)> = Vec::new();
    for (k, v) in mods.iter() {
        src_to_dst.push((k.clone(), v.dest.clone()));
    }

    for (src, v) in src_to_dst.iter() {
        for dst in v.iter() {
            if let Some(m) = mods.get_mut(dst) {
                if m.mod_type != ModuleType::Conjunction {
                    continue;
                }
                m.most_recent.insert(src.clone(), Pulse::Low);
            }
        }
    }

    mods
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    mod_type: ModuleType,
    is_on: bool,                         // flip-flop
    most_recent: HashMap<String, Pulse>, // Conjunction (input -> last pulse)
    dest: Vec<String>,
}

impl Module {
    fn hash(&self) -> String {
        let mut v = self
            .most_recent
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<(String, Pulse)>>();
        v.sort_by(|a, b| a.0.cmp(&b.0));

        format!("{}/{}/{:?}", self.name, self.is_on, v)
    }
}

impl Module {
    fn next(&mut self, from: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        let mut v: Vec<(String, Pulse)> = Vec::new();

        match self.mod_type {
            ModuleType::Broadcaster => {
                for d in self.dest.iter() {
                    v.push((d.clone(), pulse))
                }
            }
            ModuleType::FlipFlop => {
                if pulse == Pulse::Low {
                    for d in self.dest.iter() {
                        if !self.is_on {
                            v.push((d.to_string(), Pulse::High))
                        } else {
                            v.push((d.to_string(), Pulse::Low))
                        }
                    }
                    self.is_on = !self.is_on;
                }
            }
            ModuleType::Conjunction => {
                // update
                if let Some(v) = self.most_recent.get_mut(from) {
                    *v = pulse
                }
                // check all false
                let mut is_all_high = true;
                for (_, v) in self.most_recent.iter() {
                    if *v == Pulse::Low {
                        is_all_high = false;
                        break;
                    }
                }

                for d in self.dest.iter() {
                    if is_all_high {
                        v.push((d.to_string(), Pulse::Low))
                    } else {
                        v.push((d.to_string(), Pulse::High))
                    }
                }
            }
        }

        // println!("receive {} {:?}, next {:?}", from, pulse, v);

        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

impl ModuleType {
    fn from_str(s: &str) -> (Self, String) {
        if s == "broadcaster" {
            return (Self::Broadcaster, s.to_string());
        }
        if s.starts_with('%') {
            return (Self::FlipFlop, s.split_at(1).1.to_string());
        }
        if s.starts_with('&') {
            return (Self::Conjunction, s.split_at(1).1.to_string());
        }

        panic!("invalid module type [{}]", s)
    }
}
