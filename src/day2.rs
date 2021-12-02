use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    error::{context, VerboseError},
    IResult,
};
use std::fmt;

// A wrapper around IResult for our custom parser
type Res<T, U> = IResult<T, U, VerboseError<T>>;

// Parse a number like 42
fn parse_digit(input: &str) -> Res<&str, i64> {
    map_res(recognize(digit1), str::parse)(input)
}

trait Semigroup {
    fn mappend(_: Self, _: Self) -> Self;
}

trait Monoid {
    fn mempty() -> Self;
}

#[derive(Debug)]
struct Position {
    horizontal: i64,
    depth: i64,
}

impl Semigroup for Position {
    fn mappend(fst: Position, snd: Position) -> Position {
        Position {
            horizontal: fst.horizontal + snd.horizontal,
            depth: fst.depth + snd.depth,
        }
    }
}

impl Monoid for Position {
    fn mempty() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Position(horizontal: {}, depth: {})",
            self.horizontal, self.depth
        )
    }
}

// Basic structure from https://blog.logrocket.com/parsing-in-rust-with-nom
#[derive(Debug)]
enum Movement {
    Forward,
    Up,
    Down,
}

impl From<&str> for Movement {
    fn from(i: &str) -> Self {
        match i.to_owned().as_str() {
            "forward" => Movement::Forward,
            "up" => Movement::Up,
            "down" => Movement::Down,
            _ => unimplemented!("The only acceptable movements are: forward, up, or down"),
        }
    }
}

fn parse_movement(input: &str) -> Res<&str, Movement> {
    context(
        "movement",
        // Can use tag_no_case for case insensitive match
        alt((tag("forward"), tag("up"), tag("down"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn position_parser(input: &str) -> Res<&str, Position> {
    let (input, movement) = parse_movement(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, value) = parse_digit(input)?;
    match movement {
        Movement::Forward => Ok((
            input,
            Position {
                horizontal: value,
                depth: 0,
            },
        )),
        Movement::Up => Ok((
            input,
            Position {
                horizontal: 0,
                depth: -value,
            },
        )),
        Movement::Down => Ok((
            input,
            Position {
                horizontal: 0,
                depth: value,
            },
        )),
    }
}

fn aim_parser(input: &str) -> Res<&str, Aim> {
    let (input, movement) = parse_movement(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, value) = parse_digit(input)?;
    match movement {
        Movement::Forward => Ok((
            input,
            Aim {
                horizontal: value,
                depth: 0,
                aim: value,
            },
        )),
        Movement::Up => Ok((
            input,
            Aim {
                horizontal: 0,
                depth: 0,
                aim: -value,
            },
        )),
        Movement::Down => Ok((
            input,
            Aim {
                horizontal: 0,
                depth: 0,
                aim: value,
            },
        )),
    }
}

#[derive(Debug)]
struct Aim {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Semigroup for Aim {
    fn mappend(fst: Aim, snd: Aim) -> Aim {
        let is_horizontal = snd.horizontal > 0;
        Aim {
            horizontal: fst.horizontal + snd.horizontal,
            depth: if is_horizontal {
                fst.depth + fst.aim * snd.horizontal
            } else {
                fst.depth
            },
            aim: if is_horizontal {
                fst.aim
            } else {
                fst.aim + snd.aim
            },
        }
    }
}

impl Monoid for Aim {
    fn mempty() -> Aim {
        Aim {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl fmt::Display for Aim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Aim(horizontal: {}, depth: {}, aim: {})",
            self.horizontal, self.depth, self.aim
        )
    }
}

fn part1() -> Position {
    let lines: Vec<&str> = include_str!("../inputs/day2.txt")
        .trim_end()
        .split("\n")
        .collect();
    lines
        .iter()
        .map(|s| {
            // This unwrap isn't great but we know it should succeed
            let (_, pos) = position_parser(s).unwrap();
            pos
        })
        .fold(Monoid::mempty(), |x, y| Semigroup::mappend(x, y))
}

fn part2() -> Aim {
    let lines: Vec<&str> = include_str!("../inputs/day2.txt")
        .trim_end()
        .split("\n")
        .collect();
    lines
        .iter()
        .map(|s| {
            // This unwrap isn't great but we know it should succeed
            let (_, pos) = aim_parser(s).unwrap();
            pos
        })
        .fold(Monoid::mempty(), |x, y| Semigroup::mappend(x, y))
}

pub fn run() {
    let Position {
        horizontal: h_one,
        depth: d_one,
    } = part1();
    let Aim {
        horizontal: h_two,
        depth: d_two,
        aim: _,
    } = part2();
    println!("Day 2, Part 1: {}", h_one * d_one);
    println!("Day 2, Part 2: {}", h_two * d_two);
}
