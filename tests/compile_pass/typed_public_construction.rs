use surgeist_style::{
    AnimationNameList, Color, CssPx, Declarations, DimensionLength, FontFamilyList, Opacity,
    Property, TypedDeclaration, Value,
};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?)?);
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::text_color(Color::try_rgba(0.0, 0.0, 0.0, 1.0)?);

    let declarations = Declarations::from_typed([width, opacity, color])?;
    assert_eq!(declarations.len(), 3);

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
    Ok(())
}
