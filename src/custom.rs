use crate::{Error, ErrorCode, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CustomPropertyName {
    value: String,
}

impl CustomPropertyName {
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        let suffix = value.strip_prefix("--").ok_or_else(invalid_name)?;
        if suffix.is_empty() || !suffix.chars().all(is_custom_property_suffix_char) {
            return Err(invalid_name());
        }

        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AuthoredTokens {
    value: String,
}

impl AuthoredTokens {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        &self.value
    }
}

fn is_custom_property_suffix_char(value: char) -> bool {
    value.is_alphanumeric() || value == '-' || value == '_'
}

fn invalid_name() -> Error {
    Error::new(
        ErrorCode::InvalidString,
        "custom property names must start with -- and have a non-empty alphanumeric suffix using only alphanumeric characters, - or _",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorCode;

    #[test]
    fn custom_property_name_preserves_case_and_accepts_css_custom_shape() {
        let name = CustomPropertyName::try_new("--BrandColor").unwrap();
        assert_eq!(name.as_str(), "--BrandColor");
        assert_eq!(
            CustomPropertyName::try_new("--brand_color-1")
                .unwrap()
                .as_str(),
            "--brand_color-1",
        );
    }

    #[test]
    fn custom_property_name_accepts_non_ascii_alphanumeric_suffix_chars() {
        let name = CustomPropertyName::try_new("--bränd").unwrap();

        assert_eq!(name.as_str(), "--bränd");
    }

    #[test]
    fn custom_property_name_rejects_non_custom_names() {
        for invalid in ["color", "-gap", "--", "-- bad", "--;", "--gap;", "--gap\n"] {
            let error = CustomPropertyName::try_new(invalid).unwrap_err();
            assert_eq!(error.code(), ErrorCode::InvalidString);
        }
    }

    #[test]
    fn custom_property_authored_tokens_preserve_empty_and_non_empty_css() {
        assert_eq!(AuthoredTokens::new("").as_css(), "");
        assert_eq!(
            AuthoredTokens::new("calc(var(--space) * 2)").as_css(),
            "calc(var(--space) * 2)"
        );
    }
}
