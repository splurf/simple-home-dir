# simple-home-dir
[![Crate](https://img.shields.io/crates/v/simple-home-dir.svg)](https://crates.io/crates/simple-home-dir)

A minimal, fast, and reliable crate dedicated to retrieving the user's home directory.

## Usage
```rust
use simple_home_dir::*;

fn main() {
    // Windows  =>  C:\Users\jdoe
    // Linux    =>  /home/jdoe
    // Mac      =>  /Users/jdoe
    let path = home_dir().unwrap();
}
```

## Features
The `expand_tilde` feature is available [here](https://crates.io/crates/simple-expand-tilde).

### Testing
The [dirs](https://crates.io/crates/dirs) crate is relied upon to ensure that the functions of this crate are working properly.

### Credit
The majority of the Windows portion has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
