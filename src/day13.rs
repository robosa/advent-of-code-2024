use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

// ax*A + bx*B = px
// ay*A + by*B = py
struct System {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse_system(input: &str) -> IResult<&str, System> {
    map(
        tuple((
            preceded(tag("Button A: X+"), i64),
            preceded(tag(", Y+"), i64),
            preceded(tag("\nButton B: X+"), i64),
            preceded(tag(", Y+"), i64),
            preceded(tag("\nPrize: X="), i64),
            preceded(tag(", Y="), i64),
        )),
        |(ax, ay, bx, by, px, py)| System {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        },
    )(input)
}

impl System {
    fn transform(&self) -> Self {
        Self {
            px: self.px + 10_000_000_000_000,
            py: self.py + 10_000_000_000_000,
            ..*self
        }
    }

    fn solve(&self) -> Option<i64> {
        let bn = self.px * self.ay - self.py * self.ax;
        let bd = self.bx * self.ay - self.by * self.ax;
        if bd == 0 || bn % bd != 0 {
            return None;
        }
        let b = bn / bd;
        Some(3 * (self.px - self.bx * b) / self.ax + b)
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<System> {
    separated_list0(multispace1, parse_system)(input).unwrap().1
}

#[aoc(day13, part1)]
fn part1(input: &[System]) -> i64 {
    input.iter().filter_map(System::solve).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[System]) -> i64 {
    input.iter().filter_map(|s| s.transform().solve()).sum()
}
