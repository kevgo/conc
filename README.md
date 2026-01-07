# conc

_Conc_ runs multiple CLI commands concurrently and returns the first non-zero
exit code it encounters.

This is useful for development scripts, CI jobs, or Makefiles where you want to
run several tools in parallel and still keep track of test failures.

[![linux](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/conc/actions/workflows/ci_windows.yml)

## usage

Separate commands with `}{`:

```
conc echo one }{ echo two }{ echo three
```

This call executes these three commands concurrently:

- `echo one`
- `echo two`
- `echo three`

You can also write this on multiple lines:

```
conc echo one }{ \
     echo two }{ \
     echo three
```

### customize the output

When running linters, tests, or compilers, you're often only interested in the
overall success signal and the details of what failed. The `--show` flag lets
you control how much output _conc_ emits:

- `--show=all` (default) prints the output of every task once it finishes
- `--show=failed` prints output only for tasks that exit with a non-zero status

Flags for conc must appear before any commands to execute:

```bash
conc --show=failed \
     app1 arg1a arg1b }{ \
     app2 arg2a arg2b
```

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): offers similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
