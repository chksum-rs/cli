#![forbid(unsafe_code)]

#[cfg(feature = "color")]
mod color;
#[cfg(feature = "md5")]
mod md5;
#[cfg(feature = "sha1")]
mod sha1;
#[cfg(feature = "sha2-224")]
mod sha2_224;
#[cfg(feature = "sha2-256")]
mod sha2_256;
#[cfg(feature = "sha2-384")]
mod sha2_384;
#[cfg(feature = "sha2-512")]
mod sha2_512;

use std::fmt::{self, Display, Formatter};
use std::io::{self, stderr, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use chksum::{chksum, Digest, Error, Hash};
#[cfg(feature = "color")]
use colored::Colorize;
use exitcode::{IOERR as EXITCODE_IOERR, OK as EXITCODE_OK};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[cfg(feature = "color")]
pub use crate::color::Color;

#[derive(Clone, Debug)]
enum Input {
    Path(PathBuf),
    Stdin,
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path.display()),
            Self::Stdin => write!(f, "<stdin>"),
        }
    }
}

impl<T> From<T> for Input
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        let value = value.as_ref();
        Self::Path(value.to_path_buf())
    }
}

#[derive(Debug, clap::Parser)]
#[command(name = "chksum", version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub subcommand: Subcommand,
    /// Show colored output.
    #[arg(value_enum, short, long, default_value_t = Color::Auto, global = true)]
    #[cfg(feature = "color")]
    pub color: Color,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Calculate MD5 digest.
    #[cfg(feature = "md5")]
    #[command(arg_required_else_help = true)]
    MD5(md5::Subcommand),
    /// Calculate SHA-1 digest.
    #[cfg(feature = "sha1")]
    #[command(arg_required_else_help = true)]
    SHA1(sha1::Subcommand),
    /// Calculate SHA-2 224 digest.
    #[cfg(feature = "sha2-224")]
    #[command(arg_required_else_help = true)]
    SHA2_224(sha2_224::Subcommand),
    /// Calculate SHA-2 256 digest.
    #[cfg(feature = "sha2-256")]
    #[command(arg_required_else_help = true)]
    SHA2_256(sha2_256::Subcommand),
    /// Calculate SHA-2 384 digest.
    #[cfg(feature = "sha2-384")]
    #[command(arg_required_else_help = true)]
    SHA2_384(sha2_384::Subcommand),
    /// Calculate SHA-2 512 digest.
    #[cfg(feature = "sha2-512")]
    #[command(arg_required_else_help = true)]
    SHA2_512(sha2_512::Subcommand),
}

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// Path to file or directory.
    #[arg(required = true, value_name = "PATH", conflicts_with = "stdin")]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub(crate) struct Options {
    /// Calculate digest from stdin.
    #[arg(short, long, default_value_t = false, conflicts_with = "paths")]
    pub stdin: bool,
}

/// Prints result to stdout or stderr.
fn print_result(
    stdout: &mut impl Write,
    stderr: &mut impl Write,
    input: Input,
    result: Result<impl Digest, Error>,
) -> io::Result<()> {
    match result {
        Ok(digest) => writeln!(stdout, "{input}: {digest}"),
        Err(error) => {
            let error = error.to_string().to_lowercase();
            let error = format!("{input}: {error}");
            #[cfg(feature = "color")]
            let error = error.red();
            writeln!(stderr, "{error}")
        },
    }
}

/// Handles subcommand execution.
pub(crate) fn subcommand<T>(args: &Args, options: &Options) -> i32
where
    T: Hash,
    T::Digest: 'static + Send,
{
    let (tx, rx) = mpsc::sync_channel(1);

    let printer = thread::spawn(move || {
        let mut stdout = stdout().lock();
        let mut stderr = stderr().lock();
        while let Ok(pair) = rx.recv() {
            let (input, result) = pair;
            print_result(&mut stdout, &mut stderr, input, result).expect("Cannot print result");
        }
    });

    let rc = if options.stdin {
        let handle = stdin().lock();
        let result = chksum::<T>(handle);
        let rc = exitcode(&result);
        let pair = (Input::Stdin, result);
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

/// Turns result to exitcode.
fn exitcode<T>(result: &Result<T, Error>) -> i32
where
    T: Digest,
{
    if result.is_ok() {
        EXITCODE_OK
    } else {
        EXITCODE_IOERR
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use assert_fs::prelude::PathChild;
    use assert_fs::TempDir;
    use chksum::MD5;

    use super::*;

    #[test]
    fn exitcode_ok() -> Result<()> {
        let tmpdir = TempDir::new()?;

        let result = chksum::<MD5>(tmpdir.path());
        assert_eq!(exitcode(&result), EXITCODE_OK);

        Ok(())
    }

    #[test]
    fn exitcode_error() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let child = tmpdir.child("child");

        let result = chksum::<MD5>(child.path());
        assert_eq!(exitcode(&result), EXITCODE_IOERR);

        Ok(())
    }
}
