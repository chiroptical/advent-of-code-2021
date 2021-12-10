// use nom::character::complete::newline;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::error::{ContextError, ErrorKind, ParseError};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;
use std::fmt::{Display, Formatter, Result};

pub type Res<T, U> = IResult<T, U, Expected>;

#[derive(Debug)]
enum Expected {
    Parens,
    Bracket,
    Birdtrack,
    Curly,
    Unexpected,
}

impl Display for Expected {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl ParseError<&str> for Expected {
    // on one line, we show the error code and the input that caused it
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("{:?}:\t{:?}\n", kind, input);
        println!("{}", message);
        Expected::Unexpected
    }

    // if combining multiple errors, we show them one after the other
    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        let message = format!("{}{:?}:\t{:?}\n", other, kind, input);
        println!("{}", message);
        Expected::Unexpected
    }

    fn from_char(input: &str, c: char) -> Self {
        let message = format!("'{}':\t{:?}\n", c, input);
        println!("{}", message);
        Expected::Unexpected
    }

    fn or(self, other: Self) -> Self {
        let message = format!("{}\tOR\n{}\n", self, other);
        println!("{}", message);
        Expected::Unexpected
    }
}

impl ContextError<&str> for Expected {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        let message = format!("{}\"{}\":\t{:?}\n", other, ctx, input);
        println!("{}", message);
        other
    }
}

fn parse_chunk(input: &str) -> Res<&str, ()> {
    let result = many1(alt((parse_parens, parse_brackets)))(input);
    if let Err(e) = result {
        Err(e)
    } else {
        Ok(("", ()))
    }
}

fn parse_parens(input: &str) -> Res<&str, ()> {
    let result = delimited(tag("("), opt(parse_chunk), tag(")"))(input);
    if let Err(e) = result {
        Err(e)
    } else {
        Ok((input, ()))
    }
}

fn parse_brackets(input: &str) -> Res<&str, ()> {
    let result = delimited(tag("["), opt(parse_chunk), tag("]"))(input);
    if let Err(e) = result {
        Err(e)
    } else {
        Ok((input, ()))
    }
}

// fn parse_birdtracks(input: &str) -> Res<&str, ()> {
//     let (input, _) = delimited(tag("<"), opt(parse_chunk), tag(">"))(input)?;
//     Ok((input, ()))
// }
//
// fn parse_curlys(input: &str) -> Res<&str, ()> {
//     let (input, _) = delimited(tag("{"), opt(parse_chunk), tag("}"))(input)?;
//     Ok((input, ()))
// }

fn check_line(input: &str) -> bool {
    let can_parse = parse_chunk(input);
    can_parse.is_ok()
}

pub fn run() {
    let chunk = "[<>({}){}[([])<>]]";
    assert_eq!(parse_chunk(chunk).unwrap(), ());

    let test_str = include_str!("../inputs/day10.test");
}
