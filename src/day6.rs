use super::lib::Res;
use nom::{
    character::complete::{char, i64},
    multi::separated_list1,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Fish {
    countdown: usize,
}

// First usize is day, second usize is count
type FishCounts = HashMap<usize, usize>;

fn parse_fishes(input: &str) -> Res<&str, FishCounts> {
    let (input, result) = separated_list1(char(','), i64)(input)?;
    let mut hmap: FishCounts = HashMap::new();
    for x in result {
        *hmap.entry(x as usize).or_insert(0) += 1;
    }
    Ok((input, hmap))
}

fn simulate(input: &FishCounts) -> FishCounts {
    let mut result: FishCounts = HashMap::new();
    for day in 0..8 + 1 {
        let count = input.get(&day);
        match count {
            Some(c) => {
                if day == 0 {
                    *result.entry(6).or_insert(0) += *c;
                    *result.entry(8).or_insert(0) += *c;
                } else {
                    *result.entry(day - 1).or_insert(0) += *c;
                }
            }
            None => (),
        }
    }
    result
}

fn part1(number_of_days: usize, input: &FishCounts) -> usize {
    let mut result: FishCounts = input.clone();
    for _ in 0..number_of_days {
        result = simulate(&result);
        // println!("Finished day: {:?}", day);
    }
    result.into_values().sum()
}

pub fn run() {
    let test_str: &str = "3,4,3,1,2";
    let input_str = include_str!("../inputs/day6");
    // let test = parse_fishes(test_str).unwrap().1;
    let input = parse_fishes(input_str).unwrap().1;
    println!("{:?}", part1(256, &input));
}
