/// Placeholder used in expected output for the platform shell invocation.
const SHELL_PLACEHOLDER: &str = "{shell}";

/// verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output
///
/// # Panics
pub fn verify_output(name: &str, mut have: String, wants: &[String], shell: &str) {
    for want in wants {
        let found = remove_matches(&mut have, want, shell);
        assert!(
            found,
            "Didn't find '{want}' in {name}\nremaining unchecked text in {name}:\n'{have}'"
        );
    }
    have = have.trim().to_owned();
    assert!(have.is_empty(), "Extra {name} output found:\n{have}");
}

/// Removes every match of `want` from `have`.
///
/// Matching uses a view where each shell command's path prefix is `{shell}`.
/// Deletions are applied to the original text so failure output stays unchanged.
fn remove_matches(have: &mut String, want: &str, shell: &str) -> bool {
    if want.is_empty() {
        return true;
    }
    let mut found = false;
    loop {
        let normalized = normalize_shell_lines(have, shell);
        let Some(start) = normalized.text.find(want) else {
            return found;
        };
        let end = start + want.len();
        let Some(&(orig_start, _)) = normalized.origins.get(start) else {
            return found;
        };
        let Some(&(_, orig_end)) = normalized.origins.get(end - 1) else {
            return found;
        };
        if orig_start >= orig_end || orig_end > have.len() {
            return found;
        }
        let previous_len = have.len();
        have.replace_range(orig_start..orig_end, "");
        if have.len() == previous_len {
            return found;
        }
        found = true;
    }
}

/// `have` with shell path prefixes replaced by `{shell}`.
///
/// `origins[i]` is the original byte range that produced normalized byte `i`.
struct NormalizedText {
    text: String,
    origins: Vec<(usize, usize)>,
}

fn normalize_shell_lines(have: &str, shell: &str) -> NormalizedText {
    let mut text = String::with_capacity(have.len());
    let mut origins = Vec::with_capacity(have.len());
    if shell.is_empty() {
        push_verbatim(&mut text, &mut origins, have, 0);
        return NormalizedText { text, origins };
    }
    let mut offset = 0;
    for segment in have.split_inclusive('\n') {
        let segment_start = offset;
        offset += segment.len();
        let (body, newline) = split_line_ending(segment);
        if let Some(index) = body.find(shell) {
            let replaced_end = segment_start + index + shell.len();
            push_placeholder(&mut text, &mut origins, segment_start, replaced_end);
            push_verbatim(
                &mut text,
                &mut origins,
                &body[index + shell.len()..],
                replaced_end,
            );
        } else {
            push_verbatim(&mut text, &mut origins, body, segment_start);
        }
        push_verbatim(&mut text, &mut origins, newline, segment_start + body.len());
    }
    debug_assert_eq!(text.len(), origins.len());
    NormalizedText { text, origins }
}

fn push_placeholder(
    text: &mut String,
    origins: &mut Vec<(usize, usize)>,
    start: usize,
    end: usize,
) {
    text.push_str(SHELL_PLACEHOLDER);
    for _ in 0..SHELL_PLACEHOLDER.len() {
        origins.push((start, end));
    }
}

fn push_verbatim(text: &mut String, origins: &mut Vec<(usize, usize)>, piece: &str, base: usize) {
    origins.extend((0..piece.len()).map(|i| (base + i, base + i + 1)));
    text.push_str(piece);
}

fn split_line_ending(segment: &str) -> (&str, &str) {
    match segment.strip_suffix('\n') {
        Some(body) => match body.strip_suffix('\r') {
            Some(body) => (body, "\r\n"),
            None => (body, "\n"),
        },
        None => (segment, ""),
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
    #[should_panic(expected = "Extra stdout output found:\nworld")]
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
}
