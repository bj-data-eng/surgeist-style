use crate::{
    Declaration, Error, ErrorCode, Property, Result, Value,
    declaration::{canonical_declarations, canonical_properties},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AuthoredProperty {
    Property(Property),
    All,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssWideKeyword {
    Inherit,
    Initial,
    Unset,
    RevertLayer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthoredValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthoredDeclaration {
    property: AuthoredProperty,
    value: AuthoredValue,
}

impl AuthoredDeclaration {
    pub fn try_new(property: AuthoredProperty, value: AuthoredValue) -> Result<Self> {
        let AuthoredProperty::Property(style_property) = property else {
            return Err(invalid_property(
                "all accepts only explicit authored CSS-wide keywords",
            ));
        };
        let AuthoredValue::Value(style_value) = value else {
            return Err(invalid_property(
                "CSS-wide keywords must use AuthoredDeclaration::css_wide",
            ));
        };
        if matches!(style_value, Value::Keyword(_)) {
            return Err(invalid_property(
                "legacy keyword values are not valid authored declaration values",
            ));
        }
        style_property.validate_value(&style_value)?;

        Ok(Self {
            property,
            value: AuthoredValue::Value(style_value),
        })
    }

    #[must_use]
    pub const fn css_wide(property: AuthoredProperty, keyword: CssWideKeyword) -> Self {
        Self {
            property,
            value: AuthoredValue::CssWideKeyword(keyword),
        }
    }

    #[must_use]
    pub const fn property(&self) -> AuthoredProperty {
        self.property
    }

    #[must_use]
    pub fn value(&self) -> AuthoredValue {
        self.value.clone()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuthoredDeclarations {
    values: Vec<AuthoredDeclaration>,
}

impl AuthoredDeclarations {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, declaration: AuthoredDeclaration) -> &mut Self {
        self.values.push(declaration);
        self
    }

    pub fn try_push(&mut self, declaration: AuthoredDeclaration) -> Result<&mut Self> {
        Ok(self.push(declaration))
    }

    pub fn iter(&self) -> impl Iterator<Item = &AuthoredDeclaration> {
        self.values.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn to_rule_declarations(&self) -> Result<AuthoredCanonicalDeclarations> {
        let mut canonical = AuthoredCanonicalDeclarations::new();
        for declaration in &self.values {
            match (&declaration.property, &declaration.value) {
                (AuthoredProperty::Property(property), AuthoredValue::Value(value)) => {
                    for Declaration { property, value } in
                        canonical_declarations(*property, value.clone())
                    {
                        canonical.insert(property, AuthoredCascadeValue::Value(value));
                    }
                }
                (AuthoredProperty::Property(property), AuthoredValue::CssWideKeyword(keyword)) => {
                    for property in canonical_properties(*property) {
                        canonical.insert(property, AuthoredCascadeValue::CssWideKeyword(*keyword));
                    }
                }
                (AuthoredProperty::All, AuthoredValue::CssWideKeyword(keyword)) => {
                    for property in all_css_wide_properties() {
                        canonical.insert(property, AuthoredCascadeValue::CssWideKeyword(*keyword));
                    }
                }
                (AuthoredProperty::All, AuthoredValue::Value(_)) => {
                    return Err(invalid_property(
                        "all accepts only explicit authored CSS-wide keywords",
                    ));
                }
            }
        }
        Ok(canonical)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AuthoredCascadeValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct AuthoredCanonicalDeclarations {
    values: Vec<(Property, AuthoredCascadeValue)>,
}

#[allow(dead_code)]
impl AuthoredCanonicalDeclarations {
    #[must_use]
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, property: Property, value: AuthoredCascadeValue) {
        if let Some((_, existing)) = self
            .values
            .iter_mut()
            .find(|(existing_property, _)| *existing_property == property)
        {
            *existing = value;
        } else {
            self.values.push((property, value));
        }
    }

    #[must_use]
    pub(crate) fn get(&self, property: Property) -> Option<&AuthoredCascadeValue> {
        self.values
            .iter()
            .find(|(existing_property, _)| *existing_property == property)
            .map(|(_, value)| value)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Property, &AuthoredCascadeValue)> {
        self.values
            .iter()
            .map(|(property, value)| (*property, value))
    }

    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[allow(dead_code)]
fn all_css_wide_properties() -> impl Iterator<Item = Property> {
    Property::ALL
        .iter()
        .copied()
        .filter(|property| property.is_canonical() && *property != Property::Direction)
}

fn invalid_property(message: impl Into<String>) -> Error {
    Error::new(ErrorCode::InvalidProperty, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Display, ErrorCode, Keyword, Length, Property, Value};

    #[test]
    fn ordinary_authored_declaration_validates_against_property() {
        let declaration = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Display),
            AuthoredValue::Value(Value::Display(Display::Block)),
        )
        .unwrap();

        assert_eq!(
            declaration.property(),
            AuthoredProperty::Property(Property::Display)
        );
    }

    #[test]
    fn ordinary_authored_declaration_rejects_property_value_mismatch() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Width),
            AuthoredValue::Value(Value::Color(Color::BLACK)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn all_rejects_ordinary_values() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::All,
            AuthoredValue::Value(Value::Color(Color::BLACK)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn ordinary_value_path_rejects_existing_keyword_values() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Color),
            AuthoredValue::Value(Value::Keyword(Keyword::Initial)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn css_wide_keywords_must_use_explicit_constructor() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Color),
            AuthoredValue::CssWideKeyword(CssWideKeyword::Initial),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn all_accepts_css_wide_keywords() {
        let declaration =
            AuthoredDeclaration::css_wide(AuthoredProperty::All, CssWideKeyword::Initial);

        assert_eq!(declaration.property(), AuthoredProperty::All);
        assert_eq!(
            declaration.value(),
            AuthoredValue::CssWideKeyword(CssWideKeyword::Initial)
        );
    }

    #[test]
    fn all_expands_css_wide_keyword_to_canonical_properties_except_direction() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::All,
            CssWideKeyword::Unset,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.len(),
            Property::ALL
                .iter()
                .filter(|property| property.is_canonical() && **property != Property::Direction)
                .count()
        );
        assert_eq!(
            canonical.get(Property::Color),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
        );
        assert_eq!(canonical.get(Property::Direction), None);
    }

    #[test]
    fn shorthand_css_wide_keyword_expands_to_longhands() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(Property::Overflow),
            CssWideKeyword::Inherit,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.get(Property::OverflowX),
            Some(&AuthoredCascadeValue::CssWideKeyword(
                CssWideKeyword::Inherit
            ))
        );
        assert_eq!(
            canonical.get(Property::OverflowY),
            Some(&AuthoredCascadeValue::CssWideKeyword(
                CssWideKeyword::Inherit
            ))
        );
    }

    #[test]
    fn revert_layer_expands_without_entering_legacy_value_model() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(Property::Color),
            CssWideKeyword::RevertLayer,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.get(Property::Color),
            Some(&AuthoredCascadeValue::CssWideKeyword(
                CssWideKeyword::RevertLayer
            ))
        );
    }

    #[test]
    fn ordinary_values_still_expand_existing_shorthands() {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Width),
                    AuthoredValue::Value(Value::Length(Length::Px(12.0))),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            declarations
                .to_rule_declarations()
                .unwrap()
                .get(Property::Width),
            Some(&AuthoredCascadeValue::Value(Value::Length(Length::Px(
                12.0
            ))))
        );
    }
}
