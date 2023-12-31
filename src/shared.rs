use std::ops::Range;

use nom::bytes::complete::tag_no_case;
use nom::{bytes::complete::is_not, IResult};
use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::not, sequence::delimited,
};

pub type Error = (String, Range<usize>);

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn line_num(str: &str, index: usize) -> usize {
    let mut acc = 0;
    for (i, line) in str.lines().enumerate() {
        if acc + line.len() > index {
            return i;
        }
        acc += line.len();
    }
    unreachable!();
}

pub fn remove_comments(input: &str) -> String {
    let mut out: String = String::new();
    let mut in_comment = false;
    input.chars().for_each(|c| {
        if c == ';' {
            in_comment = true;
        } else if c == '\n' {
            in_comment = false;
        }
        if !in_comment {
            out.push(c);
        }
    });
    out
}

pub fn spaced<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

pub fn named(input: &str) -> IResult<&str, String> {
    let (remainder, name) = is_not(" \t\r\n():")(input)?;
    not(tag("-"))(name)?;
    not(tag("="))(name)?;
    not(tag_no_case("and"))(name)?;
    return Ok((remainder, name.to_lowercase().to_owned()));
}

#[test]
fn remove_comments_test() {
    assert_eq!("", remove_comments(""));
    assert_eq!("abc", remove_comments("abc"));
    assert_eq!("", remove_comments(";"));
    assert_eq!("", remove_comments(";abc"));
    assert_eq!("\n", remove_comments(";abc\n"));
    assert_eq!("\n123", remove_comments(";abc\n123"));
    assert_eq!("0\n123", remove_comments("0;abc\n123"));
}
