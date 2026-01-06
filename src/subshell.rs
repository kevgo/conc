/// Executes a single command with its arguments, streaming output to stdout/stderr.
fn execute_command(cmd_args: Vec<String>) -> Result<i32, CommandError> {
    let cmd_name = cmd_args.first().ok_or(CommandError::EmptyCommand)?.clone();

    let mut child = Command::new(&cmd_args[0])
        .args(&cmd_args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| CommandError::SpawnFailed(cmd_name.clone(), err))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let stdout_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                println!("{}", line);
            }
        })
    });

    let stderr_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                eprintln!("{}", line);
            }
        })
    });

    if let Some(handle) = stdout_handle {
        let _ = handle.join();
    }
    if let Some(handle) = stderr_handle {
        let _ = handle.join();
    }

    let status = child.wait().map_err(CommandError::WaitFailed)?;
    let code = status.code().unwrap_or(1);

    eprintln!("{}", cmd_name);

    Ok(code)
}
