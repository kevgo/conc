Feature: run multiple commands concurrently

  Scenario: show output
    When I run "conc --show=output 'echo one' 'echo two' 'echo three'"
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

  Scenario: show executed commands
    When I run "conc --show=commands 'echo one' 'echo two' 'echo three'"
    Then the output contains:
      """
      echo one
      """
    And the output contains:
      """
      echo two
      """
    And the output contains:
      """
      echo three
      """
    And the exit code is 0

  Scenario: show executed commands with error-on-output
    When I run "conc --show=commands --error-on-output 'echo one' 'echo two' 'echo three'"
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
    And the exit code is 1
