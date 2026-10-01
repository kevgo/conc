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

fn matched_range(haystack: &str, needle: &str, shell: &str) -> Option<std::ops::Range<usize>> {
    if needle.is_empty() {
        return None;
    }
    let Some(span) = shell_span(haystack, shell) else {
        let start = haystack.find(needle)?;
        return Some(start..start + needle.len());
    };
    let normalized = format!(
        "{}{SHELL_PLACEHOLDER}{}",
        &haystack[..span.start],
        &haystack[span.end..]
    );
    let start = normalized.find(needle)?;
    Some(span.original_range(start, start + needle.len()))
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

/// First shell invocation in the text, from the start of its line through `shell`.
struct ShellSpan {
    start: usize,
    end: usize,
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

    mod matched_range {
        use super::*;

        #[test]
        fn empty_needle() {
            let haystack = "sh -c hello";
            let needle = "";
            assert_eq!(matched_range(haystack, needle, "sh -c"), None);
        }

        #[test]
        fn literal_match() {
            let haystack = "hello world";
            let needle = "hello";
            let have = matched_range(haystack, needle, "bash.exe -c");
            assert_eq!(have, Some(0..5));
            let have_text = have.map(|span| &haystack[span]);
            assert_eq!(have_text, Some(needle));
        }

        #[test]
        fn mismatch() {
            let haystack = "hello";
            let needle = "zonk";
            assert_eq!(matched_range(haystack, needle, ""), None);

            let haystack = "sh -c hello";
            let needle = "zonk";
            let have = matched_range(haystack, needle, "sh -c");
            assert_eq!(have, None);
        }

        #[test]
        fn match_before_shell() {
            let have = "line 1\nsh -c 'echo hello'\nline 3";
            let range = matched_range(have, "line 1", "sh -c");
            assert_eq!(range, Some(0..6));
            let have_text = range.map(|span| &have[span]);
            assert_eq!(have_text, Some("line 1"));
        }

        #[test]
        fn match_after_shell() {
            let have = "line 1\nsh -c 'echo hello'\nline 3";
            let range = matched_range(have, "line 3", "sh -c");
            assert_eq!(range, Some(26..32));
            let have_text = range.map(|span| &have[span]);
            assert_eq!(have_text, Some("line 3"));
        }

        #[test]
        fn match_shell_line() {
            let have = "line 1\nc:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'\nline 3";
            let expected = "c:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'";
            let range = matched_range(have, "{shell} 'echo hello'", "bash.exe -c");
            assert_eq!(range, Some(7..7 + expected.len()));
            assert_eq!(range.map(|span| &have[span]), Some(expected));
        }

        #[test]
        fn hides_shell_span() {
            let have = "c:\\bin\\bash.exe -c hello";
            assert_eq!(matched_range(have, "c:\\bin\\", "bash.exe -c"), None);
            assert_eq!(matched_range(have, "bash.exe -c", "bash.exe -c"), None);
            assert_eq!(
                matched_range(have, "{shell} hello", "bash.exe -c"),
                Some(0..have.len())
            );
        }

        #[test]
        fn first_shell_only() {
            let have = "bash.exe -c one\nbash.exe -c two";
            assert_eq!(
                matched_range(have, "{shell} one\nbash.exe -c two", "bash.exe -c"),
                Some(0..have.len())
            );
        }

        #[test]
        fn skips_absorbed_text() {
            let have = "aa bash.exe -c aa";
            let range = matched_range(have, "aa", "bash.exe -c");
            assert_eq!(range, Some(15..17));
            assert_eq!(range.map(|span| &have[span]), Some("aa"));
        }

        #[test]
        fn across_placeholder() {
            let have = "ab\nsh -c cd";
            let range = matched_range(have, "b\n{shell} c", "sh -c");
            assert_eq!(range, Some(1..10));
            assert_eq!(range.map(|span| &have[span]), Some("b\nsh -c c"));
        }

        #[test]
        fn placeholder_alone() {
            let have = "pre\nsh -c post";
            let range = matched_range(have, "{shell}", "sh -c");
            assert_eq!(range, Some(4..9));
            assert_eq!(range.map(|span| &have[span]), Some("sh -c"));
        }

        #[test]
        fn inside_placeholder() {
            let have = "sh -c tail";
            let range = matched_range(have, "hell", "sh -c");
            assert_eq!(range, Some(0..5));
            assert_eq!(range.map(|span| &have[span]), Some("sh -c"));
        }

        #[test]
        fn into_suffix() {
            let have = "sh -c 'x'";
            let range = matched_range(have, "ell} '", "sh -c");
            assert_eq!(range, Some(0..7));
            assert_eq!(range.map(|span| &have[span]), Some("sh -c '"));
        }

        #[test]
        fn byte_offsets() {
            let have = "é\nsh -c tail";
            let range = matched_range(have, "{shell} tail", "sh -c");
            assert_eq!(range, Some(3..13));
            assert_eq!(range.map(|span| &have[span]), Some("sh -c tail"));
        }
    }
}
