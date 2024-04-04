use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(
        name: String
    ) -> Result<SubscriberName, String> {
        if !validate_name(&name) {
            return Err(format!("{} is not a valid subscriber name.", name));
        }

        Ok(Self(name))
    }
}

fn validate_name(
    name: &String
) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();

    let is_too_long = name.graphemes(true).count() > 256;

    let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_char = name.chars().any(|g| forbidden_chars.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_char)
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "ë".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_name_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_is_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_with_invalid_char_is_rejected() {
        for char in ['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = char.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_valid() {
        let name = "Maurice Ponte".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
