Feature: maintain stdout and stderr output

  Scenario: output to stderr
    When I run "conc 'echo one >&2' 'echo two >&2'"
    Then STDOUT contains:
      """
      echo one >&2
      """
    And STDERR contains:
      """
      one
      """
    And STDOUT contains:
      """
      echo two >&2
      """
    And STDERR contains:
      """
      two
      """
    And the exit code is 0
