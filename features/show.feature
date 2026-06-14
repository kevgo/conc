Feature: run multiple commands concurrently

  Scenario: --show=all
    When I run "conc --show=all 'echo one' 'echo two' 'echo three'"
    Then STDOUT contains:
      """
      echo one
      one
      """
    And STDOUT contains:
      """
      echo two
      two
      """
    And STDOUT contains:
      """
      echo three
      three
      """
    And the exit code is 0

  Scenario: --show=names
    When I run "conc --show=names 'echo one' 'echo two' 'echo three'"
    Then STDOUT contains:
      """
      echo one
      """
    And STDOUT contains:
      """
      echo two
      """
    And STDOUT contains:
      """
      echo three
      """
    And the exit code is 0

  Scenario: --show=names --error-on-output
    When I run "conc --show=names --error-on-output 'echo one' 'echo two' 'echo three'"
    Then STDOUT contains:
      """
      echo one
      one
      """
    And STDOUT contains:
      """
      echo two
      two
      """
    And STDOUT contains:
      """
      echo three
      three
      """
    And the exit code is 1

  Scenario: --show=failed
    When I run "conc --show=failed 'echo one' 'echo two' 'echo three'"
    Then the output is empty
    And the exit code is 0
