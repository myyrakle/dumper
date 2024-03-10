use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, default_value = "false", help = "trash file size")]
    pub size: String,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "dump", about = "Create trash files")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
