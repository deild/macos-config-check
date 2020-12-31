use anyhow::Result;
use clap::Clap;
use log::*;

use crate::cmd::{App, Cmd};
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};

mod checks;
mod cmd;
mod utils;

fn init(log_level: LevelFilter) {
  let _ = TermLogger::init(
    log_level,
    ConfigBuilder::new()
      .set_time_format("%Y-%m-%d %H:%M:%S".to_string())
      .build(),
    TerminalMode::Mixed,
  );
}

fn main() -> Result<()> {
  let app: App = App::parse();
  match app.quiet {
    false => {
      match app.verbose {
        0 => init(LevelFilter::Info),
        1 => init(LevelFilter::Debug),
        _ => init(LevelFilter::Trace),
      };
    }
    true => init(LevelFilter::Off),
  };
  app.subcmd.run()
}
