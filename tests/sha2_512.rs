use assert_cmd::Command;
use assert_fs::prelude::{FileTouch, PathChild, PathCreateDir};
use assert_fs::TempDir;

mod common;
use common::Result;

#[test]
fn help() -> Result {
    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg("-h")
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg("--help")
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Command::cargo_bin("chksum")?
        .arg("help")
        .arg("sha2-512")
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Ok(())
}

#[test]
fn empty_stdin() -> Result {
    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg("--stdin")
        .write_stdin("")
        .assert()
        .success();

    Ok(())
}

#[test]
fn empty_directory() -> Result {
    let tmpdir = TempDir::new()?;

    let dir = tmpdir.child("dir");
    dir.create_dir_all()?;
    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg(dir.path())
        .assert()
        .success();

    Ok(())
}

#[test]
fn empty_file() -> Result {
    let tmpdir = TempDir::new()?;

    let file = tmpdir.child("file");
    file.touch()?;
    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg(file.path())
        .assert()
        .success();

    Ok(())
}

#[test]
fn nonexistent_path() -> Result {
    let tmpdir = TempDir::new()?;

    let nonexistent = tmpdir.child("nonexistent");
    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg(nonexistent.path())
        .assert()
        .failure()
        .code(exitcode::IOERR);

    Ok(())
}

#[test]
fn stdin_and_path() -> Result {
    let tmpdir = TempDir::new()?;

    let dir = tmpdir.child("dir");
    dir.create_dir_all()?;

    Command::cargo_bin("chksum")?
        .arg("sha2-512")
        .arg("--stdin")
        .arg(dir.path())
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Ok(())
}
