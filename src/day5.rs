// This one is unfinished... sad...
use super::lib::Res;
use geo::{
    algorithm::line_intersection::{line_intersection, LineIntersection},
    Coordinate, Line,
};
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, newline},
    multi::separated_list1,
};
use std::collections::HashSet;

type Point = Coordinate<f64>;
type LineSegment = Line<f64>;

pub fn parse_point(input: &str) -> Res<&str, Point> {
    // Technically this would also parse 1,2,3
    let (input, result) = separated_list1(char(','), i64)(input)?;
    Ok((
        input,
        Point {
            x: result[0] as f64,
            y: result[1] as f64,
        },
    ))
}

fn parse_segments(input: &str) -> Res<&str, Option<LineSegment>> {
    let (input, result) = separated_list1(tag(" -> "), parse_point)(input)?;
    Ok((input, {
        let coord0 = result[0];
        let coord1 = result[1];
        let line_segment = LineSegment::new(coord0, coord1);
        match (coord0.x == coord1.x, coord0.y == coord1.y) {
            (true, _) => Some(line_segment),
            (_, true) => Some(line_segment),
            _ => None,
        }
    }))
}

fn parse_part_one(input: &str) -> Res<&str, Vec<LineSegment>> {
    let (input, result) = separated_list1(newline, parse_segments)(input)?;
    Ok((
        input,
        result
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect(),
    ))
}

fn parse_diagonals(input: &str) -> Res<&str, Option<LineSegment>> {
    let (input, result) = separated_list1(tag(" -> "), parse_point)(input)?;
    Ok((input, {
        let coord0 = result[0];
        let coord1 = result[1];
        let is_horizontal = coord0.x == coord1.x;
        let is_vertical = coord0.y == coord1.y;
        let line_segment = LineSegment::new(coord0, coord1);
        let slope_is_one = line_segment.slope().abs() == 1.0;
        if vec![is_horizontal, is_vertical, slope_is_one]
            .iter()
            .any(|x| *x)
        {
            Some(line_segment)
        } else {
            None
        }
    }))
}

fn parse_part_two(input: &str) -> Res<&str, Vec<LineSegment>> {
    let (input, result) = separated_list1(newline, parse_diagonals)(input)?;
    Ok((
        input,
        result
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect(),
    ))
}

fn swap_line_segment(input: &LineSegment) -> LineSegment {
    LineSegment {
        start: input.end,
        end: input.start,
    }
}

fn order_line_segment(input: &LineSegment) -> LineSegment {
    let is_horizontal = input.start.x == input.end.x;
    let is_horizonal_ = input.start.y > input.end.y;
    let is_vertical = input.start.y == input.end.y;
    let is_vertical_ = input.start.x > input.end.x;
    if is_horizontal && is_horizonal_ {
        swap_line_segment(&input)
    } else if is_vertical && is_vertical_ {
        swap_line_segment(&input)
    } else {
        input.clone()
    }
}

fn generate_points_part_one(input: &LineSegment) -> Vec<(usize, usize)> {
    let ordered = order_line_segment(&input);
    let mut ret: Vec<(usize, usize)> = Vec::new();
    if ordered.start.x == ordered.end.x {
        for y in ordered.start.y as usize..ordered.end.y as usize + 1 {
            ret.push((ordered.start.x as usize, y));
        }
    } else if ordered.start.y == ordered.end.y {
        for x in ordered.start.x as usize..ordered.end.x as usize + 1 {
            ret.push((x, ordered.start.y as usize));
        }
    } else {
        panic!("generate_points_part_one: {:?}", ordered.slope())
    }
    ret
}

fn build_range(x: f64, y: f64, reverse: bool) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();
    // (x as usize..y as usize + 1).collect();
    if reverse {
        vec = (y as usize..x as usize + 1).collect();
        vec.reverse();
    } else {
        vec = (x as usize..y as usize + 1).collect();
    }
    vec
}

fn build_vec_hv(input: &LineSegment) -> Vec<(usize, usize)> {
    let x_is_decreasing = input.start.x > input.end.x;
    let y_is_decreasing = input.start.y > input.end.y;
    let x_is_flat = input.start.x == input.end.x;
    let y_is_flat = input.start.y == input.end.y;
    let x: Vec<usize> = build_range(input.start.x, input.end.x, x_is_decreasing);
    let y: Vec<usize> = build_range(input.start.y, input.end.y, y_is_decreasing);
    if x_is_flat {
        y.iter().map(|y| (input.start.x as usize, *y)).collect()
    } else if y_is_flat {
        x.iter().map(|x| (*x, input.start.y as usize)).collect()
    } else {
        panic!("build_vec_hv: Oh no...")
    }
}

fn build_vec_diagonal(input: &LineSegment) -> Vec<(usize, usize)> {
    let x_is_decreasing = input.start.x > input.end.x;
    let y_is_decreasing = input.start.y > input.end.y;
    let x: Vec<usize> = build_range(input.start.x, input.end.x, x_is_decreasing);
    let y: Vec<usize> = build_range(input.start.y, input.end.y, y_is_decreasing);
    let mut ret: Vec<(usize, usize)> = Vec::new();
    for (a, b) in x.iter().zip(y.iter()) {
        ret.push((*a, *b));
    }
    ret
}

fn generate_points_part_two(input: &LineSegment) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    if input.slope() == 1.0 || input.slope() == -1.0 {
        ret.append(&mut build_vec_diagonal(&input));
    } else {
        ret.append(&mut build_vec_hv(&input));
    }
    ret
}

fn part1(input: &Vec<LineSegment>) -> usize {
    // Maintain a set of points
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..input.len() {
        for y in x..input.len() {
            if x == y {
                continue;
            }
            match line_intersection(input[x], input[y]) {
                Some(LineIntersection::SinglePoint {
                    intersection: i,
                    is_proper: _,
                }) => {
                    set.insert((i.x as usize, i.y as usize));
                }
                Some(LineIntersection::Collinear { intersection: i }) => {
                    // Need the overlapping points from line, push onto overlapping_points
                    generate_points_part_one(&i).iter_mut().for_each(|x| {
                        set.insert(*x);
                    })
                }
                _ => continue,
            }
        }
    }
    set.len()
}

fn part2(input: &Vec<LineSegment>) -> usize {
    // Maintain a set of points
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..input.len() {
        for y in x..input.len() {
            if x == y {
                continue;
            }
            match line_intersection(input[x], input[y]) {
                Some(LineIntersection::SinglePoint {
                    intersection: i,
                    is_proper: _,
                }) => {
                    if i.x.fract() == 0.0 || i.y.fract() == 0.0 {
                        set.insert((i.x as usize, i.y as usize));
                    }
                }
                Some(LineIntersection::Collinear { intersection: i }) => {
                    // Need the overlapping points from line, push onto overlapping_points
                    generate_points_part_two(&i).iter_mut().for_each(|x| {
                        set.insert(*x);
                    })
                }
                _ => continue,
            }
        }
    }
    set.len()
}

fn make_point(x: usize, y: usize) -> Point {
    Point {
        x: x as f64,
        y: y as f64,
    }
}

pub fn run() {
    let test_str: &str = include_str!("../inputs/day5.test");
    let input_str: &str = include_str!("../inputs/day5");

    // Test parsers
    assert_eq!(parse_point("0,1").unwrap().1, make_point(0, 1));

    let test_part_one = parse_part_one(test_str).unwrap().1;
    let input_part_one = parse_part_one(input_str).unwrap().1;

    // Part 1
    println!("Part 1: {:?}", part1(&input_part_one));

    let test_part_two = parse_part_two(test_str).unwrap().1;
    let input_part_two = parse_part_two(input_str).unwrap().1;

    // Testing various functions
    let segment = LineSegment {
        start: make_point(4, 1),
        end: make_point(4, 3),
    };
    assert_eq!(build_vec_hv(&segment), vec![(4, 1), (4, 2), (4, 3)]);
    let segment = LineSegment {
        start: make_point(1, 4),
        end: make_point(3, 4),
    };
    assert_eq!(build_vec_hv(&segment), vec![(1, 4), (2, 4), (3, 4)]);
    let segment = LineSegment {
        start: make_point(4, 3),
        end: make_point(4, 1),
    };
    assert_eq!(build_vec_hv(&segment), vec![(4, 3), (4, 2), (4, 1)]);
    let segment = LineSegment {
        start: make_point(3, 4),
        end: make_point(1, 4),
    };
    assert_eq!(build_vec_hv(&segment), vec![(3, 4), (2, 4), (1, 4)]);
    let segment = LineSegment {
        start: make_point(3, 3),
        end: make_point(1, 1),
    };
    assert_eq!(build_vec_diagonal(&segment), vec![(3, 3), (2, 2), (1, 1)]);
    let segment = LineSegment {
        start: make_point(1, 1),
        end: make_point(3, 3),
    };
    assert_eq!(build_vec_diagonal(&segment), vec![(1, 1), (2, 2), (3, 3)]);
    let segment = LineSegment {
        start: make_point(1, 3),
        end: make_point(3, 1),
    };
    assert_eq!(build_vec_diagonal(&segment), vec![(1, 3), (2, 2), (3, 1)]);
    let segment = LineSegment {
        start: make_point(3, 1),
        end: make_point(1, 3),
    };
    assert_eq!(build_vec_diagonal(&segment), vec![(3, 1), (2, 2), (1, 3)]);

    // Part2
    println!("Part 2: {:?}", part2(&input_part_two));
}
