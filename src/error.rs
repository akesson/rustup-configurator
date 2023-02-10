use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustupTargetError {
    #[error("Failed to run process")]
    ProcessFailed(#[from] std::io::Error),
    #[error("rustup exited with code {exitcode:?} and stderr {stderr:?} and stdout {stdout:?}")]
    RustupError {
        exitcode: Option<i32>,
        stderr: String,
        stdout: String,
    },
    #[error("Rustup returned an invalid triple '{line}' due to '{source}', please raise an issue at: https://github.com/akesson/rustup-target/issues")]
    InvalidRustupTriple {
        line: String,
        #[source]
        source: target_lexicon::ParseError,
    },
}
