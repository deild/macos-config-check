#[derive(Clone, Debug)]
pub struct TestConfig<'a> {
  /// is a human-readable string describing the configuration being checked; it should be a present-tense statement about a positive security configuration. (REQUIRED FIELD)
  pub description: &'a str,
  /// provides a link to where a user can find more information about this configuration, or a citation of where this configuration was taken from. (OPTIONAL FIELD)
  pub reference: &'a str,
  /// indicates subjective estimation of negative side-effects. valid values: "required", "recommended", "experimental". (REQUIRED FIELD)
  pub confidence: Confidence,
  /// is an ordered array of test objects. (REQUIRED FIELD, should not be empty)
  pub tests: Test<'a>,
  /// is a JSON object that specifies how to remediate a broken configuration (REQUIRED FIELD, should not be empty)
  pub fix: Fix<'a>,
}

#[derive(Clone, Debug)]
pub enum Confidence {
  REQUIRED,
  //RECOMMENDED,
  EXPERIMENTAL,
}

#[derive(Clone, Debug)]
pub enum Type {
  ExactMatch,
  RegexMatch,
  StatusMatch,
}

#[derive(Clone, Debug)]
pub struct Test<'a> {
  /// is "exact match" or "regex match" or "status match". (REQUIRED FIELD)
  pub test_type: Type,
  /// is the command you want to verify the output of (REQUIRED FIELD)
  pub command: &'a str,
  pub command_args: &'a [&'a str],
  /// is the value that `command`'s output should match. If it matches, all tests pass and subsequent tests for this config are not evaluated. (OPTIONAL FIELD)
  pub command_pass: &'a str,
  ///is the value that `command`'s output should NOT match. If it matches, all tests fail and subsequent tests for this config are not evaluated. (OPTIONAL FIELD)
  pub command_fail: &'a str,
  /// is "true" or "false" depending on whether the `command_pass` and/or `command_fail` values are case-sensitive. (REQUIRED FIELD)
  pub case_sensitive: bool,
}

#[derive(Clone, Debug)]
pub struct Fix<'a> {
  /// is the command that you use to attempt automatic remediation without sudo privileges. (OPTIONAL FIELD)
  pub command: &'a str,
  pub command_args: &'a [&'a str],
  /// is the command using sudo privileges that attempts automatic remediation if `command` fails. (OPTIONAL FIELD)
  pub sudo_command: &'a str,
  pub sudo_command_args: &'a [&'a str],
  /// is the field that provides manual instructions to be printed to the user at the end of script execution if all automatic fixes fail. (OPTIONAL FIELD)
  pub manual: &'a str,
}
