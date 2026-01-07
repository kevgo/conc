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

### show only relevant output

When running linters, you are often not interested in the output of successful
linters, especially if they are noisy.

```bash
conc --relevant \
     app1 arg1a arg1b }{ \
     app2 arg2a arg2b }{ \
     app3 arg3a arg3b
```

## alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): offers similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
