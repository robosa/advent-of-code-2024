use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn turn(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

struct Grid {
    height: usize,
    width: usize,
    obstacles: HashSet<(usize, usize)>,
    start: (usize, usize),
}

impl Grid {
    fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut start = (0, 0);
        let mut obstacles = HashSet::new();
        for (i, l) in input.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                match c {
                    '^' => start = (i, j),
                    '#' => {
                        obstacles.insert((i, j));
                    }
                    _ => (),
                }
            }
        }
        Self {
            height,
            width,
            obstacles,
            start,
        }
    }

    fn advance(&self, pos: (usize, usize), dir: Direction) -> Option<((usize, usize), Direction)> {
        let (i, j) = pos;
        match dir {
            Direction::Up if i == 0 => None,
            Direction::Right if j == self.width - 1 => None,
            Direction::Down if i == self.height - 1 => None,
            Direction::Left if j == 0 => None,

            Direction::Up if self.obstacles.contains(&(i - 1, j)) => self.advance(pos, turn(dir)),
            Direction::Right if self.obstacles.contains(&(i, j + 1)) => {
                self.advance(pos, turn(dir))
            }
            Direction::Down if self.obstacles.contains(&(i + 1, j)) => self.advance(pos, turn(dir)),
            Direction::Left if self.obstacles.contains(&(i, j - 1)) => self.advance(pos, turn(dir)),

            Direction::Up => Some(((i - 1, j), dir)),
            Direction::Right => Some(((i, j + 1), dir)),
            Direction::Down => Some(((i + 1, j), dir)),
            Direction::Left => Some(((i, j - 1), dir)),
        }
    }

    fn has_loop(&self, pos: (usize, usize), dir: Direction) -> bool {
        let mut current = Some((pos, dir));
        let mut visited = HashSet::new();
        while let Some((p, d)) = current {
            if !visited.insert((p, d)) {
                return true;
            }
            current = self.advance(p, d);
        }
        false
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut current = Some((grid.start, Direction::Up));
    let mut visited = HashSet::new();
    while let Some((p, d)) = current {
        visited.insert(p);
        current = grid.advance(p, d);
    }
    visited.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut pos = grid.start;
    let mut dir = Direction::Up;
    let mut next = grid.advance(pos, dir);
    let mut visited = HashSet::from([pos]);
    let mut count = 0;
    while let Some((next_pos, next_dir)) = next {
        if visited.insert(next_pos) && grid.obstacles.insert(next_pos) {
            if grid.has_loop(pos, dir) {
                count += 1;
            }
            grid.obstacles.remove(&next_pos);
        }
        (pos, dir) = (next_pos, next_dir);
        next = grid.advance(pos, dir);
    }
    count
}
