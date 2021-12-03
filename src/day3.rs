use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{context, VerboseError},
    multi::many1,
    IResult,
};
use std::cmp::PartialEq;
use std::iter;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

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

fn gen_gamma_rate(
    Count {
        zeros: count_zero,
        ones: count_one,
    }: &Count,
) -> u8 {
    if count_zero > count_one {
        0
    } else {
        1
    }
}

fn gen_epsilon_rate(
    Count {
        zeros: count_zero,
        ones: count_one,
    }: &Count,
) -> u8 {
    if count_zero > count_one {
        1
    } else {
        0
    }
}

#[derive(Debug)]
struct Rate {
    gamma: Vec<u8>,
    epsilon: Vec<u8>,
}

fn convert_to_number(inp: &Vec<u8>) -> u32 {
    let string = inp.iter().fold(String::new(), |mut acc, num| {
        acc.push_str(&num.to_string());
        acc
    });
    u32::from_str_radix(&string, 2).unwrap()
}

fn build_rates(counts: Vec<Count>) -> Rate {
    let mut gammas: Vec<u8> = Vec::new();
    let mut epsilons: Vec<u8> = Vec::new();
    counts.iter().for_each(|x| {
        gammas.push(gen_gamma_rate(x));
        epsilons.push(gen_epsilon_rate(x));
    });
    Rate {
        gamma: gammas,
        epsilon: epsilons,
    }
}

fn part1(inp: Vec<Vec<Bit>>) -> u32 {
    let number_of_bits = inp[0].len();
    let mut counts: Vec<Count> = iter::repeat(Count { zeros: 0, ones: 0 })
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
    let gamma_num = convert_to_number(&gamma);
    let epsilon_num = convert_to_number(&epsilon);

    gamma_num * epsilon_num
}

fn count_at(inp: &Vec<Vec<Bit>>, position: usize) -> Count {
    let mut count = Count { zeros: 0, ones: 0 };
    for outer in inp {
        match outer[position] {
            Bit::Zero => count.zeros += 1,
            Bit::One => count.ones += 1,
        }
    }
    count
}

fn oxygen_keep(Count { zeros: z, ones: o }: Count) -> Bit {
    // When z = o, keep 'One's
    if z > o {
        Bit::Zero
    } else {
        Bit::One
    }
}

fn whittle_oxygen(inp: &Vec<Vec<Bit>>, position: usize) -> Vec<Vec<Bit>> {
    let mut vec: Vec<Vec<Bit>> = Vec::new();
    let count = count_at(inp, position);
    let keep = oxygen_keep(count);
    for outer in inp.iter() {
        if outer[position] == keep {
            vec.push(outer.to_vec());
        }
    }
    vec
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

fn co2_keep(Count { zeros: z, ones: o }: Count) -> Bit {
    // Keep the least common value, or Zero if the same
    if o < z {
        Bit::One
    } else {
        Bit::Zero
    }
}

fn whittle_co2(inp: &Vec<Vec<Bit>>, position: usize) -> Vec<Vec<Bit>> {
    let mut vec: Vec<Vec<Bit>> = Vec::new();
    let count = count_at(inp, position);
    let keep = co2_keep(count);
    for outer in inp.iter() {
        if outer[position] == keep {
            vec.push(outer.to_vec());
        }
    }
    vec
}

fn gen_oxygen_generator_rating(inp: &Vec<Vec<Bit>>) -> u32 {
    let number_of_bits = inp[0].len();
    let mut accumulator: Vec<Vec<Bit>> = whittle_oxygen(inp, 0);
    for idx in 1..number_of_bits {
        if accumulator.len() == 1 {
            break;
        }
        accumulator = whittle_oxygen(&accumulator, idx);
    }
    convert_bit_vec_to_u32(accumulator[0].clone())
}

fn gen_co2_scrubber_rating(inp: &Vec<Vec<Bit>>) -> u32 {
    let number_of_bits = inp[0].len();
    let mut accumulator: Vec<Vec<Bit>> = whittle_co2(inp, 0);
    for idx in 1..number_of_bits {
        if accumulator.len() == 1 {
            break;
        }
        accumulator = whittle_co2(&accumulator, idx);
    }
    convert_bit_vec_to_u32(accumulator[0].clone())
}

fn part2(inp: &Vec<Vec<Bit>>) -> u32 {
    let oxygen_generator_rating = gen_oxygen_generator_rating(inp);
    let co2_scrubber_rating = gen_co2_scrubber_rating(inp);
    oxygen_generator_rating * co2_scrubber_rating
}

pub fn run() {
    // let day3_test: Vec<&str> = include_str!("../inputs/day3.test.txt").trim_end().lines().collect();
    let day3_input: Vec<&str> = include_str!("../inputs/day3.txt")
        .trim_end()
        .lines()
        .collect();
    println!("Day 3, Part 1 {:?}", part1(build_bit_2d_vec(&day3_input)));
    println!("Day 3, Part 2 {:?}", part2(&build_bit_2d_vec(&day3_input)));
}
