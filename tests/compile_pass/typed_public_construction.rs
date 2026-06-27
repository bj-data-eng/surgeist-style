use surgeist_style::{Color, CssPx, Declarations, DimensionLength, Opacity, TypedDeclaration};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?)?);
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::text_color(Color::try_rgba(0.0, 0.0, 0.0, 1.0)?);

    let declarations = Declarations::from_typed([width, opacity, color])?;
    assert_eq!(declarations.len(), 3);
    Ok(())
}
