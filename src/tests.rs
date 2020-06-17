use crate::elements::*;
use crate::parse;

#[cfg(not(feature="std"))]
use alloc::string::String;

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
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Test"))
    );
    assert_eq!(
        header.entry_unique("description"),
        Some(&EntryData::Scalar("Test Description"))
    );
    assert_eq!(
        header.entry_unique("version"),
        Some(&EntryData::Scalar("42069"))
    );
    assert_eq!(
        header.entry_unique("author"),
        Some(&EntryData::Scalar("TestAuthor"))
    );
}

#[test]
fn test_parse_header_usability() {
    const HEADER: &str = r#"clrmamepro (
            name "Test"
            description "Test Description"
            category TestCategory
            version 42069
            author "TestAuthor"
        )"#;

    let header_str = String::from(HEADER);
    let (_, header) = parse::parse_fragment(&header_str).unwrap();
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Test"))
    );
    assert_eq!(
        header.entry_unique("description"),
        Some(&EntryData::Scalar("Test Description"))
    );
    assert_eq!(
        header.entry_unique("version"),
        Some(&EntryData::Scalar("42069"))
    );
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

    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Test"))
    );
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Test"))
    );
}

#[test]
fn test_parse_game() {
    const GAME: &str = r#"game (
            name "Test"
            description "SCPH-101 (Version 4.4 03/24/00 A)"
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 crc hellono md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
            rom ( name psone-44a.bin size 524288 crc 6a0e22a0 crc hellono md5 9a09ab7e49b422c007e6d54d7c49b965 sha1 7771d6e90980408f753891648685def6dd42ef6d )
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
                assert_eq!(sub.value_unique("name"), Some("psone-44a.bin"));
                assert_eq!(sub.value_unique("size"), Some("524288"));
                assert_eq!(sub.value_unique("crc"), Some("6a0e22a0"));
                assert_eq!(sub.value_iter("crc").unwrap().nth(1), Some("hellono"));
                assert_eq!(
                    sub.value_unique("sha1"),
                    Some("7771d6e90980408f753891648685def6dd42ef6d")
                );
                assert_eq!(
                    sub.value_unique("md5"),
                    Some("9a09ab7e49b422c007e6d54d7c49b965")
                );
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
            assert_eq!(
                fragment.entry_unique("name"),
                Some(&EntryData::Scalar("Test"))
            );
            assert_eq!(
                fragment.entry_unique("description"),
                Some(&EntryData::Scalar("Test Description"))
            );
            assert_eq!(
                fragment.entry_unique("version"),
                Some(&EntryData::Scalar("42069"))
            );
            assert_eq!(
                fragment.entry_unique("author"),
                Some(&EntryData::Scalar("TestAuthor"))
            );
        }
    }
    let games = doc.entry("game");
    if let Some(games) = games {
        for game in games {
            assert_eq!(
                game.entry_unique("name"),
                Some(&EntryData::Scalar("psone-44a"))
            );
            assert_eq!(
                game.entry_unique("description"),
                Some(&EntryData::Scalar("SCPH-101 (Version 4.4 03/24/00 A)"))
            );

            let iter = game.entry_iter("rom");
            if let Some(roms) = iter {
                for rom in roms {
                    if let EntryData::SubEntry(sub) = rom {
                        assert_eq!(sub.value_unique("name"), Some("psone-44a.bin"));
                        assert_eq!(sub.value_unique("size"), Some("524288"));
                        assert_eq!(sub.value_unique("crc"), Some("6a0e22a0"));
                        assert_eq!(
                            sub.value_unique("sha1"),
                            Some("7771d6e90980408f753891648685def6dd42ef6d")
                        );
                        assert_eq!(
                            sub.value_unique("md5"),
                            Some("9a09ab7e49b422c007e6d54d7c49b965")
                        );
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
    }
}

#[test]
fn parse_cave_story() {
    const CAVE_STORY: &str = r#"clrmamepro (
                name "Cave Story"
                description "Cave Story"
                version 20161204
                comment "libretro | www.libretro.com"
            )
            game (
                name "Cave Story (En)"
                description "Cave Story (En)"
                developer "Studio Pixel"
                releaseyear "2004"
                rom ( 
                    name "Doukutsu.exe"
                    size 1478656 
                    crc c5a2a3f6 
                    md5 38695d3d69d7a0ada8178072dad4c58b 
                    sha1 bb2d0441e073da9c584f23c2ad8c7ab8aac293bf
                )
            )
        "#;

    let document = parse::parse_document(CAVE_STORY).unwrap();
    let header = document.entry("clrmamepro").unwrap().next().unwrap();
    let game = document.entry("game").unwrap().next().unwrap();
    let rom = game.entry_unique("rom").unwrap();
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Cave Story"))
    );
    assert_eq!(
        game.entry_unique("name"),
        Some(&EntryData::Scalar("Cave Story (En)"))
    );
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Cave Story"))
    );
    if let EntryData::SubEntry(rom) = rom {
        assert_eq!(rom.value_unique("name"), Some("Doukutsu.exe"))
    }
}

#[test]
fn parse_inner_braces() {
    const TEST_FRAGMENT: &str = r#"
    game (
        rom ( name ps-22j(v).bin size 1048576 crc 446ec5b2 md5 81328b966e6dcf7ea1e32e55e1c104bb sha1 15c94da3cc5a38a582429575af4198c487fe893c )
    )
    "#;

    let (_, fragment) = parse::parse_fragment(TEST_FRAGMENT).unwrap();
    if let Some(EntryData::SubEntry(rom)) = fragment.entry_unique("rom") {
        assert_eq!(Some("ps-22j(v).bin"), rom.value_unique("name"))
    } else {
        panic!()
    }
    
}