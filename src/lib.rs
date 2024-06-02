//! This crate provides a simple interface to the rustup target command for listing and adding targets.
//! It uses the [target-lexicon](https://crates.io/crates/target-lexicon) Triple to identify targets.
//!
//! ```rust
//! use rustup_configurator::target::Target;
//!
//! // get a list of all targets and if they are installed
//! let list: Vec<Target> = rustup_configurator::target::list().unwrap();
//!
//! // get all installed targets
//! let installed: Vec<Target> = rustup_configurator::target::installed().unwrap();
//!
//! // install some targets
//! rustup_configurator::target::install(&["aarch64-apple-ios".into()]).unwrap();
//! ```
mod error;

pub use error::RustupTargetError;

pub mod target;

fn extract_stdout(output: &std::process::Output) -> Result<String, RustupTargetError> {
    let cleaned = strip_ansi_escapes::strip(&output.stdout);
    let out = String::from_utf8(cleaned).expect("Could not read utf8");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(RustupTargetError::RustupError {
            exitcode: output.status.code(),
            stderr: err,
            stdout: out,
        });
    }
    Ok(out)
}
