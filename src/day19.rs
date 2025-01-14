use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(", "), alpha1),
        tag("\n\n"),
        separated_list1(newline, alpha1),
    )(input)
}

fn is_design_valid<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&res) = cache.get(design) {
        return res;
    }
    let res = patterns.iter().any(|pattern| {
        design.starts_with(pattern) && is_design_valid(&design[pattern.len()..], patterns, cache)
    });
    cache.insert(design, res);
    res
}

fn count_valid_designs<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&res) = cache.get(design) {
        return res;
    }
    let res = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| count_valid_designs(&design[pattern.len()..], patterns, cache))
        .sum();
    cache.insert(design, res);
    res
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (patterns, designs) = parse(input).unwrap().1;
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|design| is_design_valid(design, &patterns, &mut cache))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (patterns, designs) = parse(input).unwrap().1;
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count_valid_designs(design, &patterns, &mut cache))
        .sum()
}
