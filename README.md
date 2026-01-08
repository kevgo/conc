# conc

_Conc_ runs multiple CLI commands concurrently and returns the first non-zero
exit code it encounters.

This is useful for development scripts, CI jobs, or Makefiles where you want to
run several tools in parallel and still keep track of test failures.

[![linux](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml)

## usage

Provide the commands to execute as strings.

```
conc "echo one" "echo two" "echo three"
```

This call executes these three commands concurrently:

- `echo one`
- `echo two`
- `echo three`

You can - of course - write each command on a new line for better readability:

```
conc "echo one" \
     "echo two" \
     "echo three"
```

Commands execute inside a shell (`sh` on Linux/macOS, `cmd.exe` on Windows), so
you can use shell operators:

```
conc "echo one | grep on > file"
```

### output verbosity

When running linters, tests, or compilers, you're often only interested in the
overall success signal and detailed output only for problems. The `--show` flag
lets you control how much output _conc_ emits:

- `--show=all` (default) prints the output of every task, once the task finishes
- `--show=failed` prints only the output of tasks that exit with a non-zero
  status

Flags for conc must appear before any commands to execute, like this:

```bash
conc --show=failed "echo one" "echo two"
```

### colors

_Conc_ emits ANSI colors if STDOUT and STDERR are connected to a TTY. You can
override this behavior using environment variables:

- `CLICOLOR_FORCE=1` always enables color output, even when not writing to a TTY
- `NO_COLOR=1` disables colors

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): offers similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
