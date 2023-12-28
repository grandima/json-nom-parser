use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
pub use nom::character::complete::alpha0;
use nom::character::complete::{alpha1, char, multispace0};
use nom::{IResult, Parser};
use nom::bytes::complete::{escaped, tag, take_until, take_while1, take_while_m_n};
use nom::Err::{Failure};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};

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
    delimited(
        multispace0,
        inner,
        multispace0
    )
}
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    lazy_static! {
    static ref KEY_VALUE_SIZE: HashMap<String, usize> = {
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
    const INPUT: &str = r#" { "type" : "Ed25519", "public_key" : "1847179d3572cc1b80516ba49096efada0f1930632ab16a9f10bf24ce2c360c2" , "peer_id":"12D3KooWBT8pyJAfWJhdeGYAtKvcaUmm78ExyZ6uo6BEimYNVat1","signature":"8e5d1f0c977fc3017135032610f1cc40e5774be436e5634d090f1df52eeb401763964031070b6baa8e2b878477cc75b5a269b6bbc4da548b89ffcaac9e4db50e","payload_type":"","payload":"12112f636f64612f79616d75782f312e302e30"}"#;
    use super::*;
    #[test]
    fn test_correctness_all_in_one() {
        let mut key_value_size = KEY_VALUE_SIZE.clone();
        let input = INPUT;
        let (left, res) = parse_json(&mut key_value_size)(input).unwrap();
        assert_eq!(left.len(), 0);
        let mut correct_key_value_size = KEY_VALUE_SIZE.clone();
        for (key, value) in res {
            assert!(value.chars().all(char::is_alphanumeric));
            let (correct_key, correct_value_size) = correct_key_value_size.remove_entry(key).unwrap();
            assert_eq!(value.len(), correct_value_size);
        }
        assert_eq!(correct_key_value_size.len(), 0);
    }
}
