use std::process::Command;
use std::time::Instant;

#[derive(Debug)]
pub struct CommandOutput {
  pub stdout: String,
  pub stderr: String,
}

impl PartialEq for CommandOutput {
  fn eq(&self, other: &Self) -> bool {
    self.stdout == other.stdout && self.stderr == other.stderr
  }
}

/// Execute a command and return the output on stdout and stderr if successful
#[cfg(not(test))]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  internal_exec_cmd(cmd, args)
}

#[cfg(test)]
pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  let command = match args.len() {
    0 => String::from(cmd),
    _ => format!("{} {}", cmd, args.join(" ")),
  };
  match command.as_str() {
    "dummy_command" => Some(CommandOutput {
      stdout: String::from("stdout ok!\n"),
      stderr: String::from("stderr ok!\n"),
    }),
    // If we don't have a mocked command fall back to executing the command
    _ => internal_exec_cmd(&cmd, &args),
  }
}

fn internal_exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
  log::trace!("Executing command {:?} with args {:?}", cmd, args);

  let full_path = match which::which(cmd) {
    Ok(full_path) => {
      log::trace!("Using {:?} as {:?}", full_path, cmd);
      full_path
    }
    Err(e) => {
      log::trace!("Unable to find {:?} in PATH, {:?}", cmd, e);
      return None;
    }
  };

  let start = Instant::now();
  match Command::new(full_path).args(args).output() {
    Ok(output) => {
      let stdout_string = String::from_utf8(output.stdout).unwrap();
      let stderr_string = String::from_utf8(output.stderr).unwrap();

      log::trace!(
        "stdout: {:?}, stderr: {:?}, exit code: \"{:?}\", took {:?}",
        stdout_string,
        stderr_string,
        output.status.code(),
        start.elapsed()
      );

      if !output.status.success() {
        return None;
      }

      Some(CommandOutput {
        stdout: stdout_string,
        stderr: stderr_string,
      })
    }
    Err(error) => {
      log::info!("Executing command {:?} failed by: {:?}", cmd, error);
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exec_mocked_command() {
    let result = exec_cmd("dummy_command", &[]);
    let expected = Some(CommandOutput {
      stdout: String::from("stdout ok!\n"),
      stderr: String::from("stderr ok!\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_no_output() {
    let result = internal_exec_cmd("true", &[]);
    let expected = Some(CommandOutput {
      stdout: String::from(""),
      stderr: String::from(""),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_stdout() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello"]);
    let expected = Some(CommandOutput {
      stdout: String::from("hello\n"),
      stderr: String::from(""),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_stderr() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello >&2"]);
    let expected = Some(CommandOutput {
      stdout: String::from(""),
      stderr: String::from("hello\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_output_both() {
    let result = internal_exec_cmd("/bin/sh", &["-c", "echo hello; echo world >&2"]);
    let expected = Some(CommandOutput {
      stdout: String::from("hello\n"),
      stderr: String::from("world\n"),
    });

    assert_eq!(result, expected)
  }

  #[test]
  fn exec_with_non_zero_exit_code() {
    let result = internal_exec_cmd("false", &[]);
    let expected = None;

    assert_eq!(result, expected)
  }
}
