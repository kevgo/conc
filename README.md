# conc

This application executes multiple CLI arguments concurrently on the command line.
If one of the applications exits with an error, it exits with that error code.
This is helpful for executing multiple tools concurrently in development scripts or Makefiles.

### Usage

#### Version 1

```
conc app1 arg1 arg2
conc app2 arg1 arg2
conc app3 arg1 arg3
conc --wait
```

#### Version 2

```
conc 'app1 arg1 arg2' \
     'app2 arg1 arg2' \
     'app3 arg1 arg2'
```

### Implementation

Benefits of version 1 is that it's easy to conditionally call or not call apps.
A challenge with implementing version 1 is locating an already running instance and communicating with it.
Another challenges with version 1 is that we must not forget to call `conc --wait` at the end.
Implementing this version would require a status file in the user's home directory.

Version 2 would be straightforward to implement.

### Alternatives

- [gnu parallel](https://www.gnu.org/software/parallel): does pretty much exactly this, but somehow doesn't seem to support returning a proper exit code
