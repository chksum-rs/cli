use std::io::{stderr, stdin, stdout, Write};
use std::sync::mpsc;
use std::{process, thread};

use anyhow::Result;
#[cfg(feature = "md5")]
use chksum::MD5;
#[cfg(feature = "sha1")]
use chksum::SHA1;
#[cfg(feature = "sha2-224")]
use chksum::SHA2_224;
#[cfg(feature = "sha2-256")]
use chksum::SHA2_256;
#[cfg(feature = "sha2-384")]
use chksum::SHA2_384;
#[cfg(feature = "sha2-512")]
use chksum::SHA2_512;
use chksum::{chksum, Hash};
#[cfg(feature = "color")]
use chksum_cli::Color;
use chksum_cli::{exitcode, print_result, Args, Command, Options, Subcommand, Target};
use clap::Parser;
use exitcode::{OK as EXITCODE_OK, USAGE as EXITCODE_USAGE};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

/// Exits process with given return code.
fn exit(code: i32) -> ! {
    let _ = stdout().lock().flush();
    let _ = stderr().lock().flush();
    process::exit(code)
}

/// Handles subcommand execution.
fn subcommand<T>(args: &Args, options: &Options) -> i32
where
    T: Hash,
    T::Digest: 'static + Send,
{
    let (tx, rx) = mpsc::sync_channel(1);

    let printer = thread::spawn(move || {
        let mut stdout = stdout().lock();
        let mut stderr = stderr().lock();
        while let Ok(pair) = rx.recv() {
            let (target, result) = pair;
            print_result(&mut stdout, &mut stderr, target, result).expect("Cannot print result");
        }
    });

    let rc = if options.stdin {
        let handle = stdin().lock();
        let result = chksum::<T>(handle);
        let rc = exitcode(&result);
        let pair = (Target::Stdin, result);
        tx.send(pair).expect("Cannot send result to printer thread");
        rc
    } else {
        args.paths
            .par_iter()
            .map(|path| {
                let result = chksum::<T>(path);
                let rc = exitcode(&result);
                let pair = (path.into(), result);
                tx.send(pair).expect("Cannot send result to printer thread");
                rc
            })
            // returns first occured error
            .reduce(|| EXITCODE_OK, |acc, rc| if acc == EXITCODE_OK { rc } else { acc })
    };

    drop(tx); // must drop manually, otherwise rx.recv() never return an error

    printer.join().expect("The printer thread has panicked");

    rc
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
        Subcommand::MD5 { args, options } => subcommand::<MD5>(&args, &options),
        #[cfg(feature = "sha1")]
        Subcommand::SHA1 { args, options } => subcommand::<SHA1>(&args, &options),
        #[cfg(feature = "sha2-224")]
        Subcommand::SHA2_224 { args, options } => subcommand::<SHA2_224>(&args, &options),
        #[cfg(feature = "sha2-256")]
        Subcommand::SHA2_256 { args, options } => subcommand::<SHA2_256>(&args, &options),
        #[cfg(feature = "sha2-384")]
        Subcommand::SHA2_384 { args, options } => subcommand::<SHA2_384>(&args, &options),
        #[cfg(feature = "sha2-512")]
        Subcommand::SHA2_512 { args, options } => subcommand::<SHA2_512>(&args, &options),
    };

    exit(rc);
}
