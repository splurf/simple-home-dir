# simple-home-dir
[![Crate](https://img.shields.io/crates/v/simple-home-dir.svg)](https://crates.io/crates/simple-home-dir)

An extremely small and optimized library purposed to retrieve the user's home directory.

## Usage
```rust
// "/home/<USER>"
let home_dir = simple_home_dir::home_dir().unwrap();
```
___
There's also the `expand_tilde` feature (disabled by default) which can *expand* upon the tilde from a given path.
```rust
// "/home/<USER>/.local"
let expanded = simple_home_dir::expand_tilde("~/.local").unwrap();
```

### Credit
> The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
