Feature: run bash scripts

  Scenario: bash files
    Given I'm in an empty folder
    And a file "hello.sh" with content:
      """
      #!/bin/sh
      echo hello-from-sh
      """
    And a file "hello.bash" with content:
      """
      #!/bin/sh
      echo hello-from-bash
      """
    When I run "conc ./hello.sh ./hello.bash"
    Then STDOUT contains:
      """
      ./hello.sh
      hello-from-sh
      """
    And STDOUT contains:
      """
      ./hello.bash
      hello-from-bash
      """
    And the exit code is 0
