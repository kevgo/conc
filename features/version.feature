Feature: display the version

  Scenario: version
    When I run "conc --version"
    Then STDOUT contains:
      """
      conc 0.6.1
      """
    And the exit code is 0
