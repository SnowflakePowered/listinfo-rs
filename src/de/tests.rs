#![cfg(feature = "test_deserialize")]
use alloc::string::String;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Header {
    name: String,
    description: String,
    version: String,
    comment: String,
}

#[derive(Debug, Deserialize)]
struct Game {
    name: String,
    releaseyear: u32,
    developer: String,
    rom: Vec<Rom>,
}

#[derive(Debug, Deserialize)]
struct Rom {
    name: String,
    size: u64,
    // Supports serialize hex strings to byte arrays
    #[serde(with = "serde_bytes")]
    crc: Vec<u8>,
    #[serde(with = "serde_bytes")]
    md5: Vec<u8>,
    #[serde(with = "serde_bytes")]
    sha1: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct CaveStory {
    clrmamepro: Header,
    game: Vec<Game>,
}

#[test]
fn deserialize_cave_story() {
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

    let cave_story = super::from_str::<CaveStory>(CAVE_STORY).unwrap();
    assert_eq!(cave_story.clrmamepro.name, "Cave Story");
    assert_eq!(
        cave_story.game.first().unwrap().rom.first().unwrap().name,
        "Doukutsu.exe"
    );
    assert_eq!(
        cave_story.game.first().unwrap().rom.first().unwrap().size,
        1478656
    );
    assert_eq!(
        cave_story.game.first().unwrap().rom.first().unwrap().crc,
        &[0xc5, 0xa2, 0xa3, 0xf6]
    );
}
