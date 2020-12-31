use anyhow::Result;
use clap::Clap;

use crate::checks::{all_check, exec_check, list_checks};

use super::Cmd;

/// Control the configuration of various macos options.
#[derive(Clap, Debug)]
pub struct Control {
  #[clap(long)]
  dry_run: bool,
  #[clap(short, long)]
  list: bool,
  codes_check: Vec<String>,
}

impl Cmd for Control {
  fn run(&self) -> Result<()> {
    if self.list {
      return list_checks();
    }
    match self.codes_check.len() {
      0 => {
        for code_check in all_check().iter() {
          exec_check(code_check, self.dry_run);
        }
      }
      _ => {
        for code_check in self.codes_check.iter() {
          exec_check(code_check, self.dry_run);
        }
      }
    }
    Ok(())
  }
}
