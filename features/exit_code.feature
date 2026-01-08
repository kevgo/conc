Feature: pass the received exit code to the parent process

	Scenario: exit code 0
		When I run "conc --show=all 'exit 0'"
		Then the output contains:
			"""
			exit 0
			"""
		Then the exit code is 0

	Scenario: exit code 1
		When I run "conc --show=all 'exit 1'"
		Then the output contains:
			"""
			exit 1
			"""
		Then the exit code is 1

	Scenario: exit code 2
		When I run "conc --show=all 'exit 2'"
		Then the output contains:
			"""
			exit 2
			"""
		Then the exit code is 2

	Scenario: exit code 255
		When I run "conc --show=all 'exit 255'"
		Then the output contains:
			"""
			exit 255
			"""
		Then the exit code is 255
