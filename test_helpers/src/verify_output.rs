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
    fn test_verify_output() {
        let have = "hello world".to_string();
        let wants = vec!["hello".to_string(), "world".to_string()];
        verify_output("stdout", have, &wants);
    }
}
