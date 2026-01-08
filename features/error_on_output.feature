Feature: error on output

	Scenario: enabled
		When I run "conc --show=failed 'echo one' 'echo two' 'echo three'"
		Then the output contains these lines in any order:
			"""
			echo one
			echo two
			echo three
			"""
		And the exit code is 0
