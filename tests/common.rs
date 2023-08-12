use std::result;

use assert_cmd::cargo::CargoError;
use assert_fs::fixture::FixtureError;
use chksum::Error as ChksumError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CargoError(#[from] CargoError),
    #[error(transparent)]
    ChksumError(#[from] ChksumError),
    #[error(transparent)]
    FixtureError(#[from] FixtureError),
}

pub type Result = result::Result<(), Error>;
