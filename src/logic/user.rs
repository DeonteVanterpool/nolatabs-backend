pub fn verify_email(env: crate::state::Environment, email: &str, email_verified: bool) -> bool {
    return email_verified
        || (email.ends_with("@test.account") && env != crate::state::Environment::Production);
}

#[cfg(test)]
mod tests {
    use super::verify_email;
    #[test]
    fn verify_email_tests() {
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            false
        ));
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Testing,
            "fake@test.account",
            true
        ));
        assert!(!verify_email(
            crate::state::Environment::Testing,
            "fake@outlook.com",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Staging,
            "fake@test.account",
            true
        ));
    }
}
