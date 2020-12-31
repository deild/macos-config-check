use anyhow::Result;
use clap::{AppSettings, Clap};

pub use bug_report::BugReport;
pub use control::Control;

mod bug_report;
pub mod control;

static AFTER_HELP: &str =
  "Download the latest copy of this tool at: https://github.com/deild/mac-os-check/releases/latest";

pub trait Cmd {
  fn run(&self) -> Result<()>;
}

#[derive(Debug, Clap)]
#[clap(about, author, version, after_help = AFTER_HELP,
global_setting(AppSettings::GlobalVersion), global_setting(AppSettings::VersionlessSubcommands))]
pub struct App {
  /// Use verbose output (-vvv very verbose output)
  #[clap(short, long, parse(from_occurrences))]
  pub verbose: i32,
  /// No output printed to stdout
  #[clap(short, long)]
  pub quiet: bool,
  #[clap(subcommand)]
  pub subcmd: SubCommand,
}

#[derive(Debug, Clap)]
pub enum SubCommand {
  Control(Control),
  BugReport(BugReport),
}

impl Cmd for SubCommand {
  fn run(&self) -> Result<()> {
    match self {
      SubCommand::Control(cmd) => cmd.run(),
      SubCommand::BugReport(cmd) => cmd.run(),
    }
  }
}
