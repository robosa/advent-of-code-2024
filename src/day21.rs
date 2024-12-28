use aoc_runner_derive::aoc;
use std::collections::HashMap;

trait Pad {
    fn get_pos(&self, key: char) -> (i8, i8);
    fn get_state(&self) -> char;
    fn set_state(&mut self, key: char);
    fn is_gap(&self, pos: (i8, i8)) -> bool;

    fn press_key(&mut self, key: char) -> Vec<char> {
        let (sx, sy) = self.get_pos(self.get_state());
        let (ex, ey) = self.get_pos(key);
        self.set_state(key);
        let (dx, dy) = (ex - sx, ey - sy);
        let ver = if dx >= 0 { 'v' } else { '^' };
        let hor = if dy >= 0 { '>' } else { '<' };
        let mut ver_vec = vec![ver; dx.unsigned_abs().into()];
        let mut hor_vec = vec![hor; dy.unsigned_abs().into()];
        // We put '<' first if it is here and we don't cross the gap
        // Or '>' if we can't put '^'/'v' first
        if dy < 0 && !self.is_gap((sx, sy + dy)) || self.is_gap((sx + dx, sy)) {
            hor_vec.append(&mut ver_vec);
            hor_vec.push('A');
            hor_vec
        } else {
            ver_vec.append(&mut hor_vec);
            ver_vec.push('A');
            ver_vec
        }
    }
}

struct Numpad {
    state: char,
}

impl Pad for Numpad {
    fn get_pos(&self, key: char) -> (i8, i8) {
        match key {
            '0' => (3, 1),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            'A' => (3, 2),
            _ => panic!(),
        }
    }

    fn get_state(&self) -> char {
        self.state
    }

    fn set_state(&mut self, key: char) {
        self.state = key;
    }

    fn is_gap(&self, (x, y): (i8, i8)) -> bool {
        x == 3 && y == 0
    }
}

impl Numpad {
    fn new() -> Self {
        Self { state: 'A' }
    }

    fn input_code(&mut self, keys: &str) -> Vec<char> {
        keys.chars().flat_map(|k| self.press_key(k)).collect()
    }
}

struct Dirpad {
    state: char,
    cache: HashMap<(char, char), Vec<char>>,
}

impl Pad for Dirpad {
    fn get_pos(&self, key: char) -> (i8, i8) {
        match key {
            '<' => (1, 0),
            '>' => (1, 2),
            '^' => (0, 1),
            'v' => (1, 1),
            'A' => (0, 2),
            _ => panic!(),
        }
    }

    fn get_state(&self) -> char {
        self.state
    }

    fn set_state(&mut self, key: char) {
        self.state = key;
    }

    fn is_gap(&self, (x, y): (i8, i8)) -> bool {
        x == 0 && y == 0
    }
}

impl Dirpad {
    fn new() -> Self {
        Self {
            state: 'A',
            cache: HashMap::new(),
        }
    }

    fn press_key_cached(&mut self, key: char) -> Vec<char> {
        if let Some(res) = self.cache.get(&(self.state, key)) {
            self.state = key;
            return res.to_vec();
        }
        let state = self.state;
        let res = self.press_key(key);
        self.cache.insert((state, key), res.clone());
        res
    }

    fn seq_len(
        &mut self,
        keys: &[char],
        level: usize,
        cache: &mut HashMap<(char, char, usize), usize>,
    ) -> usize {
        if level == 0 {
            return keys.len();
        }
        self.state = 'A';
        let mut seq_len = 0;
        for &k in keys {
            if let Some(res) = cache.get(&(self.state, k, level)) {
                seq_len += res;
                self.state = k;
                continue;
            }
            let state = self.state;
            let new_seq = self.press_key_cached(k);
            let new_len = self.seq_len(&new_seq, level - 1, cache);
            cache.insert((state, k, level), new_len);
            seq_len += new_len;
            self.state = k;
        }
        seq_len
    }
}

fn solve(input: &str, level: usize) -> usize {
    let mut keypad = Numpad::new();
    let mut dirpad = Dirpad::new();
    let mut cache = HashMap::new();
    let mut complexity = 0;
    for code in input.lines() {
        let seq = keypad.input_code(code);
        let len = dirpad.seq_len(&seq, level, &mut cache);
        let code_val: usize = code[..3].parse().unwrap();
        complexity += len * code_val;
    }
    complexity
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    solve(input, 25)
}
