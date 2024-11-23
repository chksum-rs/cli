# chksum-cli

[![GitHub](https://img.shields.io/badge/github-chksum--rs%2Fcli-24292e?style=flat-square&logo=github "GitHub")](https://github.com/chksum-rs/cli)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/cli/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/cli/actions/workflows/rust.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.74.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/cli/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-cli/0.4.1/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-cli/0.4.1)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/cli?style=flat-square "LICENSE")](https://github.com/chksum-rs/cli/blob/master/LICENSE)

A simple checksum calculator.

## Motivation

Various tools like `md5sum`, `sha1sum`, `b2sum`, `sha224sum`, etc., allow users to calculate file-based hash digests. However, these tools focus on file-level checksums, making it cumbersome to handle scenarios like calculating digests of whole directories. This motivated the creation of `chksum` to offer a simplified interface for such use cases.

```shell
find dir/ -type f | sort | xargs cat | sha224sum
```

With `chksum`, you can achieve the same result with your preferred hash algorithm:

```sh
chksum sha2-224 dir/
```

## Key Features

* Implemented in pure Rust
* No unsafe code
* Configurable via Cargo features
* Multithreaded

## Installation

Install the `chksum` binary using [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html):

```shell
cargo install chksum-cli
```

## Usage

### General Help

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

### Help for a Specific Algorithm

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

### File Processing

```shell
$ chksum sha2-224 LICENSE
LICENSE: f2c3541b130a29abc5400732a573ba11a3a30a09435d3c1f15a83f77
```

### Directory Processing

```shell
$ chksum sha1 src/
src/: 65393d65360ae9915e50224ee0f49fa58cff9abd
```

### Standard Input Processing

```shell
$ echo -n admin1 | chksum md5 --stdin
<stdin>: e00cf25ad42683b3df678c61f42c6bda
```

## Library

Check out the [`chksum`](https://crates.io/crates/chksum) crate to see the library that allows you to calculate digests of files and directories with an easy-to-use interface.

## Hash Algorithms

This binary provides implementations for the following hash algorithms:

* MD5 - [RFC 1321: The MD5 Message-Digest Algorithm](https://tools.ietf.org/html/rfc1321)
* SHA-1 - [RFC 3174: US Secure Hash Algorithm 1 (SHA1)](https://tools.ietf.org/html/rfc3174)
* SHA-2 family (SHA-224, SHA-256, SHA-386, SHA-512) - [FIPS PUB 180-4: Secure Hash Standard](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)

## Features

### Algorithms

* `md5`: Enables MD5 hash algorithm.
* `sha1`: Enables SHA-1 hash algorithm.
* `sha2`: Enables SHA-2 hash family algorithms.
  * `sha2-224`: Enables only SHA-2 224 hash algorithm.
  * `sha2-256`: Enables only SHA-2 256 hash algorithm.
  * `sha2-384`: Enables only SHA-2 384 hash algorithm.
  * `sha2-512`: Enables only SHA-2 512 hash algorithm.

By default, all of them are enabled.

### Extra Options

* `color`: Enables colored output.

By default, all of them are enabled.

## Disclaimer

The code is under development, and the interface may change in the future.

## License

This crate is licensed under the MIT License.
