use assert_cmd::Command;
use predicates::prelude::predicate;

static PROGRAM: &str = "macos-config-check";

#[test]
fn start_no_args() {
  //Given
  let expected = "macos-config-check 0.0.1-alpha.0
deild <8457875+deild@users.noreply.github.com>
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

Download the latest copy of this tool at: https://github.com/deild/mac-os-check/releases/latest\n";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM).unwrap();
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected));
}

#[test]
fn start_with_args() {
  //Given
  let expected = "macos-config-check 0.0.1-alpha.0
deild <8457875+deild@users.noreply.github.com>
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

Download the latest copy of this tool at: https://github.com/deild/mac-os-check/releases/latest\n";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM).unwrap();
  cmd.arg("help");
  //Then
  cmd
    .assert()
    .success()
    .stdout(predicate::str::similar(expected));
}

#[test]
fn version_no_args() {
  //Given
  let expected = "macos-config-check 0.0.1-alpha.0\n";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM).unwrap();
  cmd.arg("-V");
  //Then
  cmd
    .assert()
    .success()
    .stdout(predicate::str::similar(expected));
}
