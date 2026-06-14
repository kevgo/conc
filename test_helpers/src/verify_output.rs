pub fn verify_output(name: &str, mut have: String, wants: &[String]) {
    for want in wants {
        assert!(
            have.contains(want),
            "Didn't find '{want}' in {name}:\n{have}"
        );
        have = have.replace(want, "");
    }
    have = have.trim().to_owned();
    assert!(have.is_empty(), "Extra {name} output found:\n{have}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let have = "hello world".to_string();
        let wants = vec!["hello".to_string(), "world".to_string()];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Extra stdout output found:\nworld")]
    fn expect_too_much() {
        let have = "hello world".to_string();
        let wants = vec!["hello".to_string()];
        verify_output("stdout", have, &wants);
    }

    #[test]
    #[should_panic(expected = "Didn't find 'extra' in stdout:\n")]
    fn expect_too_little() {
        let have = "hello world".to_string();
        let wants = vec![
            "hello".to_string(),
            "world".to_string(),
            "extra".to_string(),
        ];
        verify_output("stdout", have, &wants);
    }
}
