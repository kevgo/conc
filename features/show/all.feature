Feature: run multiple commands concurrently

	Scenario: run multiple commands
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
