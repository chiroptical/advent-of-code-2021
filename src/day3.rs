use super::lib::{Monoid, Semigroup, Res};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::context,
    multi::many1,
};
use std::cmp::PartialEq;
use std::iter;

#[derive(Debug, Clone, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl From<&str> for Bit {
    fn from(i: &str) -> Self {
        match i.to_owned().as_str() {
            "0" => Bit::Zero,
            "1" => Bit::One,
            _ => unimplemented!("The only acceptable bits are: 0, 1"),
        }
    }
}

fn parse_bit(input: &str) -> Res<&str, Bit> {
    context(
        "bit",
        // Can use tag_no_case for case insensitive match
        alt((tag("0"), tag("1"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

// We need to parse one or more u8's
// We are given a variable number in the input
fn parse_line(input: &str) -> Res<&str, Vec<Bit>> {
    let (input, result) = (many1(parse_bit))(input)?;
    Ok((input, result))
}

fn build_bit_2d_vec(inp: &Vec<&str>) -> Vec<Vec<Bit>> {
    inp.iter()
        .map(|s| {
            let (_, vec) = parse_line(s).unwrap();
            vec
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Count {
    zeros: u32,
    ones: u32,
}

impl Semigroup for Count {
    fn mappend(fst: Count, snd: Count) -> Count {
        Count {
            zeros: fst.zeros + snd.zeros,
            ones: fst.ones + snd.ones,
        }
    }
}

impl Monoid for Count {
    fn mempty() -> Count {
        Count { zeros: 0, ones: 0 }
    }
}

fn flip_bit(bit: &Bit) -> Bit {
    match bit {
        Bit::Zero => Bit::One,
        Bit::One => Bit::Zero,
    }
}

fn gen_rate(Count { zeros, ones }: &Count) -> Bit {
    if zeros > ones {
        Bit::Zero
    } else {
        Bit::One
    }
}

#[derive(Debug)]
struct Rate {
    gamma: Vec<Bit>,
    epsilon: Vec<Bit>,
}

fn convert_to_number(inp: &Vec<u8>) -> u32 {
    let string = inp.iter().fold(String::new(), |mut acc, num| {
        acc.push_str(&num.to_string());
        acc
    });
    u32::from_str_radix(&string, 2).unwrap()
}

fn build_rates(counts: Vec<Count>) -> Rate {
    let mut gammas: Vec<Bit> = Vec::new();
    let mut epsilons: Vec<Bit> = Vec::new();
    counts.iter().for_each(|x| {
        let rate = gen_rate(x);
        epsilons.push(flip_bit(&rate));
        gammas.push(rate);
    });
    Rate {
        gamma: gammas,
        epsilon: epsilons,
    }
}

fn part1(inp: Vec<Vec<Bit>>) -> u32 {
    let number_of_bits = inp[0].len();
    let mut counts: Vec<Count> = iter::repeat(Monoid::mempty())
        .take(number_of_bits)
        .collect();
    for outer in inp.iter() {
        for (idx, value) in outer.iter().enumerate() {
            match value {
                Bit::Zero => counts[idx].zeros += 1,
                Bit::One => counts[idx].ones += 1,
            }
        }
    }
    // Get the rates
    let Rate { gamma, epsilon } = build_rates(counts);

    // I super duper hate this code...
    // [0, 1, 0, 0, 1] -> 0b01001 -> 22
    let gamma_num = convert_bit_vec_to_u32(gamma);
    let epsilon_num = convert_bit_vec_to_u32(epsilon);

    gamma_num * epsilon_num
}

fn count_at(inp: &Vec<Vec<Bit>>, position: usize) -> Count {
    inp.iter()
        .fold(Monoid::mempty(), |acc, xs| match xs[position] {
            Bit::Zero => Semigroup::mappend(acc, Count { zeros: 1, ones: 0 }),
            Bit::One => Semigroup::mappend(acc, Count { zeros: 0, ones: 1 }),
        })
}

fn convert_bit_vec_to_u32(inp: Vec<Bit>) -> u32 {
    let vec: Vec<u8> = inp
        .iter()
        .map(|x| match x {
            Bit::Zero => 0,
            Bit::One => 1,
        })
        .collect();
    convert_to_number(&vec)
}

fn whittle(inp: &Vec<Vec<Bit>>, position: usize, keeper: impl Fn(Count) -> Bit) -> Vec<Vec<Bit>> {
    let mut vec: Vec<Vec<Bit>> = Vec::new();
    let count = count_at(inp, position);
    let keep = keeper(count);
    for outer in inp.iter() {
        if outer[position] == keep {
            vec.push(outer.to_vec());
        }
    }
    vec
}

fn oxygen_keep(Count { zeros, ones }: Count) -> Bit {
    // When z = o, keep 'One's
    if zeros > ones {
        Bit::Zero
    } else {
        Bit::One
    }
}

fn co2_keep(count: Count) -> Bit {
    flip_bit(&oxygen_keep(count))
}

fn gen_rating(inp: &Vec<Vec<Bit>>, keeper: &impl Fn(Count) -> Bit) -> u32 {
    let number_of_bits = inp[0].len();
    let mut accumulator: Vec<Vec<Bit>> = whittle(inp, 0, &keeper);
    for idx in 1..number_of_bits {
        if accumulator.len() == 1 {
            break;
        }
        accumulator = whittle(&accumulator, idx, &keeper);
    }
    convert_bit_vec_to_u32(accumulator[0].clone())
}

fn part2(inp: Vec<Vec<Bit>>) -> u32 {
    let oxygen_generator_rating = gen_rating(&inp, &oxygen_keep);
    let co2_scrubber_rating = gen_rating(&inp, &co2_keep);
    oxygen_generator_rating * co2_scrubber_rating
}

pub fn run() {
    // let day3_test: Vec<&str> = include_str!("../inputs/day3.test.txt").trim_end().lines().collect();
    let day3_input: Vec<&str> = include_str!("../inputs/day3.txt")
        .trim_end()
        .lines()
        .collect();
    println!("Day 3, Part 1 {:?}", part1(build_bit_2d_vec(&day3_input)));
    println!("Day 3, Part 2 {:?}", part2(build_bit_2d_vec(&day3_input)));
}
