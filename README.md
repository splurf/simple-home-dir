# simple-home-dir
[![Crate](https://img.shields.io/crates/v/simple-home-dir.svg)](https://crates.io/crates/simple-home-dir)

An extremely small and optimized library purposed to retrieve the user's home directory.

## Usage
```rust
// "/home/<USER>"
simple_home_dir::home_dir().unwrap();
```
The `expand_tilde` feature (disabled by default) *expands* upon the tilde from a given path.
```rust
// "/home/<USER>/.local"
simple_home_dir::expand_tilde("~/.local").unwrap();
```
---
### Credit
> The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
