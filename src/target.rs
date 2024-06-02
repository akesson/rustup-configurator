use std::{process::Command, str::FromStr};

use crate::{extract_stdout, RustupTargetError};

#[derive(Debug)]
pub struct Target {
    pub triple: TargetTriple,
    pub installed: bool,
}

#[derive(Debug)]
pub enum TargetTriple {
    Conied(target_lexicon::Triple),
    Uncoined(String),
}

impl FromStr for TargetTriple {
    type Err = target_lexicon::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match target_lexicon::Triple::from_str(s) {
            Ok(triple) => Ok(TargetTriple::Conied(triple)),
            Err(_) => Ok(TargetTriple::Uncoined(s.to_string())),
        }
    }
}

impl std::fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetTriple::Conied(triple) => write!(f, "{}", triple),
            TargetTriple::Uncoined(s) => write!(f, "{}", s),
        }
    }
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
pub fn install(list: &[target_lexicon::Triple]) -> Result<(), RustupTargetError> {
    let mut cmd = Command::new("rustup");
    cmd.arg("target").arg("add");
    for triple in list {
        cmd.arg(triple.to_string());
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
        // the Triple parsing of "wasm32-wasi-preview1-threads" fails
        // so we just skip it here.
        if line.contains("preview") {
            continue;
        }
        let triple = match TargetTriple::from_str(line) {
            Err(e) => {
                return Err(RustupTargetError::InvalidRustupTriple {
                    line: line.to_string(),
                    source: e,
                })
            }
            Ok(t) => t,
        };
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

#[test]
fn test_parse_list_uncoind_arch() {
    let list = r###"aarch64-apple-darwin (installed)
arch64-apple-ios (installed)
aarch64-pc-windows-msvc"###;

    let parsed_list = parse_rustup_triple_list(list).unwrap();
    let formated_list = format!("{:?}", parsed_list);

    insta::assert_snapshot!(formated_list, @r###"
    [Target { triple: Conied(Triple { architecture: Aarch64(Aarch64), vendor: Apple, operating_system: Darwin, environment: Unknown, binary_format: Macho }), installed: true }, Target { triple: Uncoined("arch64-apple-ios"), installed: true }, Target { triple: Conied(Triple { architecture: Aarch64(Aarch64), vendor: Pc, operating_system: Windows, environment: Msvc, binary_format: Coff }), installed: false }]
    "###
    );
}
