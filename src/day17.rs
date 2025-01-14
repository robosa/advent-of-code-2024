use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{u64, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::VecDeque;

struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
    instr: usize,
    output: Vec<u8>,
}

fn parse_computer(input: &str) -> IResult<&str, Computer> {
    map(
        tuple((
            preceded(tag("Register A: "), u64),
            preceded(tag("\nRegister B: "), u64),
            preceded(tag("\nRegister C: "), u64),
            preceded(tag("\n\nProgram: "), separated_list1(tag(","), u8)),
        )),
        |(reg_a, reg_b, reg_c, program)| Computer {
            reg_a,
            reg_b,
            reg_c,
            program,
            instr: 0,
            output: Vec::new(),
        },
    )(input)
}

impl Computer {
    fn new(input: &str) -> Self {
        parse_computer(input).unwrap().1
    }

    fn reset(&mut self, reg_a: u64, reg_b: u64, reg_c: u64) {
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;
        self.instr = 0;
        self.output = Vec::new();
    }

    fn get_combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.reg_a >>= self.get_combo(operand);
        self.instr += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.reg_b ^= operand as u64;
        self.instr += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.reg_b = self.get_combo(operand) & 7;
        self.instr += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.reg_a != 0 {
            self.instr = operand as usize;
        } else {
            self.instr += 2;
        };
    }

    fn bxc(&mut self, _operand: u8) {
        self.reg_b ^= self.reg_c;
        self.instr += 2;
    }

    fn out(&mut self, operand: u8) {
        self.output.push((self.get_combo(operand) & 7) as u8);
        self.instr += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.reg_b = self.reg_a >> self.get_combo(operand);
        self.instr += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.reg_c = self.reg_a >> self.get_combo(operand);
        self.instr += 2;
    }

    fn execute(&mut self) {
        while let Some(instr) = self.program.get(self.instr) {
            let operand = self.program[self.instr + 1];
            match instr {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!(),
            }
        }
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let mut computer = Computer::new(input);
    computer.execute();
    computer.output.into_iter().join(",")
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u64 {
    let mut computer = Computer::new(input);
    let (reg_b, reg_c) = (computer.reg_b, computer.reg_c);
    let mut queue = VecDeque::from([(computer.program.len() - 1, 0)]);
    while let Some((i, mut reg_a)) = queue.pop_front() {
        for _ in 0..8 {
            computer.reset(reg_a, reg_b, reg_c);
            computer.execute();
            if computer.output.get(i) == computer.program.get(i) {
                if i == 0 {
                    return reg_a;
                }
                queue.push_back((i - 1, reg_a));
            }
            reg_a += 1 << (3 * i);
        }
    }
    panic!()
}
