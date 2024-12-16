use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos((usize, usize), Dir);

fn step_in_dir(&Pos((x, y), dir): &Pos) -> (usize, usize) {
    match dir {
        Dir::North => (x - 1, y),
        Dir::East => (x, y + 1),
        Dir::South => (x + 1, y),
        Dir::West => (x, y - 1),
    }
}

fn turn_dirs(dir: Dir) -> [Dir; 2] {
    match dir {
        Dir::North | Dir::South => [Dir::West, Dir::East],
        Dir::East | Dir::West => [Dir::North, Dir::South],
    }
}

#[derive(Clone, Copy, Eq)]
struct State {
    cost: usize,
    pos: Pos,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Grid {
            data: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn get_next_states(&self, state: &State) -> Vec<State> {
        let mut next_states = Vec::new();
        let (nx, ny) = step_in_dir(&state.pos);
        if self.data[nx][ny] != '#' {
            next_states.push(State {
                cost: state.cost + 1,
                pos: Pos((nx, ny), state.pos.1),
            });
        }
        for ndir in turn_dirs(state.pos.1) {
            next_states.push(State {
                cost: state.cost + 1000,
                pos: Pos(state.pos.0, ndir),
            });
        }
        next_states
    }

    fn walk(&self) -> (usize, usize) {
        let mut costs = HashMap::new();
        let mut preds: HashMap<_, HashSet<_>> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut best = usize::MAX;
        let mut ends = HashSet::new();
        let start_state = State {
            cost: 0,
            pos: Pos((self.data.len() - 2, 1), Dir::East),
        };
        heap.push(start_state);
        costs.insert(start_state.pos, start_state.cost);
        while let Some(state) = heap.pop() {
            let Pos((x, y), _) = state.pos;
            if self.data[x][y] == 'E' {
                best = state.cost;
                ends.insert(state.pos);
                continue;
            }
            for next_state in self.get_next_states(&state) {
                let &current_best = costs.get(&next_state.pos).unwrap_or(&best);
                if next_state.cost > current_best {
                    continue;
                }
                heap.push(next_state);
                if next_state.cost == current_best {
                    preds.entry(next_state.pos).or_default().insert(state.pos);
                } else {
                    costs.insert(next_state.pos, next_state.cost);
                    preds.insert(next_state.pos, HashSet::from([state.pos]));
                }
            }
        }
        (best, walk_back(&preds, &ends))
    }
}

fn walk_back(preds_map: &HashMap<Pos, HashSet<Pos>>, ends: &HashSet<Pos>) -> usize {
    let mut valid_pos = HashSet::new();
    let mut stack = Vec::from_iter(ends);
    while let Some(pos) = stack.pop() {
        if !valid_pos.insert(pos) {
            continue;
        }
        if let Some(preds) = preds_map.get(pos) {
            stack.extend(preds);
        }
    }
    valid_pos.iter().map(|&&Pos(loc, _)| loc).unique().count()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    Grid::new(input).walk().0
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    Grid::new(input).walk().1
}
