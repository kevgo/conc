# conc

_Conc_ runs multiple CLI commands concurrently and returns the first non-zero
exit code it encounters.

This is useful for development scripts, CI jobs, or Makefiles where you want to
run several tools in parallel and still keep track of test failures.

### Usage

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

### Alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): offers similar
  functionality, but does not reliably propagate a single, meaningful exit code
  suitable for use in scripts and Makefiles.
