# Paint, Color, And Effect Families Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Operation 10 from the CSS surface sequence by adding style-owned, type-safe paint/color/effect receiving models, canonical shorthand lowering, resolved getters, tests, and a rebased coverage ledger.

**Architecture:** `surgeist-style` owns normalized style-domain values only; root will later lower `surgeist-css` syntax into the public APIs introduced here. Symbolic paint values stay symbolic until render, resource loading, host capability, or layout context exists. Shorthands canonicalize into longhands in `Declarations`/authored CSS-wide expansion, and no style-to-render, style-to-css, or style-to-layout adapter layer is added.

**Tech Stack:** Rust 2024, `surgeist-style`, crate-local unit tests, `trybuild` compile-fail/compile-pass tests, read-only inspection of `/Users/codex/Development/surgeist-css/src/syntax.rs`, `cargo fmt`, `cargo test -p surgeist-style`, `cargo clippy -p surgeist-style --all-targets -- -D warnings`.

---

## Source Context

- Operation sequence: `plans/2026-07-05-css-surface-style-operations-sequence.md`
- CSS property ledger: `plans/2026-07-05-css-property-coverage-ledger.md`
- Rust modeling guide: `guidance/surgeist-rust-modeling-guide.md`
- Read-only CSS source snapshot: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Current style model files:
  - `src/value.rs`
  - `src/property.rs`
  - `src/declaration.rs`
  - `src/resolver.rs`
  - `src/authored.rs`
  - `src/lib.rs`
  - `tests/type_safety.rs`
  - `tests/compile_pass/typed_public_construction.rs`
  - `tests/compile_fail/*.rs`

## Boundaries

- Do not add `surgeist-css`, `surgeist-text`, render, image-loading, or platform host dependencies to `Cargo.toml`.
- Do not parse CSS in this crate. Root owns CSS-to-style lowering.
- Do not add compatibility aliases for legacy broad keyword placeholder paths.
- Breaking public API changes are acceptable when they improve type safety.
- Keep symbolic colors, images, URLs, filter functions, basic shapes, masks, and transform functions symbolic. Do not evaluate color spaces, resolve `currentColor`, load resources, rasterize effects, or check backend capabilities in this crate.
- Keep generated content/counters/lists in Operation 11.
- Keep timing/keyframes in Operation 12.
- Keep resolver cache/invalidation generalization in Operation 14 unless a local property impact bit is needed by the property itself.
- Worker commits are not allowed. The coordinator commits each clean task after worker/reviewer reconciliation.

## Operation 10 Coverage

The rebased ledger currently points these families at Operation 10:

| Family | Properties |
| --- | --- |
| Color | `TextDecorationColor`, `Color`, symbolic/current colors |
| Background | `Background`, `BackgroundColor`, `BackgroundImage`, `BackgroundPosition`, `BackgroundSize`, `BackgroundRepeat`, `BackgroundOrigin`, `BackgroundClip`, `BackgroundAttachment` |
| Border and outline | `Border`, `BorderColor`, side border shorthands, side colors, side styles, `BorderStyle`, individual corner radii, `Outline`, `OutlineColor`, `OutlineStyle`, `OutlineWidth` |
| Paint and effects | `BoxDecorationBreak`, `BoxShadow`, `Opacity`, `Filter`, `BackdropFilter`, `ClipPath`, `Mask`, `MaskImage`, `MaskSize`, `MaskPosition`, `MaskRepeat` |
| Transforms | `Transform`, `TransformOrigin`, `Translate`, `Rotate`, `Scale` |
| Interaction | `Cursor`, `PointerEvents`, `UserSelect` |

Existing typed rows should be reviewed rather than blindly replaced:

- `Color`, `Background`, `BoxShadow`, `Opacity`, `Cursor`, `PointerEvents`, `Transform`, and `TransformOrigin` already have style-side primitives. `BorderColor` has pre-Operation concrete aggregate state, but Operation 10 replaces it with non-canonical four-side shorthand lowering.
- Operation 10 should keep sound existing primitives and introduce typed models only where the ledger marks `New style property needed`, `New shorthand lowering needed`, or `Symbolic style data needed`.

## Planned Type Names

Workers may adapt exact placement to existing modules, but these style-owned public names should be used unless a reviewer finds a modeling issue:

```rust
// Symbolic color.
pub enum StyleColor {
    CurrentColor,
    Rgba(Color),
    Hsl(HslColor),
    Hwb(HwbColor),
    Lab(LabColor),
    Lch(LchColor),
    Oklab(LabColor),
    Oklch(LchColor),
    ColorFunction(ColorFunction),
    System(SystemColor),
    ColorMix(ColorMix),
    Relative(RelativeColor),
}

// Background/mask resources and layers.
pub struct StyleUrl(String);
pub enum ImageLayer { None, Url(StyleUrl) }
pub struct ImageLayerList { layers: Vec<ImageLayer> }
pub enum PositionComponent { Horizontal(HorizontalPositionKeyword), Vertical(VerticalPositionKeyword), Length(Length) }
pub struct Position { components: Vec<PositionComponent> }
pub struct PositionList { positions: Vec<Position> }
pub enum BackgroundSizeComponent { Auto, Length(Length) }
pub enum BackgroundSize { Cover, Contain, Explicit { width: BackgroundSizeComponent, height: Option<BackgroundSizeComponent> } }
pub struct BackgroundSizeList { sizes: Vec<BackgroundSize> }
pub enum BackgroundRepeat { RepeatX, RepeatY, Axes { x: BackgroundRepeatStyle, y: BackgroundRepeatStyle } }
pub struct BackgroundRepeatList { repeats: Vec<BackgroundRepeat> }
pub enum BackgroundBox { BorderBox, PaddingBox, ContentBox }
pub enum BackgroundAttachment { Scroll, Fixed, Local }
pub struct BackgroundAttachmentList { attachments: Vec<BackgroundAttachment> }

// Borders/outlines/radii.
pub enum BorderLineStyle { None, Hidden, Dotted, Dashed, Solid, Double, Groove, Ridge, Inset, Outset }
pub struct BorderStyles { top: BorderLineStyle, right: BorderLineStyle, bottom: BorderLineStyle, left: BorderLineStyle }
pub struct BorderSideStyle { side: BorderSide, style: BorderLineStyle }
pub struct BorderSideColor { side: BorderSide, color: StyleColor }
pub struct CornerRadius { horizontal: Length, vertical: Length }
pub struct BorderRadii { top_left: CornerRadius, top_right: CornerRadius, bottom_right: CornerRadius, bottom_left: CornerRadius }
pub enum OutlineStyle { Auto, Border(BorderLineStyle) }
pub enum OutlineWidth { Thin, Medium, Thick, Length(OutlineWidthLength) }
pub struct Outline { width: Option<OutlineWidth>, style: Option<OutlineStyle>, color: Option<StyleColor> }

// Effects/transforms/interaction.
pub enum BoxDecorationBreak { Slice, Clone }
pub enum UserSelect { Auto, Text, None, All, Contain }
pub enum Translate { None, Values(TranslateValues) }
pub enum Rotate { None, Value(SymbolicFunctionValue) }
pub enum Scale { None, Values(ScaleValues) }
pub enum Filter { None, Functions(FilterFunctionList) }
pub enum ClipPath { None, Url(StyleUrl), BasicShape(BasicShape) }
pub struct MaskLayer { image: Option<ImageLayer>, position: Option<Position>, size: Option<BackgroundSize>, repeat: Option<BackgroundRepeat> }
pub struct MaskLayerList { layers: Vec<MaskLayer> }
```

Values with invariants must use private fields and constructor front doors. Do not expose public tuple constructors for `StyleUrl`, non-empty lists, bounded numeric wrappers, or length-domain wrappers.

---

### Task 1: Symbolic Colors And Color-Bearing Longhands

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Modify: `tests/type_safety.rs`
- Create: `tests/compile_fail/invalid_color_component_literal.rs`
- Create: `tests/compile_fail/invalid_color_component_literal.stderr`

- [ ] **Step 1: Add failing tests for symbolic color values**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn color_properties_accept_symbolic_style_colors() {
    let decoration = StyleColor::current_color();
    let rgba = StyleColor::rgba(Color::try_rgba(0.2, 0.4, 0.6, 1.0).unwrap());

    let declarations = Declarations::new()
        .try_text_color(rgba.clone())
        .unwrap()
        .try_background_color(StyleColor::current_color())
        .unwrap()
        .try_text_decoration_color(decoration.clone())
        .unwrap();

    assert_eq!(declarations.get(Property::Color), Some(&Value::StyleColor(rgba)));
    assert_eq!(
        declarations.get(Property::Background),
        Some(&Value::StyleColor(StyleColor::current_color()))
    );
    assert_eq!(
        declarations.get(Property::TextDecorationColor),
        Some(&Value::StyleColor(decoration))
    );
}

#[test]
fn text_decoration_shorthand_lowers_color_with_existing_components() {
    let line = TextDecorationLine::try_new([TextDecorationLineComponent::Underline]).unwrap();
    let color = StyleColor::system(SystemColor::CanvasText);
    let thickness = TextDecorationThickness::try_length(Length::Px(2.0)).unwrap();
    let decoration = TextDecoration::try_new(
        Some(line.clone()),
        Some(color.clone()),
        Some(TextDecorationStyle::Wavy),
        Some(thickness.clone()),
    )
    .unwrap();

    let declarations = Declarations::new().try_text_decoration(decoration).unwrap();

    assert_eq!(declarations.get(Property::TextDecoration), None);
    assert_eq!(
        declarations.get(Property::TextDecorationLine),
        Some(&Value::TextDecorationLine(line))
    );
    assert_eq!(
        declarations.get(Property::TextDecorationColor),
        Some(&Value::StyleColor(color))
    );
    assert_eq!(
        declarations.get(Property::TextDecorationStyle),
        Some(&Value::TextDecorationStyle(TextDecorationStyle::Wavy))
    );
    assert_eq!(
        declarations.get(Property::TextDecorationThickness),
        Some(&Value::TextDecorationThickness(thickness))
    );
}

#[test]
fn text_decoration_css_wide_expands_to_color_longhand() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::TextDecoration),
        CssWideKeyword::Initial,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.get(Property::TextDecoration), None);
    for property in [
        Property::TextDecorationLine,
        Property::TextDecorationColor,
        Property::TextDecorationStyle,
        Property::TextDecorationThickness,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Initial))
        );
    }
}

#[test]
fn symbolic_color_components_validate_domains() {
    assert!(Alpha::new(0.0).is_ok());
    assert!(Alpha::new(1.0).is_ok());
    assert!(Alpha::new(-0.1).is_err());
    assert!(Alpha::new(1.1).is_err());
    assert!(Alpha::new(f32::NAN).is_err());

    assert!(ColorComponent::new(Some(0.0)).is_ok());
    assert!(ColorComponent::new(Some(f32::NAN)).is_err());

    let left = ColorMixComponent::try_new(StyleColor::current_color(), Some(25.0)).unwrap();
    let right = ColorMixComponent::try_new(StyleColor::rgba(Color::BLACK), None).unwrap();
    let mix = StyleColor::color_mix(ColorMix::new(
        ColorInterpolationMethod::new(ColorInterpolationSpace::Oklab, None),
        left,
        right,
    ));
    assert!(matches!(mix, StyleColor::ColorMix(_)));
    assert!(ColorMixComponent::try_new(StyleColor::current_color(), Some(101.0)).is_err());
}

#[test]
fn relative_color_component_dependencies_include_nested_fallbacks() {
    let primary = CustomPropertyName::try_new("--a").unwrap();
    let fallback = CustomPropertyName::try_new("--b").unwrap();
    let expression = SymbolicComponentExpression::new(
        AuthoredTokens::new("calc(var(--a, var(--b)) + 1)"),
        [VariableReference::new(
            primary.clone(),
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--b)"),
                VariableExpression::Reference(VariableReference::new(fallback.clone(), None)),
            )),
        )],
    )
    .unwrap();

    assert_eq!(expression.dependencies(), vec![primary, fallback]);
}
```

Add to `src/resolver.rs` tests:

```rust
#[test]
fn resolved_color_getters_preserve_symbolic_colors() {
    let decoration = StyleColor::current_color();
    let background = StyleColor::system(SystemColor::Canvas);
    let style = resolve_single(
        Declarations::new()
            .try_text_color(StyleColor::rgba(Color::BLACK))
            .unwrap()
            .try_background_color(background.clone())
            .unwrap()
            .try_text_decoration_color(decoration.clone())
            .unwrap(),
    );

    assert_eq!(style.text_color(), &StyleColor::rgba(Color::BLACK));
    assert_eq!(style.background(), &background);
    assert_eq!(style.text_decoration_color(), &decoration);
}
```

- [ ] **Step 2: Add style-owned symbolic color types**

In `src/value.rs`, make concrete `Color` fields private and keep existing constructors:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn r(self) -> f32 { self.r }
    #[must_use]
    pub const fn g(self) -> f32 { self.g }
    #[must_use]
    pub const fn b(self) -> f32 { self.b }
    #[must_use]
    pub const fn a(self) -> f32 { self.a }
}
```

Add style-owned symbolic color types in `src/value.rs`. Import `std::collections::BTreeSet` for recursive relative-color dependency collection if it is not already in scope.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Alpha(f32);

impl Alpha {
    pub fn new(value: f32) -> Result<Self> {
        validate_finite(value, "alpha")?;
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::new(ErrorCode::InvalidValue, "alpha must be between 0 and 1"))
        }
    }

    #[must_use]
    pub const fn get(self) -> f32 { self.0 }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorComponent(Option<f32>);

impl ColorComponent {
    pub fn new(value: Option<f32>) -> Result<Self> {
        if value.is_some_and(|value| !value.is_finite()) {
            Err(Error::new(ErrorCode::InvalidValue, "color component must be finite"))
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub const fn get(self) -> Option<f32> { self.0 }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SystemColor {
    Canvas,
    CanvasText,
    LinkText,
    VisitedText,
    ActiveText,
    ButtonFace,
    ButtonText,
    ButtonBorder,
    Field,
    FieldText,
    Highlight,
    HighlightText,
    Mark,
    MarkText,
    GrayText,
    SelectedItem,
    SelectedItemText,
    AccentColor,
    AccentColorText,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PredefinedColorSpace {
    Srgb,
    SrgbLinear,
    DisplayP3,
    DisplayP3Linear,
    A98Rgb,
    ProphotoRgb,
    Rec2020,
    XyzD50,
    XyzD65,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorInterpolationSpace {
    Predefined(PredefinedColorSpace),
    Hsl,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HueInterpolationMethod {
    Shorter,
    Longer,
    Increasing,
    Decreasing,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ColorInterpolationMethod {
    space: ColorInterpolationSpace,
    hue: Option<HueInterpolationMethod>,
}

impl ColorInterpolationMethod {
    #[must_use]
    pub const fn new(space: ColorInterpolationSpace, hue: Option<HueInterpolationMethod>) -> Self {
        Self { space, hue }
    }

    #[must_use]
    pub const fn space(self) -> ColorInterpolationSpace { self.space }
    #[must_use]
    pub const fn hue(self) -> Option<HueInterpolationMethod> { self.hue }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleColor {
    CurrentColor,
    Rgba(Color),
    System(SystemColor),
    Hsl { hue: ColorComponent, saturation: ColorComponent, lightness: ColorComponent, alpha: Option<Alpha> },
    Hwb { hue: ColorComponent, whiteness: ColorComponent, blackness: ColorComponent, alpha: Option<Alpha> },
    Lab { lightness: ColorComponent, a: ColorComponent, b: ColorComponent, alpha: Option<Alpha> },
    Lch { lightness: ColorComponent, chroma: ColorComponent, hue: ColorComponent, alpha: Option<Alpha> },
    Oklab { lightness: ColorComponent, a: ColorComponent, b: ColorComponent, alpha: Option<Alpha> },
    Oklch { lightness: ColorComponent, chroma: ColorComponent, hue: ColorComponent, alpha: Option<Alpha> },
    ColorFunction(ColorFunction),
    ColorMix(Box<ColorMix>),
    Relative(Box<RelativeColor>),
}

impl StyleColor {
    #[must_use]
    pub const fn current_color() -> Self { Self::CurrentColor }
    #[must_use]
    pub const fn rgba(color: Color) -> Self { Self::Rgba(color) }
    #[must_use]
    pub const fn system(color: SystemColor) -> Self { Self::System(color) }
    #[must_use]
    pub fn color_mix(value: ColorMix) -> Self { Self::ColorMix(Box::new(value)) }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::CurrentColor | Self::System(_) => Ok(()),
            Self::Rgba(color) => color.validate(),
            Self::ColorFunction(value) => value.validate(),
            Self::ColorMix(value) => value.validate(),
            Self::Relative(value) => value.validate(),
            Self::Hsl { hue, saturation, lightness, alpha }
            | Self::Hwb { hue, whiteness: saturation, blackness: lightness, alpha }
            | Self::Lab { lightness: hue, a: saturation, b: lightness, alpha }
            | Self::Lch { lightness: hue, chroma: saturation, hue: lightness, alpha }
            | Self::Oklab { lightness: hue, a: saturation, b: lightness, alpha }
            | Self::Oklch { lightness: hue, chroma: saturation, hue: lightness, alpha } => {
                validate_color_components([*hue, *saturation, *lightness], *alpha)
            }
        }
    }
}
```

Define `ColorFunction`, `ColorMixComponent`, `ColorMix`, `RelativeColor`, and `ColorComponentExpression` with private fields and validation:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ColorFunction {
    space: PredefinedColorSpace,
    components: [ColorComponent; 3],
    alpha: Option<Alpha>,
}

impl ColorFunction {
    #[must_use]
    pub const fn new(
        space: PredefinedColorSpace,
        components: [ColorComponent; 3],
        alpha: Option<Alpha>,
    ) -> Self {
        Self { space, components, alpha }
    }

    pub fn validate(&self) -> Result<()> {
        validate_color_components(self.components, self.alpha)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorMixComponent {
    color: StyleColor,
    percentage: Option<f32>,
}

impl ColorMixComponent {
    pub fn try_new(color: StyleColor, percentage: Option<f32>) -> Result<Self> {
        if percentage.is_some_and(|value| !value.is_finite() || !(0.0..=100.0).contains(&value)) {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "color mix percentage must be between 0 and 100",
            ));
        }
        color.validate()?;
        Ok(Self { color, percentage })
    }

    #[must_use]
    pub const fn color(&self) -> &StyleColor { &self.color }
    #[must_use]
    pub const fn percentage(&self) -> Option<f32> { self.percentage }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorMix {
    interpolation: ColorInterpolationMethod,
    left: ColorMixComponent,
    right: ColorMixComponent,
}

impl ColorMix {
    #[must_use]
    pub const fn new(
        interpolation: ColorInterpolationMethod,
        left: ColorMixComponent,
        right: ColorMixComponent,
    ) -> Self {
        Self { interpolation, left, right }
    }

    pub fn validate(&self) -> Result<()> {
        self.left.color.validate()?;
        self.right.color.validate()
    }
}
```

For relative colors, preserve symbolic component expressions and their variable dependency facts without evaluating them. Do not store these expressions as plain strings only: root can pass authored component text plus style-owned `VariableReference` values at the boundary, and style must retain those references so variable-dependent colors remain symbolic and invalidation can stay honest.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolicComponentExpression {
    authored: AuthoredTokens,
    references: Vec<VariableReference>,
}

impl SymbolicComponentExpression {
    pub fn new(
        authored: AuthoredTokens,
        references: impl IntoIterator<Item = VariableReference>,
    ) -> Result<Self> {
        if authored.as_css().trim().is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "symbolic component expression cannot be empty"))
        } else {
            Ok(Self {
                authored,
                references: references.into_iter().collect(),
            })
        }
    }

    #[must_use]
    pub const fn authored(&self) -> &AuthoredTokens { &self.authored }

    #[must_use]
    pub fn references(&self) -> &[VariableReference] { &self.references }

    #[must_use]
    pub fn dependencies(&self) -> Vec<CustomPropertyName> {
        let mut dependencies = BTreeSet::new();
        for reference in &self.references {
            dependencies.insert(reference.name().clone());
            if let Some(fallback) = reference.fallback() {
                dependencies.extend(fallback.expression().dependencies());
            }
        }
        dependencies.into_iter().collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RelativeColorFunction {
    Rgb,
    Hsl,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
    Color(PredefinedColorSpace),
}

impl RelativeColorFunction {
    #[must_use]
    pub const fn component_count(self) -> usize { 3 }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelativeColor {
    function: RelativeColorFunction,
    source: StyleColor,
    components: Vec<SymbolicComponentExpression>,
    alpha: Option<SymbolicComponentExpression>,
}

impl RelativeColor {
    pub fn try_new(
        function: RelativeColorFunction,
        source: StyleColor,
        components: Vec<SymbolicComponentExpression>,
        alpha: Option<SymbolicComponentExpression>,
    ) -> Result<Self> {
        if components.len() != function.component_count() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "relative color component count does not match function",
            ));
        }
        source.validate()?;
        Ok(Self { function, source, components, alpha })
    }

    pub fn validate(&self) -> Result<()> {
        if self.components.len() == self.function.component_count() {
            self.source.validate()
        } else {
            Err(Error::new(ErrorCode::InvalidValue, "relative color component count does not match function"))
        }
    }
}
```

The recursive dependency collection above is required. It must include direct `var(...)` references and nested references inside fallback expressions, matching the existing `VariableExpression::dependencies()` behavior in `src/custom.rs`. A component expression such as `calc(var(--a, var(--b)) + 1)` must report both `--a` and `--b`; reporting only direct references is not sufficient.

When hashing `StyleColor::Relative`, hash each component expression by authored CSS text and by the ordered referenced custom property names and fallback expressions through the existing variable-reference hashing helpers. Do not hash only the authored string, because two expressions with the same printed payload but different parsed reference structure would otherwise collapse.

- [ ] **Step 3: Replace color property storage with `StyleColor`**

In `src/value.rs`, add:

```rust
Value::StyleColor(StyleColor),
```

Keep `Value::Color(Color)` temporarily only for non-CSS legacy/internal paths if still needed by existing tests, but Operation 10 CSS-facing properties should use `Value::StyleColor`.

Update:

```rust
Self::StyleColor(_) => Interpolation::Color,
Self::StyleColor(value) => value.validate(),
```

In `src/property.rs`, add `Property::TextDecorationColor` and change metadata for color-bearing CSS properties:

```rust
Self::Color => Metadata::new(Value::StyleColor(StyleColor::rgba(Color::BLACK)))
    .inherited(true)
    .impact(Impact::empty().paint().text())
    .interpolation(Interpolation::Color),
Self::Background => Metadata::new(Value::StyleColor(StyleColor::rgba(Color::TRANSPARENT)))
    .impact(Impact::empty().paint())
    .interpolation(Interpolation::Color),
Self::TextDecorationColor => Metadata::new(Value::StyleColor(StyleColor::current_color()))
    .inherited(false)
    .impact(Impact::empty().paint().text())
    .interpolation(Interpolation::Color),
```

Update `accepts()`:

```rust
Self::Color | Self::Background | Self::TextDecorationColor => {
    matches!(value, Value::StyleColor(_))
}
```

Do not finalize `Property::BorderColor` in Task 1. `border-color` is a four-side border shorthand and is completed in Task 2 after side color longhands exist. Task 1 may update shared color hashing/validation utilities needed by `border-color`, but it must not leave `Property::BorderColor` as a canonical stored aggregate.

Update the existing text-decoration shorthand model instead of creating a separate color-only path:

- Add `color: Option<StyleColor>` to `TextDecoration`.
- Change `TextDecoration::try_new` to accept `(line, color, style, thickness)` in that order so it mirrors the CSS syntax payload.
- Validate `color` with `StyleColor::validate`.
- Add `TextDecoration::color(&self) -> Option<&StyleColor>`.
- Update `TextDecoration` defaults/lowering so omitted color resets to `StyleColor::current_color()`.
- Update `canonical_properties(Property::TextDecoration)` to include `Property::TextDecorationColor`.
- Update authored CSS-wide expansion for `Property::TextDecoration` so `initial`, `inherit`, `unset`, `revert`, and `revert-layer` fan out to line, color, style, and thickness.
- Update `hash_text_decoration` to include the optional color through `hash_style_color`.

- [ ] **Step 4: Add declarations, getters, hashing, and reexports**

In `src/declaration.rs`, update existing color builders to accept `StyleColor`:

```rust
pub fn try_text_color(self, color: StyleColor) -> Result<Self> {
    self.try_set(Property::Color, Value::StyleColor(color))
}

pub fn try_background_color(self, color: StyleColor) -> Result<Self> {
    self.try_set(Property::Background, Value::StyleColor(color))
}

pub fn try_text_decoration_color(self, color: StyleColor) -> Result<Self> {
    self.try_set(Property::TextDecorationColor, Value::StyleColor(color))
}
```

Keep a concrete convenience constructor if current callers need it:

```rust
pub fn try_concrete_text_color(self, color: Color) -> Result<Self> {
    self.try_text_color(StyleColor::rgba(color))
}
```

In `src/resolver.rs`, update color getters to preserve symbolic values:

```rust
pub fn text_color(&self) -> &StyleColor { ... }
pub fn background(&self) -> &StyleColor { ... }
pub fn text_decoration_color(&self) -> &StyleColor { ... }
```

Add `hash_style_color` in `src/declaration.rs` and a new top-level `Value` hash tag after the current Operation 9 tags:

```rust
Value::StyleColor(value) => {
    70u8.hash(state);
    hash_style_color(value, state);
}
```

Hash symbolic payloads by discriminant plus scalar/string fields. Do not hash pointer addresses or debug strings.

Reexport all public color types from `src/lib.rs`.

- [ ] **Step 5: Add type-safety fixtures**

Create `tests/compile_fail/invalid_color_component_literal.rs`:

```rust
use surgeist_style::ColorComponent;

fn main() {
    let _component = ColorComponent(Some(1.0));
}
```

Update `tests/compile_pass/typed_public_construction.rs` with valid construction:

```rust
let alpha = Alpha::new(0.5)?;
let color = StyleColor::Hsl {
    hue: ColorComponent::new(Some(210.0))?,
    saturation: ColorComponent::new(Some(60.0))?,
    lightness: ColorComponent::new(Some(40.0))?,
    alpha: Some(alpha),
};
let declarations = Declarations::new()
    .try_text_color(color.clone())?
    .try_text_decoration_color(StyleColor::current_color())?;
assert_eq!(declarations.len(), 2);
```

- [ ] **Step 6: Run focused Task 1 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style color_properties_accept_symbolic_style_colors
cargo test -p surgeist-style text_decoration_shorthand_lowers_color_with_existing_components
cargo test -p surgeist-style text_decoration_css_wide_expands_to_color_longhand
cargo test -p surgeist-style symbolic_color_components_validate_domains
cargo test -p surgeist-style relative_color_component_dependencies_include_nested_fallbacks
cargo test -p surgeist-style resolved_color_getters_preserve_symbolic_colors
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
pairs = re.findall(r'Value::([A-Za-z0-9_]+)\([^)]*\) => \{\s*\n\s*([0-9]+)u8\.hash', block)
nums = [int(n) for _, n in pairs]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(pairs)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/authored.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_color_component_literal.rs tests/compile_fail/invalid_color_component_literal.stderr
git commit -m "style: add symbolic paint colors"
```

---

### Task 2: Border, Outline, And Radius Models

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_corner_radius_literal.rs`
- Create: `tests/compile_fail/invalid_corner_radius_literal.stderr`
- Create: `tests/compile_fail/invalid_outline_literal.rs`
- Create: `tests/compile_fail/invalid_outline_literal.stderr`

- [ ] **Step 1: Add failing tests for border side and outline models**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn border_shorthands_lower_to_side_longhands() {
    let color = StyleColor::rgba(Color::BLACK);
    let border = Border::try_new(
        Some(Length::Px(2.0)),
        Some(BorderLineStyle::Dashed),
        Some(color.clone()),
    )
    .unwrap();

    let declarations = Declarations::new().try_border(border).unwrap();

    assert_eq!(declarations.get(Property::Border), None);
    for property in [
        Property::BorderTopWidth,
        Property::BorderRightWidth,
        Property::BorderBottomWidth,
        Property::BorderLeftWidth,
    ] {
        assert_eq!(declarations.get(property), Some(&Value::Length(Length::Px(2.0))));
    }
    for property in [
        Property::BorderTopStyle,
        Property::BorderRightStyle,
        Property::BorderBottomStyle,
        Property::BorderLeftStyle,
    ] {
        assert_eq!(
            declarations.get(property),
            Some(&Value::BorderLineStyle(BorderLineStyle::Dashed))
        );
    }
    for property in [
        Property::BorderTopColor,
        Property::BorderRightColor,
        Property::BorderBottomColor,
        Property::BorderLeftColor,
    ] {
        assert_eq!(declarations.get(property), Some(&Value::StyleColor(color.clone())));
    }
}

#[test]
fn border_style_shorthand_lowers_to_side_styles() {
    let styles = BorderStyles::new(
        BorderLineStyle::Solid,
        BorderLineStyle::Dashed,
        BorderLineStyle::Dotted,
        BorderLineStyle::Double,
    );

    let declarations = Declarations::new().border_style(styles);

    assert_eq!(declarations.get(Property::BorderStyle), None);
    assert_eq!(
        declarations.get(Property::BorderTopStyle),
        Some(&Value::BorderLineStyle(BorderLineStyle::Solid))
    );
    assert_eq!(
        declarations.get(Property::BorderRightStyle),
        Some(&Value::BorderLineStyle(BorderLineStyle::Dashed))
    );
    assert_eq!(
        declarations.get(Property::BorderBottomStyle),
        Some(&Value::BorderLineStyle(BorderLineStyle::Dotted))
    );
    assert_eq!(
        declarations.get(Property::BorderLeftStyle),
        Some(&Value::BorderLineStyle(BorderLineStyle::Double))
    );
}

#[test]
fn border_style_css_wide_expands_to_side_styles() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::BorderStyle),
        CssWideKeyword::Inherit,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.get(Property::BorderStyle), None);
    for property in [
        Property::BorderTopStyle,
        Property::BorderRightStyle,
        Property::BorderBottomStyle,
        Property::BorderLeftStyle,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Inherit))
        );
    }
}

#[test]
fn border_color_shorthand_lowers_to_side_colors() {
    let color = StyleColor::system(SystemColor::CanvasText);

    let declarations = Declarations::new().try_border_color(color.clone()).unwrap();

    assert_eq!(declarations.get(Property::BorderColor), None);
    for property in [
        Property::BorderTopColor,
        Property::BorderRightColor,
        Property::BorderBottomColor,
        Property::BorderLeftColor,
    ] {
        assert_eq!(declarations.get(property), Some(&Value::StyleColor(color.clone())));
    }
}

#[test]
fn border_color_css_wide_expands_to_side_colors() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::BorderColor),
        CssWideKeyword::RevertLayer,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.get(Property::BorderColor), None);
    for property in [
        Property::BorderTopColor,
        Property::BorderRightColor,
        Property::BorderBottomColor,
        Property::BorderLeftColor,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
        );
    }
}

#[test]
fn border_radius_shorthand_lowers_to_individual_corners() {
    let top_left = CornerRadius::new(Length::Px(4.0), Length::Percent(40.0)).unwrap();
    let top_right = CornerRadius::new(Length::Px(8.0), Length::Percent(50.0)).unwrap();
    let bottom_right = CornerRadius::new(Length::Px(12.0), Length::Percent(60.0)).unwrap();
    let bottom_left = CornerRadius::new(Length::Px(16.0), Length::Percent(70.0)).unwrap();
    let radii = BorderRadii::new(
        top_left.clone(),
        top_right.clone(),
        bottom_right.clone(),
        bottom_left.clone(),
    );

    let declarations = Declarations::new().try_border_radius(radii).unwrap();

    assert_eq!(declarations.get(Property::Radius), None);
    assert_eq!(
        declarations.get(Property::BorderTopLeftRadius),
        Some(&Value::CornerRadius(top_left))
    );
    assert_eq!(
        declarations.get(Property::BorderTopRightRadius),
        Some(&Value::CornerRadius(top_right))
    );
    assert_eq!(
        declarations.get(Property::BorderBottomRightRadius),
        Some(&Value::CornerRadius(bottom_right))
    );
    assert_eq!(
        declarations.get(Property::BorderBottomLeftRadius),
        Some(&Value::CornerRadius(bottom_left))
    );
}

#[test]
fn outline_shorthand_lowers_to_longhands() {
    let outline = Outline::try_new(
        Some(OutlineWidth::Length(OutlineWidthLength::new(Length::Px(3.0)).unwrap())),
        Some(OutlineStyle::Border(BorderLineStyle::Dotted)),
        Some(StyleColor::current_color()),
    )
    .unwrap();

    let declarations = Declarations::new().try_outline(outline).unwrap();

    assert_eq!(declarations.get(Property::Outline), None);
    assert!(matches!(
        declarations.get(Property::OutlineWidth),
        Some(Value::OutlineWidth(OutlineWidth::Length(_)))
    ));
    assert_eq!(
        declarations.get(Property::OutlineStyle),
        Some(&Value::OutlineStyle(OutlineStyle::Border(BorderLineStyle::Dotted)))
    );
    assert_eq!(
        declarations.get(Property::OutlineColor),
        Some(&Value::StyleColor(StyleColor::current_color()))
    );
}

#[test]
fn border_radius_supports_individual_elliptical_corners() {
    let radius = CornerRadius::new(Length::Px(4.0), Length::Percent(50.0)).unwrap();
    let declarations = Declarations::new()
        .try_border_top_left_radius(radius.clone())
        .unwrap();

    assert_eq!(
        declarations.get(Property::BorderTopLeftRadius),
        Some(&Value::CornerRadius(radius))
    );
}
```

- [ ] **Step 2: Add border, outline, and radius value models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BorderLineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BorderSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BorderStyles {
    top: BorderLineStyle,
    right: BorderLineStyle,
    bottom: BorderLineStyle,
    left: BorderLineStyle,
}

impl BorderStyles {
    #[must_use]
    pub const fn new(
        top: BorderLineStyle,
        right: BorderLineStyle,
        bottom: BorderLineStyle,
        left: BorderLineStyle,
    ) -> Self {
        Self { top, right, bottom, left }
    }

    #[must_use]
    pub const fn top(self) -> BorderLineStyle { self.top }
    #[must_use]
    pub const fn right(self) -> BorderLineStyle { self.right }
    #[must_use]
    pub const fn bottom(self) -> BorderLineStyle { self.bottom }
    #[must_use]
    pub const fn left(self) -> BorderLineStyle { self.left }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Border {
    width: Option<Length>,
    style: Option<BorderLineStyle>,
    color: Option<StyleColor>,
}

impl Border {
    pub fn try_new(
        width: Option<Length>,
        style: Option<BorderLineStyle>,
        color: Option<StyleColor>,
    ) -> Result<Self> {
        if width.is_none() && style.is_none() && color.is_none() {
            return Err(Error::new(ErrorCode::InvalidValue, "border shorthand requires at least one component"));
        }
        if let Some(width) = &width {
            validate_border_width_length(width)?;
        }
        if let Some(color) = &color {
            color.validate()?;
        }
        Ok(Self { width, style, color })
    }

    #[must_use]
    pub const fn width(&self) -> Option<&Length> { self.width.as_ref() }
    #[must_use]
    pub const fn style(&self) -> Option<BorderLineStyle> { self.style }
    #[must_use]
    pub const fn color(&self) -> Option<&StyleColor> { self.color.as_ref() }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CornerRadius {
    horizontal: Length,
    vertical: Length,
}

impl CornerRadius {
    pub fn new(horizontal: Length, vertical: Length) -> Result<Self> {
        validate_radius_length(&horizontal)?;
        validate_radius_length(&vertical)?;
        Ok(Self { horizontal, vertical })
    }

    #[must_use]
    pub const fn horizontal(&self) -> &Length { &self.horizontal }
    #[must_use]
    pub const fn vertical(&self) -> &Length { &self.vertical }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BorderRadii {
    top_left: CornerRadius,
    top_right: CornerRadius,
    bottom_right: CornerRadius,
    bottom_left: CornerRadius,
}

impl BorderRadii {
    #[must_use]
    pub fn all(radius: CornerRadius) -> Self {
        Self {
            top_left: radius.clone(),
            top_right: radius.clone(),
            bottom_right: radius.clone(),
            bottom_left: radius,
        }
    }

    #[must_use]
    pub const fn new(
        top_left: CornerRadius,
        top_right: CornerRadius,
        bottom_right: CornerRadius,
        bottom_left: CornerRadius,
    ) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }

    #[must_use]
    pub const fn top_left(&self) -> &CornerRadius { &self.top_left }
    #[must_use]
    pub const fn top_right(&self) -> &CornerRadius { &self.top_right }
    #[must_use]
    pub const fn bottom_right(&self) -> &CornerRadius { &self.bottom_right }
    #[must_use]
    pub const fn bottom_left(&self) -> &CornerRadius { &self.bottom_left }
}
```

Add outline types:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OutlineStyle {
    Auto,
    Border(BorderLineStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutlineWidth {
    Thin,
    Medium,
    Thick,
    Length(OutlineWidthLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutlineWidthLength(Length);

impl OutlineWidthLength {
    pub fn new(length: Length) -> Result<Self> {
        validate_border_width_length(&length)?;
        Ok(Self(length))
    }

    #[must_use]
    pub const fn length(&self) -> &Length { &self.0 }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Outline {
    width: Option<OutlineWidth>,
    style: Option<OutlineStyle>,
    color: Option<StyleColor>,
}

impl Outline {
    pub fn try_new(
        width: Option<OutlineWidth>,
        style: Option<OutlineStyle>,
        color: Option<StyleColor>,
    ) -> Result<Self> {
        if width.is_none() && style.is_none() && color.is_none() {
            return Err(Error::new(ErrorCode::InvalidValue, "outline shorthand requires at least one component"));
        }
        if let Some(color) = &color {
            color.validate()?;
        }
        Ok(Self { width, style, color })
    }

    #[must_use]
    pub const fn width(&self) -> Option<&OutlineWidth> { self.width.as_ref() }
    #[must_use]
    pub const fn style(&self) -> Option<OutlineStyle> { self.style }
    #[must_use]
    pub const fn color(&self) -> Option<&StyleColor> { self.color.as_ref() }
}
```

Add length validation helpers:

```rust
pub(crate) fn validate_border_width_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(_) | Length::Calc(_) => {
            length.validate()?;
            validate_non_negative_style_length(length, "border width")
        }
        Length::Percent(_)
        | Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            "border width accepts only non-negative non-percentage lengths",
        )),
    }
}

pub(crate) fn validate_radius_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(_) | Length::Percent(_) | Length::Calc(_) => {
            length.validate()?;
            validate_non_negative_style_length(length, "corner radius")
        }
        Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            "corner radius accepts only non-negative length or percentage values",
        )),
    }
}
```

Use the existing sign-aware calc coefficient logic from `property.rs`/Operation 9 rather than resolving calc values.

- [ ] **Step 3: Add properties, values, metadata, and shorthand lowering**

Add `Property` variants:

```rust
Border,
BorderColor,
BorderStyle,
BorderTop,
BorderRight,
BorderBottom,
BorderLeft,
BorderTopColor,
BorderRightColor,
BorderBottomColor,
BorderLeftColor,
BorderTopStyle,
BorderRightStyle,
BorderBottomStyle,
BorderLeftStyle,
BorderTopLeftRadius,
BorderTopRightRadius,
BorderBottomRightRadius,
BorderBottomLeftRadius,
Outline,
OutlineColor,
OutlineStyle,
OutlineWidth,
```

If `Property::Radius` already exists, replace its old aggregate `Value::Corners`/length-pair behavior with `Value::BorderRadii`. If it does not exist in the worker's starting revision, add it as the style-owned aggregate front door for CSS `border-radius`; do not add both `Radius` and `BorderRadius`.

Add `Value` variants:

```rust
Border(Border),
BorderStyles(BorderStyles),
BorderLineStyle(BorderLineStyle),
CornerRadius(CornerRadius),
BorderRadii(BorderRadii),
Outline(Outline),
OutlineStyle(OutlineStyle),
OutlineWidth(OutlineWidth),
```

Mark `Border`, `BorderColor`, `BorderStyle`, side border shorthands, `Radius`, and `Outline` non-canonical. `Radius` is the existing style-owned aggregate property that corresponds to CSS `border-radius`; do not add a second public aggregate property unless the existing name has already been removed in the worker's local diff. Ensure CSS-wide keyword expansion covers canonical longhands for all non-canonical shorthands.

Canonical properties:

```rust
Property::Border => vec![
    Property::BorderTopWidth, Property::BorderRightWidth, Property::BorderBottomWidth, Property::BorderLeftWidth,
    Property::BorderTopStyle, Property::BorderRightStyle, Property::BorderBottomStyle, Property::BorderLeftStyle,
    Property::BorderTopColor, Property::BorderRightColor, Property::BorderBottomColor, Property::BorderLeftColor,
],
Property::BorderStyle => vec![
    Property::BorderTopStyle,
    Property::BorderRightStyle,
    Property::BorderBottomStyle,
    Property::BorderLeftStyle,
],
Property::BorderColor => vec![
    Property::BorderTopColor,
    Property::BorderRightColor,
    Property::BorderBottomColor,
    Property::BorderLeftColor,
],
Property::BorderTop => vec![Property::BorderTopWidth, Property::BorderTopStyle, Property::BorderTopColor],
Property::BorderRight => vec![Property::BorderRightWidth, Property::BorderRightStyle, Property::BorderRightColor],
Property::BorderBottom => vec![Property::BorderBottomWidth, Property::BorderBottomStyle, Property::BorderBottomColor],
Property::BorderLeft => vec![Property::BorderLeftWidth, Property::BorderLeftStyle, Property::BorderLeftColor],
Property::Radius => vec![
    Property::BorderTopLeftRadius,
    Property::BorderTopRightRadius,
    Property::BorderBottomRightRadius,
    Property::BorderBottomLeftRadius,
],
Property::Outline => vec![Property::OutlineWidth, Property::OutlineStyle, Property::OutlineColor],
```

Update metadata and accepts paths:

- `Property::Border` default: `Value::Border(Border::try_new(Some(Length::Px(3.0)), Some(BorderLineStyle::None), Some(StyleColor::current_color())).unwrap())`.
- `Property::Border` accepts only `Value::Border(_)`.
- `Property::BorderTop`, `Property::BorderRight`, `Property::BorderBottom`, and `Property::BorderLeft` share the same `Value::Border(...)` reset default as `Property::Border` and accept only `Value::Border(_)`.
- Update `Property::BorderWidth` default to `Value::Edges(Edges::all(Length::Px(3.0)))`, or the existing equivalent constructor, so the style default matches the CSS `medium` width approximation used by border shorthand reset.
- Update `Property::BorderTopWidth`, `Property::BorderRightWidth`, `Property::BorderBottomWidth`, and `Property::BorderLeftWidth` defaults from `Length::ZERO` to `Value::Length(Length::Px(3.0))`.
- Existing border-width aggregate and side longhands keep their Operation 8 accepted value shapes: `Property::BorderWidth` accepts `Value::Edges(_)`, and side width longhands accept `Value::Length(_)`. Task 2 must not replace them with a new width model.
- `Property::BorderColor` default: `Value::StyleColor(StyleColor::current_color())`.
- `Property::BorderColor` accepts only `Value::StyleColor(_)` and lowers to the four side color longhands; it is not canonical stored aggregate state.
- `Property::BorderTopColor`, `Property::BorderRightColor`, `Property::BorderBottomColor`, and `Property::BorderLeftColor` default to `Value::StyleColor(StyleColor::current_color())` and accept only `Value::StyleColor(_)`.
- `Property::Radius` default: `Value::BorderRadii(BorderRadii::all(CornerRadius::new(Length::Px(0.0), Length::Px(0.0)).unwrap()))`.
- `Property::Radius` accepts only `Value::BorderRadii(_)`.
- Corner radius longhands default to `Value::CornerRadius(CornerRadius::new(Length::Px(0.0), Length::Px(0.0)).unwrap())`.
- Corner radius longhands accept only `Value::CornerRadius(_)`.
- `Property::BorderStyle` default: `Value::BorderStyles(BorderStyles::new(BorderLineStyle::None, BorderLineStyle::None, BorderLineStyle::None, BorderLineStyle::None))`.
- `Property::BorderStyle` accepts only `Value::BorderStyles(_)`.
- `Property::BorderTopStyle`, `Property::BorderRightStyle`, `Property::BorderBottomStyle`, and `Property::BorderLeftStyle` default to `Value::BorderLineStyle(BorderLineStyle::None)` and accept only `Value::BorderLineStyle(_)`.
- `Property::Outline` default: `Value::Outline(Outline::try_new(Some(OutlineWidth::Medium), Some(OutlineStyle::Border(BorderLineStyle::None)), Some(StyleColor::current_color())).unwrap())`.
- `Property::Outline` accepts only `Value::Outline(_)`.
- `Property::OutlineWidth` default: `Value::OutlineWidth(OutlineWidth::Medium)` and accepts only `Value::OutlineWidth(_)`.
- `Property::OutlineStyle` default: `Value::OutlineStyle(OutlineStyle::Border(BorderLineStyle::None))` and accepts only `Value::OutlineStyle(_)`.
- `Property::OutlineColor` default: `Value::StyleColor(StyleColor::current_color())` and accepts only `Value::StyleColor(_)`.

Update inherited/impact/interpolation/animatable policy explicitly:

- All border, radius, and outline properties introduced or changed in Task 2 are non-inherited.
- `Property::Border`, `Property::BorderTop`, `Property::BorderRight`, `Property::BorderBottom`, `Property::BorderLeft`, `Property::BorderWidth`, and border width side longhands use `Impact::empty().layout().paint()`. Width longhands use `Interpolation::Length` and are animatable; `BorderWidth` uses `Interpolation::Edges` and is animatable; composite border shorthands remain discrete and non-animatable.
- `Property::BorderColor` and border color side longhands use `Impact::empty().paint()`, `Interpolation::Color`, and are animatable.
- `Property::BorderStyle` and border style side longhands use `Impact::empty().layout().paint()`, `Interpolation::Discrete`, and are non-animatable. The layout impact is intentional because `none`/`hidden` affects used border width.
- `Property::Radius` and corner radius longhands use `Impact::empty().paint()`, `Interpolation::Corners`, and are animatable.
- `Property::Outline`, `Property::OutlineWidth`, `Property::OutlineStyle`, and `Property::OutlineColor` use `Impact::empty().paint()` only; outline does not affect layout. `OutlineWidth` is animatable with `Interpolation::Length`, `OutlineColor` is animatable with `Interpolation::Color`, and `Outline`/`OutlineStyle` are discrete and non-animatable.

Shorthand lowering must reset omitted components to defaults:

- border width default: `Length::Px(3.0)`. The current CSS source model exposes border shorthand width as `CssLength`, not a border-width keyword enum, so Operation 10 keeps the existing style length storage for border widths.
- border style default: `BorderLineStyle::None`.
- border color default: `StyleColor::current_color()`.
- `BorderColor` lowers its single `StyleColor` to all four side color longhands, and CSS-wide keywords on `BorderColor` expand to those same four side color longhands.
- `BorderStyle` lowers `BorderStyles` directly to the four side style longhands, and CSS-wide keywords on `BorderStyle` expand to those same four longhands.
- `Radius`/CSS `border-radius` lowers `BorderRadii` directly to the four corner longhands and CSS-wide keywords on `Radius` expand to those same four longhands.
- outline width default: `OutlineWidth::Medium`.
- outline style default: `OutlineStyle::Border(BorderLineStyle::None)`.
- outline color default: `StyleColor::current_color()`.

Add tests for reset behavior when previous longhands are overwritten by partial shorthands.

- [ ] **Step 4: Add builders, getters, hashing, and reexports**

Add declaration builders:

```rust
pub fn try_border(self, value: Border) -> Result<Self>;
pub fn try_border_top(self, value: Border) -> Result<Self>;
pub fn try_border_right(self, value: Border) -> Result<Self>;
pub fn try_border_bottom(self, value: Border) -> Result<Self>;
pub fn try_border_left(self, value: Border) -> Result<Self>;
pub fn border_style(self, value: BorderStyles) -> Self;
pub fn border_top_style(self, value: BorderLineStyle) -> Self;
pub fn border_right_style(self, value: BorderLineStyle) -> Self;
pub fn border_bottom_style(self, value: BorderLineStyle) -> Self;
pub fn border_left_style(self, value: BorderLineStyle) -> Self;
pub fn try_border_color(self, value: StyleColor) -> Result<Self>;
pub fn try_border_top_color(self, value: StyleColor) -> Result<Self>;
pub fn try_border_right_color(self, value: StyleColor) -> Result<Self>;
pub fn try_border_bottom_color(self, value: StyleColor) -> Result<Self>;
pub fn try_border_left_color(self, value: StyleColor) -> Result<Self>;
pub fn try_border_top_left_radius(self, value: CornerRadius) -> Result<Self>;
pub fn try_border_top_right_radius(self, value: CornerRadius) -> Result<Self>;
pub fn try_border_bottom_right_radius(self, value: CornerRadius) -> Result<Self>;
pub fn try_border_bottom_left_radius(self, value: CornerRadius) -> Result<Self>;
pub fn try_border_radius(self, value: BorderRadii) -> Result<Self>;
pub fn try_outline(self, value: Outline) -> Result<Self>;
pub fn try_outline_color(self, value: StyleColor) -> Result<Self>;
pub fn outline_style(self, value: OutlineStyle) -> Self;
pub fn try_outline_width(self, value: OutlineWidth) -> Result<Self>;
```

Add resolver getters for side colors/styles, corner radii, and outline longhands.

Add hash tags starting at `71u8` and keep them unique.

Reexport new types from `src/lib.rs`.

- [ ] **Step 5: Add type-safety fixtures**

Create `tests/compile_fail/invalid_corner_radius_literal.rs`:

```rust
use surgeist_style::{CornerRadius, Length};

fn main() {
    let _radius = CornerRadius {
        horizontal: Length::Px(1.0),
        vertical: Length::Px(1.0),
    };
}
```

Create `tests/compile_fail/invalid_outline_literal.rs`:

```rust
use surgeist_style::Outline;

fn main() {
    let _outline = Outline {
        width: None,
        style: None,
        color: None,
    };
}
```

- [ ] **Step 6: Run focused Task 2 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style border_shorthands_lower_to_side_longhands
cargo test -p surgeist-style border_style_shorthand_lowers_to_side_styles
cargo test -p surgeist-style border_style_css_wide_expands_to_side_styles
cargo test -p surgeist-style border_color_shorthand_lowers_to_side_colors
cargo test -p surgeist-style border_color_css_wide_expands_to_side_colors
cargo test -p surgeist-style border_radius_shorthand_lowers_to_individual_corners
cargo test -p surgeist-style outline_shorthand_lowers_to_longhands
cargo test -p surgeist-style border_radius_supports_individual_elliptical_corners
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
pairs = re.findall(r'Value::([A-Za-z0-9_]+)\([^)]*\) => \{\s*\n\s*([0-9]+)u8\.hash', block)
nums = [int(n) for _, n in pairs]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(pairs)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/authored.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_corner_radius_literal.rs tests/compile_fail/invalid_corner_radius_literal.stderr tests/compile_fail/invalid_outline_literal.rs tests/compile_fail/invalid_outline_literal.stderr
git commit -m "style: add border and outline paint models"
```

---

### Task 3: Background And Mask Layer Models

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Modify: `tests/type_safety.rs`
- Create: `tests/compile_fail/invalid_style_url_literal.rs`
- Create: `tests/compile_fail/invalid_style_url_literal.stderr`
- Create: `tests/compile_fail/invalid_image_layer_list_empty.rs`
- Create: `tests/compile_fail/invalid_image_layer_list_empty.stderr`

- [ ] **Step 1: Add failing tests for background and mask layer values**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn background_layer_properties_accept_symbolic_values() {
    let images = ImageLayerList::try_new([ImageLayer::url(StyleUrl::new("hero.png").unwrap())]).unwrap();
    let position = Position::try_new([
        PositionComponent::Horizontal(HorizontalPositionKeyword::Left),
        PositionComponent::Length(Length::Percent(25.0)),
    ])
    .unwrap();
    let positions = PositionList::try_new([position.clone()]).unwrap();
    let size = BackgroundSize::Explicit {
        width: BackgroundSizeComponent::Length(Length::Percent(100.0)),
        height: Some(BackgroundSizeComponent::Auto),
    };
    let sizes = BackgroundSizeList::try_new([size.clone()]).unwrap();

    let declarations = Declarations::new()
        .background_image(images.clone())
        .background_position(positions.clone())
        .background_size(sizes.clone())
        .background_repeat(BackgroundRepeatList::try_new([BackgroundRepeat::RepeatX]).unwrap())
        .background_origin(BackgroundBox::PaddingBox)
        .background_clip(BackgroundBox::ContentBox)
        .background_attachment(BackgroundAttachmentList::try_new([BackgroundAttachment::Fixed]).unwrap());

    assert_eq!(declarations.get(Property::BackgroundImage), Some(&Value::ImageLayerList(images)));
    assert_eq!(declarations.get(Property::BackgroundPosition), Some(&Value::PositionList(positions)));
    assert_eq!(declarations.get(Property::BackgroundSize), Some(&Value::BackgroundSizeList(sizes)));
}

#[test]
fn mask_shorthand_lowers_to_layer_longhands() {
    let layer = MaskLayer::try_new(
        Some(ImageLayer::url(StyleUrl::new("mask.svg").unwrap())),
        Some(Position::try_new([PositionComponent::Vertical(VerticalPositionKeyword::Top)]).unwrap()),
        Some(BackgroundSize::Contain),
        Some(BackgroundRepeat::Axes {
            x: BackgroundRepeatStyle::NoRepeat,
            y: BackgroundRepeatStyle::NoRepeat,
        }),
    )
    .unwrap();
    let mask = MaskLayerList::try_new([layer]).unwrap();
    let declarations = Declarations::new().mask(mask.clone()).unwrap();

    assert_eq!(declarations.get(Property::Mask), None);
    assert!(matches!(declarations.get(Property::MaskImage), Some(Value::ImageLayerList(_))));
    assert!(matches!(declarations.get(Property::MaskPosition), Some(Value::PositionList(_))));
    assert!(matches!(declarations.get(Property::MaskSize), Some(Value::BackgroundSizeList(_))));
    assert!(matches!(declarations.get(Property::MaskRepeat), Some(Value::BackgroundRepeatList(_))));
}
```

- [ ] **Step 2: Add resource and layer value models**

In `src/value.rs`, add private-field models with non-empty constructors:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StyleUrl(String);

impl StyleUrl {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "style URL cannot be empty"))
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ImageLayer {
    None,
    Url(StyleUrl),
}

impl ImageLayer {
    pub fn url(url: StyleUrl) -> Self { Self::Url(url) }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImageLayerList {
    layers: Vec<ImageLayer>,
}

impl ImageLayerList {
    pub fn try_new(layers: impl IntoIterator<Item = ImageLayer>) -> Result<Self> {
        let layers = layers.into_iter().collect::<Vec<_>>();
        if layers.is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "image layer list cannot be empty"))
        } else {
            Ok(Self { layers })
        }
    }

    #[must_use]
    pub fn layers(&self) -> &[ImageLayer] { &self.layers }
}
```

Add position, size, repeat, box, attachment, and mask models using the names from Planned Type Names. Constructors must reject:

- empty `Position` and `PositionList`,
- `Position` with more than four components,
- duplicate horizontal side keywords (`left`/`right`) or duplicate vertical side keywords (`top`/`bottom`),
- empty `BackgroundSizeList`, `BackgroundRepeatList`, `BackgroundAttachmentList`, and `MaskLayerList`,
- empty `MaskLayer`.

Add a convenience constructor for the CSS initial layer position:

```rust
impl Position {
    pub fn origin() -> Self {
        Self {
            components: vec![
                PositionComponent::Length(Length::Percent(0.0)),
                PositionComponent::Length(Length::Percent(0.0)),
            ],
        }
    }
}
```

If the final `Position` representation distinguishes axes more explicitly, keep the semantics exactly equivalent to CSS `0% 0%`/left top. Do not use centered position for metadata defaults.

- [ ] **Step 3: Add properties, values, and metadata**

Add `Property` variants:

```rust
BackgroundImage,
BackgroundPosition,
BackgroundSize,
BackgroundRepeat,
BackgroundOrigin,
BackgroundClip,
BackgroundAttachment,
Mask,
MaskImage,
MaskSize,
MaskPosition,
MaskRepeat,
```

Add `Value` variants:

```rust
ImageLayerList(ImageLayerList),
PositionList(PositionList),
BackgroundSizeList(BackgroundSizeList),
BackgroundRepeatList(BackgroundRepeatList),
BackgroundBox(BackgroundBox),
BackgroundAttachmentList(BackgroundAttachmentList),
MaskLayerList(MaskLayerList),
```

Metadata defaults:

- `BackgroundImage`: one `ImageLayer::None`.
- `BackgroundPosition`: one `Position::origin()` (`0% 0%` / left top).
- `BackgroundSize`: one `BackgroundSize::Explicit { width: Auto, height: None }`.
- `BackgroundRepeat`: one `Axes { x: Repeat, y: Repeat }`.
- `BackgroundOrigin`: `BackgroundBox::PaddingBox`.
- `BackgroundClip`: `BackgroundBox::BorderBox`.
- `BackgroundAttachment`: one `BackgroundAttachment::Scroll`.
- `MaskImage`: one `ImageLayer::None`.
- `MaskPosition`: one `Position::origin()` (`0% 0%` / left top).
- `MaskSize`: auto size.
- `MaskRepeat`: repeat.

Mark `Mask` non-canonical and lower it to `MaskImage`, `MaskPosition`, `MaskSize`, and `MaskRepeat`, defaulting omitted layer components.

Do not implement full `background` shorthand layering in style unless all background longhand components are present in the `Background` value model. Current CSS `Background` row remains color-only in the ledger; root may lower color-only background through `try_background_color`.

- [ ] **Step 4: Add builders, getters, hashing, and reexports**

Add declaration builders:

```rust
pub fn background_image(self, value: ImageLayerList) -> Self;
pub fn background_position(self, value: PositionList) -> Self;
pub fn background_size(self, value: BackgroundSizeList) -> Self;
pub fn background_repeat(self, value: BackgroundRepeatList) -> Self;
pub fn background_origin(self, value: BackgroundBox) -> Self;
pub fn background_clip(self, value: BackgroundBox) -> Self;
pub fn background_attachment(self, value: BackgroundAttachmentList) -> Self;
pub fn mask(self, value: MaskLayerList) -> Result<Self>;
pub fn mask_image(self, value: ImageLayerList) -> Self;
pub fn mask_position(self, value: PositionList) -> Self;
pub fn mask_size(self, value: BackgroundSizeList) -> Self;
pub fn mask_repeat(self, value: BackgroundRepeatList) -> Self;
```

Add resolver getters and hash tags after Task 2 tags. Reexport all new types.

- [ ] **Step 5: Add type-safety fixtures**

Create `tests/compile_fail/invalid_style_url_literal.rs`:

```rust
use surgeist_style::StyleUrl;

fn main() {
    let _url = StyleUrl(String::from("x.png"));
}
```

Create `tests/compile_fail/invalid_image_layer_list_empty.rs`:

```rust
use surgeist_style::ImageLayerList;

fn main() {
    let _layers = ImageLayerList { layers: Vec::new() };
}
```

- [ ] **Step 6: Run focused Task 3 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style background_layer_properties_accept_symbolic_values
cargo test -p surgeist-style mask_shorthand_lowers_to_layer_longhands
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
pairs = re.findall(r'Value::([A-Za-z0-9_]+)\([^)]*\) => \{\s*\n\s*([0-9]+)u8\.hash', block)
nums = [int(n) for _, n in pairs]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(pairs)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/authored.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_style_url_literal.rs tests/compile_fail/invalid_style_url_literal.stderr tests/compile_fail/invalid_image_layer_list_empty.rs tests/compile_fail/invalid_image_layer_list_empty.stderr
git commit -m "style: add background and mask paint models"
```

---

### Task 4: Transforms, Filters, Clip Paths, And Paint Effects

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_filter_function_list_empty.rs`
- Create: `tests/compile_fail/invalid_filter_function_list_empty.stderr`
- Create: `tests/compile_fail/invalid_scale_values_literal.rs`
- Create: `tests/compile_fail/invalid_scale_values_literal.stderr`

- [ ] **Step 1: Add failing tests for effects and individual transforms**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn effect_properties_accept_symbolic_values() {
    let filter = Filter::Functions(
        FilterFunctionList::try_new([FilterFunction::Blur(SymbolicFunctionValue::new("4px").unwrap())]).unwrap(),
    );
    let clip = ClipPath::BasicShape(BasicShape::Circle(SymbolicFunctionValue::new("50%").unwrap()));

    let declarations = Declarations::new()
        .box_decoration_break(BoxDecorationBreak::Clone)
        .filter(filter.clone())
        .backdrop_filter(Filter::None)
        .clip_path(clip.clone());

    assert_eq!(
        declarations.get(Property::BoxDecorationBreak),
        Some(&Value::BoxDecorationBreak(BoxDecorationBreak::Clone))
    );
    assert_eq!(declarations.get(Property::Filter), Some(&Value::Filter(filter)));
    assert_eq!(declarations.get(Property::BackdropFilter), Some(&Value::Filter(Filter::None)));
    assert_eq!(declarations.get(Property::ClipPath), Some(&Value::ClipPath(clip)));
}

#[test]
fn individual_transform_properties_accept_symbolic_values() {
    let translate = Translate::try_values([Length::Px(10.0), Length::Percent(5.0)]).unwrap();
    let scale = Scale::try_values([1.0, 2.0]).unwrap();
    let rotate = Rotate::Value(SymbolicFunctionValue::new("45deg").unwrap());

    let declarations = Declarations::new()
        .translate(translate.clone())
        .rotate(rotate.clone())
        .scale(scale.clone());

    assert_eq!(declarations.get(Property::Translate), Some(&Value::Translate(translate)));
    assert_eq!(declarations.get(Property::Rotate), Some(&Value::Rotate(rotate)));
    assert_eq!(declarations.get(Property::Scale), Some(&Value::Scale(scale)));
}
```

- [ ] **Step 2: Add symbolic function and effect models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SymbolicFunctionValue(String);

impl SymbolicFunctionValue {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "symbolic function value cannot be empty"))
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FilterFunction {
    Blur(SymbolicFunctionValue),
    Brightness(SymbolicFunctionValue),
    Contrast(SymbolicFunctionValue),
    DropShadow(SymbolicFunctionValue),
    Grayscale(SymbolicFunctionValue),
    HueRotate(SymbolicFunctionValue),
    Invert(SymbolicFunctionValue),
    Opacity(SymbolicFunctionValue),
    Saturate(SymbolicFunctionValue),
    Sepia(SymbolicFunctionValue),
    Url(StyleUrl),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FilterFunctionList {
    functions: Vec<FilterFunction>,
}

impl FilterFunctionList {
    pub fn try_new(functions: impl IntoIterator<Item = FilterFunction>) -> Result<Self> {
        let functions = functions.into_iter().collect::<Vec<_>>();
        if functions.is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "filter function list cannot be empty"))
        } else {
            Ok(Self { functions })
        }
    }

    #[must_use]
    pub fn functions(&self) -> &[FilterFunction] { &self.functions }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Filter {
    None,
    Functions(FilterFunctionList),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BasicShape {
    Inset(SymbolicFunctionValue),
    Circle(SymbolicFunctionValue),
    Ellipse(SymbolicFunctionValue),
    Polygon(SymbolicFunctionValue),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClipPath {
    None,
    Url(StyleUrl),
    BasicShape(BasicShape),
}
```

Add transform property models:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Translate {
    None,
    Values(TranslateValues),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TranslateValues {
    values: Vec<Length>,
}

impl Translate {
    pub fn try_values(values: impl IntoIterator<Item = Length>) -> Result<Self> {
        Ok(Self::Values(TranslateValues::try_new(values)?))
    }
}

impl TranslateValues {
    pub fn try_new(values: impl IntoIterator<Item = Length>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() || values.len() > 3 {
            return Err(Error::new(ErrorCode::InvalidValue, "translate requires one to three values"));
        }
        for value in &values {
            value.validate()?;
        }
        Ok(Self { values })
    }

    #[must_use]
    pub fn values(&self) -> &[Length] { &self.values }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Rotate {
    None,
    Value(SymbolicFunctionValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scale {
    None,
    Values(ScaleValues),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScaleValues {
    values: Vec<f32>,
}

impl Scale {
    pub fn try_values(values: impl IntoIterator<Item = f32>) -> Result<Self> {
        Ok(Self::Values(ScaleValues::try_new(values)?))
    }
}

impl ScaleValues {
    pub fn try_new(values: impl IntoIterator<Item = f32>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() || values.len() > 3 || values.iter().any(|value| !value.is_finite()) {
            Err(Error::new(ErrorCode::InvalidValue, "scale requires one to three finite values"))
        } else {
            Ok(Self { values })
        }
    }

    #[must_use]
    pub fn values(&self) -> &[f32] { &self.values }
}
```

Review existing `Transform`/`TransformOp`. Do not replace it in this task unless necessary. Operation 10 adds individual `translate`, `rotate`, and `scale` properties; full CSS transform function parity may stay symbolic in the existing `Transform` path unless the worker can preserve current API and tests cleanly.

- [ ] **Step 3: Add properties, values, metadata, builders, getters**

Add `Property` variants:

```rust
BoxDecorationBreak,
BackdropFilter,
ClipPath,
Translate,
Rotate,
Scale,
```

`Property::Filter` already exists but currently rejects ordinary values. Replace its placeholder path with `Value::Filter`.

Add `Value` variants:

```rust
BoxDecorationBreak(BoxDecorationBreak),
Filter(Filter),
ClipPath(ClipPath),
Translate(Translate),
Rotate(Rotate),
Scale(Scale),
```

Add metadata defaults:

- `BoxDecorationBreak::Slice`.
- `Filter::None`.
- `BackdropFilter` uses `Filter::None`.
- `ClipPath::None`.
- `Translate::None`.
- `Rotate::None`.
- `Scale::None`.

Add builders:

```rust
pub fn box_decoration_break(self, value: BoxDecorationBreak) -> Self;
pub fn filter(self, value: Filter) -> Self;
pub fn backdrop_filter(self, value: Filter) -> Self;
pub fn clip_path(self, value: ClipPath) -> Self;
pub fn translate(self, value: Translate) -> Self;
pub fn rotate(self, value: Rotate) -> Self;
pub fn scale(self, value: Scale) -> Self;
```

Add resolver getters and hash tags after Task 3 tags.

- [ ] **Step 4: Add type-safety fixtures**

Create `tests/compile_fail/invalid_filter_function_list_empty.rs`:

```rust
use surgeist_style::FilterFunctionList;

fn main() {
    let _list = FilterFunctionList { functions: Vec::new() };
}
```

Create `tests/compile_fail/invalid_scale_values_literal.rs`:

```rust
use surgeist_style::ScaleValues;

fn main() {
    let _values = ScaleValues { values: vec![1.0] };
}
```

- [ ] **Step 5: Run focused Task 4 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style effect_properties_accept_symbolic_values
cargo test -p surgeist-style individual_transform_properties_accept_symbolic_values
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
pairs = re.findall(r'Value::([A-Za-z0-9_]+)\([^)]*\) => \{\s*\n\s*([0-9]+)u8\.hash', block)
nums = [int(n) for _, n in pairs]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(pairs)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 6: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_filter_function_list_empty.rs tests/compile_fail/invalid_filter_function_list_empty.stderr tests/compile_fail/invalid_scale_values_literal.rs tests/compile_fail/invalid_scale_values_literal.stderr
git commit -m "style: add symbolic effect and transform models"
```

---

### Task 5: Interaction Integration And Operation 10 Smoke Coverage

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add `UserSelect` model and focused tests**

Add to `src/value.rs`:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum UserSelect {
    #[default]
    Auto,
    Text,
    None,
    All,
    Contain,
}
```

Add `Property::UserSelect`, `Value::UserSelect(UserSelect)`, metadata with inherited false paint/interaction impact, declaration builder:

```rust
pub fn user_select(self, value: UserSelect) -> Self {
    self.set(Property::UserSelect, Value::UserSelect(value))
}
```

Add resolver getter:

```rust
pub fn user_select(&self) -> UserSelect { ... }
```

Add test:

```rust
#[test]
fn interaction_properties_accept_typed_values() {
    let declarations = Declarations::new()
        .cursor(Cursor::Pointer)
        .pointer_events(PointerEvents::None)
        .user_select(UserSelect::All);

    assert_eq!(declarations.get(Property::Cursor), Some(&Value::Cursor(Cursor::Pointer)));
    assert_eq!(declarations.get(Property::PointerEvents), Some(&Value::PointerEvents(PointerEvents::None)));
    assert_eq!(declarations.get(Property::UserSelect), Some(&Value::UserSelect(UserSelect::All)));
}
```

- [ ] **Step 2: Add Operation 10 resolved smoke test**

Add to `src/resolver.rs` tests:

```rust
#[test]
fn paint_operation_ten_values_resolve_together() {
    let background = StyleColor::system(SystemColor::Canvas);
    let border_color = StyleColor::current_color();
    let corner = CornerRadius::new(Length::Px(8.0), Length::Px(12.0)).unwrap();
    let images = ImageLayerList::try_new([ImageLayer::url(StyleUrl::new("bg.png").unwrap())]).unwrap();
    let filter = Filter::Functions(
        FilterFunctionList::try_new([FilterFunction::Brightness(
            SymbolicFunctionValue::new("120%").unwrap(),
        )])
        .unwrap(),
    );

    let style = resolve_single(
        Declarations::new()
            .try_background_color(background.clone())
            .unwrap()
            .background_image(images.clone())
            .try_border_top_color(border_color.clone())
            .unwrap()
            .border_top_style(BorderLineStyle::Solid)
            .try_border_top_left_radius(corner.clone())
            .unwrap()
            .box_decoration_break(BoxDecorationBreak::Clone)
            .filter(filter.clone())
            .clip_path(ClipPath::None)
            .translate(Translate::try_values([Length::Px(2.0)]).unwrap())
            .scale(Scale::try_values([1.0, 1.2]).unwrap())
            .user_select(UserSelect::Text),
    );

    assert_eq!(style.background(), &background);
    assert_eq!(style.border_top_color(), &border_color);
    assert_eq!(style.border_top_style(), BorderLineStyle::Solid);
    assert_eq!(style.border_top_left_radius(), &corner);
    assert_eq!(style.box_decoration_break(), BoxDecorationBreak::Clone);
    assert_eq!(style.filter(), &filter);
    assert_eq!(style.clip_path(), &ClipPath::None);
    assert_eq!(style.user_select(), UserSelect::Text);
}
```

- [ ] **Step 3: Update compile-pass public construction**

Add valid use of all Operation 10 public front doors to `tests/compile_pass/typed_public_construction.rs`. The compile-pass file should construct at least one value from each new family:

- `StyleColor::current_color()`
- `Border::try_new(...)`
- `Outline::try_new(...)`
- `ImageLayerList::try_new(...)`
- `MaskLayerList::try_new(...)`
- `FilterFunctionList::try_new(...)`
- `Translate::try_values(...)`
- `Scale::try_values(...)`
- `UserSelect::Text`

- [ ] **Step 4: Run focused Task 5 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style interaction_properties_accept_typed_values
cargo test -p surgeist-style paint_operation_ten_values_resolve_together
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
pairs = re.findall(r'Value::([A-Za-z0-9_]+)\([^)]*\) => \{\s*\n\s*([0-9]+)u8\.hash', block)
nums = [int(n) for _, n in pairs]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(pairs)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 5: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: integrate paint property coverage"
```

---

### Task 6: Ledger Rebase And Final Operation 10 Integration

**Files:**
- Modify: `src/authored.rs`
- Modify: `src/resolver.rs`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Add authored CSS-wide shorthand coverage**

Add to `src/authored.rs` tests:

```rust
#[test]
fn paint_shorthands_expand_css_wide_keywords_to_canonical_longhands() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Border),
        CssWideKeyword::Unset,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Radius),
        CssWideKeyword::Revert,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Outline),
        CssWideKeyword::Initial,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Mask),
        CssWideKeyword::RevertLayer,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.get(Property::Border), None);
    for property in [
        Property::BorderTopWidth,
        Property::BorderRightWidth,
        Property::BorderBottomWidth,
        Property::BorderLeftWidth,
        Property::BorderTopStyle,
        Property::BorderRightStyle,
        Property::BorderBottomStyle,
        Property::BorderLeftStyle,
        Property::BorderTopColor,
        Property::BorderRightColor,
        Property::BorderBottomColor,
        Property::BorderLeftColor,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
        );
    }

    assert_eq!(canonical.get(Property::Radius), None);
    for property in [
        Property::BorderTopLeftRadius,
        Property::BorderTopRightRadius,
        Property::BorderBottomRightRadius,
        Property::BorderBottomLeftRadius,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Revert))
        );
    }

    assert_eq!(canonical.get(Property::Outline), None);
    for property in [Property::OutlineWidth, Property::OutlineStyle, Property::OutlineColor] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Initial))
        );
    }

    assert_eq!(canonical.get(Property::Mask), None);
    for property in [
        Property::MaskImage,
        Property::MaskPosition,
        Property::MaskSize,
        Property::MaskRepeat,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
        );
    }
}
```

- [ ] **Step 2: Rebase Operation 10 ledger rows**

Update `plans/2026-07-05-css-property-coverage-ledger.md` only for rows affected by Operation 10:

- Change implemented `New style property needed` rows to `Existing style property`.
- Change implemented `New shorthand lowering needed` rows to `Existing style shorthand`.
- Change implemented `Symbolic style data needed` rows to `Existing style property`, with notes that symbolic payloads remain style-owned and unresolved.
- Keep `Background` honest: if only color-only background remains supported, do not claim full background shorthand lowering.
- Keep `Color` honest: concrete RGBA and symbolic colors are style-owned; actual color-space/system-color resolution remains outside style.
- Update family rollup rows for Color, Background, Border and outline, Paint and effects, Transforms, and Interaction.
- Update Next Sequence Context so Operation 11 generated content/counters/lists comes next.

Run ledger consistency:

```sh
python3 - <<'PY'
from pathlib import Path
import re
css = Path('/Users/codex/Development/surgeist-css/src/syntax.rs').read_text()
start = css.index('pub enum CssProperty {')
end = css.index('\n}\n\n#[derive(Clone, Debug, Eq, Hash, PartialEq)]\npub struct CssCustomPropertyName', start)
source = []
for line in css[start:end].splitlines()[1:]:
    line = line.strip()
    if line and not line.startswith('//'):
        source.append(line.rstrip(','))
ledger = Path('plans/2026-07-05-css-property-coverage-ledger.md').read_text()
rows = re.findall(r'^\| `CssProperty::([^`]+)` \|', ledger, flags=re.MULTILINE)
missing = sorted(set(source) - set(rows))
extra = sorted(set(rows) - set(source))
duplicates = sorted({row for row in rows if rows.count(row) > 1})
print(f'source={len(source)} rows={len(rows)} missing={missing} extra={extra} duplicates={duplicates}')
raise SystemExit(0 if len(source) == len(rows) == 180 and not missing and not extra and not duplicates else 1)
PY
```

- [ ] **Step 3: Run full crate checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 4: Commit after worker/reviewer clean**

```sh
git add src/authored.rs src/resolver.rs plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: rebase paint property ledger"
```

---

## Final Verification

After all task commits are complete, run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
python3 - <<'PY'
from pathlib import Path
import re
css = Path('/Users/codex/Development/surgeist-css/src/syntax.rs').read_text()
start = css.index('pub enum CssProperty {')
end = css.index('\n}\n\n#[derive(Clone, Debug, Eq, Hash, PartialEq)]\npub struct CssCustomPropertyName', start)
source = []
for line in css[start:end].splitlines()[1:]:
    line = line.strip()
    if line and not line.startswith('//'):
        source.append(line.rstrip(','))
ledger = Path('plans/2026-07-05-css-property-coverage-ledger.md').read_text()
rows = re.findall(r'^\| `CssProperty::([^`]+)` \|', ledger, flags=re.MULTILINE)
missing = sorted(set(source) - set(rows))
extra = sorted(set(rows) - set(source))
duplicates = sorted({row for row in rows if rows.count(row) > 1})
print(f'source={len(source)} rows={len(rows)} missing={missing} extra={extra} duplicates={duplicates}')
raise SystemExit(0 if len(source) == len(rows) == 180 and not missing and not extra and not duplicates else 1)
PY
```

Expected final state:

- no direct `surgeist-css` or `surgeist-text` dependency,
- clean formatting,
- all tests pass,
- clippy has no warnings,
- ledger still has 180 rows with no missing/extra/duplicate property rows,
- working tree is clean except the branch being ahead by new task commits.

## Final Holistic Review Prompt

Use a clean-context reviewer with this prompt:

```text
You are a final holistic reviewer for surgeist-style Operation 10 paint/color/effects property family expansion. Do not edit files.

Repo: /Users/codex/Development/surgeist-style

Read:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger.md
- plans/2026-07-05-paint-color-effects-property-families-implementation.md
- read-only /Users/codex/Development/surgeist-css/src/syntax.rs

Review the completed implementation against:
- style owns typed paint/color/effect receiving and resolved models;
- symbolic colors, URLs, filters, clip paths, masks, and function payloads remain symbolic;
- `currentColor` is preserved as symbolic style data and not prematurely resolved;
- border, outline, radius, mask, and other shorthands canonicalize into longhands;
- broad legacy keyword placeholders are not left as the only path for Operation 10 properties;
- style does not depend on surgeist-css, surgeist-text, render, image loading, or platform host crates;
- no style-to-render/style-to-css workaround adapter was added;
- symbolic lengths and calc values remain symbolic;
- public APIs have front doors and invalid states are hard to construct;
- generated content/counters/lists remain honestly deferred to Operation 11;
- timing/keyframes remain honestly deferred to Operation 12;
- Operation 10 ledger rows were rebased honestly and Operation 11 remains the next plan;
- implementation follows the Rust modeling guide.

Run:
- cargo fmt --check
- cargo test -p surgeist-style
- cargo clippy -p surgeist-style --all-targets -- -D warnings
- `! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests` passes because the expected state is no direct dependency references.
- git diff --check
- git status --short --branch
- ledger consistency script from the plan

Report findings first with file/line references. If clean, say clean and include commands run.
```

## This Will Come Next

After Operation 10 lands and reviewers are clean, write the next sequential implementation plan for Operation 11: generated content and counter families.

Operation 11 should start from the rebased ledger and cover:

- `content`,
- list style type/position/image/shorthand,
- counter reset/increment/set,
- counter and counters function payloads,
- quote and attr generated-content payloads,
- marker styling policy.

Operation 11 must keep generated-content policy in style while leaving retained tree projection, pseudo-element materialization, text shaping, and render resources outside this crate.
