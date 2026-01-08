Feature: run multiple commands concurrently

	Scenario: run multiple commands
		When I run "conc --show=failed 'echo one' 'echo two' 'echo three'"
		Then the output contains these lines in any order:
			"""
			echo one
			echo two
			echo three
			"""
