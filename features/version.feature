Feature: display the version

  Scenario: version
    When I run "conc --version"
    Then the output contains:
      """
      conc 0.3.2
      """
    And the exit code is 0
