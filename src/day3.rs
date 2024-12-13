use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    part1(
        &input
            .split("do()")
            .map(|s| s.split_once("don't()").unzip().0.unwrap_or(s))
            .collect::<Vec<_>>()
            .concat(),
    )
}
