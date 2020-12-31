use anyhow::Result;
use clap::crate_version;
use clap::Clap;

use crate::utils::exec_cmd;

use super::Cmd;

#[cfg(feature = "http")]
const GIT_IO_BASE_URL: &str = "https://git.io/";

/// Report bugs/issues at https://github.com/deild/macos-config-check/issues
#[derive(Clap, Debug)]
pub struct BugReport {}

impl Cmd for BugReport {
  fn run(&self) -> Result<()> {
    create();
    Ok(())
  }
}

fn create() {
  let os_info = os_info::get();

  let environment = Environment {
    os_type: os_info.os_type(),
    os_version: os_info.version().to_owned(),
    shell_info: get_shell_info(),
    terminal_info: get_terminal_info(),
  };

  let link = make_github_issue_link(crate_version!(), environment);
  let short_link = shorten_link(&link);

  if open::that(&link)
    .map(|status| status.success())
    .unwrap_or(false)
  {
    println!(
      "Take a look at your browser. A GitHub issue has been populated with your configuration."
    );
    println!("If your browser has failed to open, please click this link:\n");
  } else {
    println!("Click this link to create a GitHub issue populated with your configuration:\n");
  }

  println!(" {}", short_link.unwrap_or(link));
}

#[cfg(feature = "http")]
fn shorten_link(link: &str) -> Option<String> {
  attohttpc::post(&format!("{}{}", GIT_IO_BASE_URL, "create"))
    .form(&[("url", link)])
    .ok()
    .and_then(|r| r.send().ok())
    .and_then(|r| r.text().ok())
    .map(|slug| format!("{}{}", GIT_IO_BASE_URL, slug))
}

#[cfg(not(feature = "http"))]
fn shorten_link(_url: &str) -> Option<String> {
  None
}

const UNKNOWN_SHELL: &str = "<unknown shell>";
const UNKNOWN_TERMINAL: &str = "<unknown terminal>";
const UNKNOWN_VERSION: &str = "<unknown version>";
const GITHUB_CHAR_LIMIT: usize = 8100; // Magic number accepted by Github

struct Environment {
  os_type: os_info::Type,
  os_version: os_info::Version,
  shell_info: ShellInfo,
  terminal_info: TerminalInfo,
}

fn make_github_issue_link(app_version: &str, environment: Environment) -> String {
  let body = urlencoding::encode(&format!("#### Current Behavior
<!-- A clear and concise description of the behavior. -->

#### Expected Behavior
<!-- A clear and concise description of what you expected to happen. -->

#### Additional context/Screenshots
<!-- Add any other context about the problem here. If applicable, add screenshots to help explain. -->

#### Possible Solution
<!--- Only if you have suggestions on a fix for the bug -->

#### Environment
- application version: {app_version}
- {shell_name} version: {shell_version}
- Operating system: {os_name} {os_version}
- Terminal emulator: {terminal_name} {terminal_version}

#### Additional context

Add any other context about the bug report here.
",
                                          app_version = app_version,
                                          shell_name = environment.shell_info.name,
                                          shell_version = environment.shell_info.version,
                                          terminal_name = environment.terminal_info.name,
                                          terminal_version = environment.terminal_info.version,
                                          os_name = environment.os_type,
                                          os_version = environment.os_version,
  ))
    .replace("%20", "+");

  format!(
    "https://github.com/deild/macos-config-check/issues/new?template={}&body={}",
    urlencoding::encode("bug_report.md"),
    body
  )
  .chars()
  .take(GITHUB_CHAR_LIMIT)
  .collect()
}

#[derive(Debug)]
struct ShellInfo {
  name: String,
  version: String,
}

fn get_shell_info() -> ShellInfo {
  let shell = std::env::var("SHELL");
  if shell.is_err() {
    return ShellInfo {
      name: UNKNOWN_SHELL.to_string(),
      version: UNKNOWN_VERSION.to_string(),
    };
  }

  let shell = shell.unwrap();

  let version = exec_cmd(&shell, &["--version"])
    .map(|output| output.stdout.trim().to_string())
    .unwrap_or_else(|| UNKNOWN_VERSION.to_string());

  ShellInfo {
    name: shell,
    version,
  }
}

#[derive(Debug)]
struct TerminalInfo {
  name: String,
  version: String,
}

fn get_terminal_info() -> TerminalInfo {
  let terminal = std::env::var("TERM_PROGRAM")
    .or_else(|_| std::env::var("LC_TERMINAL"))
    .unwrap_or_else(|_| UNKNOWN_TERMINAL.to_string());

  let version = std::env::var("TERM_PROGRAM_VERSION")
    .or_else(|_| std::env::var("LC_TERMINAL_VERSION"))
    .unwrap_or_else(|_| UNKNOWN_VERSION.to_string());

  TerminalInfo {
    name: terminal,
    version,
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use super::*;

  #[test]
  fn test_make_github_link() {
    let app_version = crate_version!();
    let environment = Environment {
      os_type: os_info::Type::Linux,
      os_version: os_info::Version::Semantic(1, 2, 3),
      shell_info: ShellInfo {
        name: "test_shell".to_string(),
        version: "2.3.4".to_string(),
      },
      terminal_info: TerminalInfo {
        name: "test_terminal".to_string(),
        version: "5.6.7".to_string(),
      },
    };

    let link = make_github_issue_link(app_version, environment);

    assert!(link.contains(app_version));
    assert!(link.contains("Linux"));
    assert!(link.contains("1.2.3"));
    assert!(link.contains("test_shell"));
    assert!(link.contains("2.3.4"));
  }

  #[test]
  fn test_get_shell_info() {
    env::remove_var("SHELL");
    let unknown_shell = get_shell_info();
    assert_eq!(UNKNOWN_SHELL, &unknown_shell.name);

    env::set_var("SHELL", "fish");

    let fish_shell = get_shell_info();
    assert_eq!("fish", &fish_shell.name);
  }
}
