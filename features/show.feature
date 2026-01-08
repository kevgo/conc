Feature: run multiple commands concurrently

	Scenario: all
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

	Scenario: failed
		When I run "conc --show=failed 'echo one' 'echo two' 'echo three'"
		Then the output contains these lines in any order:
			"""
			echo one
			echo two
			echo three
			"""
		And the exit code is 0
