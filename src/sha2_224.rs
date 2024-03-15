use chksum::SHA2_224;

use crate::{subcommand, Args, Options};

/// Calculate SHA-2 224 digest.
#[derive(Debug, clap::Args)]
pub struct Subcommand {
    #[command(flatten)]
    args: Args,
    #[command(flatten)]
    options: Options,
}

impl Subcommand {
    /// Handles subcommand execution.
    pub fn execute(&self) -> i32 {
        let Self { args, options } = self;
        subcommand::<SHA2_224>(args, options)
    }
}
