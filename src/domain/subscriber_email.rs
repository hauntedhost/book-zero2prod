use regex::bytes::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if is_valid_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{:?} is not a valid subscriber email.", s))
        }
    }
}

fn is_valid_email(s: &str) -> bool {
    is_not_blank(&s) && has_length_lte(&s, 256) && is_all_valid_characters(&s) && contains_exactly_one_at_sign(&s)
}

fn is_not_blank(s: &str) -> bool {
    !s.trim().is_empty()
}

fn has_length_lte(s: &str, n: usize) -> bool {
    s.graphemes(true).count() <= n
}

fn is_all_valid_characters(s: &str) -> bool {
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    !s.chars().any(|c| forbidden_characters.contains(&c))
}

fn contains_exactly_one_at_sign(s: &str) -> bool {
    let re = Regex::new(r".+@.+").unwrap();
    re.is_match(s.as_bytes())
}
