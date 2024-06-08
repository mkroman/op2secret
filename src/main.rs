use std::collections::BTreeMap;

use api::Secret;
use k8s_openapi::api::core::v1 as api;

mod cli;
mod error;
mod onepassword;

pub use error::Error;
pub use onepassword::Item;

fn map_from_item_fields(item: &Item) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();

    for field in &item.fields {
        // Skip item notes
        if field.purpose == Some("NOTES".to_string()) {
            continue;
        }

        // Only read fields with immediate values
        if let Some(value) = &field.value {
            result.insert(field.label.clone(), value.clone());
        }
    }

    result
}

fn secret_manifest_from_item(
    item: &Item,
    name: String,
    namespace: Option<String>,
    type_: String,
) -> Result<String, Error> {
    let mut secret = Secret::default();

    secret.metadata.name = Some(name);
    secret.metadata.namespace = namespace;
    secret.type_ = Some(type_);

    secret.string_data = Some(map_from_item_fields(item));

    serde_yaml::to_string(&secret).map_err(Error::SerializeYamlSecret)
}

fn main() -> Result<(), Error> {
    let opts: cli::Opts = argh::from_env();

    let item = onepassword::get(opts.op_bin.as_path(), opts.reference.as_str())?;
    let secret_str = secret_manifest_from_item(&item, opts.name, opts.namespace, opts.type_)?;

    println!("{secret_str}");

    Ok(())
}
