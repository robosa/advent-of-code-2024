use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = data.len();
        let width = data[0].len();
        Grid {
            data,
            height,
            width,
        }
    }

    fn get_valid_neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let curr_val = self.data[i][j];
        let mut neighbors = Vec::new();
        if i < self.height - 1 && self.data[i + 1][j] == curr_val {
            neighbors.push((i + 1, j));
        }
        if j < self.width - 1 && self.data[i][j + 1] == curr_val {
            neighbors.push((i, j + 1));
        }
        if i > 0 && self.data[i - 1][j] == curr_val {
            neighbors.push((i - 1, j));
        }
        if j > 0 && self.data[i][j - 1] == curr_val {
            neighbors.push((i, j - 1));
        }
        neighbors
    }

    // dir: 0..7 <=> S, E, N, W, SE, NE, NW, SW
    fn get_value_in_dir(&self, pos: (usize, usize), dir: u8) -> Option<char> {
        let (i, j) = pos;
        (match dir {
            0 if i < self.height - 1 => Some((i + 1, j)),
            1 if j < self.height - 1 => Some((i, j + 1)),
            2 if i > 0 => Some((i - 1, j)),
            3 if j > 0 => Some((i, j - 1)),
            4 if i < self.height - 1 && j < self.width - 1 => Some((i + 1, j + 1)),
            5 if i > 0 && j < self.height - 1 => Some((i - 1, j + 1)),
            6 if i > 0 && j > 0 => Some((i - 1, j - 1)),
            7 if i < self.height - 1 && j > 0 => Some((i + 1, j - 1)),
            _ => None,
        })
        .map(|(i, j)| self.data[i][j])
    }

    fn count_corners(&self, pos: (usize, usize)) -> usize {
        let (i, j) = pos;
        let curr_val = self.data[i][j];
        let surrounding_vals: Vec<_> = (0..8).map(|dir| self.get_value_in_dir(pos, dir)).collect();
        (0..4)
            .filter(|&i| {
                (surrounding_vals[i] == Some(curr_val)
                    && surrounding_vals[(i + 1) % 4] == Some(curr_val)
                    && surrounding_vals[i + 4] != Some(curr_val))
                    || (surrounding_vals[i] != Some(curr_val)
                        && surrounding_vals[(i + 1) % 4] != Some(curr_val))
            })
            .count()
    }

    fn get_area_and_edges(
        &self,
        start: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> (usize, usize) {
        if !visited.insert(start) {
            return (0, 0);
        }
        let neighbors = self.get_valid_neighbors(start);
        neighbors
            .iter()
            .fold((1, 4 - neighbors.len()), |(a, p), &n| {
                let (na, np) = self.get_area_and_edges(n, visited);
                (a + na, p + np)
            })
    }

    fn get_area_and_corners(
        &self,
        start: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> (usize, usize) {
        if !visited.insert(start) {
            return (0, 0);
        }
        self.get_valid_neighbors(start)
            .iter()
            .fold((1, self.count_corners(start)), |(a, c), &n| {
                let (na, nc) = self.get_area_and_corners(n, visited);
                (a + na, c + nc)
            })
    }
}

fn run<T>(input: &Grid, method: T) -> usize
where
    T: Fn(&Grid, (usize, usize), &mut HashSet<(usize, usize)>) -> (usize, usize),
{
    let mut visited = HashSet::new();
    (0..input.height)
        .cartesian_product(0..input.width)
        .map(|pos| {
            let (a, p) = method(input, pos, &mut visited);
            a * p
        })
        .sum()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Grid {
    Grid::new(input)
}

#[aoc(day12, part1)]
pub fn part1(input: &Grid) -> usize {
    run(input, Grid::get_area_and_edges)
}

#[aoc(day12, part2)]
pub fn part2(input: &Grid) -> usize {
    run(input, Grid::get_area_and_corners)
}
