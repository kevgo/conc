Feature: run multiple commands concurrently

	Scenario: exit code 0
		When I run "conc --show=all 'exit 0'"
		Then the output contains:
			"""
			exit 0
			"""
		Then the exit code is 0
