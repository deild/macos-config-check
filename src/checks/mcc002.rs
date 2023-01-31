/*{
description: "Homebrew analytics are disabled."
confidence: "required"
reference: "https://github.com/Homebrew/brew/blob/master/share/doc/homebrew/Analytics.md"
tests:
[
{
type: "exact match"
//test based on: https://github.com/Homebrew/brew/blob/master/Library/Homebrew/utils/analytics.sh
command: "[[ -n $HOMEBREW_NO_ANALYTICS ]] && echo 1 || echo 0"
command_pass: "1"
command_fail: "0"
case_sensitive: "false"
}
]
fix:
{
command: "grep -q 'export HOMEBREW_NO_ANALYTICS=1' ~/.profile || echo 'export HOMEBREW_NO_ANALYTICS=1' >> ~/.profile ; source ~/.profile"
manual:
'''
1. Bring the Terminal application to the foreground if it is not already. You should see the word "Terminal" in the top left corner of your screen.
2. Select Terminal->Quit
3. Re-open the Terminal application and run the tool again; this check should now pass.
'''
}
},*/

use crate::checks::model::{Confidence, Fix, Test, TestConfig, Type};

pub const MCCOO2: TestConfig = TestConfig {
  description:"Homebrew analytics are disabled",
  reference:"https://github.com/Homebrew/brew/blob/master/share/doc/homebrew/Analytics.md",
  confidence: Confidence::Required,
  tests: Test {
    test_type: Type::Exact,
    command: "brew",
    command_args: &["analytics"],
    command_pass: "Analytics are disabled.",
    command_fail: "Analytics are enabled.",
    case_sensitive: false
  } ,
  fix: Fix{
    command: "brew analytics off",
    command_args: &[],
    sudo_command: "",
    sudo_command_args: &[],
    manual: "\
    1. Bring the Terminal application to the foreground if it is not already. You should see the word \"Terminal\" in the top left corner of your screen.
    2. Select Terminal->Quit
    3. Re-open the Terminal application and run the tool again; this check should now pass.
"
  }
};
