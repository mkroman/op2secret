//! Integration with the 1Password CLI

use std::{path::Path, process::Command};

use serde::Deserialize;

use crate::Error;

pub const DEFAULT_OP_BIN: &str = "op";

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Item {
    /// The ID of the item.
    pub id: String,
    /// The title of the item.
    pub title: String,
    /// List of sections
    pub sections: Vec<Section>,
    /// List of fields
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
/// Item section.
pub struct Section {
    pub id: String,
    pub label: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub purpose: Option<String>,
    pub label: String,
    pub section: Option<Section>,
    /// Reference URL of the field
    pub reference: Option<String>,
    /// Value of the field
    pub value: Option<String>,
}

pub fn get(op_path: &Path, reference: &str) -> Result<Item, Error> {
    let mut cmd = Command::new(op_path);
    let cmd = cmd
        .arg("item")
        .arg("get")
        .args(["--format", "json"])
        .arg(reference);

    if let Ok(output) = cmd.output() {
        let item: Item = serde_json::from_slice(&output.stdout).map_err(Error::OpReadItemJson)?;

        Ok(item)
    } else {
        Err(Error::OpReadItem)
    }
}
