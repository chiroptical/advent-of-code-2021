use nom::{error::VerboseError, IResult};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result as IOResult};
use std::path::Path;

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn read_lines<P>(filename: P) -> IOResult<Lines<BufReader<File>>>
// This is similar to a constraint in Haskell
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_and_parse_lines<P, T>(p: P, f: fn(&String) -> Option<T>) -> Vec<T>
where
    P: AsRef<Path>,
{
    match read_lines(p) {
        Ok(lines) => {
            let mut result: Vec<T> = Vec::new();
            for line in lines {
                // We're not really concerned about being unable to read a line...
                let s = line.unwrap();

                match f(&s) {
                    Some(i) => result.push(i),
                    None => panic!(
                        "Unable to read the following unsigned integer, got: {:?}",
                        s
                    ),
                };
            }
            result
        }
        _ => panic!("Unable to read the input..."),
    }
}

pub trait Semigroup {
    fn mappend(_: Self, _: Self) -> Self;
}

pub trait Monoid {
    fn mempty() -> Self;
}
