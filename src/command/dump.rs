use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, default_value = ".", help = "trash file path")]
    pub path: String,

    #[clap(short, long, help = "trash file size")]
    pub size: String,

    #[clap(short, long, help = "trash file size")]
    pub count: Option<u32>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "dump", about = "Create trash files")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
