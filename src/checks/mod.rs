use std::process::Command;

use anyhow::Result;
use colored::{ColoredString, Colorize};
use phf::phf_map;
use regex::RegexSetBuilder;

use model::{Fix, TestConfig, Type};

mod mcc000;
mod mcc001;
mod mcc002;
mod model;

const FAILED_FIX: &str = "failed to execute fix command";
const FAILED_TEST: &str = "failed to execute command";

static CHECKS: phf::Map<&'static str, TestConfig> = phf_map! {
    "mcc000" => mcc000::MCC000,
    "mcc001" => mcc001::MCCOO1,
    "mcc002" => mcc002::MCCOO2,
};

fn find_check_phf(name: &str) -> Option<TestConfig> {
  CHECKS.get(name.to_lowercase().as_str()).cloned()
}

pub fn all_check() -> Vec<&'static str> {
  let mut vec: Vec<&str> = Vec::new();
  for key in CHECKS.keys() {
    vec.push(key);
  }
  vec.sort_unstable();
  vec
}

pub fn list_checks() -> Result<()> {
  log::info!("List all checks:");
  for check in all_check() {
    log::info!(
      "  - {}: {}",
      check.to_uppercase(),
      find_check_phf(check).unwrap().description
    );
  }
  Ok(())
}

pub fn exec_check(code_check: &str, dry_run: bool) {
  let test_config = match find_check_phf(code_check) {
    Some(value) => value,
    None => {
      log::error!("Unknown {} control", code_check.red());
      return;
      //unimplemented!("Unknown {} control", code_check.red());
      //std::process::exit(1);
    }
  };
  if dry_run {
    log::info!(
      "{}: {} with command '{}' {}",
      code_check.to_uppercase(),
      test_config.description,
      test_config.tests.command,
      "DRY_RUN".yellow()
    );
  } else {
    match test_config.tests.test_type {
      Type::Status => status_match(code_check, test_config),
      Type::Exact => exact_match(code_check, test_config),
      Type::Regex => regexp_match(code_check, test_config),
    }
  }
}

fn status_match(code_check: &str, test_config: TestConfig) {
  let message_status = match Command::new(test_config.tests.command)
    .args(test_config.tests.command_args)
    .output()
    .expect(FAILED_TEST)
    .status
    .success()
  {
    true => "PASSED".green().bold(),
    false => fix(test_config.fix),
  };
  log::info!(
    "{}: {} ..... {}",
    code_check.to_uppercase(),
    test_config.description,
    message_status
  );
}

fn exact_match(code_check: &str, test_config: TestConfig) {
  let output = Command::new(test_config.tests.command)
    .args(test_config.tests.command_args)
    .output()
    .expect(FAILED_TEST)
    .stdout;
  //log::debug!("#output:{}#", String::from_utf8(output.clone()).unwrap().as_str().trim());
  //log::debug!("#command_pass:{}#", test_config.tests.command_pass);
  let message_status = match test_config
    .tests
    .command_pass
    .eq(String::from_utf8(output).unwrap().as_str().trim())
  {
    true => "PASSED".green().bold(),
    false => fix(test_config.fix),
  };
  log::info!(
    "{}: {} ..... {}",
    code_check.to_uppercase(),
    test_config.description,
    message_status
  );
}

fn regexp_match(code_check: &str, test_config: TestConfig) {
  let output = Command::new(test_config.tests.command)
    .args(test_config.tests.command_args)
    .output()
    .expect(FAILED_TEST)
    .stdout;
  let set = RegexSetBuilder::new([test_config.tests.command_pass])
    .case_insensitive(true)
    .build()
    .unwrap();
  let message_status = match set.is_match(String::from_utf8(output).unwrap().as_str().trim()) {
    true => "PASSED".green().bold(),
    false => fix(test_config.fix),
  };
  log::info!(
    "{}: {} ..... {}",
    code_check.to_uppercase(),
    test_config.description,
    message_status
  );
}

fn fix(fix: Fix) -> ColoredString {
  // match fix.command {
  //   Some(command)=> {
  log::debug!("Fix {}", fix.manual);
  match Command::new(fix.command)
    .args(fix.command_args)
    .status()
    .expect(FAILED_FIX)
    .success()
  {
    true => "PASSED".green().bold(),
    false => "FAILED".red().bold(),
  }
  //   },
  //   None => "FAILED".red().bold(),
  // }
}
