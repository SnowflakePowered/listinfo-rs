use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::{char, multispace0},
    combinator::complete,
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::result::Result;

use crate::elements::*;
use crate::error::Error;

enum ParsedValue<'a> {
    Subentry(&'a str),
    Value(&'a str),
}

fn open_entry(input: &str) -> IResult<&str, char> {
    let (input, _) = multispace0(input)?;
    let (input, open) = char('(')(input)?;
    Ok((input, open))
}

fn close_entry(input: &str) -> IResult<&str, char> {
    let (input, _) = multispace0(input)?;
    let (input, close) = char(')')(input)?;
    Ok((input, close))
}

fn subentry_contents(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(input)
}

fn unquoted_string(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ' ' || c == '\n')(input)
}

fn string_key(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, key) = take_till1(|c| c == ' ' || c == '\n' || c == '"')(input)?;
    Ok((input, key))
}

fn parse_string_value(input: &str) -> IResult<&str, (&str, ParsedValue)> {
    let (input, _) = multispace0(input)?;
    let (input, key) = string_key(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, value) = alt((quoted_string, unquoted_string))(input)?;
    Ok((input, (key, ParsedValue::Value(value))))
}

fn parse_sub_entry(input: &str) -> IResult<&str, (&str, ParsedValue)> {
    let (input, _) = multispace0(input)?;
    let (input, key) = string_key(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, contents) = subentry_contents(input)?;
    Ok((input, (key, ParsedValue::Subentry(contents))))
}

fn parse_sub_entry_data<'a>(input: &'a str) -> IResult<&'a str, SubEntry<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, keys) = complete(many1(parse_string_value))(input)?;

    let mut map = BTreeMap::new();
    for (key, value) in keys {
        match value {
            ParsedValue::Value(value) => {
                if let Some(node) = map.remove(key) {
                    match node {
                        Node::Unique(prev) => {
                            map.insert(
                                key,
                                Node::Many(vec![prev, value]),
                            );
                        }
                        Node::Many(mut prevs) => {
                            prevs.push(value);
                            map.insert(key, Node::Many(prevs));
                        }
                    }
                } else {
                    map.insert(key,Node::Unique(value));
                }
            }
            _ => unreachable!(),
        }
    }
    Ok((input, SubEntry { keys: map }))
}

/// Parse multiple ListInfo entries as a document.
pub fn parse_document<'a>(input: &'a str) -> Result<DatDocument<'a>, Error> {
    let (_, fragments) = complete(many1(parse_fragment_internal))(input)?;
    let mut document: BTreeMap<&'a str, Vec<EntryFragment<'a>>> = BTreeMap::new();
    for (key, entry) in fragments {
        if let Some(existing) = document.get_mut(key) {
            existing.push(entry);
        } else {
            document.insert(key, vec![entry]);
        }
    }
    Ok(DatDocument { document })
}

/// Parse a single ListInfo entry, returning it's type and the entry.
pub fn parse_fragment<'a, 'b>(input: &'a str) -> Result<(&'a str, EntryFragment<'a>), Error> {
    let (_, fragment) = parse_fragment_internal(input)?;
    Ok(fragment)
}

fn parse_fragment_internal<'a, 'b>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, EntryFragment<'a>)> {
    let (input, _) = multispace0(input)?;
    let (input, entry_key) = string_key(input)?;
    let (input, _) = open_entry(input)?;

    let mut map = BTreeMap::new();

    let (input, keys) = many0(alt((parse_sub_entry, parse_string_value)))(input)?;
    for (key, value) in keys {
        match value {
            ParsedValue::Subentry(value) => {
                if let Ok((_, subentry)) = parse_sub_entry_data(value) {
                    if let Some(node) = map.remove(key) {
                        match node {
                            Node::Unique(prev) => {
                                map.insert(
                                    key,
                                    Node::Many(vec![prev, EntryData::SubEntry(subentry)]),
                                );
                            }
                            Node::Many(mut prevs) => {
                                prevs.push(EntryData::SubEntry(subentry));
                                map.insert(key, Node::Many(prevs));
                            }
                        }
                    } else {
                        map.insert(key, Node::Unique(EntryData::SubEntry(subentry)));
                    }
                }
            }
            ParsedValue::Value(value) => {
                if let Some(node) = map.remove(key) {
                    match node {
                        Node::Unique(prev) => {
                            map.insert(key, Node::Many(vec![prev, EntryData::Scalar(value)]));
                        }
                        Node::Many(mut prevs) => {
                            prevs.push(EntryData::Scalar(value));
                            map.insert(key, Node::Many(prevs));
                        }
                    }
                } else {
                    map.insert(key, Node::Unique(EntryData::Scalar(value)));
                }
            }
        }
    }
    let (input, _) = close_entry(input)?;
    Ok((input, (entry_key, EntryFragment::new(map))))
}
