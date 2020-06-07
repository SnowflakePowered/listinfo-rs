use nom::{
    bytes::complete::is_not,
    bytes::complete::take_till1,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    character::complete::multispace0,
    combinator::map_res,
    sequence::delimited,
    sequence::tuple,
    sequence::pair,
    multi::many0,
    branch::alt,
    IResult,
};

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
    let (input, key) = take_till1(|c| c == ' ' || c == '\n')(input)?;
    Ok((input, key))
}

fn parse_string_key(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = multispace0(input)?;
    let (input, key) = string_key(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, value) = alt((quoted_string, unquoted_string))(input)?;
    Ok((input, (key, value)))
}

fn parse_sub_entry(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = multispace0(input)?;
    let (input, key) = string_key(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, contents) = subentry_contents(input)?;
    Ok((input, (key, contents)))
}

pub fn parse_header<'a>(input: &'a str) -> IResult<&'a str, Header<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("clrmamepro")(input)?;
    let (input, _) = open_entry(input)?;
    
    let mut map = BTreeMap::new();
    let (input, keys) = many0(parse_string_key)(input)?;
    let (input, _) = close_entry(input)?;

    for (key, value) in keys {
        map.insert(key, value);
    }
    Ok((input, Header::new(map)))
}

pub fn parse_rom_entry<'a>(input: &'a str) -> IResult<&'a str, RomEntry<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, keys) = many0(parse_string_key)(input)?;

    let mut name: Option<&'a str> = None;
    let mut size: Option<u64> = None;
    let mut crc: Option<&'a str> = None;
    let mut md5: Option<&'a str> = None;
    let mut sha1: Option<&'a str> = None;
    let mut name: Option<&'a str> = None;
    let mut merge: Option<&'a str> = None;

    for (key, value) in keys {
        match key {
            "name" => name = Some(value),
            "crc" => crc = Some(value),
            "md5" => md5 = Some(value),
            "sha1" => sha1 = Some(value),
            "merge" => merge = Some(value),
            "size" => size = value.parse::<u64>().ok(),
            _ => ()
        }
    }

    Ok((input, RomEntry {
        name,
        merge,
        size,
        crc,
        md5,
        sha1
    }))

}

pub fn parse_game_entry<'a, 'b>(entry_type: &'b str, input: &'a str) -> IResult<&'a str, GameEntry<'a>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(entry_type)(input)?;
    let (input, _) = open_entry(input)?;
    
    let mut map = BTreeMap::new();
    let mut roms =  Vec::new();
    let mut disks =  Vec::new();
    let mut samples = Vec::new();

    let (input, keys) = many0(alt((parse_sub_entry, parse_string_key)))(input)?;
    for (key, value) in keys {
        match key {
            "rom" => {
                let (_, rom) = parse_rom_entry(value)?;
                roms.push(rom);
            }
            "disk" => {
                let (_, disk) = parse_rom_entry(value)?;
                disks.push(disk);
            }
            "sample" => {
                samples.push(value);
            }
            _ => { map.insert(key, value); }
        }
    }
    let (input, _) = close_entry(input)?;
    Ok((input, GameEntry::new(map, roms, disks, samples)))
}