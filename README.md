# simple-home-dir

An extremely small and optimized library purposed to retrieve the user's home directory.

## How to Use:
```rust
use simple_home_dir::*;

fn main() {
    let path = home_dir().unwrap();
    println!("{}", path.display())
}
```
And that's it!
```rust
use simple_home_dir::expand_tilde;

fn main() {
    //  "/home/USER/.local/share"
    let expanded = simple_home_dir::expand_tilde("~/.local/share").unwrap();
}
```
There's also the `expand_tilde` feature (disabled by default) which can *expand* upon the tilde from a given path.

## Compatibility
This works on most operating systems.

### Credit
The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
