use std::{fs, io::Read};

fn main() {
    let probs = read("input_1.txt");
    // println!("{:#?}", probs);
    solve1(&probs);
    solve2(&probs);
}

#[derive(Debug, Clone, Default)]
pub struct Button {
    pub name: String,
    pub del_x: f64,
    pub del_y: f64,
    pub cost: f64,
}

impl Button {
    fn from_str(s: &str) -> Self {
        let tmp = s.split_whitespace().collect::<Vec<&str>>();
        // println!("tmp button {:?}", tmp);

        Self {
            name: tmp[1].trim_end_matches(':').to_string(),
            del_x: tmp[2]
                .trim_start_matches("X+")
                .trim_end_matches(',')
                .parse()
                .unwrap(),
            del_y: tmp[3]
                .trim_start_matches("Y+")
                .trim_end_matches(',')
                .parse()
                .unwrap(),
            cost: match tmp[1] {
                "A:" => 3.0,
                "B:" => 1.0,
                _ => panic!("wtf {}", tmp[1]),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Problem1 {
    pub button_a: Button,
    pub button_b: Button,
    pub prize_x: f64,
    pub prize_y: f64,
}

impl Problem1 {
    fn solve_1(&self) -> i64 {
        let b_count = (self.prize_x * self.button_a.del_y - self.prize_y * self.button_a.del_x)
            / (self.button_b.del_x * self.button_a.del_y
                - self.button_b.del_y * self.button_a.del_x);
        if b_count.fract() != 0.0 || b_count <= 0.0 {
            return 0;
        }

        let a_count = (self.prize_x - b_count * self.button_b.del_x) / self.button_a.del_x;
        if a_count.fract() != 0.0 || a_count <= 0.0 {
            return 0;
        }

        // println!("b {} | a {}", b_count, a_count);

        (a_count * self.button_a.cost + b_count * self.button_b.cost) as i64
    }
}

fn read(p: &str) -> Vec<Problem1> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    let lines = s.split('\n').collect::<Vec<&str>>();
    let mut problems = vec![];
    let mut i = 0;
    while i < lines.len() {
        let (button_a, button_b, prize) = (lines[i], lines[i + 1], lines[i + 2]);
        i += 4;

        let prize_tmp = prize.split_ascii_whitespace().collect::<Vec<&str>>();

        problems.push(Problem1 {
            button_a: Button::from_str(button_a),
            button_b: Button::from_str(button_b),
            prize_x: prize_tmp[1]
                .trim_start_matches("X=")
                .trim_end_matches(',')
                .parse()
                .unwrap(),
            prize_y: prize_tmp[2].trim_start_matches("Y=").parse().unwrap(),
        });
    }

    problems
}

fn solve1(problems: &[Problem1]) {
    let mut tokens = 0;
    for (i, p) in problems.iter().enumerate() {
        let cost = p.solve_1();
        // println!("i: {} | prob {:?} | cost: {}", i, p, cost);
        tokens += cost;
    }

    println!("solve1: {}", tokens);
}

fn solve2(problems: &[Problem1]) {
    let mut probs = vec![Problem1::default(); problems.len()];
    probs.clone_from_slice(problems);
    let mut tokens = 0;

    for (i, p) in probs.iter_mut().enumerate() {
        p.prize_x += 10000000000000.0;
        p.prize_y += 10000000000000.0;
        let cost = p.solve_1();
        // println!("i: {} | prob {:?} | cost: {}", i, p, cost);
        tokens += cost;
    }

    println!("solve2: {}", tokens);
}
