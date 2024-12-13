use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Grid {
    height: usize,
    width: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (i, l) in input.lines().enumerate() {
            for (j, c) in l.chars().enumerate().filter(|&(_, c)| c != '.') {
                antennas.entry(c).or_default().push((i, j));
            }
        }
        Self {
            height,
            width,
            antennas,
        }
    }

    fn antinode(&self, first: (usize, usize), second: (usize, usize)) -> Option<(usize, usize)> {
        let ((x1, y1), (x2, y2)) = (first, second);
        if (2 * x2 < x1) || (2 * y2 < y1) {
            return None;
        }
        match (2 * x2 - x1, 2 * y2 - y1) {
            (x, y) if x < self.height && y < self.width => Some((x, y)),
            _ => None,
        }
    }

    fn get_first_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();
        for pair in self
            .antennas
            .values()
            .flat_map(|antennas| antennas.iter().permutations(2))
        {
            if let Some(node) = self.antinode(*pair[0], *pair[1]) {
                antinodes.insert(node);
            }
        }
        antinodes
    }

    fn get_all_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();
        for (mut prev, mut current) in self
            .antennas
            .values()
            .flat_map(|antennas| antennas.iter().permutations(2))
            .map(|pair| (*pair[0], Some(*pair[1])))
        {
            while let Some(node) = current {
                antinodes.insert(node);
                (prev, current) = (node, self.antinode(prev, node));
            }
        }
        antinodes
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    Grid::new(input).get_first_antinodes().len()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    Grid::new(input).get_all_antinodes().len()
}
