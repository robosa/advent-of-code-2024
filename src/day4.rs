use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        Grid {
            height: data.len(),
            width: data.first().map(|l| l.len()).unwrap_or(0),
            data,
        }
    }

    fn has_xmas_in_dir(&self, start: (usize, usize), dir: (i32, i32)) -> bool {
        let (i, j) = start;
        let (di, dj) = dir;
        "XMAS".chars().enumerate().all(|(k, c)| {
            self.data
                .get((i as i32 + k as i32 * di) as usize)
                .and_then(|l| l.get((j as i32 + k as i32 * dj) as usize))
                == Some(&c)
        })
    }

    fn count_xmas_at(&self, start: (usize, usize)) -> usize {
        DIRS.into_iter()
            .filter(|&dir| self.has_xmas_in_dir(start, dir))
            .count()
    }

    fn has_cross_mas(&self, start: (usize, usize)) -> bool {
        let (i, j) = start;
        if self.data[i][j] != 'A' {
            return false;
        }
        let d1 = (self.data[i - 1][j - 1], self.data[i + 1][j + 1]);
        let d2 = (self.data[i + 1][j - 1], self.data[i - 1][j + 1]);
        (d1 == ('M', 'S') || d1 == ('S', 'M')) && (d2 == ('M', 'S') || d2 == ('S', 'M'))
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    Grid::new(input)
}

#[aoc(day4, part1)]
pub fn part1(input: &Grid) -> usize {
    (0..input.height)
        .cartesian_product(0..input.width)
        .map(|start| input.count_xmas_at(start))
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &Grid) -> usize {
    (1..input.height - 1)
        .cartesian_product(1..input.width - 1)
        .filter(|&start| input.has_cross_mas(start))
        .count()
}
