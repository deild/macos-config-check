use crate::checks::model::{Confidence, Fix, Test, TestConfig, Type};

pub const MCC000: TestConfig = TestConfig {
  description: "Test for dummy",
  reference: "nothing",
  confidence: Confidence::Experimental,
  tests: Test {
    test_type: Type::Regex,
    command: "pmset",
    command_args: &["-g"],
    command_pass: ".*hibernatemode\\s+3.*",
    command_fail: "",
    case_sensitive: false,
  },
  fix: Fix {
    command: "true",
    command_args: &[],
    sudo_command: "",
    sudo_command_args: &[],
    manual: "Test Fix",
  },
};
