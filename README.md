cc_license
=====
A Rust library for parsing Creative Commons license URLs.

[![Build status](https://github.com/thoth-pub/cc-license/workflows/test-and-check/badge.svg)](https://github.com/thoth-pub/cc-license/actions)
[![Crates.io](https://img.shields.io/crates/v/cc_license.svg)](https://crates.io/crates/cc_license)

### Usage

To bring this crate into your repository, either add `cc_license` to your
`Cargo.toml`, or run `cargo add cc_license`.

Here's an example parsing a CC license URL:

```rust
use cc_license::License;

fn main() {
    let license = License::from_url("https://creativecommons.org/licenses/by-nc-sa/4.0/")?;

    assert_eq!(license.to_string(), "Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International license (CC BY-NC-SA 4.0).".to_string());
    assert_eq!(license.rights(), "CC BY-NC-SA".to_string());
    assert_eq!(license.rights_full(), "Attribution-NonCommercial-ShareAlike".to_string());
    assert_eq!(license.version(), "4.0".to_string());
    assert_eq!(license.short(), "CC BY-NC 4.0".to_string());
}
```
