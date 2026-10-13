Feature: run commands one after the other

  Scenario: a slow command before a fast one
    When I run "conc --sequential 'sleep 0.2 && echo one' 'echo two'"
    Then STDOUT contains:
      """
      sleep 0.2 && echo one
      one
      echo two
      two
      """
    And the exit code is 0

  Scenario: an earlier command fails
    When I run "conc --sequential 'exit 2' 'echo two'"
    Then STDOUT contains:
      """
      exit 2
      echo two
      two
      """
    And the exit code is 2
