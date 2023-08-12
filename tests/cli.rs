use assert_cmd::Command;

mod common;
use common::Result;

#[test]
fn no_args() -> Result {
    Command::cargo_bin("chksum")?.assert().failure().code(exitcode::USAGE);

    Ok(())
}

#[test]
fn help() -> Result {
    Command::cargo_bin("chksum")?
        .arg("-h")
        .assert()
        .failure()
        .code(exitcode::USAGE);
    Command::cargo_bin("chksum")?
        .arg("--help")
        .assert()
        .failure()
        .code(exitcode::USAGE);
    Command::cargo_bin("chksum")?
        .arg("help")
        .assert()
        .failure()
        .code(exitcode::USAGE);

    Ok(())
}
