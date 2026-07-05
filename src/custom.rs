use std::collections::BTreeSet;

use crate::{CssWideKeyword, Error, ErrorCode, Property, Result, Value};

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

#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
    Reference(VariableReference),
}

impl VariableExpression {
    #[must_use]
    pub fn dependencies(&self) -> Vec<CustomPropertyName> {
        let mut dependencies = BTreeSet::new();
        self.collect_dependencies(&mut dependencies);
        dependencies.into_iter().collect()
    }

    #[must_use]
    pub fn contains_reference(&self) -> bool {
        match self {
            Self::Value(_) | Self::CssWideKeyword(_) => false,
            Self::Reference(_) => true,
        }
    }

    pub fn validate_for_property(&self, property: Property) -> Result<()> {
        match self {
            Self::Value(value) => property.validate_value(value),
            Self::CssWideKeyword(_) => Ok(()),
            Self::Reference(reference) => {
                if let Some(fallback) = reference.fallback() {
                    fallback.expression().validate_for_property(property)?;
                }
                Ok(())
            }
        }
    }

    fn collect_dependencies(&self, dependencies: &mut BTreeSet<CustomPropertyName>) {
        match self {
            Self::Value(_) | Self::CssWideKeyword(_) => {}
            Self::Reference(reference) => {
                dependencies.insert(reference.name().clone());
                if let Some(fallback) = reference.fallback() {
                    fallback.expression().collect_dependencies(dependencies);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableReference {
    name: CustomPropertyName,
    fallback: Option<VariableFallback>,
}

impl VariableReference {
    #[must_use]
    pub const fn new(name: CustomPropertyName, fallback: Option<VariableFallback>) -> Self {
        Self { name, fallback }
    }

    #[must_use]
    pub const fn name(&self) -> &CustomPropertyName {
        &self.name
    }

    #[must_use]
    pub const fn fallback(&self) -> Option<&VariableFallback> {
        self.fallback.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableFallback {
    authored: AuthoredTokens,
    expression: Box<VariableExpression>,
}

impl VariableFallback {
    #[must_use]
    pub fn new(authored: AuthoredTokens, expression: VariableExpression) -> Self {
        Self {
            authored,
            expression: Box::new(expression),
        }
    }

    #[must_use]
    pub const fn authored(&self) -> &AuthoredTokens {
        &self.authored
    }

    #[must_use]
    pub const fn expression(&self) -> &VariableExpression {
        &self.expression
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDependentValue {
    property: Property,
    authored: AuthoredTokens,
    expression: VariableExpression,
    dependencies: Vec<CustomPropertyName>,
}

impl VariableDependentValue {
    pub fn try_new(
        property: Property,
        authored: AuthoredTokens,
        expression: VariableExpression,
    ) -> Result<Self> {
        if !expression.contains_reference() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "variable-dependent values must contain at least one variable reference",
            ));
        }
        expression.validate_for_property(property)?;
        let dependencies = expression.dependencies();

        Ok(Self {
            property,
            authored,
            expression,
            dependencies,
        })
    }

    #[must_use]
    pub const fn property(&self) -> Property {
        self.property
    }

    #[must_use]
    pub const fn authored(&self) -> &AuthoredTokens {
        &self.authored
    }

    #[must_use]
    pub const fn expression(&self) -> &VariableExpression {
        &self.expression
    }

    #[must_use]
    pub fn dependencies(&self) -> &[CustomPropertyName] {
        &self.dependencies
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
    use crate::{Color, ErrorCode, Length, Property, Value};

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

    #[test]
    fn variable_reference_exposes_custom_property_name() {
        let name = CustomPropertyName::try_new("--space").unwrap();
        let reference = VariableReference::new(name.clone(), None);

        assert_eq!(reference.name(), &name);
        assert!(reference.fallback().is_none());
    }

    #[test]
    fn variable_fallback_preserves_authored_css() {
        let fallback = VariableFallback::new(
            AuthoredTokens::new("8px"),
            VariableExpression::Value(Value::Length(Length::px(8.0))),
        );

        assert_eq!(fallback.authored().as_css(), "8px");
        assert_eq!(
            fallback.expression(),
            &VariableExpression::Value(Value::Length(Length::px(8.0)))
        );
    }

    #[test]
    fn variable_nested_fallback_references_are_included_in_dependencies() {
        let expression = VariableExpression::Reference(VariableReference::new(
            CustomPropertyName::try_new("--space").unwrap(),
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--fallback, 4px)"),
                VariableExpression::Reference(VariableReference::new(
                    CustomPropertyName::try_new("--fallback").unwrap(),
                    Some(VariableFallback::new(
                        AuthoredTokens::new("4px"),
                        VariableExpression::Value(Value::Length(Length::px(4.0))),
                    )),
                )),
            )),
        ));

        let dependencies = expression.dependencies();

        assert_eq!(dependencies.len(), 2);
        assert!(
            dependencies
                .iter()
                .any(|dependency| dependency.as_str() == "--space")
        );
        assert!(
            dependencies
                .iter()
                .any(|dependency| dependency.as_str() == "--fallback")
        );
    }

    #[test]
    fn variable_dependent_value_requires_at_least_one_reference() {
        let error = VariableDependentValue::try_new(
            Property::Width,
            AuthoredTokens::new("8px"),
            VariableExpression::Value(Value::Length(Length::px(8.0))),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn variable_dependent_value_validates_literal_branches_against_target_property() {
        let error = VariableDependentValue::try_new(
            Property::Width,
            AuthoredTokens::new("var(--space, black)"),
            VariableExpression::Reference(VariableReference::new(
                CustomPropertyName::try_new("--space").unwrap(),
                Some(VariableFallback::new(
                    AuthoredTokens::new("black"),
                    VariableExpression::Value(Value::Color(Color::BLACK)),
                )),
            )),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);

        let value = VariableDependentValue::try_new(
            Property::Width,
            AuthoredTokens::new("var(--space, 8px)"),
            VariableExpression::Reference(VariableReference::new(
                CustomPropertyName::try_new("--space").unwrap(),
                Some(VariableFallback::new(
                    AuthoredTokens::new("8px"),
                    VariableExpression::Value(Value::Length(Length::px(8.0))),
                )),
            )),
        )
        .unwrap();

        assert_eq!(value.property(), Property::Width);
        assert_eq!(value.authored().as_css(), "var(--space, 8px)");
        assert_eq!(value.dependencies().len(), 1);
        assert_eq!(value.dependencies()[0].as_str(), "--space");
    }
}
