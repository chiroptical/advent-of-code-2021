use nom::{error::VerboseError, IResult, character::complete::u8};
use std::iter;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

// We need to parse one or more u8's
// We are given a variable number in the input
fn parse_line(input: &str) -> Res<&str, Vec<u8>> {
    let (input, zero) = u8(input)?;
    let (input, one) = u8(input)?;
    let (input, two) = u8(input)?;
    let (input, three) = u8(input)?;
    let (input, four) = u8(input)?;
    Ok((
        input,
        vec![
            zero,
            one,
            two,
            three,
            four,
        ],
    ))
}

fn build_2d_vec(input: Vec<&str>) -> Vec<Vec<u8>> {
    input.iter().map(|s| {
        let (_, vec) = parse_line(s).unwrap();
        vec
    }).collect()
}

enum ZeroOrOne {
    Zero,
    One
}

#[derive(Clone)]
struct Count {
    zeros: u8,
    ones: u8
}

fn determine_gamma(inp: Vec<Vec<u8>>) -> u8 {
    let mut counts: Vec<Count> = iter::repeat(Count {zeros: 0, ones: 0}).take(5).collect();
    for outer in inp.iter() {
        for (idx, value) in outer.iter().enumerate() {
            let zero_or_one = match value {
                0 => ZeroOrOne::Zero,
                1 => ZeroOrOne::One,
                _ => panic!("It is broken...")
            };
            match zero_or_one {
                ZeroOrOne::Zero => {
                    counts[idx].zeros += 1
                },
                ZeroOrOne::One => {
                    counts[idx].ones += 1
                }
            }
        }
    }
    // Now we need to take the counts and reduce them
    let mut gamma_vector: Vec<u8> = Vec::new();
    for Count { zeros: zero, ones: one } in counts {
        if zero > one { gamma_vector.push(0) } else { gamma_vector.push(1) }
    }
    // I super duper hate this code...
    // [0, 1, 0, 0, 1] -> 0b01001 -> 22
    let as_string = gamma_vector.iter().fold(String::new(), |mut acc, num| {
        acc.push_str(&num.to_string());
        acc
    });
    as_string.parse::<u8>().unwrap()
}

pub fn run() {
    let initial_input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let day3_input = include_str!("../inputs/day3.txt").trim_end().lines().collect();
    println!("{:?}", determine_gamma(build_2d_vec(day3_input)));
}
