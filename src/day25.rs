use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::iter::zip;

fn parse_schematics(schematics: &str) -> [u8; 5] {
    let mut heights = [0; 5];
    for line in schematics.lines().skip(1).take(5) {
        for (i, _) in line.chars().enumerate().filter(|&(_, c)| c == '#') {
            heights[i] += 1;
        }
    }
    heights
}

fn are_compatibles(key: [u8; 5], lock: [u8; 5]) -> bool {
    zip(key, lock).all(|(k, l)| k + l <= 5)
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schematics in input.split("\n\n") {
        if schematics.starts_with("#####") {
            locks.push(parse_schematics(schematics));
        } else {
            keys.push(parse_schematics(schematics));
        }
    }
    keys.into_iter()
        .cartesian_product(locks)
        .filter(|&(k, l)| are_compatibles(k, l))
        .count()
}
