# conc

conc runs multiple CLI commands concurrently. If any command exits with a
non-zero status, conc terminates and returns that exit code.

This is useful for development scripts, CI jobs, or Makefiles where you want to
run several tools in parallel and still keep track of test failures.

### Usage

Separate apps to run via `}{`:

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

- [gnu parallel](https://www.gnu.org/software/parallel): does pretty much
  exactly this, but somehow doesn't seem to support returning a proper exit code
