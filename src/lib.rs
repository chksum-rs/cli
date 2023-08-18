use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chksum::hash::Digest;
use chksum::Result;
use exitcode::{IOERR as EXITCODE_IOERR, OK as EXITCODE_OK};

#[derive(Clone, Debug)]
pub enum Target {
    Path(PathBuf),
    Stdin,
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path.display()),
            Self::Stdin => write!(f, "<stdin>"),
        }
    }
}

impl<T> From<T> for Target
where
    T: AsRef<Path>,
{
    #[inline]
    fn from(value: T) -> Self {
        let value = value.as_ref();
        Self::Path(value.to_path_buf())
    }
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Calculate MD5 digest.
    #[command(arg_required_else_help = true)]
    MD5 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
    /// Calculate SHA-1 digest.
    #[command(arg_required_else_help = true)]
    SHA1 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
    /// Calculate SHA-2 224 digest.
    #[command(arg_required_else_help = true)]
    SHA2_224 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
    /// Calculate SHA-2 256 digest.
    #[command(arg_required_else_help = true)]
    SHA2_256 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
    /// Calculate SHA-2 384 digest.
    #[command(arg_required_else_help = true)]
    SHA2_384 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
    /// Calculate SHA-2 512 digest.
    #[command(arg_required_else_help = true)]
    SHA2_512 {
        #[command(flatten)]
        args: Args,
        #[command(flatten)]
        options: Options,
    },
}

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Path to file or directory.
    #[arg(required = true, name = "PATH")]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct Options {
    /// Calculate digest from stdin.
    #[arg(short, long, default_value_t = false, exclusive = true)]
    pub stdin: bool,
}

/// Prints result to stdout or stderr.
#[inline]
pub fn print_result<T, U, V>(stdout: &mut T, stderr: &mut U, target: Target, result: Result<V>) -> io::Result<()>
where
    T: Write,
    U: Write,
    V: Digest,
{
    match result {
        Ok(digest) => writeln!(stdout, "{target}: {digest:x}"),
        Err(error) => {
            let error = error.to_string().to_lowercase();
            writeln!(stderr, "{target}: {error}")
        },
    }
}

/// Turns result to exitcode.
#[inline]
pub fn exitcode<T>(result: &Result<T>) -> i32
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
    use chksum::chksum;
    use chksum::hash::MD5;

    use super::*;

    #[test]
    fn exitcode_ok() -> Result<()> {
        let tmpdir = TempDir::new()?;

        let result = chksum::<MD5, _>(tmpdir.path());
        assert_eq!(exitcode(&result), EXITCODE_OK);

        Ok(())
    }

    #[test]
    fn exitcode_error() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let child = tmpdir.child("child");

        let result = chksum::<MD5, _>(child.path());
        assert_eq!(exitcode(&result), EXITCODE_IOERR);

        Ok(())
    }
}
