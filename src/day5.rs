use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::HashSet;

struct PageOrder {
    pairs: HashSet<(u32, u32)>,
}

impl PageOrder {
    fn new(input: &str) -> Self {
        let mut pairs = HashSet::new();
        for (a, b) in input.lines().map(|l| l.split_once('|').unwrap()) {
            pairs.insert((a.parse().unwrap(), b.parse().unwrap()));
        }
        PageOrder { pairs }
    }

    fn is_ordered(&self, update: &[u32]) -> bool {
        update
            .windows(2)
            .all(|p| self.pairs.contains(&(p[0], p[1])))
    }

    fn cmp(&self, a: u32, b: u32) -> Ordering {
        if self.pairs.contains(&(a, b)) {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

fn parse_update(input: &str) -> Vec<u32> {
    input.split(',').map(|p| p.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let order = PageOrder::new(rules);
    updates
        .lines()
        .map(parse_update)
        .filter(|u| order.is_ordered(u))
        .map(|u| u[(u.len() - 1) / 2])
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let order = PageOrder::new(rules);
    updates
        .lines()
        .map(parse_update)
        .filter(|u| !order.is_ordered(u))
        .map(|mut u| {
            u.sort_by(|&a, &b| order.cmp(a, b));
            u[(u.len() - 1) / 2]
        })
        .sum()
}
