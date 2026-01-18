Feature: run multiple commands concurrently

  Scenario: --show=all
    When I run "conc --show=all 'echo one' 'echo two' 'echo three'"
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

  Scenario: --show=commands
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

  Scenario: --show=commands --error-on-output
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

  Scenario: --show=min
    When I run "conc --show=min 'echo one' 'echo two' 'echo three'"
    Then the output is empty
    And the exit code is 0
