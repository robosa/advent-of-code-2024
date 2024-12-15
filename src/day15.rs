use aoc_runner_derive::aoc;

struct Grid {
    data: Vec<Vec<char>>,
    robot: (usize, usize),
}

fn update_pos((x, y): (usize, usize), dir: char) -> (usize, usize) {
    match dir {
        'v' => (x + 1, y),
        '>' => (x, y + 1),
        '^' => (x - 1, y),
        '<' => (x, y - 1),
        _ => panic!(),
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut robot: (usize, usize) = (0, 0);
        let data = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '@' => {
                            robot = (i, j);
                            '.'
                        }
                        c => c,
                    })
                    .collect()
            })
            .collect();
        Self { data, robot }
    }

    fn new_wide(input: &str) -> Self {
        let mut robot: (usize, usize) = (0, 0);
        let data = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .flat_map(|(j, c)| match c {
                        '@' => {
                            robot = (i, 2 * j);
                            ['.', '.']
                        }
                        'O' => ['[', ']'],
                        c => [c, c],
                    })
                    .collect()
            })
            .collect();
        Self { data, robot }
    }

    fn push_standard_crate(&mut self, (x, y): (usize, usize), dir: char) {
        let (nx, ny) = update_pos((x, y), dir);
        match self.data[nx][ny] {
            'O' | '[' | ']' => self.push_standard_crate((nx, ny), dir),
            _ => {}
        }
        if self.data[nx][ny] == '.' {
            self.data[nx][ny] = self.data[x][y];
            self.data[x][y] = '.';
        }
    }

    fn can_push_wide_crate_up_down(&self, (x, y): (usize, usize), dir: char) -> bool {
        let (nx, ny) = update_pos((x, y), dir);
        !(self.data[nx][ny] == '#'
            || self.data[nx][ny + 1] == '#'
            || self.data[nx][ny] == '[' && !self.can_push_wide_crate_up_down((nx, ny), dir)
            || self.data[nx][ny] == ']' && !self.can_push_wide_crate_up_down((nx, ny - 1), dir)
            || self.data[nx][ny + 1] == '[' && !self.can_push_wide_crate_up_down((nx, ny + 1), dir))
    }

    fn force_push_wide_crate_up_down(&mut self, (x, y): (usize, usize), dir: char) {
        let (nx, ny) = update_pos((x, y), dir);
        if self.data[nx][ny] == '[' {
            self.force_push_wide_crate_up_down((nx, ny), dir);
        }
        if self.data[nx][ny] == ']' {
            self.force_push_wide_crate_up_down((nx, ny - 1), dir);
        }
        if self.data[nx][ny + 1] == '[' {
            self.force_push_wide_crate_up_down((nx, ny + 1), dir);
        }
        self.data[x][y] = '.';
        self.data[x][y + 1] = '.';
        self.data[nx][ny] = '[';
        self.data[nx][ny + 1] = ']';
    }

    fn move_robot(&mut self, dir: char) {
        let (nx, ny) = update_pos(self.robot, dir);
        match self.data[nx][ny] {
            '[' if (dir == 'v' || dir == '^') => {
                if self.can_push_wide_crate_up_down((nx, ny), dir) {
                    self.force_push_wide_crate_up_down((nx, ny), dir)
                }
            }
            ']' if (dir == 'v' || dir == '^') => {
                if self.can_push_wide_crate_up_down((nx, ny - 1), dir) {
                    self.force_push_wide_crate_up_down((nx, ny - 1), dir)
                }
            }
            'O' | '[' | ']' => self.push_standard_crate((nx, ny), dir),
            _ => {}
        }
        if self.data[nx][ny] == '.' {
            self.robot = (nx, ny)
        }
    }

    fn sum_gps(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                r.iter().enumerate().filter_map(move |(j, c)| match c {
                    'O' | '[' => Some(100 * i + j),
                    _ => None,
                })
            })
            .sum()
    }

    fn run(&mut self, instructions: &str) {
        instructions
            .lines()
            .flat_map(str::chars)
            .for_each(|dir| self.move_robot(dir));
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    let (grid_data, instructions) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::new(grid_data);
    grid.run(instructions);
    grid.sum_gps()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let (grid_data, instructions) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::new_wide(grid_data);
    grid.run(instructions);
    grid.sum_gps()
}

