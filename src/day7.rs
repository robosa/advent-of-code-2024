use aoc_runner_derive::{aoc, aoc_generator};

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(input: &str) -> Self {
        let (r, l) = input.split_once(": ").unwrap();
        Equation {
            result: r.parse().unwrap(),
            numbers: l.split(' ').map(|s| s.parse().unwrap()).collect(),
        }
    }
}

fn is_valid(numbers: &[u64], current: u64, result: u64) -> bool {
    match numbers {
        [] => current == result,
        [h, t @ ..] => is_valid(t, h * current, result) || is_valid(t, h + current, result),
    }
}

fn is_valid2(numbers: &[u64], current: u64, result: u64) -> bool {
    match numbers {
        _ if current > result => false,
        [] => current == result,
        [h, t @ ..] => {
            is_valid2(t, h * current, result)
                || is_valid2(t, h + current, result)
                || is_valid2(t, current * 10u64.pow(h.ilog10() + 1) + h, result)
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::new).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|e| is_valid(&e.numbers[1..], e.numbers[0], e.result))
        .map(|e| e.result)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|e| is_valid2(&e.numbers[1..], e.numbers[0], e.result))
        .map(|e| e.result)
        .sum()
}
