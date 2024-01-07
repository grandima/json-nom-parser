
use std::collections::{BTreeMap, HashMap};


use nom::character::complete::{char};
use nom::{IResult};
use nom::bytes::complete::{take_while_m_n};
use nom::Err::{Failure};
use nom::error::{Error, ErrorKind};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LEN_KV_SIZE: BTreeMap<usize, HashMap<String, usize>> = {
        let mut m = HashMap::new();
        m.insert("type".to_string(), 7);
        m.insert("public_key".to_string(), 64);
        m.insert("peer_id".to_string(), 52);
        m.insert("signature".to_string(), 128);
        m.insert("payload_type".to_string(), 0);
        m.insert("payload".to_string(), 38);
        let mut map = BTreeMap::new();
        m.iter().for_each(|entry|{
            let e = map.entry(entry.0.len()).or_insert(HashMap::new());
            e.insert(entry.0.clone(), *entry.1);
        });
        map
    };
}
pub fn parse_sized_json<'a, 'b: 'a>(keys: &'b mut BTreeMap<usize, HashMap<String, usize>>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    delimited(char('{'), parse_list(keys), char('}'))
}
fn parse_list<'a, 'b: 'a>(keys: &'b mut BTreeMap<usize, HashMap<String, usize>>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> + 'a {
    separated_list1(char(','), parse_pair(keys))
}
fn parse_pair<'a, 'b: 'a>(keys: &'b mut BTreeMap<usize, HashMap<String, usize>>) -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, &'a str)> + 'b  {
    move |input: &'a str| -> IResult<&'a str, (&'a str, &'a str)> {
        let start_len = keys.first_entry().map(|x|*x.key()).unwrap_or(0);
        let end_len = keys.last_entry().map(|x|*x.key()).unwrap_or(0);
        let (left, key) = parse_sized_key(start_len, end_len)(&input)?;

        let key_value_len = keys.get_mut(&key.len()).ok_or(Failure(Error::new(input, ErrorKind::Tag)))?;
        let value_size = key_value_len.remove(key).ok_or(Failure(Error::new(input, ErrorKind::Tag)))?;
        if key_value_len.is_empty() {
            keys.remove(&key.len());
        }
        parse_preceded_value(value_size, char::is_alphanumeric)(left).map(|(left, value)|(left, (key, value)))
    }
}
fn parse_preceded_value<'a, 'b: 'a>(n: usize, condition: impl Fn(char) -> bool + 'b + Copy) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> + 'a {
    preceded(char(':'), parse_sized_value(n, condition))
}
fn parse_sized_value<'a>(n: usize, condition: impl Fn(char) -> bool) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('"'), take_while_m_n(n, n, condition), char('"'))
}

fn parse_sized_key<'a>(m: usize, n :usize, ) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(char('\"'), take_while_m_n(m, n, |c|{
        c != '\"'
    }), char('\"'))
}