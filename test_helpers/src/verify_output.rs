/// verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output
pub fn verify_output(name: &str, mut have: String, wants: &[String]) {
    for want in wants {
        assert!(
            have.contains(want),
            "Didn't find '{want}' in {name}\nremaining unchecked text in {name}:\n'{have}'"
        );
        have = have.replace(want, "");
    }
    have = have.trim().to_owned();
    assert!(have.is_empty(), "Extra {name} output found:\n{have}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use big_s::S;

    #[test]
    fn exact_match() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world")];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Extra stdout output found:\nworld")]
    fn expect_too_little() {
        let have = S("hello world");
        let wants = vec![S("hello")];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'extra' in stdout\nremaining unchecked text in stdout:\n' '"
    )]
    fn expect_too_much() {
        let have = S("hello world");
        let wants = vec![S("hello"), S("world"), S("extra")];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(
        expected = "Didn't find 'hallo' in stdout\nremaining unchecked text in stdout:\n'hello'"
    )]
    fn different() {
        let have = S("hello");
        let wants = vec![S("hallo")];
        verify_output("stdout", have, &wants);
    }
}
