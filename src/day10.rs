use aoc_runner_derive::aoc;
use std::collections::HashSet;

type Trails = (HashSet<(usize, usize)>, usize);

struct Map {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
    starts: Vec<(usize, usize)>,
    memo: Vec<Vec<Option<Trails>>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut starts = Vec::new();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let val = c.to_digit(10).unwrap();
                        if val == 0 {
                            starts.push((i, j));
                        }
                        val
                    })
                    .collect()
            })
            .collect();
        Self {
            grid,
            height,
            width,
            starts,
            memo: vec![vec![None; width]; height],
        }
    }

    fn get_neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let val = self.grid[i][j];
        let mut res = Vec::new();
        if i < self.height - 1 && self.grid[i + 1][j] == val + 1 {
            res.push((i + 1, j));
        }
        if j < self.width - 1 && self.grid[i][j + 1] == val + 1 {
            res.push((i, j + 1));
        }
        if i > 0 && self.grid[i - 1][j] == val + 1 {
            res.push((i - 1, j));
        }
        if j > 0 && self.grid[i][j - 1] == val + 1 {
            res.push((i, j - 1));
        }
        res
    }

    fn compute_trails(&mut self, pos: (usize, usize)) -> Trails {
        let (i, j) = pos;
        if let Some(res) = &self.memo[i][j] {
            return res.clone();
        }
        let height = self.grid[i][j];
        let mut dests = HashSet::new();
        let mut trails = 0;
        if height == 9 {
            dests.insert((i, j));
            trails = 1;
        } else {
            for &neighbor in self.get_neighbors(pos).iter() {
                let (n_dests, n_trails) = self.compute_trails(neighbor);
                dests = dests.union(&n_dests).copied().collect();
                trails += n_trails;
            }
        }
        self.memo[i][j] = Some((dests.clone(), trails));
        (dests, trails)
    }

    fn count_dests(&mut self) -> usize {
        self.starts
            .clone()
            .into_iter()
            .map(|pos| self.compute_trails(pos).0.len())
            .sum()
    }

    fn count_trails(&mut self) -> usize {
        self.starts
            .clone()
            .into_iter()
            .map(|pos| self.compute_trails(pos).1)
            .sum()
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    Map::new(input).count_dests()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    Map::new(input).count_trails()
}
