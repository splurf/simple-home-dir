# simple-home-dir
[![Crate](https://img.shields.io/crates/v/simple-home-dir.svg)](https://crates.io/crates/simple-home-dir)

An extremely tiny and reliable library purposed to retrieve the user's home directory.

## Usage
```rust
// /home/<USER>
home_dir().unwrap();
```
The `expand_tilde` feature (disabled by default) *expands* upon the tilde from a given path.
```rust
// /home/<USER>/.local
expand_tilde("~/.local").unwrap();
```

## Testing
The [dirs](https://crates.io/crates/dirs) crate is relied upon to ensure that the functions of this crate are working properly.

## Credit
The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
