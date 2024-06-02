# Rustup Configurator

This crate provides a simple interface to the `rustup target` command for listing and adding targets in Rust. It's designed to make managing your Rust targets easier and more efficient.

## Usage

```rust
use rustup_configurator::target::Target;

// Get a list of all targets and if they are installed
let list: Vec<Target> = rustup_configurator::target::list().unwrap();

// Get all installed targets
let installed: Vec<Target> = rustup_configurator::target::installed().unwrap();

// Install some targets
rustup_configurator::target::install(&["aarch64-apple-ios".into()]).unwrap();
# Contributions
```

Contributions are welcome! Please open an issue or PR on [GitHub](https://github.com/akesson/rustup-configurator)
