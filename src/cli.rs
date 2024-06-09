use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Generate a Kubernetes Secrets manifest from a 1Password secret.
pub struct Opts {
    /// reference to 1password secret
    #[argh(positional)]
    pub reference: String,

    /// optional name of the kubernetes secret.
    ///
    /// If not specified, the name will be derived from the secret title. Also see the description
    /// for `namespace'
    #[argh(positional)]
    pub name: Option<String>,

    /// type of the kubernetes secret (defaults to `Opaque')
    #[argh(option, short = 't', default = "String::from(\"Opaque\")")]
    pub type_: String,

    /// namespace of the kubernetes secret - if not specified, and the 1Password secret has a title
    /// in the format `<namespace>/<name>`, then the namespace will be derived from that
    #[argh(option, short = 'n')]
    pub namespace: Option<String>,

    /// path to `op` CLI executable (defaults to `op')
    #[argh(option, default = "default_op_bin_path()")]
    pub op_bin: PathBuf,
}

fn default_op_bin_path() -> PathBuf {
    if let Ok(path) = std::env::var("OP_BIN") {
        PathBuf::from(path)
    } else {
        PathBuf::from(crate::onepassword::DEFAULT_OP_BIN)
    }
}
