use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|c| c.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[u8]) -> bool {
    let is_level_safe = |a, b| b > a && b - a < 4;
    report.windows(2).all(|p| is_level_safe(p[0], p[1]))
        || report.windows(2).all(|p| is_level_safe(p[1], p[0]))
}

fn is_safe_with_skip(report: &[u8]) -> bool {
    (0..report.len())
        .map(|i| [&report[..i], &report[i + 1..]].concat())
        .any(|r| is_safe(&r))
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    input.iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    input.iter().filter(|r| is_safe_with_skip(r)).count()
}
