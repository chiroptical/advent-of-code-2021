use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    IResult,
};
use std::fmt;

fn parse_digit(input: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), str::parse)(input)
}

#[derive(Debug)]
struct Position {
    horizontal: i64,
    depth: i64,
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

// fn determine_position(lines: &Vec<&str>) -> Position {
//     let mut initial_position = Position(0, 0);
//     lines.iter().map(|s| {
//         match s {
//             "forward N" => ...
//         }
//     });
// }

fn position_parser(input: &str) -> IResult<&str, Position> {
    let (input, forward) = opt(tag("forward "))(input)?;
    let (input, up) = opt(tag("up "))(input)?;
    let (input, down) = opt(tag("down "))(input)?;
    let (input, u) = parse_digit(input)?;
    match (forward, up, down) {
        (Some(_), None, None) => Ok((
            input,
            Position {
                horizontal: u,
                depth: 0,
            },
        )),
        (None, Some(_), None) => Ok((
            input,
            Position {
                horizontal: 0,
                depth: -u,
            },
        )),
        (None, None, Some(_)) => Ok((
            input,
            Position {
                horizontal: 0,
                depth: u,
            },
        )),
        _ => panic!("Unable to parse string {}", input),
    }
}

#[derive(Debug)]
struct Aim {
    horizontal: i64,
    depth: i64,
    aim: i64,
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

fn aim_parser(input: &str) -> IResult<&str, Aim> {
    let (input, forward) = opt(tag("forward "))(input)?;
    let (input, up) = opt(tag("up "))(input)?;
    let (input, down) = opt(tag("down "))(input)?;
    let (input, u) = parse_digit(input)?;
    match (forward, up, down) {
        (Some(_), None, None) => Ok((
            input,
            Aim {
                horizontal: u,
                depth: 0,
                aim: u,
            },
        )),
        (None, Some(_), None) => Ok((
            input,
            Aim {
                horizontal: 0,
                depth: 0,
                aim: -u,
            },
        )),
        (None, None, Some(_)) => Ok((
            input,
            Aim {
                horizontal: 0,
                depth: 0,
                aim: u,
            },
        )),
        _ => panic!("Unable to parse string {}", input),
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
        .fold(
            Position {
                horizontal: 0,
                depth: 0,
            },
            |Position {
                 horizontal: ih,
                 depth: id,
             },
             Position {
                 horizontal: x,
                 depth: y,
             }| Position {
                horizontal: ih + x,
                depth: id + y,
            },
        )
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
        .fold(
            Aim {
                horizontal: 0,
                depth: 0,
                aim: 0,
            },
            |Aim {
                 horizontal: ih,
                 depth: id,
                 aim: ia,
             },
             Aim {
                 horizontal: h,
                 depth: _,
                 aim: a,
             }| Aim {
                horizontal: ih + h,
                depth: if h > 0 { id + ia * h } else { id },
                aim: if h > 0 { ia } else { ia + a },
            },
        )
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
