# macos-config-check 
![Continuous integration](https://github.com/deild/macos-config-check/workflows/Continuous%20integration/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://raw.githubusercontent.com/deild/macos-config-check/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/macos-config-check?style=flat-square)](https://crates.io/crates/macos-config-check)


Checks your macOS machine against various hardened configuration settings.

## Disclaimer

The authors of this tool are not responsible if running it breaks stuff; 
disabling features of your operating system and applications may disrupt normal functionality.

Once applied, the security configurations do not not guarantee security. 
You will still need to make good decisions in order to stay secure. 
The configurations will generally not help you if your computer has been previously compromised.

To suggest an improvement, please send a [pull request](https://github.com/deild/macos-config-check/pulls) or open [an issue](https://github.com/deild/macos-config-check/issues).

## Installation

Install the **macos-config-check** binary:

   #### From source on [crates.io](https://crates.io/):

   ```sh
   cargo install macos-config-check
   ```

   ### Install via Package Manager

   #### With [Homebrew](https://brew.sh/):

   ```sh
   brew install macos-config-check
   ```

## Usage

**You should download and run this application once for each macOS user account you have on your machine.** 
Each user may be configured differently, and so each should be audited.


```bash
macos-config-check 0.0.1-alpha.1
Checks your macOS machine against various hardened configuration settings

USAGE:
    macos-config-check [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      No output printed to stdout
    -v, --verbose    Use verbose output (-vvv very verbose output)
    -V, --version    Prints version information

SUBCOMMANDS:
    bug-report    Report bugs/issues at https://github.com/deild/macos-config-check/issues
    control       Control the configuration of various macos options
    help          Prints this message or the help of the given subcommand(s)

Download the latest copy of this tool at: https://github.com/deild/mac-os-check/releases/latest
```