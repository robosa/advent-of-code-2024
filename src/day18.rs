use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashSet, VecDeque};

fn parse_bytes(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
    separated_list1(newline, separated_pair(u8, tag(","), u8))(input)
}

fn neighbors((x, y): (u8, u8)) -> Vec<(u8, u8)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x < 70 {
        neighbors.push((x + 1, y));
    }
    if y < 70 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(u8, u8)> {
    parse_bytes(input).unwrap().1
}

#[aoc(day18, part1)]
fn part1(input: &[(u8, u8)]) -> usize {
    let bytes: HashSet<_> = HashSet::from_iter(&input[..1024]);
    let mut queue = VecDeque::from([(0, (0, 0))]);
    let mut visited = HashSet::new();
    while let Some((cost, pos)) = queue.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        if pos == (70, 70) {
            return cost;
        }
        for neighbor in neighbors(pos) {
            if !bytes.contains(&neighbor) {
                queue.push_back((cost + 1, neighbor));
            }
        }
    }
    panic!()
}

#[aoc(day18, part2)]
fn part2(input: &[(u8, u8)]) -> String {
    let mut bytes: HashSet<_> = HashSet::from_iter(input);
    let mut stack = vec![(0, 0)];
    let mut visited = HashSet::new();
    for byte in input.iter().rev() {
        bytes.remove(byte);
        for neighbor in neighbors(*byte) {
            if visited.contains(&neighbor) {
                stack.push(*byte);
                break;
            }
        }
        while let Some(pos) = stack.pop() {
            if !visited.insert(pos) {
                continue;
            }
            if pos == (70, 70) {
                return format!("{},{}", byte.0, byte.1);
            }
            for neighbor in neighbors(pos) {
                if !bytes.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }
    panic!()
}
