use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

const SEPARATOR: &str = "}{";

fn parse_commands(args: Vec<String>) -> Vec<Vec<String>> {
    let mut commands = Vec::new();
    let mut current_command = Vec::new();

    for arg in args {
        if arg == SEPARATOR {
            if !current_command.is_empty() {
                commands.push(current_command.clone());
                current_command.clear();
            }
        } else {
            current_command.push(arg);
        }
    }

    if !current_command.is_empty() {
        commands.push(current_command);
    }

    commands
}

fn execute_command(cmd_args: Vec<String>) -> i32 {
    let cmd_name = cmd_args[0].clone();

    let mut child = Command::new(&cmd_args[0])
        .args(&cmd_args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to execute {}", cmd_name));

    let stdout_handle = if let Some(stdout) = child.stdout.take() {
        Some(thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                println!("{}", line);
            }
        }))
    } else {
        None
    };

    let stderr_handle = if let Some(stderr) = child.stderr.take() {
        Some(thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                eprintln!("{}", line);
            }
        }))
    } else {
        None
    };

    if let Some(h) = stdout_handle {
        h.join().unwrap();
    }
    if let Some(h) = stderr_handle {
        h.join().unwrap();
    }

    let status = child.wait().expect("Failed to wait for child");
    let code = status.code().unwrap_or(1);

    eprintln!("{}", cmd_name);

    code
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let commands = parse_commands(args);

    if commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }

    let exit_codes = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for cmd_args in commands {
        let exit_codes = Arc::clone(&exit_codes);

        let handle = thread::spawn(move || {
            let code = execute_command(cmd_args);
            exit_codes.lock().unwrap().push(code);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let codes = exit_codes.lock().unwrap();
    for &code in codes.iter() {
        if code != 0 {
            std::process::exit(code);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands_single_command() {
        let args = vec!["echo".to_string(), "hello".to_string()];
        let result = parse_commands(args);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["echo", "hello"]);
    }

    #[test]
    fn test_parse_commands_multiple_commands() {
        let args = vec![
            "echo".to_string(),
            "hello".to_string(),
            "}{".to_string(),
            "ls".to_string(),
            "-la".to_string(),
            "}{".to_string(),
            "pwd".to_string(),
        ];
        let result = parse_commands(args);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec!["echo", "hello"]);
        assert_eq!(result[1], vec!["ls", "-la"]);
        assert_eq!(result[2], vec!["pwd"]);
    }

    #[test]
    fn test_parse_commands_empty() {
        let args: Vec<String> = vec![];
        let result = parse_commands(args);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_commands_with_trailing_separator() {
        let args = vec!["echo".to_string(), "hello".to_string(), "}{".to_string()];
        let result = parse_commands(args);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["echo", "hello"]);
    }

    #[test]
    fn test_parse_commands_with_leading_separator() {
        let args = vec!["}{".to_string(), "echo".to_string(), "hello".to_string()];
        let result = parse_commands(args);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["echo", "hello"]);
    }

    #[test]
    fn test_parse_commands_with_consecutive_separators() {
        let args = vec![
            "echo".to_string(),
            "hello".to_string(),
            "}{".to_string(),
            "}{".to_string(),
            "pwd".to_string(),
        ];
        let result = parse_commands(args);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["echo", "hello"]);
        assert_eq!(result[1], vec!["pwd"]);
    }

    #[test]
    fn test_execute_command_success() {
        let cmd_args = vec!["echo".to_string(), "test".to_string()];
        let exit_code = execute_command(cmd_args);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_execute_command_with_exit_code() {
        let cmd_args = vec!["sh".to_string(), "-c".to_string(), "exit 42".to_string()];
        let exit_code = execute_command(cmd_args);
        assert_eq!(exit_code, 42);
    }
}
