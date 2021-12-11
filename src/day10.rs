fn is_opening_char(c: &char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '<' => true,
        '{' => true,
        _ => false,
    }
}

fn check_if_match(x: &char, y: &char) -> bool {
    match (x, y) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn process_chunk(input: &str) -> Result<(), char> {
    let mut char_stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if is_opening_char(&c) {
            char_stack.push(c)
        } else {
            match char_stack.last() {
                None => return Err(c),
                Some(last_char) => {
                    if check_if_match(last_char, &c) {
                        char_stack.pop();
                    } else {
                        return Err(c);
                    }
                }
            }
        }
    }
    Ok(())
}

fn generate_score(input: Result<(), char>) -> usize {
    match input {
        Err(')') => 3,
        Err(']') => 57,
        Err('}') => 1197,
        Err('>') => 25137,
        _ => 0,
    }
}

fn process(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|x| process_chunk(x))
        .fold(0, |acc, x| acc + generate_score(x))
}

fn find_stack(input: &str) -> Result<Vec<char>, ()> {
    let mut char_stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if is_opening_char(&c) {
            char_stack.push(c)
        } else {
            match char_stack.last() {
                None => return Err(()),
                Some(last_char) => {
                    if check_if_match(last_char, &c) {
                        char_stack.pop();
                    } else {
                        return Err(());
                    }
                }
            }
        }
    }
    Ok(char_stack)
}

fn clear_it_out(input: char) -> char {
    match input {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("Got {:?} but expected opening character", input),
    }
}

fn complete_stack(input: &Vec<char>) -> Vec<char> {
    // let mut result: Vec<char> = Vec::new();
    input.iter().rev().map(|x| clear_it_out(*x)).collect()
}

fn character_scores_part_2(input: char) -> usize {
    match input {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Got {:?} but expected opening character", input),
    }
}

fn generate_score_part_2(input: Vec<char>) -> usize {
    let mut current_score: usize = 0;
    for c in input {
        current_score = current_score * 5 + character_scores_part_2(c);
    }
    current_score
}

fn part2(input: &Vec<&str>) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for line in input {
        if let Ok(stack) = find_stack(line) {
            let completed_stack = complete_stack(&stack);
            scores.push(generate_score_part_2(completed_stack));
        }
    }
    scores.sort();
    // The number of scores must be odd
    assert_eq!(scores.len() % 2, 1);
    let index: usize = scores.len() / 2;
    scores[index]
}

pub fn run() {
    let chunk = "([])";
    assert_eq!(process_chunk(&chunk), Ok(()));

    let chunk = "[<>({}){}[([])<>]]";
    assert_eq!(process_chunk(&chunk), Ok(()));

    let chunk = "{()()()>";
    assert_eq!(process_chunk(&chunk), Err('>'));

    let chunk = "<([]){()}[{}])";
    assert_eq!(process_chunk(&chunk), Err(')'));

    let chunk = "{([(<{}[<>[]}>{[]{[(<()>";
    assert_eq!(process_chunk(&chunk), Err('}'));

    let chunk = "<{([([[(<>()){}]>(<<{{";
    assert_eq!(process_chunk(&chunk), Err('>'));

    let test_str: Vec<&str> = include_str!("../inputs/day10.test").lines().collect();
    assert_eq!(process(&test_str), 26397);

    let input_str: Vec<&str> = include_str!("../inputs/day10").lines().collect();
    assert_eq!(process(&input_str), 318081);
    assert_eq!(part2(&test_str), 288957);
    println!("{:?}", part2(&input_str));
}
