This crate provides a simple interface to the `rustup target` command for listing and adding targets.
It uses the [target-lexicon](https://crates.io/crates/target-lexicon) Triple to identify targets.

```rust
use rustup_configurator::Triple;

// get a list of all targets and if they are installed
let list: Vec<(Triple, bool)> = rustup_configurator::list().unwrap();

// get all installed targets
let installed: Vec<Triple> = rustup_configurator::installed().unwrap();

// install some targets
rustup_configurator::install(&["aarch64-apple-ios".parse().unwrap()]).unwrap();
```

# Contributions

Contributions are welcome! Please open an issue or PR on [GitHub](https://github.com/akesson/rustup-configurator)
