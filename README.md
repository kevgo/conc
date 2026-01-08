# conc

_Conc_ runs multiple CLI commands concurrently and returns the first non-zero
exit code it encounters.

It is intended for development scripts, CI pipelines, and Makefiles that execute
many tools in parallel and keep track of failures.

[![linux](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml)

## usage

Pass the commands to execute as strings.

```
conc "echo one" "echo two" "echo three"
```

This runs these three commands concurrently:

- `echo one`
- `echo two`
- `echo three`

For better readability, you can put each command on its own line:

```
conc "echo one" \
     "echo two" \
     "echo three"
```

Commands execute inside a shell (`sh` on Linux/macOS, `cmd.exe` on Windows), so
you can use shell features like pipes and redirection:

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

Flags must appear before the commands to execute:

```bash
conc --show=failed "echo one" "echo two"
```

### colors

_Conc_ emits ANSI colors when STDOUT and STDERR are connected to a TTY. You can
override this behavior using environment variables:

- `CLICOLOR_FORCE=1` always enables color output, even when not connected to a
  TTY
- `NO_COLOR=1` disables colors

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): provides similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
