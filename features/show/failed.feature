Feature: run multiple commands concurrently

	Scenario: run multiple commands
		When I run "conc --show=failed 'echo one' 'echo two' 'echo three'"
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
