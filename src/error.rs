use std::io;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("The 1password CLI path is not valid")]
    InvalidOpPath,
    #[error("Could not execute 1password CLI")]
    OpExec(#[source] io::Error),
    #[error("Could not read 1password item: {0}")]
    OpRead(String),
    #[error("Could not parse 1password item output")]
    OpReadItemJson(#[source] serde_json::Error),
    #[error("Could not serialize secret as YAML")]
    SerializeYamlSecret(#[source] serde_yaml::Error),
}
