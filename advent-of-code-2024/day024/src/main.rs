use std::collections::HashMap;

use graphviz_rust::{
    dot_structures::{Edge, Graph},
    printer::{DotPrinter, PrinterContext},
};

fn main() {
    let data = include_str!("../input_1.txt");
    let prob = Problem::new(data);

    // println!("{:#?}", prob);

    // solve1(&prob);
    draw_graphviz(&prob);
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub wires: HashMap<String, i64>,
    pub operations: Vec<Operation>,
}

impl Problem {
    pub fn new(data: &str) -> Self {
        let mut wires = HashMap::<String, i64>::new();
        let mut operations = Vec::<Operation>::new();
        let lines = data.lines().collect::<Vec<&str>>();

        let mut idx = 0;
        loop {
            if lines[idx].is_empty() {
                break;
            }

            let (wire_name, value) = lines[idx].split_once(": ").unwrap();
            wires.insert(wire_name.to_string(), value.parse().unwrap());

            idx += 1;
        }

        idx += 1;
        loop {
            if idx >= lines.len() || lines[idx].is_empty() {
                break;
            }
            let split = lines[idx].split(' ').collect::<Vec<&str>>();
            let wire_a = split[0];
            let op = split[1];
            let wire_b = split[2];
            let dest = split[4];

            operations.push(Operation {
                dest: dest.to_string(),
                a: wire_a.to_string(),
                b: wire_b.to_string(),
                op: op.to_string(),
            });

            idx += 1
        }

        Self { wires, operations }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub dest: String,
    pub a: String,
    pub b: String,
    pub op: String,
}

fn calculate(
    ops: &HashMap<String, Operation>,
    values: &mut HashMap<String, i64>,
    wire: String,
) -> i64 {
    if let Some(v) = values.get(&wire) {
        return *v;
    }

    let op = ops.get(&wire).unwrap();

    let v = do_op(
        calculate(ops, values, op.a.clone()),
        calculate(ops, values, op.b.clone()),
        &op.op,
    );
    values.insert(wire.clone(), v);

    v
}

fn do_op(value_a: i64, value_b: i64, op: &str) -> i64 {
    match op {
        "AND" => value_a & value_b,
        "OR" => value_a | value_b,
        "XOR" => value_a ^ value_b,
        _ => {
            panic!("wtf? {}", op)
        }
    }
}

fn solve1(prob: &Problem) {
    let dest_to_ops: HashMap<String, Operation> = HashMap::from_iter(
        prob.operations
            .iter()
            .map(|op| (op.dest.clone(), op.clone())),
    );

    let mut z_keys = dest_to_ops
        .keys()
        .filter(|&k| k.starts_with('z'))
        .cloned()
        .collect::<Vec<String>>();
    z_keys.sort();

    let mut values = prob.wires.clone();
    for zk in z_keys.iter() {
        calculate(&dest_to_ops, &mut values, zk.clone());
    }

    let mut value_keys = values.keys().cloned().collect::<Vec<String>>();
    value_keys.sort();

    let mut res = 0;
    for (i, k) in z_keys.iter().enumerate() {
        println!(
            "{} {} {}",
            k,
            values.get(k).unwrap(),
            values.get(k).unwrap() << i
        );
        res += values.get(k).unwrap() << i
    }

    println!("solve1: {}", res)
}

fn draw_graphviz(prob: &Problem) {
    let mut g = Graph::DiGraph {
        id: graphviz_rust::dot_structures::Id::Plain("g".to_string()),
        strict: false,
        stmts: vec![],
    };

    for op in prob.operations.iter() {
        let dest = graphviz_rust::dot_structures::Node {
            id: graphviz_rust::dot_structures::NodeId(
                graphviz_rust::dot_structures::Id::Plain(op.dest.to_string()),
                None,
            ),
            attributes: vec![],
        };
        let a = graphviz_rust::dot_structures::Node {
            id: graphviz_rust::dot_structures::NodeId(
                graphviz_rust::dot_structures::Id::Plain(op.a.to_string()),
                None,
            ),
            attributes: vec![],
        };
        let b = graphviz_rust::dot_structures::Node {
            id: graphviz_rust::dot_structures::NodeId(
                graphviz_rust::dot_structures::Id::Plain(op.b.to_string()),
                None,
            ),
            attributes: vec![],
        };
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(dest.clone()));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(a.clone()));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Node(b.clone()));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Edge(Edge {
            ty: graphviz_rust::dot_structures::EdgeTy::Pair(
                graphviz_rust::dot_structures::Vertex::N(a.id.clone()),
                graphviz_rust::dot_structures::Vertex::N(dest.id.clone()),
            ),
            attributes: vec![],
        }));
        g.add_stmt(graphviz_rust::dot_structures::Stmt::Edge(Edge {
            ty: graphviz_rust::dot_structures::EdgeTy::Pair(
                graphviz_rust::dot_structures::Vertex::N(b.id.clone()),
                graphviz_rust::dot_structures::Vertex::N(dest.id.clone()),
            ),
            attributes: vec![],
        }));
    }

    let v = g.print(&mut PrinterContext::default());
    println!("{}", v);
}
