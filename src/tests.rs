use crate::elements::*;
use crate::parse;

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
    assert_eq!(header.entry_unique("name"), Some(&EntryData::Scalar("Test")));
    assert_eq!(
        header.entry_unique("description"),
        Some(&EntryData::Scalar("Test Description"))
    );
    assert_eq!(header.entry_unique("version"), Some(&EntryData::Scalar("42069")));
    assert_eq!(
        header.entry_unique("author"),
        Some(&EntryData::Scalar("TestAuthor"))
    );
}

#[test]
fn test_parse_singular_iter() {
    const HEADER: &str = r#"clrmamepro (
            name "Test"
            description "Test Description"
            category TestCategory
            version 42069
            author "TestAuthor"
        )"#;
    let (_, header) = parse::parse_fragment(HEADER).unwrap();

    // Test singular iterator
    for val in header.entry_iter("name").unwrap() {
        assert_eq!(val, &EntryData::Scalar("Test"));
    }
}

#[test]
fn test_parse_multi_unique() {
    const HEADER: &str = r#"clrmamepro (
            name "Test"
            name "Test 2"
        )"#;
    let (_, header) = parse::parse_fragment(HEADER).unwrap();

    assert_eq!(header.entry_unique("name"), Some(&EntryData::Scalar("Test")));
    assert_eq!(header.entry_unique("name"), Some(&EntryData::Scalar("Test")));
}

#[test]
fn test_parse_game() {
    const GAME: &str = r#"game (
            name "Test"
            description "SCPH-101 (Version 4.4 03/24/00 A)"
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            sample hello
            sample hello
        )"#;

    let (_, game) = parse::parse_fragment(GAME).unwrap();
    assert_eq!(game.entry_unique("name"), Some(&EntryData::Scalar("Test")));
    assert_eq!(
        game.entry_unique("description"),
        Some(&EntryData::Scalar("SCPH-101 (Version 4.4 03/24/00 A)"))
    );

    let iter = game.entry_iter("rom");
    if let Some(roms) = iter {
        for rom in roms {
            if let EntryData::SubEntry(sub) = rom {
                assert_eq!(sub.value("name"), Some("psone-44a.bin"));
                assert_eq!(sub.value("size"), Some("524288"));
                assert_eq!(sub.value("crc"), Some("6a0e22a0"));
                assert_eq!(
                    sub.value("sha1"),
                    Some("7771d6e90980408f753891648685def6dd42ef6d")
                );
                assert_eq!(sub.value("md5"), Some("9a09ab7e49b422c007e6d54d7c49b965"));
            } else {
                unreachable!()
            }
        }
    }

    let iter = game.entry_iter("sample");
    if let Some(sample) = iter {
        for sample in sample {
            if let &EntryData::Scalar(value) = sample {
                assert_eq!(value, "hello");
            } else {
                unreachable!()
            }
        }
    }
}

#[test]
fn test_parse_document() {
    const DOCUMENT: &str = r#"clrmamepro (
            name "Test"
            description "Test Description"
            category TestCategory
            version 42069
            author "TestAuthor"
        )
        game (
            name "psone-44a"
            description "SCPH-101 (Version 4.4 03/24/00 A)"
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            sample hello
        )
        game (
            name "psone-44a"
            description "SCPH-101 (Version 4.4 03/24/00 A)"
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            sample hello
        )"#;

    let doc = parse::parse_document(DOCUMENT).unwrap();
    if let Some(header) = doc.entry("clrmamepro") {
        for fragment in header {
            assert_eq!(fragment.unique("name"), Some(&EntryData::Scalar("Test")));
            assert_eq!(
                fragment.unique("description"),
                Some(&EntryData::Scalar("Test Description"))
            );
            assert_eq!(
                fragment.unique("version"),
                Some(&EntryData::Scalar("42069"))
            );
            assert_eq!(
                fragment.unique("author"),
                Some(&EntryData::Scalar("TestAuthor"))
            );
        }
    }
    let games = doc.entry("game");
    if let Some(games) = games {
        for game in games {
            assert_eq!(game.unique("name"), Some(&EntryData::Scalar("psone-44a")));
            assert_eq!(
                game.unique("description"),
                Some(&EntryData::Scalar("SCPH-101 (Version 4.4 03/24/00 A)"))
            );

            let iter = game.iter("rom");
            if let Some(roms) = iter {
                for rom in roms {
                    if let EntryData::SubEntry(sub) = rom {
                        assert_eq!(sub.value("name"), Some("psone-44a.bin"));
                        assert_eq!(sub.value("size"), Some("524288"));
                        assert_eq!(sub.value("crc"), Some("6a0e22a0"));
                        assert_eq!(
                            sub.value("sha1"),
                            Some("7771d6e90980408f753891648685def6dd42ef6d")
                        );
                        assert_eq!(sub.value("md5"), Some("9a09ab7e49b422c007e6d54d7c49b965"));
                    } else {
                        unreachable!()
                    }
                }
            }

            let iter = game.iter("sample");
            if let Some(sample) = iter {
                for sample in sample {
                    if let &EntryData::Scalar(value) = sample {
                        assert_eq!(value, "hello");
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }
}
