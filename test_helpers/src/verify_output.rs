use std::ops::Range;

/// Placeholder used in expected output for the platform shell invocation.
const SHELL_PLACEHOLDER: &str = "{shell}";

/// Verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output.
///
/// # Panics
pub fn verify_output(name: &str, mut have: String, wants: &[String], shell: &str) {
    for want in wants {
        let Some(range) = matched_range(&have, want, shell) else {
            panic!("Didn't find '{want}' in {name}\nremaining unchecked text in {name}:\n'{have}'")
        };
        have.replace_range(range, "");
    }
    assert!(have.trim().is_empty(), "Extra {name} output found:\n{have}");
}

fn matched_range(haystack: &str, needle: &str, shell: &str) -> Option<Range<usize>> {
    assert!(!needle.is_empty(), "empty needle");
    let Some(span) = shell_span(haystack, shell) else {
        // no shell invocation found --> return the location of the needle
        let start = haystack.find(needle)?;
        return Some(start..start + needle.len());
    };
    // found a shell invocation --> return the location of the shell invocation
    let normalized = format!(
        "{}{SHELL_PLACEHOLDER}{}",
        &haystack[..span.start],
        &haystack[span.end..]
    );
    let start = normalized.find(needle)?;
    Some(original_range(&span, start, start + needle.len()))
}

/// Finds the first shell invocation in the given text, returns the start of the line until the match.
fn shell_span(have: &str, shell: &str) -> Option<Range<usize>> {
    assert!(!shell.is_empty(), "empty shell");
    let shell_at = have.find(shell)?;
    let start = match have[..shell_at].rfind('\n') {
        Some(newline) => newline + 1,
        None => 0,
    };
    Some(start..shell_at + shell.len())
}

/// Maps a match in the normalized text back onto the original text.
///
/// `{shell}` begins at `span.start` and replaces `have[span.start..span.end]`.
fn original_range(span: &Range<usize>, match_start: usize, match_end: usize) -> Range<usize> {
    let placeholder_end = span.start + SHELL_PLACEHOLDER.len();
    // An index inside `{shell}` expands to `at_placeholder`.
    let expand = |index: usize, at_placeholder: usize| {
        if index <= span.start {
            index
        } else if index < placeholder_end {
            at_placeholder
        } else {
            span.end + index - placeholder_end
        }
    };
    expand(match_start, span.start)..expand(match_end, span.end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use big_s::S;

    #[test]
    fn exact_match() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world")];
        verify_output("stdout", have, &wants, "sh -c");
    }

    #[test]
    #[should_panic(expected = "Extra stdout output found:\n world")]
    fn expect_too_little() {
        let have = S("hello world");
        let wants = vec![S("hello")];
        verify_output("stdout", have, &wants, "sh -c");
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'extra' in stdout\nremaining unchecked text in stdout:\n' '"
    )]
    fn expect_too_much() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world"), S("extra")];
        verify_output("stdout", have, &wants, "sh -c");
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'hallo' in stdout\nremaining unchecked text in stdout:\n'hello'"
    )]
    fn different() {
        let have = S("hello");
        let wants = vec![S("hallo")];
        verify_output("stdout", have, &wants, "sh -c");
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

    mod shell_span {
        use super::*;

        #[test]
        fn absent() {
            assert_eq!(shell_span("hello world", "bash.exe -c"), None);
            assert_eq!(shell_span("", "sh -c"), None);
        }

        #[test]
        fn at_start_excludes_rest_of_line() {
            let text = "sh -c 'echo hello'";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(0..5));
            assert_eq!(have.map(|span| &text[span]), Some("sh -c"));
        }

        #[test]
        fn includes_same_line_prefix() {
            let text = "c:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'";
            let have = shell_span(text, "bash.exe -c");
            let want = "c:\\Program Files\\Git\\bin\\bash.exe -c";
            assert_eq!(have, Some(0..want.len()));
            assert_eq!(have.map(|span| &text[span]), Some(want));
        }

        #[test]
        fn starts_after_preceding_newline() {
            let text = "line 1\nsh -c 'echo hello'\nline 3";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(7..12));
            assert_eq!(have.map(|span| &text[span]), Some("sh -c"));
        }

        #[test]
        fn first_occurrence_only() {
            let text = "bash.exe -c one\nbash.exe -c two";
            let have = shell_span(text, "bash.exe -c");
            assert_eq!(have, Some(0..11));
            assert_eq!(have.map(|span| &text[span]), Some("bash.exe -c"));
        }

        #[test]
        fn nearest_newline() {
            let text = "a\n\nsh -c";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(3..8));
            assert_eq!(have.map(|span| &text[span]), Some("sh -c"));
        }

        #[test]
        fn crlf_excludes_carriage_return() {
            let text = "line 1\r\nsh -c rest";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(8..13));
            assert_eq!(have.map(|span| &text[span]), Some("sh -c"));
        }

        #[test]
        fn unicode_on_same_line() {
            let text = "ésh -c";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(0..7));
            assert_eq!(have.map(|span| &text[span]), Some(text));
        }

        #[test]
        fn unicode_on_previous_line() {
            let text = "é\nsh -c tail";
            let have = shell_span(text, "sh -c");
            assert_eq!(have, Some(3..8));
            assert_eq!(have.map(|span| &text[span]), Some("sh -c"));
        }
    }

    mod matched_range {
        use super::*;

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
            assert_eq!(matched_range(haystack, needle, "sh -c"), None);

            let haystack = "sh -c hello";
            let needle = "zonk";
            let have = matched_range(haystack, needle, "sh -c");
            assert_eq!(have, None);
        }

        #[test]
        fn match_before_shell() {
            let haystack = "line 1\nsh -c 'echo hello'\nline 3";
            let needle = "line 1";
            let have = matched_range(haystack, needle, "sh -c");
            assert_eq!(have, Some(0..6));
            let have_text = have.map(|span| &haystack[span]);
            assert_eq!(have_text, Some(needle));
        }

        #[test]
        fn match_after_shell() {
            let haystack = "line 1\nsh -c 'echo hello'\nline 3";
            let needle = "line 3";
            let have = matched_range(haystack, needle, "sh -c");
            assert_eq!(have, Some(26..32));
            let have_text = have.map(|span| &haystack[span]);
            assert_eq!(have_text, Some(needle));
        }

        #[test]
        fn match_shell_line() {
            let haystack = "line 1\nc:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'\nline 3";
            let needle = "{shell} 'echo hello'";
            let want = "c:\\Program Files\\Git\\bin\\bash.exe -c 'echo hello'";
            let have = matched_range(haystack, needle, "bash.exe -c");
            let have_text = have.map(|span| &haystack[span]);
            assert_eq!(have_text, Some(want));
        }

        #[test]
        fn partial_match() {
            let haystack = "c:\\bin\\bash.exe -c hello";
            assert_eq!(matched_range(haystack, "c:\\bin\\", "bash.exe -c"), None);
            assert_eq!(matched_range(haystack, "bash.exe -c", "bash.exe -c"), None);
        }

        #[test]
        fn matches_only_once() {
            let haystack = "bash.exe -c one\nbash.exe -c one";
            let needle = "{shell} one";
            let have = matched_range(haystack, needle, "bash.exe -c");
            assert_eq!(have, Some(0..15));
            let have_text = have.map(|span| &haystack[span]);
            assert_eq!(have_text, Some("bash.exe -c one"));
        }

        #[test]
        fn skips_absorbed_text() {
            let haystack = "aa bash.exe -c aa";
            let needle = "aa";
            let have = matched_range(haystack, needle, "bash.exe -c");
            assert_eq!(have, Some(15..17));
            assert_eq!(have.map(|span| &haystack[span]), Some(needle));
        }

        #[test]
        fn unicode() {
            let haystack = "é\nsh -c tail";
            let needle = "{shell} tail";
            let have = matched_range(haystack, needle, "sh -c");
            assert_eq!(have, Some(3..13));
            assert_eq!(have.map(|span| &haystack[span]), Some("sh -c tail"));
        }
    }
}
