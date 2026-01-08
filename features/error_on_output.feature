Feature: error on output

	Scenario: enabled with show=failed
		Given I'm in an empty folder
		When I run "conc --error-on-output --show=failed 'echo one' 'mkdir test'"
		Then the output contains:
			"""
			echo one
			one
			"""
		And the output contains:
			"""
			mkdir test
			"""
		And the exit code is 1
