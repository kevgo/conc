mod cli;
mod errors;
mod subshell;

use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let commands = cli::arguments::parse_commands(env::args().into_iter());

    if commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }

    let (tx, rx) = mpsc::channel();
    let handles: Vec<_> = commands
        .into_iter()
        .map(|cmd_args| {
            let tx = tx.clone();
            thread::spawn(move || {
                let result = execute_command(cmd_args);
                let exit_code = match result {
                    Ok(code) => code,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        1
                    }
                };
                let _ = tx.send(exit_code);
            })
        })
        .collect();

    // Drop the original sender so the receiver knows when all threads are done
    drop(tx);

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    // Collect all exit codes and exit with the first non-zero code found
    if let Some(error_code) = rx.iter().find(|&code| code != 0) {
        std::process::exit(error_code);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_execute_command_success() {
        let cmd_args = strs(&["echo", "test"]);
        let result = execute_command(cmd_args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_execute_command_with_exit_code() {
        let cmd_args = strs(&["sh", "-c", "exit 42"]);
        let result = execute_command(cmd_args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_execute_command_empty_args() {
        let cmd_args: Vec<String> = Vec::new();
        let result = execute_command(cmd_args);
        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::EmptyCommand)));
    }

    #[test]
    fn test_execute_command_nonexistent() {
        let cmd_args = strs(&["nonexistent_command_xyz123"]);
        let result = execute_command(cmd_args);
        assert!(result.is_err());
        assert!(matches!(result, Err(CommandError::SpawnFailed(_, _))));
    }
}
