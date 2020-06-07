mod elements;
pub mod parse;

pub use elements::*;

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::elements::*;
    #[test]
    fn test_parse_header() {
        const HEADER: &str = r#"clrmamepro (
            name "Test"
            description "Test Description"
            category TestCategory
            version 42069
            author "TestAuthor"
        )"#;

        let (_, header) = parse::parse_fragment(HEADER).unwrap();
        assert_eq!(header.get_unique("name"), Some(&EntryData::Value("Test")));
        // assert_eq!(header.get("description"), Some("Test Description"));
        // assert_eq!(header.get("version"), Some("42069"));
        // assert_eq!(header.get("author"), Some("TestAuthor"));
    }

    #[test]
    fn test_parse_game() {
        const GAME: &str = r#"game (
            name "psone-44a"
            description "SCPH-101 (Version 4.4 03/24/00 A)"
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            sample hello.0
        )"#;

        let (_, game) = parse::parse_fragment(GAME).unwrap();
        println!("{:?}", game);

        // assert_eq!(header.get("name"), Some("Test"));
        // assert_eq!(header.get("description"), Some("Test Description"));
        // assert_eq!(header.get("version"), Some("42069"));
        // assert_eq!(header.get("author"), Some("TestAuthor"));
    }
}
