use super::lib::Res;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::{character::complete::one_of, multi::many1};
use std::collections::HashMap;
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
        // from_vec reads in column major order, but index reads (row, col)
        Matrix::from_vec(num_cols, num_rows, res.into_iter().flatten().collect()).transpose()
    }))
}

type RowAndCol = (usize, usize);

fn look_left(matrix: &Matrix, (row, col): &RowAndCol) -> bool {
    if *col == 0 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row, *col - 1))
}

fn look_right(matrix: &Matrix, (row, col): &RowAndCol) -> bool {
    if *col == matrix.ncols() - 1 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row, *col + 1))
}

fn look_down(matrix: &Matrix, (row, col): &RowAndCol) -> bool {
    if *row == matrix.nrows() - 1 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row + 1, *col))
}

fn look_up(matrix: &Matrix, (row, col): &RowAndCol) -> bool {
    if *row == 0 {
        return true;
    }
    matrix.index((*row, *col)) < matrix.index((*row - 1, *col))
}

fn is_lower_than_neighbors(matrix: &Matrix, row_and_col: &RowAndCol) -> bool {
    look_up(matrix, row_and_col)
        && look_down(matrix, row_and_col)
        && look_left(matrix, row_and_col)
        && look_right(matrix, row_and_col)
}

fn part1(matrix: &Matrix) -> usize {
    let mut risk_level: usize = 0;
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            if is_lower_than_neighbors(matrix, &(row, col)) {
                risk_level += matrix.index((row, col)) + 1
            }
        }
    }
    risk_level
}

type BasinMatrix = na::DMatrix<Option<usize>>;

// [   | b |   ]
// [ d | x | e ]
// [   | g |   ]
fn propogate(basin_matrix: &mut BasinMatrix, basin_counter: usize) {
    for row in 0..basin_matrix.nrows() {
        for col in 0..basin_matrix.ncols() {
            if basin_matrix[(row, col)] == Some(basin_counter) {
                let is_safe_sub_row = row != 0;
                let is_safe_sub_col = col != 0;
                let is_safe_add_row = row != basin_matrix.nrows() - 1;
                let is_safe_add_col = col != basin_matrix.ncols() - 1;
                // b
                if is_safe_sub_row {
                    if basin_matrix[(row - 1, col)] != Some(0) {
                        basin_matrix[(row - 1, col)] = Some(basin_counter);
                    }
                }
                // d
                if is_safe_sub_col {
                    if basin_matrix[(row, col - 1)] != Some(0) {
                        basin_matrix[(row, col - 1)] = Some(basin_counter);
                    }
                }
                // e
                if is_safe_add_col {
                    if basin_matrix[(row, col + 1)] != Some(0) {
                        basin_matrix[(row, col + 1)] = Some(basin_counter);
                    }
                }
                // g
                if is_safe_add_row {
                    if basin_matrix[(row + 1, col)] != Some(0) {
                        basin_matrix[(row + 1, col)] = Some(basin_counter);
                    }
                }
            }
        }
    }
}

// [   | b |   ]
// [ d | x | e ]
// [   | g |   ]
fn fill_larger_number(basin_matrix: &mut BasinMatrix) {
    for row in 0..basin_matrix.nrows() {
        for col in 0..basin_matrix.ncols() {
            if let Some(val) = basin_matrix[(row, col)] {
                // Skip zeros
                if val == 0 {
                    continue;
                }
                // Determine safe directions
                let is_safe_sub_row = row != 0;
                let is_safe_sub_col = col != 0;
                let is_safe_add_row = row != basin_matrix.nrows() - 1;
                let is_safe_add_col = col != basin_matrix.ncols() - 1;
                // b
                if is_safe_sub_row {
                    if basin_matrix[(row - 1, col)] != Some(0) {
                        if basin_matrix[(row - 1, col)] > basin_matrix[(row, col)] {
                            basin_matrix[(row, col)] = basin_matrix[(row - 1, col)]
                        } else {
                            basin_matrix[(row - 1, col)] = basin_matrix[(row, col)];
                        }
                    }
                }
                // d
                if is_safe_sub_col {
                    if basin_matrix[(row, col - 1)] != Some(0) {
                        if basin_matrix[(row, col - 1)] > basin_matrix[(row, col)] {
                            basin_matrix[(row, col)] = basin_matrix[(row, col - 1)];
                        } else {
                            basin_matrix[(row, col - 1)] = basin_matrix[(row, col)];
                        }
                    }
                }
                // e
                if is_safe_add_col {
                    if basin_matrix[(row, col + 1)] != Some(0) {
                        if basin_matrix[(row, col + 1)] > basin_matrix[(row, col)] {
                            basin_matrix[(row, col)] = basin_matrix[(row, col + 1)];
                        } else {
                            basin_matrix[(row, col + 1)] = basin_matrix[(row, col)];
                        }
                    }
                }
                // g
                if is_safe_add_row {
                    if basin_matrix[(row + 1, col)] != Some(0) {
                        if basin_matrix[(row + 1, col)] > basin_matrix[(row, col)] {
                            basin_matrix[(row, col)] = basin_matrix[(row + 1, col)];
                        } else {
                            basin_matrix[(row + 1, col)] = basin_matrix[(row, col)];
                        }
                    }
                }
            }
        }
    }
}

fn count_basins(basin_matrix: &BasinMatrix) -> HashMap<usize, usize> {
    let mut basin_map: HashMap<usize, usize> = HashMap::new();
    for row in 0..basin_matrix.nrows() {
        for col in 0..basin_matrix.ncols() {
            if let Some(val) = basin_matrix[(row, col)] {
                if val != 0 {
                    *basin_map.entry(val).or_insert(0) += 1;
                }
            }
        }
    }
    basin_map
}

fn part2(matrix: &Matrix) -> usize {
    let mut basin_matrix: BasinMatrix =
        BasinMatrix::from_element(matrix.nrows(), matrix.ncols(), None);
    // println!("{:?} {:?} {:?} {:?}", matrix.nrows(), matrix.ncols(), basin_matrix.nrows(), basin_matrix.ncols());

    let mut basin_counter: usize = 1;
    basin_matrix[(0, 0)] = Some(basin_counter);

    // Fill all the nines with zero
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            let parent_value = matrix.index((row, col));
            // If we get a nine, it is a zero and we shouldn't set values
            if *parent_value == 9 {
                basin_matrix[(row, col)] = Some(0);
            }
        }
    }

    propogate(&mut basin_matrix, basin_counter);
    while basin_matrix.iter().any(|x| x.is_none()) {
        let mut index: (usize, usize) = (0, 0);
        for row in 0..matrix.nrows() {
            for col in 0..matrix.ncols() {
                if basin_matrix[(row, col)] == None {
                    index = (row, col);
                    break;
                }
            }
        }
        basin_counter += 1;
        basin_matrix[index] = Some(basin_counter);
        propogate(&mut basin_matrix, basin_counter);
    }
    // Initialize for iteration
    let mut before = count_basins(&basin_matrix);
    fill_larger_number(&mut basin_matrix);
    let mut after = count_basins(&basin_matrix);
    // Run this until the hashmap doesn't change
    while before != after {
        before = after.clone();
        fill_larger_number(&mut basin_matrix);
        after = count_basins(&basin_matrix);
    }

    let mut sizes: Vec<usize> = after.values().map(|x| *x).collect();
    sizes.sort();
    sizes.iter().rev().take(3).fold(1, |acc, x| acc * *x)
}

fn display(matrix: &BasinMatrix) {
    for row in matrix.nrows() - 10..matrix.nrows() {
        for col in 0..11 {
            if let Some(v) = matrix[(row, col)] {
                print!("{:7}", v);
            }
        }
        println!();
    }
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
    println!("Part 1 {:?}", part1(&input));
    // 54756 is too low
    // 79376 is too low
    // 893564 is too low
    // 1123524 is correct :)
    println!("{:?}", part2(&input));
}
