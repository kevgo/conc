# conc

This application executes multiple CLI arguments concurrently on the command line.
If one of the applications exits with an error, it exits with that error code.
This is helpful for executing multiple tools concurrently in development scripts or Makefiles.

### Usage

This CLI would be ideal:

```
conc app1 arg1 arg2
conc app2 arg1 arg2
conc app3 arg1 arg3
```

Alternatively we could provide all apps to execute at the first call.

```
conc 'app1 arg1 arg2' \
     'app2 arg1 arg2' \
     'app3 arg1 arg2'
```

### Alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): does pretty much exactly this, but somehow doesn't seem to support returning a proper exit code
