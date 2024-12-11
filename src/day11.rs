use std::collections::HashMap;

use aoc_runner_derive::aoc;

enum TransformedStone {
    One(u64),
    Two((u64, u64)),
}

fn transform(stone: u64) -> TransformedStone {
    match (stone, stone.to_string()) {
        (0, _) => TransformedStone::One(1),
        (_, s) if s.len() % 2 == 0 => {
            let (s1, s2) = s.split_at(s.len() / 2);
            TransformedStone::Two((s1.parse().unwrap(), s2.parse().unwrap()))
        }
        _ => TransformedStone::One(stone * 2024),
    }
}

fn count_stones(stone: u64, steps: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(res) = memo.get(&(stone, steps)) {
        return *res;
    }
    let res = match transform(stone) {
        TransformedStone::One(s) => count_stones(s, steps - 1, memo),
        TransformedStone::Two((s1, s2)) => {
            count_stones(s1, steps - 1, memo) + count_stones(s2, steps - 1, memo)
        }
    };
    memo.insert((stone, steps), res);
    res
}

fn run(input: &str, steps: usize) -> usize {
    let mut memo = HashMap::new();
    input
        .split(' ')
        .map(|s| count_stones(s.parse().unwrap(), steps, &mut memo))
        .sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    run(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    run(input, 75)
}
