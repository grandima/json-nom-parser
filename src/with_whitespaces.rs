use std::collections::{HashMap};


use nom::character::complete::{char, multispace0};
use nom::{IResult, Parser};
use nom::bytes::complete::{take_while_m_n};
use nom::Err::{Failure};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEY_VALUE_SIZE: HashMap<String, usize> = {
        let mut m = HashMap::new();
        m.insert("type".to_string(), 7);
        m.insert("public_key".to_string(), 64);
        m.insert("peer_id".to_string(), 52);
        m.insert("signature".to_string(), 128);
        m.insert("payload_type".to_string(), 0);
        m.insert("payload".to_string(), 38);
        m
    };
}

pub fn parse_json<'a, 'b: 'a>(keys: &'b mut HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    delimited(ws(char('{')), parse_list(keys), ws(char('}')))
}
fn parse_list<'a, 'b: 'a>(keys: &'b mut HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    separated_list1(ws(char(',')), parse_pair(keys))
}
fn parse_pair<'a, 'b: 'a>(keys: &'b mut HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, &'a str)> + 'a  {
    move |input: &'a str| -> IResult<&'a str, (&'a str, &'a str)> {
        let (left, key) = parse_unsized_key()(&input)?;
        let value_size = keys.remove(key).ok_or(Failure(Error::new(input, ErrorKind::Tag)))?;
        parse_preceded_value(value_size, char::is_alphanumeric)(left).map(|(left, value)|(left, (key, value)))
    }
}
fn parse_preceded_value<'a, 'b: 'a>(n: usize, condition: impl Fn(char) -> bool + 'b + Copy) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> + 'b {
    move |input: &'a str| {
        let (left, _) = delimited(multispace0, char(':'), multispace0)(input)?;
        let result = parse_sized_value(n, condition)(left);
        result
    }
}
fn parse_sized_value<'a>(n: usize, condition: impl Fn(char) -> bool) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), take_while_m_n(n, n, condition), char('"'))
}
fn parse_unsized_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('\"'), nom::bytes::complete::take_till(|c|c == '\"'), char('\"'))
}
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}