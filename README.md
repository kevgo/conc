# conc

_Conc_ runs multiple CLI commands concurrently, returns the first non-zero exit
code it encounters, and filters command output.

It is intended for development scripts and CI pipelines.

[![linux](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml)

## usage

Pass the commands to execute as strings.

```
conc "echo one" "echo two" "echo three"
```

This executes the following commands concurrently:

- `echo one`
- `echo two`
- `echo three`

For readability, you can put each command on its own line:

```
conc "echo one" \
     "echo two" \
     "echo three"
```

Commands are executed inside a shell (`sh` on Linux/macOS, `cmd.exe` on
Windows), so you can use shell features:

```
conc "echo one && echo two | grep on > file"
```

### output verbosity

When running linters, tests, or compilers, you often only care whether
everything passed and want detailed output only for problems. The `--show` flag
controls how much output _conc_ prints:

- `--show=all` (default) prints the name and output of every command after it
  finishes
- `--show=failed` prints the name of every command after it finishes, but only
  the output of failed commands
- `--show=min` prints only the name and output of failed commands

Flags for _conc_ must appear before the first command to execute:

```bash
conc --show=failed "echo one" "echo two"
```

### colors

_Conc_ emits ANSI colors when STDOUT and STDERR are connected to a TTY. You can
override this behavior using environment variables:

- `CLICOLOR_FORCE=1` always enables color output, even when not connected to a
  TTY
- `NO_COLOR=1` disables colors

### error on output

Some tools report findings via STDOUT or STDERR but still exit with a success
code. The `--error-on-output` flag treats any output as failure.

```
conc --error-on-output "echo foo"
```

This exits with status code 1 because the command produced output.

If you want this behavior only for specific commands, wrap them in a nested
_conc_ call. For example, to enable error on output only for `command 2`:

```
conc "command 1" \
     "conc --error-on-output 'command 2'" \
     "command 3"
```

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): also runs commands
  concurrently, but does not reliably propagate a single, meaningful exit code
  suitable for scripts
