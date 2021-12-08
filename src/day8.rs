use super::lib::Res;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
};

#[derive(Debug)]
struct InsAndOuts<'a> {
    ins: Vec<&'a str>,
    outs: Vec<&'a str>,
}

fn parse_line(input: &str) -> Res<&str, InsAndOuts> {
    let (input, ins) = separated_list1(space1, alpha1)(input)?;
    let (input, _) = tuple((space1, tag("|"), space1))(input)?;
    let (input, outs) = separated_list1(space1, alpha1)(input)?;
    Ok((input, InsAndOuts { ins, outs }))
}

fn parse_lines(input: &str) -> Res<&str, Vec<InsAndOuts>> {
    let (input, result) = separated_list1(newline, parse_line)(input)?;
    Ok((input, result))
}

fn find_obvious_numbers(input: &str) -> Option<usize> {
    match input.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

pub fn run() {
    let test_: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    // println!("{:?}", parse_line(test).unwrap().1);

    let test_str = include_str!("../inputs/day8.test");
    let input_str = include_str!("../inputs/day8");
    let test = parse_lines(test_str).unwrap().1;
    let input = parse_lines(input_str).unwrap().1;

    let result: usize = input
        .iter()
        .map(|InsAndOuts { ins: _, outs }| {
            let mut obvious_numbers = 0;
            for out in outs {
                if find_obvious_numbers(&out).is_some() {
                    obvious_numbers += 1
                }
            }
            obvious_numbers
        })
        .sum();
    println!("{:?}", result);
}
