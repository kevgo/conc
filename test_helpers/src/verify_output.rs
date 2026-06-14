/// verifies STDOUT or STDERR output collected in Cucumber tests
/// against the collected expected output
pub fn verify_output(name: &str, mut have: String, wants: &[String]) {
    for want in wants {
        assert!(
            have.contains(want),
            "Didn't find '{want}' in {name}\nremaining text: '{have}'"
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
        let have = "hello world".to_string();
        let wants = vec!["hello".to_string(), "world".to_string()];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Extra stdout output found:\nworld")]
    fn expect_too_little() {
        let have = "hello world".to_string();
        let wants = vec!["hello".to_string()];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Didn't find 'extra' in stdout\nremaining text: ' '")]
    fn expect_too_much() {
        let have = S("hello world");
        let wants = vec![
            "hello".to_string(),
            "world".to_string(),
            "extra".to_string(),
        ];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Didn't find 'hallo' in stdout\nremaining text: 'hello'")]
    fn different() {
        let have = "hello".to_string();
        let wants = vec!["hallo".to_string()];
        verify_output("stdout", have, &wants);
    }
}
