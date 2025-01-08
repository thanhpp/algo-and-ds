use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alphanumeric1, multi::separated_list1,
    sequence::tuple, IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl Display for GateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateType::And => write!(f, "AND"),
            GateType::Or => write!(f, "OR"),
            GateType::Xor => write!(f, "XOR"),
        }
    }
}

impl From<&str> for GateType {
    fn from(s: &str) -> Self {
        match s {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("invalid gate type: {}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    q: &'a str,
    t: GateType,
}

impl<'a> Gate<'a> {
    fn new(a: &'a str, b: &'a str, t: GateType, q: &'a str) -> Self {
        Self { a, b, q, t }
    }

    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, gate) = nom::combinator::map(
            tuple((
                alphanumeric1,
                tag(" "),
                alt((tag("AND"), tag("OR"), tag("XOR"))),
                tag(" "),
                alphanumeric1,
                tag(" -> "),
                alphanumeric1,
            )),
            |(a, _, t, _, b, _, s)| Gate::new(a, b, t.into(), s),
        )(input)?;

        Ok((input, gate))
    }

    fn run(&self, a: bool, b: bool) -> bool {
        match self.t {
            GateType::And => a && b,
            GateType::Or => a || b,
            GateType::Xor => a ^ b,
        }
    }

    fn has_input(&self, wire: &str) -> bool {
        self.a == wire || self.b == wire
    }

    fn has_output(&self, wire: &str) -> bool {
        self.q == wire
    }

    fn is_half_adder_input(&self) -> bool {
        self.a == "x00" || self.b == "x00"
    }

    fn is_input(&self) -> bool {
        self.a.starts_with('x')
            || self.b.starts_with('x')
            || self.a.starts_with('y')
            || self.b.starts_with('y')
    }

    fn is_output(&self) -> bool {
        self.q.starts_with('z')
    }

    fn is_type(&self, t: GateType) -> bool {
        self.t == t
    }
}

#[derive(Debug)]
struct Wires<'a> {
    wires: HashMap<&'a str, bool>,
}

impl<'a> Wires<'a> {
    fn parse_bool(input: &'a str) -> IResult<&'a str, bool> {
        let (input, b) = alt((tag("0"), tag("1")))(input)?;
        Ok((input, b == "1"))
    }

    fn parse_wire(input: &'a str) -> IResult<&'a str, (&'a str, bool)> {
        let (input, (name, _, state)) =
            tuple((alphanumeric1, tag(": "), Wires::parse_bool))(input)?;
        Ok((input, (name, state)))
    }

    fn parse_all(input: &'a str) -> IResult<&'a str, Self> {
        let (input, wires) = separated_list1(tag("\n"), Wires::parse_wire)(input)?;
        let wires = wires.into_iter().collect();
        Ok((input, Self { wires }))
    }

    fn get(&self, wire: &'a str) -> Option<bool> {
        self.wires.get(wire).copied()
    }

    fn get_inputs(&self, a: &'a str, b: &'a str) -> Option<(bool, bool)> {
        let a = self.get(a)?;
        let b = self.get(b)?;
        Some((a, b))
    }

    fn set(&mut self, wire: &'a str, value: bool) {
        self.wires.insert(wire, value);
    }

    fn with_prefix(&self, prefix: &str) -> Vec<(&'a str, bool)> {
        self.wires
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(&k, &v)| (k, v))
            .collect()
    }
}

#[derive(Debug)]
struct Devices<'a> {
    gates: Vec<Gate<'a>>,
    wires: Wires<'a>,
}

impl<'a> Devices<'a> {
    fn new(input: &'a str) -> IResult<&'a str, Self> {
        let (input, wires) = Wires::parse_all(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, gates) = separated_list1(tag("\n"), Gate::parse)(input)?;
        Ok((input, Devices { gates, wires }))
    }
}

pub fn solve_part2(input: &str) {
    let (_, mut device) = Devices::new(input).unwrap();

    // track all of the bad wires
    let mut bad_wires = vec![];

    let input_xor_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();

    println!("1: {:?}", bad_wires);

    // run check input xor gates
    for gate in &input_xor_gates {
        // only the init half adder gate should have z00 outout
        match (gate.is_half_adder_input(), gate.has_output("z00")) {
            (true, true) | (false, false) => {
                continue;
            }
            _ => {
                bad_wires.push(gate.q);
            }
        }

        // all other input gates should not have an output, because it needs to be
        // XOR with the previous carry wire
        if gate.is_output() {
            bad_wires.push(gate.q);
        }
    }

    println!("2: {:?}", bad_wires);

    // check intermediate xore gates
    let intermediate_xor_gates = device
        .gates
        .iter()
        .filter(|gate| !gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();
    // other XOR gates should be inside a full adders and produce an output
    for gate in &intermediate_xor_gates {
        if gate.is_output() {
            continue;
        }
        bad_wires.push(gate.q);
    }

    println!("3: {:?}", bad_wires);

    // check output gates
    let output_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_output())
        .cloned()
        .collect::<Vec<_>>();
    // The last output gate should come from a carry wire and the XOR of the input wires
    // The last wire is just the last carry wire of the last full adder
    let last_z_wire = format!(
        "z{:02}",
        device.gates.iter().filter(|gate| gate.is_output()).count() - 1
    );

    println!("last z: {}", last_z_wire);

    for gate in &output_gates {
        if gate.has_output(&last_z_wire) {
            if !gate.is_type(GateType::Or) {
                bad_wires.push(gate.q);
            }
            continue;
        }
        if !gate.is_type(GateType::Xor) {
            // other output gate should have type XOR
            bad_wires.push(gate.q);
        }
    }

    println!("4: {:?}", bad_wires);

    let mut check = vec![];
    for gate in &input_xor_gates {
        // not checking found bad wires
        // not checking the first half adder (already checked)
        if bad_wires.contains(&gate.q) || gate.has_output("z00") {
            continue;
        }

        // look for intermediate XOR gates that don't have the output of an input XOR gate
        if intermediate_xor_gates
            .iter()
            .filter(|gate1| gate1.has_input(gate.q))
            .count()
            == 0
        {
            bad_wires.push(gate.q);
            check.push(gate); // check later
        }
    }

    for gate in check {
        let expected = format!("z{}", &gate.a[1..]);
        let m = intermediate_xor_gates
            .iter()
            .find(|gate| gate.has_output(&expected))
            .unwrap();

        let o = device
            .gates
            .iter()
            .find(|gate| gate.is_type(GateType::Or) && (gate.q == m.a || gate.q == m.b))
            .unwrap();

        if m.a != o.q {
            bad_wires.push(m.a);
        }

        if m.b != o.q {
            bad_wires.push(m.b);
        }
    }

    bad_wires.sort();
    println!("{}", bad_wires.join(","))
}

// https://gist.github.com/icub3d/acbd4e7c487a6eedb8daf5c1fbb2ad25
pub fn part_2_solution(input: &str) {
    let (_, mut device) = Devices::new(input).unwrap();
    // Track bad wires.
    let mut bad_wires = vec![];

    let input_xor_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();

    println!("1: {:?}", bad_wires);

    // First check all the input xor gates.
    for gate in &input_xor_gates {
        // The initial half adder gate should have a z00 output. No other input
        // gates should have z00.
        match (gate.is_half_adder_input(), gate.has_output("z00")) {
            (true, true) => continue,
            (true, false) => bad_wires.push(gate.q),
            (false, true) => bad_wires.push(gate.q),
            (false, false) => continue,
        }
        // All other input gates should not have an output because it needs to
        // be XOR with the previous carry wire.
        if gate.is_output() {
            bad_wires.push(gate.q);
        }
    }

    println!("2: {:?}", bad_wires);

    let intermediate_xor_gates = device
        .gates
        .iter()
        .filter(|gate| !gate.is_input() && gate.is_type(GateType::Xor))
        .cloned()
        .collect::<Vec<_>>();

    // All other XOR gates should be internal to the full adders and produce an
    // output.
    intermediate_xor_gates.iter().for_each(|gate| {
        if !gate.is_output() {
            bad_wires.push(gate.q);
        }
    });

    println!("3: {:?}", bad_wires);

    let output_gates = device
        .gates
        .iter()
        .filter(|gate| gate.is_output())
        .cloned()
        .collect::<Vec<_>>();

    // All output gates but the last one should come from a carry wire and the
    // XOR of the input wires. The last wire is just the last carry wire of the last
    // full adder.
    let last_z_wire = format!(
        "z{:02}",
        device.gates.iter().filter(|gate| gate.is_output()).count() - 1
    );

    println!("last z: {}", last_z_wire);

    for gate in &output_gates {
        if gate.has_output(&last_z_wire) {
            if !gate.is_type(GateType::Or) {
                bad_wires.push(gate.q);
            }
            continue;
        } else if !gate.is_type(GateType::Xor) {
            bad_wires.push(gate.q);
        }
    }

    println!("4: {:?}", bad_wires);

    // Gather all the variables from the input_gates
    let mut check = vec![];
    for gate in &input_xor_gates {
        // No need to check the first half adder or any bad wires we've already found.
        if bad_wires.contains(&gate.q) || gate.has_output("z00") {
            continue;
        }

        // Look for intermediate xor gates that don't have the output of an input XOR gate.
        if intermediate_xor_gates
            .iter()
            .filter(|g| g.has_input(gate.q))
            .count()
            == 0
        {
            // They are bad wires and we need to check them later.
            bad_wires.push(gate.q);
            check.push(gate);
        }
    }

    for gate in check {
        let expected = format!("z{}", &gate.a[1..]);
        let m = intermediate_xor_gates
            .iter()
            .find(|gate| gate.has_output(&expected))
            .unwrap();

        let o = device
            .gates
            .iter()
            .find(|gate| gate.is_type(GateType::Or) && (gate.q == m.a || gate.q == m.b))
            .unwrap();

        if m.a != o.q {
            bad_wires.push(m.a);
        }
        if m.b != o.q {
            bad_wires.push(m.b);
        }
    }

    bad_wires.sort();
    println!("p2: {} ", bad_wires.join(","));
}
