use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn split_line(line: &str) -> (u32, u32) {
    line.split("   ")
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn get_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(split_line).unzip()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut l1, mut l2) = get_input(input);
    l1.sort();
    l2.sort();
    l1.iter().zip(l2).fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (l1, l2) = get_input(input);
    let mut counter = HashMap::new();
    l2.iter().for_each(|&a| *counter.entry(a).or_default() += 1);
    l1.iter()
        .fold(0, |acc, a| acc + a * counter.get(a).unwrap_or(&0))
}
