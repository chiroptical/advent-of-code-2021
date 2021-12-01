use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result as IOResult};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> IOResult<Lines<BufReader<File>>>
// This is similar to a constraint in Haskell
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
