#![cfg(feature="test_deserialize")]
use serde::Deserialize as De;

#[derive(Debug, De)]
struct Help(i32);
#[derive(Debug, De)]
struct TestStruct {
    // hello: Vec<u8>,
    // number: serde_bytes::ByteBuf,
    test: Option<String>
}

#[derive(Debug, De)]
struct Document {
    clrmamepro: Header
    // game: Game,
    // clrmamepro: std::collections::HashMap<String, String>,
}

#[derive(Debug, De)]
struct Header {
    name: String,
    description: String,
    category: String,
    version: i32,
    author: Vec<String>,
}

#[derive(Debug, De)]
struct Game {
    name: String,
}
#[derive(Debug, De)]
struct Rom {
    crc: String
}

#[test]
fn test_deserialize() {
    const HEADER: &str = r#"clrmamepro (
        name "Test"
        description "Test Description"
        category TestCategory
        version 42069
        author "TestAuthor"
        author "TestAuthor"
    )"#;

    let header_str = String::from(HEADER);
    // let (_, header) = crate::parse::parse_fragment(&header_str).unwrap();
    let doc = crate::parse::parse_document(&header_str).unwrap();
    let t = super::from_document::<Document>(&doc);
    println!("{:?}", t);
}
