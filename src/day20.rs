use aoc_runner_derive::aoc;
use std::collections::{HashSet, VecDeque};

struct Grid {
    cells: Vec<Vec<(char, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
    best: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => start = (i, j),
                    'E' => end = (i, j),
                    _ => {}
                }
                row.push((c, usize::MAX));
            }
            cells.push(row);
        }
        Self {
            cells,
            start,
            end,
            best: usize::MAX,
        }
    }

    fn bfs_end(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back((0, self.end));
        let mut visited = HashSet::new();
        while let Some((d, (x, y))) = queue.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }
            self.cells[x][y] = (self.cells[x][y].0, d);
            if (x, y) == self.start {
                self.best = d;
            }
            for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                if self.cells[nx][ny].0 != '#' {
                    queue.push_back((d + 1, (nx, ny)));
                }
            }
        }
    }

    fn count_cheats(&self, duration: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((0, self.start));
        let mut visited = HashSet::new();
        let mut res = 0;
        while let Some((d, (x, y))) = queue.pop_front() {
            if !visited.insert((x, y)) || d >= self.best || (x, y) == self.end {
                continue;
            }
            res += self.check_cheats((x, y), d, duration);
            for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                if self.cells[nx][ny].0 != '#' {
                    queue.push_back((d + 1, (nx, ny)));
                }
            }
        }
        res
    }

    fn check_cheats(&self, (x, y): (usize, usize), dist: usize, duration: usize) -> usize {
        let mut res = 0;
        for i in x.saturating_sub(duration)..=(x + duration).min(self.cells.len() - 1) {
            let dur_x = x.abs_diff(i);
            let rem_dur = duration - dur_x;
            for j in y.saturating_sub(rem_dur)..=(y + rem_dur).min(self.cells[0].len() - 1) {
                let (c, de) = self.cells[i][j];
                if c != '#' && dist + dur_x + y.abs_diff(j) + de <= self.best - 100 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.bfs_end();
    grid.count_cheats(2)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.bfs_end();
    grid.count_cheats(20)
}
