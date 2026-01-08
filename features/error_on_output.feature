Feature: error on output

  Scenario: enabled, no output
    Given I'm in an empty folder
    When I run "conc --error-on-output 'mkdir test'"
    Then the output contains:
      """
      			mkdir test
      """
    And the exit code is 0

  Scenario: enabled, with output
    When I run "conc --error-on-output 'echo one'"
    Then the output contains:
      """
      			echo one
      			one
      """
    And the exit code is 1

  Scenario: disabled, no output
    Given I'm in an empty folder
    When I run "conc 'mkdir test'"
    Then the output contains:
      """
      			mkdir test
      """
    And the exit code is 0

  Scenario: disabled, with output
    When I run "conc 'echo one'"
    Then the output contains:
      """
      			echo one
      			one
      """
    And the exit code is 0
