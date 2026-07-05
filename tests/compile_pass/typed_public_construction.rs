use surgeist_style::{
    AnimationNameList, AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue,
    AuthoredTokens, Color, CssPx, CssWideKeyword, CustomPropertyName, Declarations,
    DimensionLength, DurationSeconds, FontFamilyList, GridTrackList, LayerOrder, Length, Opacity,
    Property, RulePrecedence, SourceOrder, TypedDeclaration, Value, VariableDependentValue,
    VariableExpression, VariableFallback, VariableReference,
};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?)?);
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::try_text_color(Color::try_rgba(0.0, 0.0, 0.0, 1.0)?)?;

    let declarations = Declarations::from_typed([width, opacity, color])?;
    assert_eq!(declarations.len(), 3);

    let declarations = Declarations::new()
        .width(DimensionLength::px(CssPx::new(240.0)?)?)
        .opacity(Opacity::new(0.5)?)
        .transition_duration(DurationSeconds::new(0.2)?)
        .try_grid_template_rows(GridTrackList::new(Vec::new()))?;
    assert_eq!(declarations.len(), 4);

    let declarations = Declarations::new()
        .try_set(
            Property::FontFamily,
            Value::FontFamilyList(FontFamilyList::new(["Inter", "system-ui"])?),
        )?
        .try_set(
            Property::AnimationName,
            Value::AnimationNameList(AnimationNameList::new(["fade-in"])?),
        )?;
    assert_eq!(declarations.len(), 2);

    let precedence = RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(8));
    assert_eq!(precedence.layer_order(), LayerOrder::new(2));

    let custom_name = CustomPropertyName::try_new("--brand")?;
    let authored_tokens = AuthoredTokens::new("var(--brand, #000)");
    assert_eq!(custom_name.as_str(), "--brand");
    assert_eq!(authored_tokens.as_css(), "var(--brand, #000)");

    let space_name = CustomPropertyName::try_new("--space")?;
    let variable_gap = VariableDependentValue::try_new(
        Property::Gap,
        AuthoredTokens::new("var(--space, 8px)"),
        VariableExpression::Reference(VariableReference::new(
            space_name,
            Some(VariableFallback::new(
                AuthoredTokens::new("8px"),
                VariableExpression::Value(Value::Length(Length::px(8.0))),
            )),
        )),
    )?;
    assert_eq!(variable_gap.property(), Property::Gap);
    assert_eq!(variable_gap.dependencies().len(), 1);

    let nested_fallback_width = VariableDependentValue::try_new(
        Property::Width,
        AuthoredTokens::new("var(--space, var(--fallback, 4px))"),
        VariableExpression::Reference(VariableReference::new(
            CustomPropertyName::try_new("--space")?,
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--fallback, 4px)"),
                VariableExpression::Reference(VariableReference::new(
                    CustomPropertyName::try_new("--fallback")?,
                    Some(VariableFallback::new(
                        AuthoredTokens::new("4px"),
                        VariableExpression::Value(Value::Length(Length::px(4.0))),
                    )),
                )),
            )),
        )),
    )?;
    assert_eq!(nested_fallback_width.dependencies().len(), 2);

    let mut authored = AuthoredDeclarations::new();
    authored.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::All,
        CssWideKeyword::Initial,
    ));
    authored.try_push(AuthoredDeclaration::try_new(
        AuthoredProperty::Property(Property::Color),
        AuthoredValue::Value(Value::Color(Color::BLACK)),
    )?)?;
    assert!(authored.len() >= 2);
    Ok(())
}
