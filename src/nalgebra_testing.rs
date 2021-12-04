use super::day4;
use super::lib::Res;
extern crate nalgebra as na;
use nom::{
    character::complete::{newline, space0, space1, u16},
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, tuple},
};

type Mat = na::DMatrix<usize>;

pub fn parse_board_line(input: &str) -> Res<&str, Vec<usize>> {
    let (input, result) = separated_list1(space1, u16)(input)?;
    Ok((input, result.iter().map(|x| *x as usize).collect()))
}

fn parse_board(input: &str) -> Res<&str, Mat> {
    let (input, result) = separated_list1(pair(newline, opt(space0)), parse_board_line)(input)?;
    let m = result.len();
    let n = result[0].len();
    Ok((
        input,
        Mat::from_vec(m, n, result.into_iter().flatten().collect()),
    ))
}

fn parse_boards(input: &str) -> Res<&str, Vec<Mat>> {
    let (input, result) =
        separated_list1(tuple((newline, newline, opt(space0))), parse_board)(input)?;
    Ok((input, result))
}

fn parse_input(input: &str) -> Res<&str, Vec<Mat>> {
    let (input, _) = day4::parse_marks(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, boards) = parse_boards(input)?;

    Ok((input, boards))
}

pub fn run() {
    let input_str: &str = include_str!("../inputs/day4.test");
    let input = parse_input(input_str).unwrap().1;

    let mat = &input[0];
    println!("{:?}", mat);

    // Folds over rows and columns
    println!(
        "Fold over rows {:?}",
        mat.row_iter().fold(0, |acc, x| acc + x.sum())
    );
    println!(
        "Fold over columns {:?}",
        mat.column_iter().fold(0, |acc, x| acc + x.sum())
    );

    // Returns a column vector of sums
    println!("Row sum: {:?}", mat.row_sum());
    // Returns a row vector of sums
    println!("Column sum: {:?}", mat.column_sum());

    println!("Transpose: {:?}", mat.transpose());
}
