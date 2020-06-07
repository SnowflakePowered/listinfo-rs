use nom::{
    branch::alt,
    bytes::complete::is_not,
    bytes::complete::take_till1,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    character::complete::multispace0,
    combinator::complete,
    combinator::map_res,
    multi::many0,
    multi::many1,
    sequence::delimited,
    sequence::pair,
    sequence::tuple,
    IResult,
};

enum ParsedValue<'a> {
    Subentry(&'a str),
    Value(&'a str),
}

use std::collections::BTreeMap;

use crate::elements::*;

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

fn parse_string_key(input: &str) -> IResult<&str, (&str, ParsedValue)> {
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

fn parse_sub_entry_data<'a>(input: &'a str) -> IResult<&'a str, SubEntryData<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, keys) = complete(many1(parse_string_key))(input)?;

    let mut map = BTreeMap::new();
    for (key, value) in keys {
        match value {
            ParsedValue::Value(value) => { map.insert(key, value); },
            _ => unreachable!()
        }
    }
    Ok((input, SubEntryData { keys: map }))
}

pub fn parse_fragment<'a, 'b>(
    input: &'a str,
) -> IResult<&'a str, InfoEntry<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = string_key(input)?;
    let (input, _) = open_entry(input)?;

    let mut map = BTreeMap::new();

    let (input, keys) = many0(alt((parse_sub_entry, parse_string_key)))(input)?;
    for (key, value) in keys {
        match value {
            ParsedValue::Subentry(value) => {
                if let Ok((_, subentry)) = parse_sub_entry_data(value) {
                    if let Some(node) = map.remove(key) {
                        match node {
                            InfoNode::Unique(prev) => {
                                map.insert(key, InfoNode::Multiple(vec![prev, EntryData::Node(subentry)]));
                            }
                            InfoNode::Multiple(mut prevs) => {
                                prevs.push(EntryData::Node(subentry));
                                map.insert(key, InfoNode::Multiple(prevs));
                            }
                        }
                    } else {
                        map.insert(key, InfoNode::Unique(EntryData::Node(subentry)));
                    }
                }
            },
            ParsedValue::Value(value) => {
                if let Some(node) = map.remove(key) {
                    match node {
                        InfoNode::Unique(prev) => {
                            map.insert(key, InfoNode::Multiple(vec![prev, EntryData::Value(value)]));
                        }
                        InfoNode::Multiple(mut prevs) => {
                            prevs.push(EntryData::Value(value));
                            map.insert(key, InfoNode::Multiple(prevs));
                        }
                    }
                } else {
                    map.insert(key, InfoNode::Unique(EntryData::Value(value)));
                }
            }
        }
    }
    let (input, _) = close_entry(input)?;
    Ok((input, InfoEntry::new(map)))
}
