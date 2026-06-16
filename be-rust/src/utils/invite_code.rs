use uuid::Uuid;

pub fn generate_invite_code() -> String {
    Uuid::new_v4().simple().to_string()[..8].to_uppercase()
}

pub fn build_invite_link(origin: &str, invite_code: &str) -> String {
    format!(
        "{}/invite/{}",
        origin.trim_end_matches('/'),
        invite_code
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_invite_code_is_8_chars_uppercase() {
        let code = generate_invite_code();
        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c| c.is_ascii_hexdigit() && !c.is_lowercase()));
    }

    #[test]
    fn generate_invite_code_is_unique() {
        let codes: Vec<_> = (0..100).map(|_| generate_invite_code()).collect();
        let unique: std::collections::HashSet<_> = codes.iter().collect();
        assert_eq!(unique.len(), codes.len());
    }

    #[test]
    fn build_invite_link_trims_trailing_slash() {
        let link = build_invite_link("https://runetalk.app/", "ABCD1234");
        assert_eq!(link, "https://runetalk.app/invite/ABCD1234");
    }

    #[test]
    fn build_invite_link_without_trailing_slash() {
        let link = build_invite_link("https://runetalk.app", "ABCD1234");
        assert_eq!(link, "https://runetalk.app/invite/ABCD1234");
    }
}
