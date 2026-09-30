/// replaces platform-specific placeholders in expected output with their concrete values
pub fn substitute_placeholders(text: &str, shell: &str) -> String {
    text.replace("{shell}", shell)
}
