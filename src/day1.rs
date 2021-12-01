// Use main's lib module here?
use super::lib::read_lines;
use std::iter::Peekable;

fn count_increasing_integers(vec: Vec<u32>) -> u32 {
    if vec.len() < 2 {
        return 0;
    };
    let zipped = vec.iter().zip(vec[1..].iter());
    return zipped.fold(0, |acc, (x, y)| if x < y { acc + 1 } else { acc });
}

fn make_triples(vec: Vec<u32>) -> Vec<Vec<u32>> {
    let mut peeky = vec.iter().peekable();
    let mut triplets: Vec<u32> = Vec::new();

    let mut x = 0;
    while x < vec.len() {
        let snd = peeky.peek();
        let third = peeky.peek();
        // If snd and third are Some, push a new triple...
        peeky.next();
        x += 1;
    }

    return vec![vec![0]];
}

pub fn part1() {
    match read_lines("./inputs/day1.txt") {
        Ok(lines) => {
            let mut sonar_sweep_depths: Vec<u32> = Vec::new();
            for line in lines {
                // We're not really concerned about being unable to read a line...
                let s = line.unwrap();

                match s.parse::<u32>() {
                    Ok(i) => sonar_sweep_depths.push(i),
                    _ => panic!(
                        "Unable to read the following unsigned integer, got: {:?}",
                        s
                    ),
                };
            }
            println!(
                "The result is {:?}",
                count_increasing_integers(sonar_sweep_depths)
            );
        }
        _ => println!("Unable to read the input..."),
    }
}
