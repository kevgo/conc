# conc changelog

## 0.6.1

- `Runnable` emits a more readable `Debug` format

## 0.6.0

- new API methods: `Runnable.len()` and `Runnable.is_empty()`

## 0.5.0

- the API allows that some of the executed commands are a sequence of commands

## 0.4.1

- all relevant output including error messages go to STDOUT,
  STDERR is only for internal logging

## 0.4.0

- [--stderr-to-stdout](https://github.com/kevgo/conc#output-redirection) flag

## 0.3.2

Publishes the crate including all source code
and tests to allow the Rust team to include this in crater runs.

## 0.3.1

Rust API updates

## 0.3.0

Improved arguments for `--show`: `all`, `names`, `failed`

## 0.2.0

- `--show=min` flag to only show failed commands

## 0.1.0

- runs given apps concurrently
- runs apps inside a shell
- error-on-output flag
- `--show=failed` flag to only show the output of failed commands
- `--help` command
- `--version` command
