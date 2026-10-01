/// Placeholder used in expected output for the platform shell invocation.
const SHELL_PLACEHOLDER: &str = "{shell}";

/// Verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output.
///
/// # Panics
pub fn verify_output(name: &str, mut have: String, wants: &[String], shell: &str) {
    for want in wants {
        let found = if let Some(range) = matched_range(&have, want, shell) {
            have.replace_range(range, "");
            true
        } else {
            false
        };
        assert!(
            found,
            "Didn't find '{want}' in {name}\nremaining unchecked text in {name}:\n'{have}'"
        );
    }
    assert!(have.trim().is_empty(), "Extra {name} output found:\n{have}");
}

/// First shell invocation in the text, from the start of its line through `shell`.
struct ShellSpan {
    start: usize,
    end: usize,
}

fn matched_range(have: &str, want: &str, shell: &str) -> Option<std::ops::Range<usize>> {
    if want.is_empty() {
        return None;
    }
    let Some(span) = shell_span(have, shell) else {
        let start = have.find(want)?;
        return Some(start..start + want.len());
    };
    let normalized = format!(
        "{}{SHELL_PLACEHOLDER}{}",
        &have[..span.start],
        &have[span.end..]
    );
    let start = normalized.find(want)?;
    Some(span.original_range(start, start + want.len()))
}

fn shell_span(have: &str, shell: &str) -> Option<ShellSpan> {
    if shell.is_empty() {
        return None;
    }
    let shell_at = have.find(shell)?;
    let start = match have[..shell_at].rfind('\n') {
        Some(newline) => newline + 1,
        None => 0,
    };
    Some(ShellSpan {
        start,
        end: shell_at + shell.len(),
    })
}

impl ShellSpan {
    /// Maps a match in the normalized text back onto the original text.
    ///
    /// `{shell}` begins at `self.start` and replaces `have[self.start..self.end]`.
    fn original_range(&self, match_start: usize, match_end: usize) -> std::ops::Range<usize> {
        let placeholder_end = self.start + SHELL_PLACEHOLDER.len();
        // An index inside `{shell}` expands to `at_placeholder`.
        let expand = |index: usize, at_placeholder: usize| {
            if index <= self.start {
                index
            } else if index < placeholder_end {
                at_placeholder
            } else {
                self.end + index - placeholder_end
            }
        };
        expand(match_start, self.start)..expand(match_end, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use big_s::S;

    #[test]
    fn exact_match() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world")];
        verify_output("stdout", have, &wants, "");
    }

    #[test]
    #[should_panic(expected = "Extra stdout output found:\n world")]
    fn expect_too_little() {
        let have = S("hello world");
        let wants = vec![S("hello")];
        verify_output("stdout", have, &wants, "");
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'extra' in stdout\nremaining unchecked text in stdout:\n' '"
    )]
    fn expect_too_much() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world"), S("extra")];
        verify_output("stdout", have, &wants, "");
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'hallo' in stdout\nremaining unchecked text in stdout:\n'hello'"
    )]
    fn different() {
        let have = S("hello");
        let wants = vec![S("hallo")];
        verify_output("stdout", have, &wants, "");
    }

    #[test]
    fn windows_bash_correct() {
        let have = S("line 1\nc:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'\nline 3");
        let wants = vec![S("line 1"), S("{shell} 'echo hello'"), S("line 3")];
        verify_output("stdout", have, &wants, "bash.exe -c");
    }

    #[test]
    #[should_panic(
        expected = "Didn't find '{shell} 'echo hello'' in stdout\nremaining unchecked text in stdout:\n'\nc:\\Program Files\\Git\\bin\\bash.exe -c 'echo zonk'\nline 3'"
    )]
    fn windows_bash_incorrect() {
        let have = S("line 1\nc:\\Program Files\\Git\\bin\\bash.exe -c 'echo zonk'\nline 3");
        let wants = vec![S("line 1"), S("{shell} 'echo hello'"), S("line 3")];
        verify_output("stdout", have, &wants, "bash.exe -c");
    }

    #[test]
    fn unix_sh() {
        let have = S("line 1\nsh -c 'echo hello'\nline 3");
        let wants = vec![S("line 1"), S("{shell} 'echo hello'"), S("line 3")];
        verify_output("stdout", have, &wants, "sh -c");
    }

    #[test]
    fn replaces_only_first_shell() {
        let have = S("c:\\Program Files\\Git\\bin\\bash.exe -c 'echo one'\n\
             c:\\Program Files\\Git\\bin\\bash.exe -c 'echo two'");
        let wants = vec![S("{shell} 'echo one'\n\
             c:\\Program Files\\Git\\bin\\bash.exe -c 'echo two'")];
        verify_output("stdout", have, &wants, "bash.exe -c");
    }
}
