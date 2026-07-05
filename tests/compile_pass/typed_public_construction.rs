use surgeist_style::{
    AnimationNameList, AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue,
    AuthoredTokens, Color, CssPx, CssWideKeyword, CustomPropertyName, Declarations,
    DimensionLength, DurationSeconds, FontFamilyList, GridTrackList, LayerOrder, Opacity, Property,
    RulePrecedence, SourceOrder, TypedDeclaration, Value,
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
