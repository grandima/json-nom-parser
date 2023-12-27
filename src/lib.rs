use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::take;
pub use nom::character::complete::alpha0;
use nom::character::complete::{alpha1, char, multispace0};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_until, take_while1, take_while_m_n};
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::Err::{Failure};
use nom::error::{Error, ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};


struct KeyValueLen {
    key: String,
    val_len: usize
}
impl Hash for KeyValueLen {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.key.as_bytes());
    }
}
impl PartialEq<Self> for KeyValueLen {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialEq<&str> for KeyValueLen {
    fn eq(&self, other: &&str) -> bool {
        self.key.as_str() == *other
    }
}

impl Eq for KeyValueLen {

}

fn parse_json<'a, 'b: 'a>(keys: &'b mut HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    delimited(char('{'), parse_list(keys), char('}'))
}
fn parse_list<'a, 'b: 'a>(keys: &'b mut HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    separated_list1(char(','), parse_pair(keys))
}

fn parse_pair<'a, 'b: 'a>(keys: &'b mut std::collections::HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, &'a str)> + 'a  {
    move |input: &'a str| -> IResult<&'a str, (&'a str, &'a str)> {
        let (left, key) = parse_unknown_key()(&input)?;
        //TODO: write normal error
        let value_size = keys.remove(key).ok_or(Failure(Error::new(input, ErrorKind::Tag)))?;
        parse_preceded_value(value_size, char::is_alphanumeric)(left).map(|(left, value)|(left, (key, value)))
    }
}

//
// fn parse_key_value<'a>(key: &'a str, val_len: usize) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
//     map(separated_pair(parse_key(key), nom::sequence::tuple((char(':'), multispace0)), parse_value(val_len, char::is_alphanumeric)), |(x, y): (&str, &str)| y)
// }
fn parse_preceded_value<'a>(n: usize, condition: impl Fn(char) -> bool + 'a + Copy) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> + 'a {
    move |input: &'a str| {
    let (left, tuple) = nom::sequence::tuple((char(':'), multispace0))(input)?;
        let result = parse_value(n, condition)(left);
        println!("{:?}", result);
        result
    }
}
fn parse_value<'a>(n: usize, condition: impl Fn(char) -> bool) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), take_while_m_n(n, n, condition), char('"'))
}

fn parse_unknown_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    // preceded(char('\"'), escaped())
    delimited(char('\"'), nom::bytes::complete::take_till(|c|c == '\"'), char('\"'))
}
fn parse_key<'a, 'b: 'a>(key: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('\"'), tag(key), char('\"'))
}
#[cfg(test)]
mod tests {
    use nom::character::is_alphabetic;
    use nom::error::VerboseError;
    use super::*;
    #[test]
    fn it_works() {
        let mut keys= HashMap::from([("type".to_string(), 7usize),
        ("public_key".to_string(), 64),
        ("peer_id".to_string(), 52),
        ("signature".to_string(), 128),
        ("payload_type".to_string(), 0),
        ("payload".to_string(), 38)
        ]);
        let input = r#"{"type":"Ed25519","public_key":"1847179d3572cc1b80516ba49096efada0f1930632ab16a9f10bf24ce2c360c2","peer_id":"12D3KooWBT8pyJAfWJhdeGYAtKvcaUmm78ExyZ6uo6BEimYNVat1","signature":"8e5d1f0c977fc3017135032610f1cc40e5774be436e5634d090f1df52eeb401763964031070b6baa8e2b878477cc75b5a269b6bbc4da548b89ffcaac9e4db50e","payload_type":"","payload":"12112f636f64612f79616d75782f312e302e30"}"#;
        // let res = delimited(char('{'), parse_pair(&mut keys), char('}'))(input);
        // let res = parse_pair(&mut keys)(input);
        let res = parse_json(&mut keys)(input);
        println!("{:?}", res);
    }
}
