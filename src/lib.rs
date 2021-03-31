use md5::compute;

pub fn get_avatar_url(email: &str) -> Result<String, AvatarFetchError> {
    let email_cleaned = email.trim().to_ascii_lowercase();
    let hash = compute(email_cleaned.as_bytes());
    let hash_str = format!("{:x}", hash);

    let base_url = "https://cdn.libravatar.org/avatar/"; // TODO: federation

    Ok(format!("{}{}", base_url, hash_str))
}

#[derive(Debug)]
pub enum AvatarFetchError {}

#[cfg(test)]
mod tests {
    #[test]
    fn normal_email() {
        let result = super::get_avatar_url("kot@yukata.tech");
        assert!(!result.is_err());

        let url = result.unwrap();
        assert_eq!(
            url,
            "https://cdn.libravatar.org/avatar/0b35f3ba043d931801146c6bdb7f6368"
        );
    }
}
