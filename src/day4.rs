use super::lib::Res;
use nom::{
    character::complete::{char, newline, space0, space1, u16},
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, tuple},
};

fn parse_marks(input: &str) -> Res<&str, Vec<u16>> {
    let (input, result) = separated_list1(char(','), u16)(input)?;
    Ok((input, result))
}

fn parse_board_line(input: &str) -> Res<&str, Vec<u16>> {
    let (input, result) = separated_list1(space1, u16)(input)?;
    Ok((input, result))
}

fn parse_board(input: &str) -> Res<&str, Vec<Vec<u16>>> {
    let (input, result) = separated_list1(pair(newline, opt(space0)), parse_board_line)(input)?;
    Ok((input, result))
}

fn parse_boards(input: &str) -> Res<&str, Vec<Vec<Vec<u16>>>> {
    let (input, result) =
        separated_list1(tuple((newline, newline, opt(space0))), parse_board)(input)?;
    Ok((input, result))
}

#[derive(Debug, Clone)]
struct Value {
    value: u16,
    marked: bool,
}

type Board = Vec<Vec<Value>>;

type Boards = Vec<Vec<Vec<Value>>>;

#[derive(Debug, Clone)]
struct Input {
    markers: Vec<u16>,
    boards: Boards,
}

fn parse_input(input: &str) -> Res<&str, Input> {
    let (input, markers) = parse_marks(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, boards) = parse_boards(input)?;
    let unmarked_board = boards
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    y.iter()
                        .map(|z| Value {
                            value: *z,
                            marked: false,
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    Ok((
        input,
        Input {
            markers,
            boards: unmarked_board,
        },
    ))
}

fn mark_board(number_to_mark: u16, board: &Board) -> Board {
    board
        .iter()
        .map(|x| {
            x.iter()
                .map(|val @ Value { value, marked: _ }| {
                    if *value == number_to_mark {
                        Value {
                            value: *value,
                            marked: true,
                        }
                    } else {
                        val.clone()
                    }
                })
                .collect()
        })
        .collect()
}

fn is_board_winner(board: &Board) -> bool {
    let row_win: bool = board
        .iter()
        .map(|x| x.iter().all(|Value { value: _, marked }| marked == &true))
        .any(|x| x == true);

    let mut column_res: Vec<bool> = Vec::new();
    for column_idx in 0..board.len() {
        let mut tmp: Vec<bool> = vec![];
        for row_idx in 0..board.len() {
            tmp.push(board[row_idx][column_idx].marked);
        }
        column_res.push(tmp.iter().all(|x| x == &true));
    }
    row_win || column_res.iter().any(|x| x == &true)
}

#[derive(Debug)]
struct Result {
    board: Board,
    number: u16,
}

fn part1(input: &Input) -> Option<Result> {
    // Temporary board while marking
    let mut marked_boards: Boards = input.boards.clone();

    for number_to_mark in &input.markers {
        for board_idx in 0..marked_boards.len() {
            // Mark board at board index with number_to_mark
            marked_boards[board_idx] = mark_board(*number_to_mark, &marked_boards[board_idx]);

            // Check if board is a winner, and return it
            if is_board_winner(&marked_boards[board_idx]) {
                return Some(Result {
                    board: marked_boards[board_idx].clone(),
                    number: *number_to_mark,
                });
            }
        }
    }

    None
}

fn part2(input: &Input) -> Result {
    // Temporary board while marking
    let mut marked_boards: Boards = input.boards.clone();
    let mut losing_result: Result = Result {
        board: Vec::new(),
        number: 0,
    };

    for number_to_mark in &input.markers {
        for board_idx in 0..marked_boards.len() {
            // If the board is already a winner skip it
            if is_board_winner(&marked_boards[board_idx]) {
                continue;
            }

            // Mark board at board index with number_to_mark
            marked_boards[board_idx] = mark_board(*number_to_mark, &marked_boards[board_idx]);

            // Check if board is a winner, and return it
            if is_board_winner(&marked_boards[board_idx]) {
                losing_result = Result {
                    board: marked_boards[board_idx].clone(),
                    number: *number_to_mark,
                };
            }
        }
    }

    losing_result
}

fn sum_unmarked_values(board: &Board) -> u32 {
    board.iter().fold(0, |acc, x| {
        x.iter().fold(acc, |inner_acc, Value { value, marked }| {
            if marked == &false {
                inner_acc + *value as u32
            } else {
                inner_acc
            }
        })
    })
}

fn combine_result(Result { board, number }: &Result) -> u32 {
    *number as u32 * sum_unmarked_values(&board)
}

pub fn run() {
    let day4_test: &str = include_str!("../inputs/day4.test");
    // let day4_test_board: &str = include_str!("../inputs/day4.test.board");
    let day4_input: &str = include_str!("../inputs/day4");

    // Experiments
    let mark_str: &str = "7,4,9,5,11,17";
    assert_eq!(parse_marks(mark_str).unwrap().1, vec![7, 4, 9, 5, 11, 17]);
    // println!("{:?}", parse_boards(day4_test_board));

    // Test inputs
    let day4_test_input = parse_input(day4_test).unwrap().1;
    let day4_input = parse_input(day4_input).unwrap().1;

    // Test mark_board and is_board_winner
    let board = day4_test_input.boards[0].clone();
    let marks: Vec<Vec<u16>> = vec![
        vec![22, 13, 17, 11, 0],
        vec![22, 8, 21, 6, 1],
        vec![1, 12, 20, 15, 19],
        vec![0, 24, 7, 5, 19],
    ];
    marks.iter().for_each(|x| {
        assert_eq!(
            is_board_winner(
                &x.iter()
                    .fold(board.clone(), |acc, y| { mark_board(*y, &acc) })
            ),
            true
        )
    });

    // Determine results
    let result_one = combine_result(&part1(&day4_input).unwrap());
    println!("Part 1: {:?}", result_one);

    let result_two = combine_result(&part2(&day4_input));
    println!("Part 2: {:?}", result_two);
}
