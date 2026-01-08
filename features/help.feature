Feature: display the help text

  Scenario: help
    When I run "conc --help"
    Then the output is:
      """
      Conc runs commands concurrently and returns the first non-zero exit code it encounters.

      Usage: conc [flags] [commands...]

      Flags:
        --error-on-output   error if any command produces output
        --help, -h          this help text
        --show=all          show the output of all commands
        --show=failed       show the output of only failed commands
        --version, -V       show the version

      Examples:

      conc --show=failed "echo one" "echo two" "echo three"

      This executes the following commands concurrently:

      - echo one
      - echo two
      - echo three

      If any of the commands exit with a non-zero exit code,
      conc will print the output of the failed command
      and return that exit code.
      """
    And the exit code is 0
