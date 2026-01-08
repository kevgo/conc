Feature: run multiple commands concurrently

  Scenario: multiple commands
    When I run "conc 'echo one' 'echo two' 'echo three'"
    Then the output contains:
      """
      echo one
      one
      """
    And the output contains:
      """
      echo two
      two
      """
    And the output contains:
      """
      echo three
      three
      """
    And the exit code is 0

  Scenario: one command
    When I run "conc 'echo one'"
    Then the output contains:
      """
      echo one
      one
      """
    And the exit code is 0

  Scenario: no command
    When I run "conc"
    Then the exit code is 0
