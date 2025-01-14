use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

fn transform(secret: &u64) -> Option<u64> {
    let step_one = ((secret << 6) ^ secret) & 16777215;
    let step_two = (step_one >> 5) ^ step_one;
    Some(((step_two << 11) ^ step_two) & 16777215)
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Option<u64>> {
    input.lines().map(|s| s.parse().ok()).collect()
}

fn get_2000th(secret: &Option<u64>) -> u64 {
    successors(*secret, transform).nth(2000).unwrap()
}

#[aoc(day22, part1)]
fn part1(input: &[Option<u64>]) -> u64 {
    input.iter().map(get_2000th).sum()
}

fn scan_prices(last_price: &mut i8, new_price: i8) -> Option<(i8, i8)> {
    let diff = new_price - *last_price;
    *last_price = new_price;
    Some((new_price, diff))
}

#[aoc(day22, part2)]
fn part2(input: &[Option<u64>]) -> u16 {
    let mut sequences_total = HashMap::new();
    for &secret in input {
        let mut seen = HashSet::new();
        successors(secret, transform)
            .skip(1)
            .take(2000)
            .map(|s| (s % 10) as i8)
            .scan((secret.unwrap() % 10) as i8, scan_prices)
            .tuple_windows()
            .for_each(|((_, a), (_, b), (_, c), (price, d))| {
                let seq = (a, b, c, d);
                if seen.insert(seq) {
                    *sequences_total.entry(seq).or_default() += price as u16;
                }
            });
    }
    *sequences_total.values().max().unwrap()
}
