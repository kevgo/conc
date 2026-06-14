Feature: stderr to stdout

  Scenario: enabled
    When I run "conc --stderr-to-stdout 'echo error-output >&2'"
    Then STDOUT contains:
      """
      echo error-output >&2
      error-output
      """
    And STDERR is empty
    And the exit code is 0

  Scenario: disabled
    When I run "conc 'echo error-output >&2'"
    Then STDOUT contains:
      """
      echo error-output >&2
      """
    And STDERR contains:
      """
      error-output
      """
    And the exit code is 0
