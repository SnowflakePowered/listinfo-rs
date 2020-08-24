# listinfo-rs

[![Latest Version](https://img.shields.io/crates/v/listinfo.svg)](https://crates.io/crates/listinfo) [![Docs](https://docs.rs/listinfo/badge.svg)](https://docs.rs/listinfo) ![License](https://img.shields.io/crates/l/listinfo)


A zero-copy MAME ListInfo format DAT files parser. 

---

## Usage
listinfo-rs provides a lower-level zero-copy expression tree API as well as a more user friendly Serde deserialization API. See the crate documentation or tests for more detail examples.

```rust
#[test]
fn parse_cave_story() {
    const CAVE_STORY: &str =
     r#"clrmamepro (
        name "Cave Story"
        description "Cave Story"
        version 20161204
        comment "libretro | www.libretro.com"
    )"#;

    let document = parse::parse_document(CAVE_STORY).unwrap();
    let header = document.entry("clrmamepro").unwrap().next().unwrap();
    let game = document.entry("game").unwrap().next().unwrap();
    let rom = game.entry_unique("rom").unwrap();
    assert_eq!(
        header.entry_unique("name"),
        Some(&EntryData::Scalar("Cave Story"))
    );
}
```

## Serde Deserialization 
listinfo-rs supports deserialization with `serde`, after enabling support in `Cargo.toml`

```toml
listinfo = { version = "0.4", features = ["deserialize"] }
```

Deserialization works like any other Serde deserializer crate.

```rust
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

    let cave_story = listinfo::de::from_str::<CaveStory>(CAVE_STORY).unwrap();
    assert_eq!(cave_story.clrmamepro.name, "Cave Story");
    assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().name, "Doukutsu.exe");
    assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().size, 1478656);
    assert_eq!(cave_story.game.first().unwrap().rom.first().unwrap().crc, &[0xc5, 0xa2, 0xa3, 0xf6]);
}
```

## `no_std`
listinfo-rs supports `no_std`, but requires `alloc`.

You can disable `std` support in `Cargo.toml`. 

```toml
listinfo = { version = "0.4", default-features = false }
```
