use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut commands = Vec::new();
    let mut current_command = Vec::new();

    for arg in args {
        if arg == "}{" {
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

    if commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }

    let exit_codes = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for cmd_args in commands {
        let exit_codes = Arc::clone(&exit_codes);

        let handle = thread::spawn(move || {
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
