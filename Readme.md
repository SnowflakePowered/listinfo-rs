# listinfo-rs

A library to parse MAME ListInfo format DAT files. 


## `no_std`
listinfo-rs supports `no_std`, but requires `alloc`.

By default `std` is imported but you can disable this feature in `Cargo.toml`

```toml
listinfo = { version = "0.1", default-features = false }
```

Again, `alloc` is required and can not be disabled.