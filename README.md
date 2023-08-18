# chksum-cli

[![Build](https://img.shields.io/github/actions/workflow/status/ferric-bytes/chksum-cli/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/ferric-bytes/chksum-cli/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/chksum-cli?style=flat-square&logo=rust "crates.io")](https://crates.io/crates/chksum-cli)
[![Coverage](https://img.shields.io/codecov/c/gh/ferric-bytes/chksum-cli?style=flat-square&logo=codecov "Coverage")](https://app.codecov.io/gh/ferric-bytes/chksum-cli)
[![MSRV](https://img.shields.io/badge/MSRV-1.66.0-informational?style=flat-square "MSRV")](https://github.com/ferric-bytes/chksum-cli/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-cli/0.2.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-cli/0.2.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/ferric-bytes/chksum-cli?style=flat-square "LICENSE")](https://github.com/ferric-bytes/chksum-cli/blob/master/LICENSE)

A simple checksum calculator.

## Motivation

There are variety of tools that allows calculate hash digests.

However tools like `md5sum`, `sha1sum`, `b2sum`, `sha224sum` and others offer only file-based checksums.

```shell
find dir/ -type f | sort | xargs cat | sha224sum
```

Instead you can just use `chksum` with preffered hash algorithm.

```sh
chksum sha2-224 dir/
```

## Features

- Written in pure Rust
- No unsafe code
- Configurable via Cargo features
- Multithread

## Installation

Use [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) to install `chksum` binary in `$HOME/.cargo/bin` directory.

```shell
cargo install chksum-cli
```

## Usage

```shell
$ chksum help
A simple checksum calculator.

Usage: chksum [OPTIONS] <COMMAND>

Commands:
  md5       Calculate MD5 digest
  sha1      Calculate SHA-1 digest
  sha2-224  Calculate SHA-2 224 digest
  sha2-256  Calculate SHA-2 256 digest
  sha2-384  Calculate SHA-2 384 digest
  sha2-512  Calculate SHA-2 512 digest
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --color <COLOR>  Show colored output [default: auto] [possible values: always, auto, never]
  -h, --help           Print help
  -V, --version        Print version
```

```shell
$ chksum help sha2-224
Calculate SHA-2 224 digest

Usage: chksum sha2-224 [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Path to file or directory

Options:
  -s, --stdin          Calculate digest from stdin
  -c, --color <COLOR>  Show colored output [default: auto] [possible values: always, auto, never]
  -h, --help           Print help
```

### File

```shell
$ chksum sha2-224 LICENSE
LICENSE: 99258bca0d23c69388dd53412f1009132753b89459359a401a6ed158
```

### Directory

```shell
$ chksum sha1 src/
src/: 03e4ae615c034f5db47c72bd5c6c9e5bf450a2bd
```

### Standard input

```shell
$ echo -n admin1 | chksum md5 --stdin
<stdin>: e00cf25ad42683b3df678c61f42c6bda
```

## Library

Check [`chksum`](https://github.com/ferric-bytes/chksum) repository to see the library that allows you to calculate digests of files and directories with easy-to-use interface.

## Hash algorithms

Implemented hash algorithms:

* MD5 - [RFC 1321: The MD5 Message-Digest Algorithm](https://tools.ietf.org/html/rfc1321)
* SHA-1 - [RFC 3174: US Secure Hash Algorithm 1 (SHA1)](https://tools.ietf.org/html/rfc3174)
* SHA-2 family (SHA-224, SHA-256, SHA-386, SHA-512) - [FIPS PUB 180-4: Secure Hash Standard](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)

## Feature flags

### Options

* `color`: Enables colored output.

By default all of them are enabled.

## Disclaimer

Code is under development. The interface may change in the future.

## License

MIT
