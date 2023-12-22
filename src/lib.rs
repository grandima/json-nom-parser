use std::error::Error;
pub use nom::character::complete::alpha0;
use nom::character::complete::{alpha1, char, multispace0};
use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};

fn parse_key_value<'a>(key: &'a str, val_len: usize) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    map(separated_pair(parse_key(key), nom::sequence::tuple((char(':'), multispace0)), parse_value(val_len, char::is_alphanumeric)), |(x, y): (&str, &str)| y)
}
fn parse_value<'a>(n: usize, condition: impl Fn(char) -> bool) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), take_while_m_n(n, n, condition), char('"'))
}
fn parse_key<'a, 'b: 'a>(key: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), tag(key), char('"'))
}
#[cfg(test)]
mod tests {
    use nom::character::is_alphabetic;
    use super::*;
    #[test]
    fn it_works() {
        let input = r#""key": "va2""#;
        let res = parse_key_value("key", 3)(input);
        println!("{:?}", res);
    }
}
