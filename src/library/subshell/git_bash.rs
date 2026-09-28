//! Finds Git for Windows' non-interactive Bash executable.
//!
//! `git-bash.exe` opens a terminal window.
//! Conc uses `bin\bash.exe`, which runs a script and exits.

use std::env;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::process::Command;
use std::sync::OnceLock;

/// Returns the Git Bash executable, if one can be found.
///
/// The result is cached for the process.
#[must_use]
pub(super) fn find() -> Option<PathBuf> {
    static GIT_BASH: OnceLock<Option<PathBuf>> = OnceLock::new();
    GIT_BASH.get_or_init(locate).clone()
}

fn locate() -> Option<PathBuf> {
    bash_near_git_on_path()
        .or_else(git_bash_on_path)
        .or_else(known_git_bash)
        .or_else(bash_from_registry)
}

/// `bin\bash.exe` or `usr\bin\bash.exe` next to a `git.exe` that belongs to Git.
fn bash_near_git_on_path() -> Option<PathBuf> {
    files_on_path("git.exe")
        .into_iter()
        .find_map(|git_exe| bash_for_git_exe(&git_exe))
}

fn bash_for_git_exe(git_exe: &Path) -> Option<PathBuf> {
    let root = git_install_root(git_exe)?;
    if !path_contains_git(&root) && !path_contains_git(git_exe) {
        return None;
    }
    bash_in_install(&root)
}

fn git_install_root(git_exe: &Path) -> Option<PathBuf> {
    let parent = git_exe.parent()?;
    let parent_name = parent.file_name()?;
    if parent_name.eq_ignore_ascii_case("cmd") {
        return parent.parent().map(Path::to_path_buf);
    }
    if parent_name.eq_ignore_ascii_case("bin") {
        let root = parent.parent()?;
        if root.file_name().is_some_and(|name| {
            name.eq_ignore_ascii_case("mingw64")
                || name.eq_ignore_ascii_case("mingw32")
                || name.eq_ignore_ascii_case("usr")
        }) {
            return root.parent().map(Path::to_path_buf);
        }
        return Some(root.to_path_buf());
    }
    None
}

/// Prefers `bin\bash.exe` over `usr\bin\bash.exe`.
///
/// `bin\bash.exe` is the wrapper that sets up the Git Bash environment.
fn bash_in_install(root: &Path) -> Option<PathBuf> {
    let preferred = root.join("bin").join("bash.exe");
    if preferred.is_file() {
        return Some(preferred);
    }
    let usr = root.join("usr").join("bin").join("bash.exe");
    if usr.is_file() {
        return Some(usr);
    }
    None
}

fn git_bash_on_path() -> Option<PathBuf> {
    let mut fallback = None;
    for bash in files_on_path("bash.exe") {
        if !is_git_bash(&bash) {
            continue;
        }
        if is_preferred_layout(&bash) {
            return Some(bash);
        }
        if fallback.is_none() {
            fallback = Some(bash);
        }
    }
    fallback
}

fn known_git_bash() -> Option<PathBuf> {
    known_bash_paths().into_iter().find(|path| path.is_file())
}

fn known_bash_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for key in ["ProgramFiles", "ProgramW6432", "ProgramFiles(x86)"] {
        if let Some(dir) = env::var_os(key) {
            paths.push(PathBuf::from(dir).join("Git").join("bin").join("bash.exe"));
        }
    }
    if let Some(dir) = env::var_os("LOCALAPPDATA") {
        paths.push(
            PathBuf::from(dir)
                .join("Programs")
                .join("Git")
                .join("bin")
                .join("bash.exe"),
        );
    }
    if let Some(dir) = env::var_os("USERPROFILE") {
        paths.push(
            PathBuf::from(dir)
                .join("scoop")
                .join("apps")
                .join("git")
                .join("current")
                .join("bin")
                .join("bash.exe"),
        );
    }
    paths
}

#[cfg(windows)]
fn bash_from_registry() -> Option<PathBuf> {
    const REGISTRY_KEYS: &[&str] = &[
        r"HKLM\SOFTWARE\GitForWindows",
        r"HKLM\SOFTWARE\WOW6432Node\GitForWindows",
        r"HKCU\SOFTWARE\GitForWindows",
    ];
    for key in REGISTRY_KEYS {
        if let Some(bash) = bash_from_registry_key(key) {
            return Some(bash);
        }
    }
    None
}

#[cfg(not(windows))]
fn bash_from_registry() -> Option<PathBuf> {
    None
}

#[cfg(windows)]
fn bash_from_registry_key(key: &str) -> Option<PathBuf> {
    let install = query_install_path(key)?;
    bash_in_install(Path::new(&install))
}

#[cfg(windows)]
fn query_install_path(key: &str) -> Option<String> {
    let output = Command::new("reg")
        .args(["query", key, "/v", "InstallPath"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_install_path(&text).map(ToOwned::to_owned)
}

fn parse_install_path(output: &str) -> Option<&str> {
    for line in output.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("InstallPath") else {
            continue;
        };
        if !rest.starts_with(char::is_whitespace) {
            continue;
        }
        let rest = rest.trim_start();
        let Some(rest) = rest
            .strip_prefix("REG_EXPAND_SZ")
            .or_else(|| rest.strip_prefix("REG_SZ"))
        else {
            continue;
        };
        if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
            continue;
        }
        let path = rest.trim();
        if !path.is_empty() {
            return Some(path);
        }
    }
    None
}

fn files_on_path(file_name: &str) -> Vec<PathBuf> {
    let Some(path_var) = env::var_os("PATH") else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for dir in env::split_paths(&path_var) {
        if dir.as_os_str().is_empty() {
            continue;
        }
        let candidate = dir.join(file_name);
        if candidate.is_file() {
            found.push(candidate);
        }
    }
    found
}

fn is_git_bash(path: &Path) -> bool {
    is_bash_exe(path) && !is_windows_system_bash(path) && path_contains_git(path)
}

fn is_bash_exe(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("bash.exe"))
}

fn path_contains_git(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str().eq_ignore_ascii_case("git"))
}

/// `...\bin\bash.exe` whose parent is not `usr`.
fn is_preferred_layout(path: &Path) -> bool {
    let mut components = path.components().rev();
    let Some(file) = components.next() else {
        return false;
    };
    if !file.as_os_str().eq_ignore_ascii_case("bash.exe") {
        return false;
    }
    let Some(bin_dir) = components.next() else {
        return false;
    };
    if !bin_dir.as_os_str().eq_ignore_ascii_case("bin") {
        return false;
    }
    let Some(parent) = components.next() else {
        return false;
    };
    !parent.as_os_str().eq_ignore_ascii_case("usr")
}

fn is_windows_system_bash(path: &Path) -> bool {
    let mut components = path.components().rev();
    let Some(file) = components.next() else {
        return false;
    };
    let Some(dir) = components.next() else {
        return false;
    };
    let Some(windows) = components.next() else {
        return false;
    };
    file.as_os_str().eq_ignore_ascii_case("bash.exe")
        && windows.as_os_str().eq_ignore_ascii_case("windows")
        && (dir.as_os_str().eq_ignore_ascii_case("system32")
            || dir.as_os_str().eq_ignore_ascii_case("sysnative")
            || dir.as_os_str().eq_ignore_ascii_case("syswow64"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(path: &Path) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, b"").unwrap();
    }

    mod git_install_root {
        use super::*;

        fn git_root() -> PathBuf {
            PathBuf::from("C:/Program Files/Git")
        }

        #[test]
        fn cmd_directory() {
            let root = git_root();
            let have = git_install_root(&root.join("cmd").join("git.exe"));
            let want = Some(root);
            assert_eq!(have, want);
        }

        #[test]
        fn bin_directory() {
            let root = git_root();
            let have = git_install_root(&root.join("bin").join("git.exe"));
            let want = Some(root);
            assert_eq!(have, want);
        }

        #[test]
        fn mingw64_bin() {
            let root = git_root();
            let have = git_install_root(&root.join("mingw64").join("bin").join("git.exe"));
            let want = Some(root);
            assert_eq!(have, want);
        }

        #[test]
        fn usr_bin() {
            let root = git_root();
            let have = git_install_root(&root.join("usr").join("bin").join("git.exe"));
            let want = Some(root);
            assert_eq!(have, want);
        }

        #[test]
        fn unrelated_executable() {
            let have = git_install_root(Path::new("git.exe"));
            assert_eq!(have, None);
        }
    }

    mod bash_for_git_exe {
        use super::*;

        #[test]
        fn prefers_bin_over_usr() {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().join("Git");
            let git_exe = root.join("mingw64").join("bin").join("git.exe");
            let bin_bash = root.join("bin").join("bash.exe");
            let usr_bash = root.join("usr").join("bin").join("bash.exe");
            touch(&git_exe);
            touch(&bin_bash);
            touch(&usr_bash);
            let have = bash_for_git_exe(&git_exe);
            let want = Some(bin_bash);
            assert_eq!(have, want);
        }

        #[test]
        fn falls_back_to_usr_bin() {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().join("Git");
            let git_exe = root.join("cmd").join("git.exe");
            let usr_bash = root.join("usr").join("bin").join("bash.exe");
            touch(&git_exe);
            touch(&usr_bash);
            let have = bash_for_git_exe(&git_exe);
            let want = Some(usr_bash);
            assert_eq!(have, want);
        }

        #[test]
        fn ignores_unrelated_git_executable() {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().join("chocolatey");
            let git_exe = root.join("bin").join("git.exe");
            touch(&git_exe);
            touch(&root.join("bin").join("bash.exe"));
            let have = bash_for_git_exe(&git_exe);
            assert_eq!(have, None);
        }

        #[test]
        fn missing_bash() {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().join("Git");
            let git_exe = root.join("cmd").join("git.exe");
            touch(&git_exe);
            let have = bash_for_git_exe(&git_exe);
            assert_eq!(have, None);
        }
    }

    mod parse_install_path {
        use super::parse_install_path;

        #[test]
        fn reg_sz() {
            let output = "\r\nHKEY_LOCAL_MACHINE\\SOFTWARE\\GitForWindows\r\n    InstallPath    REG_SZ    C:\\Program Files\\Git\r\n";
            let have = parse_install_path(output);
            let want = Some(r"C:\Program Files\Git");
            assert_eq!(have, want);
        }

        #[test]
        fn reg_expand_sz() {
            let output = "    InstallPath    REG_EXPAND_SZ    D:\\Git\r\n";
            let have = parse_install_path(output);
            let want = Some(r"D:\Git");
            assert_eq!(have, want);
        }

        #[test]
        fn missing() {
            let have = parse_install_path("not a registry listing");
            assert_eq!(have, None);
        }
    }

    mod is_windows_system_bash {
        use super::*;

        #[test]
        fn system32() {
            let have = is_windows_system_bash(Path::new("C:/Windows/System32/bash.exe"));
            assert!(have);
        }

        #[test]
        fn syswow64() {
            let have = is_windows_system_bash(Path::new("C:/Windows/SysWOW64/bash.exe"));
            assert!(have);
        }

        #[test]
        fn git_bash() {
            let have = is_windows_system_bash(Path::new("C:/Program Files/Git/bin/bash.exe"));
            assert!(!have);
        }
    }

    mod is_preferred_layout {
        use super::*;

        #[test]
        fn bin_bash() {
            let have = is_preferred_layout(Path::new("C:/Program Files/Git/bin/bash.exe"));
            assert!(have);
        }

        #[test]
        fn usr_bin_bash() {
            let have = is_preferred_layout(Path::new("C:/Program Files/Git/usr/bin/bash.exe"));
            assert!(!have);
        }
    }

    #[test]
    fn locate_is_a_git_bash_when_present() {
        let Some(bash) = locate() else {
            return;
        };
        assert!(bash.is_file(), "{}", bash.display());
        assert!(!is_windows_system_bash(&bash), "{}", bash.display());
        assert!(is_bash_exe(&bash), "{}", bash.display());
    }
}
