use super::lib::Res;
use itertools::Itertools;
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

fn search_left(basin_matrix: &BasinMatrix, (row, col): &RowAndCol) -> Option<(usize, RowAndCol)> {
    if *col == 0 {
        None
    } else if let (row_and_col, Some(v)) = basin_matrix.index((*row, *col - 1)) {
        Some((*v, *row_and_col))
    } else {
        None
    }
}

fn search_right(basin_matrix: &BasinMatrix, (row, col): &RowAndCol) -> Option<(usize, RowAndCol)> {
    if *col == basin_matrix.ncols() - 1 {
        None
    } else if let (row_and_col, Some(v)) = basin_matrix.index((*row, *col + 1)) {
        Some((*v, *row_and_col))
    } else {
        None
    }
}

fn search_down(basin_matrix: &BasinMatrix, (row, col): &RowAndCol) -> Option<(usize, RowAndCol)> {
    if *row == basin_matrix.nrows() - 1 {
        None
    } else if let (row_and_col, Some(v)) = basin_matrix.index((*row + 1, *col)) {
        Some((*v, *row_and_col))
    } else {
        None
    }
}

fn search_up(basin_matrix: &BasinMatrix, (row, col): &RowAndCol) -> Option<(usize, RowAndCol)> {
    if *row == 0 {
        None
    } else if let (row_and_col, Some(v)) = basin_matrix.index((*row - 1, *col)) {
        Some((*v, *row_and_col))
    } else {
        None
    }
}

fn search(basin_matrix: &BasinMatrix, row_and_col: &RowAndCol) -> Vec<(usize, RowAndCol)> {
    let vec: Vec<Option<(usize, RowAndCol)>> = vec![
        search_left(basin_matrix, row_and_col),
        search_right(basin_matrix, row_and_col),
        search_up(basin_matrix, row_and_col),
        search_down(basin_matrix, row_and_col),
    ];
    println!("search: {:?}", vec);
    vec.into_iter().flatten().collect()
}

type BasinMatrix = na::DMatrix<(RowAndCol, Option<usize>)>;

// The usize is the basin the 'RowAndCol' belongs to
type Basins = HashMap<RowAndCol, usize>;

fn part2(matrix: &Matrix) -> usize {
    let basin_matrix: BasinMatrix = matrix.map_with_location(|row, col, x| {
        if x == 9 {
            ((row, col), None)
        } else {
            ((row, col), Some(x))
        }
    });
    let mut basins: Basins = HashMap::new();
    let mut basin_counter: usize = 0;

    for row in 0..basin_matrix.nrows() {
        for col in 0..basin_matrix.ncols() {
            // We should only look for basins around 'Some(_)'
            if basin_matrix.index((row, col)).1.is_some() {
                // Skip search for entries that are already in the basins
                if let Some(_) = basins.get(&(row, col)) {
                    continue;
                }

                let search_results = search(&basin_matrix, &(row, col));

                // From the search results, build a list of 'the_basin's
                let mut the_basin: Vec<usize> = Vec::new();
                for row_and_col in search_results.clone() {
                    if let Some(b) = basins.get(&row_and_col.1) {
                        the_basin.push(*b);
                    }
                }

                // If the_basin is empty, we make a new basin and insert the 'RowAndCol's
                if the_basin.len() == 0 {
                    for row_and_col in search_results {
                        basins.entry(row_and_col.1).or_insert(basin_counter);
                    }
                    basin_counter += 1;
                }
                // otherwise, 'the_basin' exists, all entries in 'the_basin' should be equal, and
                // we can insert all of the 'RowAndCol's into the basin
                else {
                    let should_be = the_basin[0];
                    println!(
                        "{:?} {:?} {:?} {:?}",
                        the_basin,
                        search_results,
                        (row, col),
                        basin_matrix.index((row, col))
                    );
                    assert_eq!(the_basin.clone().iter().all(|x| *x == should_be), true);
                    for row_and_col in search_results {
                        basins.entry(row_and_col.1).or_insert(should_be);
                    }
                }
            }
        }
    }

    let mut count_vec: Vec<usize> = Vec::new();
    for (_, group) in &basins.values().sorted().group_by(|x| *x) {
        count_vec.push(group.collect::<Vec<&usize>>().len());
    }
    count_vec.sort();
    count_vec.iter().rev().take(3).fold(1, |acc, x| acc * x)
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
    println!("Part 2 {:?}", part2(&input));
}
