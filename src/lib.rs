//! This crate provides a simple interface to the rustup target command for listing and adding targets.
//! It uses the [target-lexicon](https://crates.io/crates/target-lexicon) Triple to identify targets.
//!
//! ```rust
//! use rustup_target::Triple;
//!
//! // get a list of all targets and if they are installed
//! let list: Vec<(Triple, bool)> = rustup_target::list().unwrap();
//!
//! // get all installed targets
//! let installed: Vec<Triple> = rustup_target::installed().unwrap();
//! 
//! // install some targets
//! rustup_target::install(&["aarch64-apple-ios".parse().unwrap()]).unwrap();
//! ```
mod error;

use std::{process::Command, str::FromStr};

pub use error::RustupTargetError;

// re-exported Triple
pub use target_lexicon::Triple;

/// List all available rust targets using the `rustup target list` command
///
/// Returns a list of targets triples and a bool indicating if the target is installed
pub fn list() -> Result<Vec<(Triple, bool)>, RustupTargetError> {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .output()
        .map_err(|e| RustupTargetError::ProcessFailed(e))?;

    let out = extract_stdout(&output)?;

    parse_rustup_triple_list(&out)
}

/// List all installed rust targets using the `rustup target list` command
///
/// Returns a list of targets triples and a bool indicating if the target is installed
pub fn installed() -> Result<Vec<Triple>, RustupTargetError> {
    Ok(list()?.into_iter().filter(|(_, inst)| *inst).map(|(t, _)| t).collect())
}

/// Install a list of rust targets, using the `rustup target add` command
pub fn install(list: &[Triple]) -> Result<(), RustupTargetError> {
    let mut cmd = Command::new("rustup");
    cmd.arg("target").arg("add");
    for triple in list {
        cmd.arg(triple.to_string());
    }
    let output = cmd
        .output()
        .map_err(|e| RustupTargetError::ProcessFailed(e))?;

    // just making sure that no error was returned
    let _out = extract_stdout(&output)?;

    Ok(())
}

fn extract_stdout(output: &std::process::Output) -> Result<String, RustupTargetError> {
    let cleaned = strip_ansi_escapes::strip(&output.stdout).expect("Could not strip ansii escapes");
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

fn parse_rustup_triple_list(list: &str) -> Result<Vec<(Triple, bool)>, RustupTargetError> {
    let mut triples = Vec::new();
    for mut line in list.lines() {
        let installed = if line.ends_with(" (installed)") {
            line = &line[..line.len() - 12];
            true
        } else {
            false
        };
        let triple = match Triple::from_str(line) {
            Err(e) => {
                return Err(RustupTargetError::InvalidRustupTriple {
                    line: line.to_string(),
                    source: e,
                })
            }
            Ok(t) => t,
        };
        triples.push((triple, installed));
    }
    Ok(triples)
}

#[test]
fn test_parse_list() {
    let list = r###"aarch64-apple-darwin (installed)
aarch64-apple-ios (installed)
aarch64-apple-ios-sim (installed)
aarch64-fuchsiaarch64-linux-android (installed)
aarch64-pc-windows-msvc"###;

    let triples = parse_rustup_triple_list(list).unwrap();

    let re_composed = format!(
        "{}",
        triples
            .iter()
            .map(|(t, inst)| format!("{} - installed: {inst:?}", t.to_string()))
            .collect::<Vec<_>>()
            .join("\n")
    );
    insta::assert_snapshot!(re_composed, @r###"
    aarch64-apple-darwin - installed: true
    aarch64-apple-ios - installed: true
    aarch64-apple-ios-sim - installed: true
    aarch64-fuchsiaarch64-linux-android - installed: true
    aarch64-pc-windows-msvc - installed: false
    "###
    );
}

#[test]
fn test_parse_list_wrong_arch() {
    let list = r###"aarch64-apple-darwin (installed)
arch64-apple-ios (installed)
aarch64-pc-windows-msvc"###;

    let err = parse_rustup_triple_list(list).unwrap_err();
    insta::assert_snapshot!(err.to_string(), 
    @"Rustup returned an invalid triple 'arch64-apple-ios' due to 'Unrecognized architecture: arch64', please raise an issue at: https://github.com/akesson/rustup-target/issues");
}
