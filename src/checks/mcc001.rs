/*
//Install Homebrew as a useful tool for semi-securely install or updating other tools
        description: "Homebrew is installed."
        confidence: "required"
        tests:
        [
            {
                type: "exact match"
                command:
                    echo $(homebrew_is_installed)
                command_pass: "1"
                command_fail: "0"
                case_sensitive: "false"
            }
        ]
        fix:
        {
            //This homebrew script requries sudo privs and so the user of this tool should be alerted as to why she is being prompted for a password
            sudo_command: "/usr/bin/ruby ./scripts/homebrew_install_ed33f044812cc9c509a4d8e6997c44441b06dd4e1fc87f131ee9f319d77fcd50.rb"
            manual:
                '''
                Homebrew is a useful tool for installing and updating programs from the command line.
                There are various things that can go wrong when attempting to install Homebrew.
                Please review their installation guide here:
                https://brew.sh/
                '''
        }
 */
use crate::checks::model::{Confidence, Fix, Test, TestConfig, Type};

pub const MCCOO1: TestConfig = TestConfig {
  description: "Homebrew is installed",
  reference: "",
  confidence: Confidence::Required,
  tests: Test {
    test_type: Type::Status,
    command: "command",
    command_args: &["-v", "brew" ],
    command_pass: "0",
    command_fail: "1",
    case_sensitive: false,
  },
  fix: Fix {
    command: "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"",
    command_args: &[],
    sudo_command: "",
    sudo_command_args: &[],
    manual: "\
    Homebrew is a useful tool for installing and updating programs from the command line.
    There are various things that can go wrong when attempting to install Homebrew.
    Please review their installation guide here: https://brew.sh/
",
  },
};
