#[cfg(any(windows, test))]
mod bash_script;
#[cfg(any(windows, test))]
mod git_bash;
mod run;
mod run_error;
mod shell_executable;

pub use run::{CallResult, run};
pub use shell_executable::shell_executable;
