use super::lib::Res;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::{character::complete::one_of, multi::many1};
extern crate nalgebra as na;

fn parse_single_number(input: &str) -> Res<&str, usize> {
    let (input, res) = one_of("0123456789")(input)?;
    Ok((input, res.to_string().parse::<usize>().unwrap()))
}

fn parse_line(input: &str) -> Res<&str, Vec<usize>> {
    let (input, res) = many1(parse_single_number)(input)?;
    Ok((input, res.iter().map(|x| *x as usize).collect()))
}

type Matrix = na::DMatrix<usize>;

fn parse_lines(input: &str) -> Res<&str, Matrix> {
    let (input, res) = separated_list1(newline, parse_line)(input)?;
    Ok((input, {
        let num_rows = res.len();
        let num_cols = res[0].len();
        Matrix::from_vec(num_cols, num_rows, res.into_iter().flatten().collect())
    }))
}

fn look_left(matrix: &Matrix, row: &usize, col: &usize) -> bool {
    if *col == 0 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row, *col - 1))
}

fn look_right(matrix: &Matrix, row: &usize, col: &usize) -> bool {
    if *col == matrix.ncols() - 1 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row, *col + 1))
}

fn look_down(matrix: &Matrix, row: &usize, col: &usize) -> bool {
    if *row == matrix.nrows() - 1 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row + 1, *col))
}

fn look_up(matrix: &Matrix, row: &usize, col: &usize) -> bool {
    if *row == 0 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row - 1, *col))
}

fn is_lower_than_neighbors(matrix: &Matrix, col: &usize, row: &usize) -> bool {
    look_up(matrix, row, col)
        && look_down(matrix, row, col)
        && look_left(matrix, row, col)
        && look_right(matrix, row, col)
}

fn part1(matrix: &Matrix) -> usize {
    let mut risk_level: usize = 0;

    assert_eq!(look_left(matrix, &0, &0), true);
    assert_eq!(look_right(matrix, &0, &(matrix.ncols() - 1)), true);
    assert_eq!(look_up(matrix, &0, &0), true);
    assert_eq!(look_down(matrix, &(matrix.nrows() - 1), &0), true);

    println!("matrix[0][0]: {:?}", matrix.index((0, 0)));
    println!("matrix[1][0]: {:?}", matrix.index((1, 0)));

    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            if is_lower_than_neighbors(matrix, &col, &row) {
                println!("row: {:?} col: {:?}", row, col);
                risk_level += matrix.index((col, row)) + 1
            }
        }
    }
    risk_level
}

pub fn run() {
    assert_eq!(
        parse_line("2199943210").unwrap().1,
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]
    );

    let test_str = include_str!("../inputs/day9.test");
    let input_str = include_str!("../inputs/day9");
    let test = parse_lines(test_str).unwrap().1;
    let input = parse_lines(input_str).unwrap().1;
    println!("{:?}", test);
    println!("{:?}", part1(&test));
}
