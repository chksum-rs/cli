use std::io::{stderr, stdin, stdout, StdinLock, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::{process, thread};

use anyhow::Result;
use chksum::hash::{MD5, SHA1, SHA2_224, SHA2_256, SHA2_384, SHA2_512};
use chksum::{chksum, Chksum, Error};
use chksum_cli::{exitcode, print_result, Args, Command, Options, Subcommand};
use clap::Parser;
use exitcode::{OK as EXITCODE_OK, USAGE as EXITCODE_USAGE};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

/// Exits process with given return code.
#[inline]
fn exit(code: i32) -> ! {
    let _ = stdout().lock().flush();
    let _ = stderr().lock().flush();
    process::exit(code)
}

/// Handles subcommand execution.
#[inline]
fn subcommand<T>(args: &Args, options: &Options) -> i32
where
    T: for<'a> Chksum<&'a PathBuf, Error = Error> + for<'a> Chksum<StdinLock<'a>, Error = Error>,
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
        let result = chksum::<T, _>(handle);
        let rc = exitcode(&result);
        let pair = ("<stdin>".to_string(), result);
        tx.send(pair).expect("Cannot send result to printer thread");
        rc
    } else {
        args.paths
            .par_iter()
            .map(|path| {
                let result = chksum::<T, _>(path);
                let rc = exitcode(&result);
                let pair = (path.display().to_string(), result);
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

    let rc = match command.subcommand {
        Subcommand::MD5 { args, options } => subcommand::<MD5>(&args, &options),
        Subcommand::SHA1 { args, options } => subcommand::<SHA1>(&args, &options),
        Subcommand::SHA2_224 { args, options } => subcommand::<SHA2_224>(&args, &options),
        Subcommand::SHA2_256 { args, options } => subcommand::<SHA2_256>(&args, &options),
        Subcommand::SHA2_384 { args, options } => subcommand::<SHA2_384>(&args, &options),
        Subcommand::SHA2_512 { args, options } => subcommand::<SHA2_512>(&args, &options),
    };

    exit(rc);
}
