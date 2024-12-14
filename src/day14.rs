use std::{cmp::Ordering, collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline, space1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn step_n(&self, n: i32) -> (i32, i32) {
        (
            (self.pos.0 + n * self.vel.0).rem_euclid(101),
            (self.pos.1 + n * self.vel.1).rem_euclid(103),
        )
    }
}

fn get_quadrant((x, y): (i32, i32)) -> Option<usize> {
    match (x.cmp(&50), y.cmp(&51)) {
        (Ordering::Less, Ordering::Less) => Some(0),
        (Ordering::Less, Ordering::Greater) => Some(1),
        (Ordering::Greater, Ordering::Less) => Some(2),
        (Ordering::Greater, Ordering::Greater) => Some(3),
        _ => None,
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        separated_pair(
            preceded(tag("p="), separated_pair(i32, tag(","), i32)),
            space1,
            preceded(tag("v="), separated_pair(i32, tag(","), i32)),
        ),
        |(pos, vel)| Robot { pos, vel },
    )(input)
}

fn display_grid(positions: HashSet<(i32, i32)>) {
    for j in 0..103 {
        println!(
            "{}",
            (0..101)
                .map(|i| {
                    if positions.contains(&(i, j)) {
                        '*'
                    } else {
                        ' '
                    }
                })
                .join("")
        );
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    separated_list0(newline, parse_robot)(input).unwrap().1
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    let mut quadrants = [0; 4];
    for robot in input {
        if let Some(idx) = get_quadrant(robot.step_n(100)) {
            quadrants[idx] += 1;
        }
    }
    quadrants.iter().product()
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> i32 {
    // Found distinctive patterns at 10 + i * 101 and 70 + i * 103
    let n = (0..)
        .map(|i| i * 101 + 10)
        .find(|&i| (i - 70) % 103 == 0)
        .unwrap();
    display_grid(input.iter().map(|robot| robot.step_n(n)).collect());
    n
}
