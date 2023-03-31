# simple-home-dir

An extremely small library that contains the one function that it needs to get the job done.

## How to Use:
```rust
use simple_home_dir::*;

fn main() {
    let home = home_dir().unwrap();
    println!("{:?}", home)
}
```
And that's it!

## Compatibility
This should work on most operating systems.

### Credit
The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
