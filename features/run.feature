Feature: run multiple commands concurrently

  Scenario: multiple commands
    When I run "conc 'echo one' 'echo two' 'echo three'"
    Then STDOUT contains:
      """
      echo one
      one
      """
    Then STDOUT contains:
      """
      echo two
      two
      """
    Then STDOUT contains:
      """
      echo three
      three
      """
    And the exit code is 0

  Scenario: one command
    When I run "conc 'echo one'"
    Then STDOUT contains:
      """
      echo one
      one
      """
    And the exit code is 0

  Scenario: no command
    When I run "conc"
    Then the exit code is 0
