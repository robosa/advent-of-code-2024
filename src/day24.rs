use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1, newline, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Node<'a> {
    Value(u8),
    Gate(Gate<'a>),
}

#[derive(Clone, Copy)]
struct Gate<'a> {
    op: &'a str,
    in1: &'a str,
    in2: &'a str,
}

struct Circuit<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

fn parse_init(input: &str) -> IResult<&str, (&str, Node)> {
    separated_pair(alphanumeric1, tag(": "), map(u8, Node::Value))(input)
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    map(
        tuple((
            alphanumeric1,
            multispace1,
            alpha1,
            multispace1,
            alphanumeric1,
        )),
        |(in1, _, op, _, in2)| Gate { op, in1, in2 },
    )(input)
}

fn parse_gate_node(input: &str) -> IResult<&str, (&str, Node)> {
    map(
        separated_pair(parse_gate, tag(" -> "), alphanumeric1),
        |(gate, label)| (label, Node::Gate(gate)),
    )(input)
}

fn parse_circuit(input: &str) -> IResult<&str, Circuit> {
    map(
        separated_pair(
            separated_list1(newline, parse_init),
            tag("\n\n"),
            separated_list1(newline, parse_gate_node),
        ),
        |(inits, gates)| Circuit {
            nodes: HashMap::from_iter(inits.into_iter().chain(gates)),
        },
    )(input)
}

impl<'a> Circuit<'a> {
    fn new(input: &'a str) -> Self {
        parse_circuit(input).unwrap().1
    }

    fn get_value(&mut self, label: &'a str) -> u8 {
        let node = *self.nodes.get(label).unwrap();
        match node {
            Node::Value(b) => b,
            Node::Gate(g) => {
                let val1 = self.get_value(g.in1);
                let val2 = self.get_value(g.in2);
                let val = match g.op {
                    "AND" => val1 & val2,
                    "OR" => val1 | val2,
                    "XOR" => val1 ^ val2,
                    _ => panic!(),
                };
                self.nodes.insert(label, Node::Value(val));
                val
            }
        }
    }

    fn get_output(&mut self) -> u64 {
        self.nodes
            .keys()
            .cloned()
            .filter(|k| k.starts_with('z'))
            .sorted()
            .rev()
            .fold(0, |acc, node| (acc << 1) | self.get_value(node) as u64)
    }

    fn match_op(&self, label: &str, op: &str) -> bool {
        match self.nodes[label] {
            Node::Gate(g) => g.op == op,
            Node::Value(_) => false,
        }
    }

    fn get_ins(&self, label: &str) -> (&str, &str) {
        match self.nodes[label] {
            Node::Gate(g) => (g.in1, g.in2),
            Node::Value(_) => panic!(),
        }
    }
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    Circuit::new(input).get_output()
}

#[aoc(day24, part2)]
fn part2(input: &str) -> String {
    // This is far from exhaustive, but my input only had XORs
    // that were misplaced, so I only check that.
    let circuit = Circuit::new(input);
    let mut res = Vec::new();
    let mut xors = [""; 45];

    for &node in circuit.nodes.keys() {
        // XORs should only be xn ^ yn or have a zxx tag
        if circuit.match_op(node, "XOR") {
            let (in1, in2) = circuit.get_ins(node);
            if in1.starts_with('x') && in2.starts_with('y')
                || in1.starts_with('y') && in2.starts_with('x')
            {
                xors[in1[1..].parse::<usize>().unwrap()] = node;
            } else if !node.starts_with('z') {
                // Those should be zxx
                res.push(node.to_string());
            }
        }
    }
    if xors[0] != "z00" {
        res.push("z00".to_string());
        res.push(xors[0].to_string());
    }
    for (i, &xor) in xors.iter().enumerate().skip(1) {
        let z_node = format!("z{:02}", i);
        if !circuit.match_op(&z_node, "XOR") {
            // A zxx should be a XOR (except for z45)
            // The correct node should already be in the list after
            // the first pass
            res.push(z_node);
            continue;
        }
        let (in1, in2) = circuit.get_ins(&z_node);
        if in1 != xor && in2 != xor {
            // A zxx should have xors[i] as one of its member
            res.push(xors[i].to_string());
            // Assume that the correct member is the one with OR (the carry)
            // (zxx is xors[i] XOR carry)
            if circuit.match_op(in1, "OR") {
                res.push(in2.to_string());
            } else {
                res.push(in1.to_string());
            }
        }
    }
    res.iter().sorted().join(",")
}
