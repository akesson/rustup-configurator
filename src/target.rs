use std::process::Command;

use crate::{extract_stdout, RustupTargetError};

/// The unique identifier for a target.
pub type Triple = String;

#[derive(Debug)]
pub struct Target {
    pub triple: Triple,
    pub installed: bool,
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - installed: {}", self.triple, self.installed)
    }
}

/// List all available rust targets using the `rustup target list` command
///
/// Returns a list of targets triples and a bool indicating if the target is installed
pub fn list() -> Result<Vec<Target>, RustupTargetError> {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .output()
        .map_err(RustupTargetError::ProcessFailed)?;

    let out = extract_stdout(&output)?;

    parse_rustup_triple_list(&out)
}

/// List all installed rust targets using the `rustup target list` command
///
/// Returns a list of targets triples and a bool indicating if the target is installed
pub fn installed() -> Result<Vec<Target>, RustupTargetError> {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
        .map_err(RustupTargetError::ProcessFailed)?;
    let out = extract_stdout(&output)?;

    parse_rustup_triple_list(&out)
}

/// Install a list of rust targets, using the `rustup target add` command
pub fn install(list: &[Triple]) -> Result<(), RustupTargetError> {
    let mut cmd = Command::new("rustup");
    cmd.arg("target").arg("add");
    for triple in list {
        cmd.arg(triple);
    }
    let output = cmd.output().map_err(RustupTargetError::ProcessFailed)?;

    // just making sure that no error was returned
    let _out = extract_stdout(&output)?;

    Ok(())
}

fn parse_rustup_triple_list(list: &str) -> Result<Vec<Target>, RustupTargetError> {
    let mut targets = Vec::new();
    for mut line in list.lines() {
        let installed = if line.ends_with(" (installed)") {
            line = &line[..line.len() - 12];
            true
        } else {
            false
        };
        let triple = Triple::from(line);
        targets.push(Target { triple, installed });
    }
    Ok(targets)
}

#[test]
fn test_parse_list() {
    let list = r###"aarch64-apple-darwin (installed)
aarch64-apple-ios (installed)
aarch64-apple-ios-sim (installed)
aarch64-fuchsiaarch64-linux-android (installed)
aarch64-pc-windows-msvc"###;

    let triples = parse_rustup_triple_list(list).unwrap();

    let re_composed = triples
        .iter()
        .map(|t| format!("{t}"))
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(re_composed, @r###"
    aarch64-apple-darwin - installed: true
    aarch64-apple-ios - installed: true
    aarch64-apple-ios-sim - installed: true
    aarch64-fuchsiaarch64-linux-android - installed: true
    aarch64-pc-windows-msvc - installed: false
    "###
    );
}
