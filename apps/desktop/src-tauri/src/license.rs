pub const FREE_CONTENT_LIMIT: usize = 50;

pub fn is_pro(license_key: Option<&str>) -> bool {
    license_key
        .map(|key| key.trim().to_uppercase().starts_with("PKPRO-"))
        .unwrap_or(false)
}

pub fn validate_activation_key(key: &str) -> bool {
    let key = key.trim();
    key.len() >= 12 && key.to_uppercase().starts_with("PKPRO-")
}

pub fn tier_label(license_key: Option<&str>) -> &'static str {
    if is_pro(license_key) {
        "pro"
    } else {
        "free"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_pro_prefix() {
        assert!(validate_activation_key("PKPRO-DEMO-2026"));
        assert!(is_pro(Some("pkpro-test")));
    }

    #[test]
    fn rejects_invalid_keys() {
        assert!(!validate_activation_key("FREE-123"));
        assert!(!is_pro(None));
    }
}
