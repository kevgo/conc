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
conc app1 arg1a arg1b }{ \
     app2 arg2a arg2b }{ \
     app3 arg3a arg3b
```

This call executes:

- `app1 arg1a arg1b`
- `app2 arg2a arg2b`
- `app3 arg3a arg3b`

### customize the output

When running linters or compilers, you are often only interested in knowing
whether everything succeeded, and only want to know specifics about things that
failed. The `--show` flag allows customizing the output this way:

- `--show=all` (default) displays all output, once the respective task finishes
- `--show=failed` displays only the output of tasks that failed

Flags for _conc_ must come before any commands to execute:

```bash
conc --show=failed \
     app1 arg1a arg1b }{ \
     app2 arg2a arg2b
```

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): offers similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
