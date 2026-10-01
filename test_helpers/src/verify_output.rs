/// Placeholder used in expected output for the platform shell invocation.
const SHELL_PLACEHOLDER: &str = "{shell}";

/// verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output
///
/// # Panics
pub fn verify_output(name: &str, mut have: String, wants: &[String], shell: &str) {
    for want in wants {
        let found;
        (have, found) = remove_matches(have, want, shell);
        assert!(
            found,
            "Didn't find '{want}' in {name}\nremaining unchecked text in {name}:\n'{have}'"
        );
    }
    assert!(have.trim().is_empty(), "Extra {name} output found:\n{have}");
}

/// Returns `have` with the first match of `want` removed, and whether any match was found.
fn remove_matches(mut have: String, want: &str, shell: &str) -> (String, bool) {
    if want.is_empty() {
        return (have, false);
    }
    let normalized = normalize_shell_lines(&have, shell);
    let Some(start) = normalized.text.find(want) else {
        return (have, false);
    };
    let end = start + want.len();
    let Some((orig_start, orig_end)) = normalized.original_range(start, end, have.len()) else {
        return (have, false);
    };
    have.replace_range(orig_start..orig_end, "");
    (have, true)
}

/// `have` with the first shell path prefix replaced by `{shell}`.
struct NormalizedText {
    text: String,
    /// The replaced span. `{shell}` begins at `origin_start` in `text`.
    replacement: Option<ShellReplacement>,
}

/// Original bytes replaced by the first `{shell}`.
///
/// The span runs from the start of that line through the end of `shell`.
#[derive(Clone, Copy)]
struct ShellReplacement {
    origin_start: usize,
    origin_end: usize,
}

fn normalize_shell_lines(have: &str, shell: &str) -> NormalizedText {
    let Some(shell_at) = find_shell(have, shell) else {
        return NormalizedText {
            text: have.to_owned(),
            replacement: None,
        };
    };
    let origin_start = line_start(have, shell_at);
    let origin_end = shell_at + shell.len();
    let text = format!(
        "{}{SHELL_PLACEHOLDER}{}",
        &have[..origin_start],
        &have[origin_end..]
    );
    NormalizedText {
        text,
        replacement: Some(ShellReplacement {
            origin_start,
            origin_end,
        }),
    }
}

fn find_shell(have: &str, shell: &str) -> Option<usize> {
    if shell.is_empty() {
        None
    } else {
        have.find(shell)
    }
}

/// Byte index of the line that contains `index`.
fn line_start(text: &str, index: usize) -> usize {
    match text[..index].rfind('\n') {
        Some(newline) => newline + 1,
        None => 0,
    }
}

impl NormalizedText {
    /// Maps a match `start..end` in `text` back to a byte range in the original text.
    fn original_range(&self, start: usize, end: usize, have_len: usize) -> Option<(usize, usize)> {
        if start >= end || end > self.text.len() {
            return None;
        }
        let Some(replacement) = self.replacement else {
            return valid_range(start, end, have_len);
        };
        let placeholder_end = replacement.origin_start + SHELL_PLACEHOLDER.len();
        let orig_start = if start < replacement.origin_start {
            start
        } else if start >= placeholder_end {
            replacement.origin_end + (start - placeholder_end)
        } else {
            replacement.origin_start
        };
        let last = end - 1;
        let orig_end = if last < replacement.origin_start {
            end
        } else if last >= placeholder_end {
            replacement.origin_end + (end - placeholder_end)
        } else {
            replacement.origin_end
        };
        valid_range(orig_start, orig_end, have_len)
    }
}

fn valid_range(start: usize, end: usize, have_len: usize) -> Option<(usize, usize)> {
    if start >= end || end > have_len {
        None
    } else {
        Some((start, end))
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
