use super::lib::Res;
use nom::{
    character::complete::{char, i64},
    multi::separated_list1,
};
use std::collections::HashMap;

// First one is location, second is count
type Locations = HashMap<usize, usize>;

fn parse_crabs(input: &str) -> Res<&str, Locations> {
    let (input, result) = separated_list1(char(','), i64)(input)?;
    let mut hmap: Locations = HashMap::new();
    for x in result {
        *hmap.entry(x as usize).or_insert(0) += 1;
    }
    Ok((input, hmap))
}

type FuelCount = HashMap<usize, usize>;

#[derive(Debug)]
enum Part {
    One,
    Two,
}

fn get_triangular_number(input: usize) -> usize {
    input * (input + 1) / 2
}

fn simulate(input: &Locations, part: &Part, debug: bool) -> Option<usize> {
    // All possible horizontal positions
    let min = input.keys().min().unwrap();
    let max = input.keys().max().unwrap();

    // Brute force compute the fuel costs for moving to each horizontal position
    let mut fuel_counts: FuelCount = HashMap::new();
    for move_to in *min..*max {
        let mut fuel: isize = 0;
        for (location, count) in input {
            let movement = (*location as isize - move_to as isize).abs();
            let cost = match part {
                Part::One => movement,
                Part::Two => get_triangular_number(movement as usize) as isize,
            };
            fuel += cost * *count as isize;
        }
        *fuel_counts.entry(move_to).or_insert(0) += fuel as usize;
    }

    if debug {
        let mut keys: Vec<_> = fuel_counts.clone().into_iter().collect();
        keys.sort_by(|x, y| x.0.cmp(&y.0));
        for (k, v) in keys {
            println!("key: {:?} value: {:?}", k, v);
        }
    }

    // Determine the minimum fuel count
    fuel_counts
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| *v)
}

pub fn run() {
    let test_str: &str = "16,1,2,0,4,2,7,1,2,14";
    let input_str = include_str!("../inputs/day7");
    let test = parse_crabs(test_str).unwrap().1;
    let input = parse_crabs(input_str).unwrap().1;
    println!("Part 1: {:?}", simulate(&input, &Part::One, false));
    println!("Part 2: {:?}", simulate(&input, &Part::Two, false));
}
