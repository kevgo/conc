# conc

_Conc_ runs multiple CLI commands concurrently and returns the first non-zero
exit code it encounters.

It is designed for development scripts and CI pipelines that execute many tools
in parallel while reliably detecting failures.

[![linux](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml)

## usage

Pass the commands to execute as strings.

```
conc "echo one" "echo two" "echo three"
```

This runs the following commands concurrently:

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
Windows), so you can use shell features like pipes and redirection:

```
conc "echo one | grep on > file"
```

### output verbosity

When running linters, tests, or compilers, you often only care whether
everything succeeded and want detailed output only when something fails. The
`--show` flag controls much output _conc_ prints:

- `--show=all` (default) prints the output of every command after it finishes
- `--show=failed` prints output only for commands that exit with a non-zero
  status

Flags must appear before the commands:

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
code. To catch these cases, the `--error-on-output` flag causes _conc_ to fail
when any command produces output.

```
conc --error-on-output "echo foo"
```

This invocation exits with code 1 because a command printed `foo`.

To enable error on output for only one of the executed commands, wrap that
command in a nested _conc_ call. For example, to enable error on output only for
`command 2`:

```
conc "command 1" "conc --error-on-output 'command 2'" "command 3"
```

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): also runs commands
  concurrently, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
