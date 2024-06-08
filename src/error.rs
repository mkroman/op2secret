use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The 1password CLI path is not valid")]
    InvalidOpPath,
    #[error("Could not read 1password item")]
    OpReadItem,
    #[error("Could not parse 1password item output")]
    OpReadItemJson(#[source] serde_json::Error),
    #[error("Could not serialize secret as YAML")]
    SerializeYamlSecret(#[source] serde_yaml::Error),
}
