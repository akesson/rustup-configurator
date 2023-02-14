# rustup target

This crate provides a simple interface to the rustup target command for listing and adding targets.
It uses the [target-lexicon](https://crates.io/crates/target-lexicon) Triple to identify targets.

```rust
use rustup_target::Triple;

// get a list of all targets and if they are installed
let list: Vec<(Triple, bool)> = rustup_target::list().unwrap();

// get all installed targets
let installed: Vec<Triple> = rustup_target::installed().unwrap();

// install some targets
rustup_target::install(&["aarch64-apple-ios".parse().unwrap()]).unwrap();
```
