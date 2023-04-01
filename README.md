# simple-home-dir

An extremely small library purposed to retrieve the user's home directory.

## How to Use:
```rust
use simple_home_dir::*;

fn main() {
    let path = home_dir().unwrap();
    println!("{}", path.display())
}
```
And that's it!

## Compatibility
This works on most operating systems.

### Credit
The majority of the Windows portion of this has been noted from the [windows-sys](https://crates.io/crates/windows-sys) and [directories](https://crates.io/crates/directories) crates.
