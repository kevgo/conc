# conc

This application executes multiple CLI arguments concurrently on the command line.
If one of the applications exits with an error, it exits with that error code.
This is helpful for executing multiple tools concurrently in development scripts or Makefiles.

### Usage

```
conc 'app1 arg1a arg1b' \
     'app2 arg2a arg2b' \
     'app3 arg3a arg3b'
```

Alternatively: separate apps to run via `}{`

```
conc app1 arg1a arg1b }{ \
     app2 arg2a arg2b {} \
     app3 arg3a arg3b
```

### Alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): does pretty much exactly this, but somehow doesn't seem to support returning a proper exit code
