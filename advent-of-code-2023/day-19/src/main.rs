use std::{collections::HashMap, fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";
    let (w, p) = p1_parse_input(INPUT);
    p1_solve(&w, &p);

    part_2(&w);
}

#[derive(Clone, Debug)]
struct Bound {
    x_min: i64,
    x_max: i64,
    m_min: i64,
    m_max: i64,
    a_min: i64,
    a_max: i64,
    s_min: i64,
    s_max: i64,
}

impl Bound {
    fn total(&self) -> i64 {
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }
}

fn part_2(workflows: &[Workflow]) {
    let mut m_workflows: HashMap<String /*name*/, Workflow> = HashMap::new();
    for w in workflows {
        m_workflows.insert(w.name.clone(), w.clone());
    }
    let bound = Bound {
        x_min: 1,
        x_max: 4_000,
        m_min: 1,
        m_max: 4_000,
        a_min: 1,
        a_max: 4_000,
        s_min: 1,
        s_max: 4_000,
    };
    let mut total = 0;

    dfs(&m_workflows, "in".to_string(), 0, bound, &mut total);

    println!("total {}", total)
}

fn dfs(
    worksflows: &HashMap<String, Workflow>,
    workflow_name: String,
    step_idx: usize,
    bound: Bound,
    total: &mut i64,
) {
    println!("visiting {} {} {:?}", workflow_name, step_idx, bound);
    let w = worksflows.get(&workflow_name).unwrap();
    let s = &w.steps[step_idx];

    // update bound
    let (mut b_true, mut b_false) = (bound.clone(), bound.clone());
    match s.check_val {
        'x' => {
            if s.is_gt {
                // true
                b_true.x_min = s.value + 1;
                // false
                b_false.x_max = s.value
            } else {
                // true
                b_true.x_max = s.value - 1;
                // false
                b_false.x_min = s.value;
            }
        }
        'm' => {
            if s.is_gt {
                // true
                b_true.m_min = s.value + 1;
                // false
                b_false.m_max = s.value
            } else {
                // true
                b_true.m_max = s.value - 1;
                // false
                b_false.m_min = s.value;
            }
        }
        'a' => {
            if s.is_gt {
                // true
                b_true.a_min = s.value + 1;
                // false
                b_false.a_max = s.value
            } else {
                // true
                b_true.a_max = s.value - 1;
                // false
                b_false.a_min = s.value;
            }
        }
        's' => {
            if s.is_gt {
                // true
                b_true.s_min = s.value + 1;
                // false
                b_false.s_max = s.value
            } else {
                // true
                b_true.s_max = s.value - 1;
                // false
                b_false.s_min = s.value;
            }
        }
        _ => {}
    }

    // if true or if false
    match s.if_true.clone() {
        Destination::Accept => {
            println!(
                "true: {} {} {:?} {}",
                workflow_name,
                step_idx,
                b_true,
                b_true.total()
            );
            *total += b_true.total()
        }
        Destination::Reject => {
            println!("rejected {} {}", workflow_name, step_idx);
        }
        Destination::Workflow(n) => dfs(worksflows, n.clone(), 0, b_true, total),
        Destination::NextStep => dfs(
            worksflows,
            workflow_name.clone(),
            step_idx + 1,
            b_true,
            total,
        ),
    }

    match s.if_false.clone() {
        Destination::Accept => {
            println!(
                "fals: {} {} {:?} {}",
                workflow_name,
                step_idx,
                b_false,
                b_false.total()
            );
            *total += b_false.total()
        }
        Destination::Reject => {
            println!("rejected {} {}", workflow_name, step_idx);
        }
        Destination::Workflow(n) => dfs(worksflows, n.clone(), 0, b_false, total),
        Destination::NextStep => dfs(
            worksflows,
            workflow_name.clone(),
            step_idx + 1,
            b_false,
            total,
        ),
    }
}

fn p1_solve(workflows: &[Workflow], parts: &[Part]) {
    let mut m_workflows: HashMap<String /*name*/, Workflow> = HashMap::new();
    for w in workflows {
        m_workflows.insert(w.name.clone(), w.clone());
    }

    let mut sum = 0;
    for p in parts.iter() {
        let mut w = m_workflows.get("in").unwrap();
        loop {
            let mut is_next = false;
            for s in w.steps.iter() {
                match s.next(p) {
                    Destination::Accept => {
                        sum += p.sum();
                        is_next = true;
                        break;
                    }
                    Destination::Reject => {
                        is_next = true;
                        break;
                    }
                    Destination::Workflow(n) => {
                        w = m_workflows.get(&n).unwrap();
                        is_next = false;
                        break;
                    }
                    _ => {}
                }
            }
            if is_next {
                break;
            }
        }
    }

    println!("sum {}", sum);
}

fn p1_parse_input(file: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().collect();
    let mut workflows: Vec<Workflow> = Vec::new();
    for l in lines.iter() {
        if l.is_empty() {
            break;
        }
        // parse workflow
        let (workflow_name, steps) = l.split_once('{').unwrap();
        let steps: Vec<&str> = steps.trim_end_matches('}').split(',').collect();
        let mut w_steps: Vec<Step> = Vec::new();
        for (i, s) in steps.iter().enumerate() {
            // last value -> false direction of the last step
            // println!("step {}", s);
            if i == steps.len() - 1 {
                let dst = Destination::from_str(s);
                w_steps.last_mut().unwrap().if_false = dst;
                break;
            }
            // parse step: a<2006:qkq
            let (cond, if_true) = s.split_once(':').unwrap();
            let is_gt = cond[1..2].eq(">");

            w_steps.push(Step {
                check_val: cond[0..].chars().next().unwrap(),
                is_gt,
                value: cond[2..].parse().unwrap(),
                if_true: Destination::from_str(if_true),
                if_false: Destination::NextStep,
            })
        }
        workflows.push(Workflow {
            name: workflow_name.to_string(),
            steps: w_steps,
        })
    }

    let mut parts: Vec<Part> = Vec::new();
    for l in lines.iter().skip_while(|l| !l.is_empty()).skip(1) {
        if l.is_empty() {
            break;
        }

        let l = l.trim_start_matches('{').trim_end_matches('}');
        let part_data: Vec<&str> = l.split(',').collect();
        let mut p = Part::default();
        for v in part_data {
            let (char, val): (&str, i64) = (&v[0..1], v[2..].parse().unwrap());
            match char {
                "x" => p.x = val,
                "m" => p.m = val,
                "a" => p.a = val,
                "s" => p.s = val,
                _ => {
                    panic!("invalid parse char {}", char)
                }
            }
        }
        parts.push(p);
    }

    (workflows, parts)
}

#[derive(Default, Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    steps: Vec<Step>,
}

#[derive(Debug, Clone)]
struct Step {
    check_val: char,
    is_gt: bool,
    value: i64,
    if_true: Destination,
    if_false: Destination,
}

impl Step {
    fn next(&self, p: &Part) -> Destination {
        let v = match self.check_val {
            'x' => p.x,
            'm' => p.m,
            'a' => p.a,
            's' => p.s,
            _ => panic!("invalid val {}", self.check_val),
        };

        if self.is_gt {
            if v > self.value {
                return self.if_true.clone();
            }
            return self.if_false.clone();
        }

        if v < self.value {
            return self.if_true.clone();
        }
        self.if_false.clone()
    }
}

#[derive(Debug, Clone)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
    NextStep,
}

impl Destination {
    fn from_str(c: &str) -> Destination {
        match c {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(c.to_string()),
        }
    }
}
