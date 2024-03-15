use std::io::{stderr, stdout, Write};
use std::process;

use anyhow::Result;
#[cfg(feature = "color")]
use chksum_cli::Color;
use chksum_cli::{Command, Subcommand};
use clap::Parser;
use exitcode::USAGE as EXITCODE_USAGE;

/// Exits process with given return code.
fn exit(code: i32) -> ! {
    let _ = stdout().lock().flush();
    let _ = stderr().lock().flush();
    process::exit(code)
}

/// Main function.
fn main() -> Result<()> {
    let command = Command::try_parse().unwrap_or_else(|error| {
        let _ = error.print();
        exit(EXITCODE_USAGE);
    });

    #[cfg(feature = "color")]
    match command.color {
        Color::Always => colored::control::set_override(true),
        Color::Auto => colored::control::unset_override(),
        Color::Never => colored::control::set_override(false),
    }

    let rc = match command.subcommand {
        #[cfg(feature = "md5")]
        Subcommand::MD5(subcommand) => subcommand.execute(),
        #[cfg(feature = "sha1")]
        Subcommand::SHA1(subcommand) => subcommand.execute(),
        #[cfg(feature = "sha2-224")]
        Subcommand::SHA2_224(subcommand) => subcommand.execute(),
        #[cfg(feature = "sha2-256")]
        Subcommand::SHA2_256(subcommand) => subcommand.execute(),
        #[cfg(feature = "sha2-384")]
        Subcommand::SHA2_384(subcommand) => subcommand.execute(),
        #[cfg(feature = "sha2-512")]
        Subcommand::SHA2_512(subcommand) => subcommand.execute(),
    };

    exit(rc);
}
