# Text-Facing Property Families Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Operation 9 from the CSS surface sequence by expanding `surgeist-style` text-facing computed property families with style-owned, type-safe models and canonical shorthand lowering.

**Architecture:** Keep `surgeist-style` independent from `surgeist-css` and `surgeist-text`; root will lower CSS parser syntax into the style-owned APIs introduced here. Text-facing values are authored/resolved-style-facing property data, not font loading, shaping, glyph layout, or text backend contracts. Shorthands such as `font` and `text-decoration` canonicalize into longhands before resolution.

**Tech Stack:** Rust 2024, `surgeist-style`, crate-local unit tests, `trybuild` compile-fail/compile-pass tests, `cargo fmt`, `cargo test -p surgeist-style`, `cargo clippy -p surgeist-style --all-targets -- -D warnings`.

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

- Do not add a `surgeist-css` dependency to `Cargo.toml`.
- Do not add or reintroduce a `surgeist-text` dependency to `Cargo.toml`.
- Do not add a style-to-text adapter in this crate.
- Do not parse CSS syntax in `surgeist-style`.
- Do not add compatibility aliases for the existing keyword placeholders or broad text aggregate contracts.
- Breaking API changes are acceptable.
- Preserve symbolic `Length::Calc` values; this pass must not resolve percentages, calc values, font-relative lengths, or layout bases.
- Keep font loading, font fallback realization, shaping features application, glyph layout, cursor/selection geometry, and text backend capability checks outside this pass.
- Keep `TextDecorationColor` in Operation 10 per the rebased ledger. The `text-decoration` shorthand in this pass lowers only line/style/thickness. Root must reject or defer CSS `text-decoration` values carrying a color until the Operation 10 color plan adds the color property path.
- Worker commits are not allowed. The coordinator commits each clean task after worker/reviewer reconciliation.

## Operation 9 Coverage

The rebased ledger marks these rows for `Operation 9 text-facing properties`:

| Gap | Style result |
| --- | --- |
| `font` | style-owned shorthand lowering to font style, variant, weight, stretch, size, line-height, and family |
| `font-weight` | style-owned `FontWeight` and `FontWeightNumber` |
| `font-style` | style-owned font slant value using existing `TextSlant` behind a property-specific `Value` variant |
| `font-stretch` | style-owned `FontStretch` enum |
| `font-variant` | style-owned `FontVariant` enum |
| `font-feature-settings` | symbolic style-owned `FontFeatureSettings`, `FontFeature`, `FontFeatureTag`, and `FontFeatureValue` |
| `text-align-last` | style-owned `TextAlignLast` enum |
| `text-indent` | style-owned `TextIndent` struct with length, hanging, and each-line flags |
| `vertical-align` | style-owned `VerticalAlign` enum with validated `VerticalAlignLength` variant |
| `letter-spacing` | style-owned `LetterSpacing` enum preserving `normal` and validated `LetterSpacingLength` |
| `text-wrap` | CSS-facing `TextWrap` variants accepted by a dedicated property value |
| `white-space` | CSS-facing `WhiteSpace` variants accepted by a dedicated property value |
| `word-break` | CSS-facing `WordBreak` variants accepted by a dedicated property value |
| `overflow-wrap` | CSS-facing `OverflowWrap` variants accepted by a dedicated property value |
| `text-overflow` | style-owned `TextOverflow` enum |
| `text-decoration` | style-owned shorthand lowering to line, style, and thickness |
| `text-decoration-line` | style-owned `TextDecorationLine` with duplicate prevention |
| `text-decoration-style` | style-owned `TextDecorationStyle` enum |
| `text-decoration-thickness` | style-owned `TextDecorationThickness` enum preserving `auto`, `from-font`, and validated `TextDecorationThicknessLength` |
| `text-transform` | style-owned `TextTransform` enum |

Existing typed rows remain in Operation 9 for verification:

- `font-family` remains `Property::FontFamily` + `Value::FontFamilyList`.
- `font-size` and `line-height` remain `Value::Length`, with CSS-facing length domain validation.
- `text-align` remains `Property::TextAlign` + `Value::TextAlign`.

## File Structure

- `src/value.rs`
  - Add style-owned text/font value models listed in Operation 9 Coverage.
  - Extend `Value` with dedicated variants for the new text/font property values.
  - Update validation, interpolation, and defaults.
  - Keep fields private for values with invariants.
- `src/property.rs`
  - Add new property variants and metadata.
  - Mark `Font` and `TextDecoration` non-canonical shorthands.
  - Replace legacy keyword placeholder acceptance with typed value acceptance.
  - Add domain validation for text length scopes.
- `src/declaration.rs`
  - Add typed front-door builders.
  - Lower `font` and `text-decoration` shorthands to canonical longhands.
  - Hash all new `Value` variants with unique top-level discriminants.
- `src/resolver.rs`
  - Add typed getters for new canonical text/font values.
  - Add smoke tests proving Operation 9 values resolve together.
- `src/authored.rs`
  - Add CSS-wide keyword expansion tests for `font` and `text-decoration` shorthands.
- `src/lib.rs`
  - Reexport new public style-owned text/font types.
- `tests/compile_pass/typed_public_construction.rs`
  - Add public construction examples for new valid front doors.
- `tests/compile_fail/*.rs`
  - Add compile-fail coverage for private newtype/struct fields.
- `plans/2026-07-05-css-property-coverage-ledger.md`
  - Rebase Operation 9 rows after implementation so Operation 10 starts from an honest ledger.

---

### Task 1: Font Longhands And Font Shorthand

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_font_feature_tag_literal.rs`
- Create: `tests/compile_fail/invalid_font_feature_tag_literal.stderr`
- Create: `tests/compile_fail/invalid_font_weight_number_literal.rs`
- Create: `tests/compile_fail/invalid_font_weight_number_literal.stderr`

- [ ] **Step 1: Add failing tests for font value construction and shorthand lowering**

Add this test module content to `src/declaration.rs` under the existing `#[cfg(test)]` module:

```rust
#[test]
fn font_shorthand_lowers_to_canonical_font_longhands() {
    let families = FontFamilyList::new(["Inter", "system-ui"]).unwrap();
    let font = Font::try_new(
        Some(TextSlant::Italic),
        Some(FontVariant::SmallCaps),
        Some(FontWeight::Number(FontWeightNumber::new(650).unwrap())),
        Some(FontStretch::Condensed),
        Length::Px(18.0),
        Some(Length::Percent(125.0)),
        families.clone(),
    )
    .unwrap();

    let declarations = Declarations::new().try_font(font).unwrap();

    assert_eq!(declarations.get(Property::Font), None);
    assert_eq!(
        declarations.get(Property::FontStyle),
        Some(&Value::TextSlant(TextSlant::Italic))
    );
    assert_eq!(
        declarations.get(Property::FontVariant),
        Some(&Value::FontVariant(FontVariant::SmallCaps))
    );
    assert_eq!(
        declarations.get(Property::FontWeight),
        Some(&Value::FontWeight(FontWeight::Number(
            FontWeightNumber::new(650).unwrap()
        )))
    );
    assert_eq!(
        declarations.get(Property::FontStretch),
        Some(&Value::FontStretch(FontStretch::Condensed))
    );
    assert_eq!(
        declarations.get(Property::FontSize),
        Some(&Value::Length(Length::Px(18.0)))
    );
    assert_eq!(
        declarations.get(Property::LineHeight),
        Some(&Value::Length(Length::Percent(125.0)))
    );
    assert_eq!(
        declarations.get(Property::FontFamily),
        Some(&Value::FontFamilyList(families))
    );
}

#[test]
fn font_values_validate_css_facing_domains() {
    assert!(FontWeightNumber::new(1).is_ok());
    assert!(FontWeightNumber::new(1000).is_ok());
    assert!(FontWeightNumber::new(0).is_err());
    assert!(FontWeightNumber::new(1001).is_err());

    assert!(FontFeatureTag::new("kern").is_ok());
    assert!(FontFeatureTag::new("liga").is_ok());
    assert!(FontFeatureTag::new("abc").is_err());
    assert!(FontFeatureTag::new("abcde").is_err());

    let features = FontFeatureSettings::features([FontFeature::new(
        FontFeatureTag::new("kern").unwrap(),
        Some(FontFeatureValue::On),
    )])
    .unwrap();
    assert_eq!(features.len(), 1);
    assert!(FontFeatureSettings::features([]).is_err());
}

#[test]
fn font_shorthand_rejects_invalid_length_domains() {
    let families = FontFamilyList::new(["Inter"]).unwrap();
    let invalid_size = Font::try_new(
        None,
        None,
        None,
        None,
        Length::Auto,
        None,
        families.clone(),
    )
    .unwrap_err();
    assert_eq!(invalid_size.code(), ErrorCode::InvalidValue);

    let invalid_line_height = Font::try_new(
        None,
        None,
        None,
        None,
        Length::Px(16.0),
        Some(Length::Auto),
        families,
    )
    .unwrap_err();
    assert_eq!(invalid_line_height.code(), ErrorCode::InvalidValue);
}
```

Add this resolver test to `src/resolver.rs`:

```rust
#[test]
fn resolved_font_getters_return_typed_values() {
    let features = FontFeatureSettings::features([FontFeature::new(
        FontFeatureTag::new("kern").unwrap(),
        Some(FontFeatureValue::On),
    )])
    .unwrap();

    let style = resolve_single(
        Declarations::new()
            .try_font_family(FontFamilyList::new(["Inter", "serif"]).unwrap())
            .unwrap()
            .try_font_size(Length::Px(17.0))
            .unwrap()
            .try_line_height(Length::Percent(140.0))
            .unwrap()
            .font_weight(FontWeight::Bold)
            .font_style(TextSlant::Oblique(None))
            .font_stretch(FontStretch::Expanded)
            .font_variant(FontVariant::SmallCaps)
            .try_font_feature_settings(features.clone())
            .unwrap(),
    );

    assert_eq!(
        style.font_family().as_slice(),
        &["Inter".to_string(), "serif".to_string()]
    );
    assert_eq!(style.font_size(), Length::Px(17.0));
    assert_eq!(style.line_height(), Length::Percent(140.0));
    assert_eq!(style.font_weight(), FontWeight::Bold);
    assert_eq!(style.font_style(), TextSlant::Oblique(None));
    assert_eq!(style.font_stretch(), FontStretch::Expanded);
    assert_eq!(style.font_variant(), FontVariant::SmallCaps);
    assert_eq!(style.font_feature_settings(), &features);
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run each test separately because Cargo accepts only one test-name filter:

```sh
cargo test -p surgeist-style font_shorthand_lowers_to_canonical_font_longhands
cargo test -p surgeist-style font_values_validate_css_facing_domains
cargo test -p surgeist-style font_shorthand_rejects_invalid_length_domains
cargo test -p surgeist-style resolved_font_getters_return_typed_values
```

Expected before implementation: compile failures for missing `Font`, `FontWeight`, `FontWeightNumber`, `FontStretch`, `FontVariant`, `FontFeatureSettings`, `FontFeature`, `FontFeatureTag`, `FontFeatureValue`, property variants, value variants, front-door methods, and resolver getters.

- [ ] **Step 3: Add font value models**

In `src/value.rs`, add these style-owned models near the existing text value types:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FontWeightNumber(i32);

impl FontWeightNumber {
    pub fn new(value: i32) -> Result<Self> {
        if (1..=1000).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "font weight number must be between 1 and 1000",
            ))
        }
    }

    #[must_use]
    pub const fn normal() -> Self {
        Self(400)
    }

    #[must_use]
    pub const fn bold() -> Self {
        Self(700)
    }

    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}

impl Default for FontWeightNumber {
    fn default() -> Self {
        Self::normal()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
    Bolder,
    Lighter,
    Number(FontWeightNumber),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontStretch {
    #[default]
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontVariant {
    #[default]
    Normal,
    SmallCaps,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FontFeatureTag(String);

impl FontFeatureTag {
    pub fn new(tag: impl Into<String>) -> Result<Self> {
        let tag = tag.into();
        if tag.chars().count() == 4 {
            Ok(Self(tag))
        } else {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "font feature tag must contain exactly four characters",
            ))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FontFeatureValue {
    On,
    Off,
    Integer(i32),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FontFeature {
    tag: FontFeatureTag,
    value: Option<FontFeatureValue>,
}

impl FontFeature {
    #[must_use]
    pub const fn new(tag: FontFeatureTag, value: Option<FontFeatureValue>) -> Self {
        Self { tag, value }
    }

    #[must_use]
    pub const fn tag(&self) -> &FontFeatureTag {
        &self.tag
    }

    #[must_use]
    pub const fn value(&self) -> Option<FontFeatureValue> {
        self.value
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontFeatureSettings {
    #[default]
    Normal,
    Features(Vec<FontFeature>),
}

impl FontFeatureSettings {
    pub fn features(features: impl IntoIterator<Item = FontFeature>) -> Result<Self> {
        let features = features.into_iter().collect::<Vec<_>>();
        if features.is_empty() {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "font feature settings must contain at least one feature",
            ))
        } else {
            Ok(Self::Features(features))
        }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[FontFeature] {
        match self {
            Self::Normal => &[],
            Self::Features(features) => features.as_slice(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Font {
    style: Option<TextSlant>,
    variant: Option<FontVariant>,
    weight: Option<FontWeight>,
    stretch: Option<FontStretch>,
    size: Length,
    line_height: Option<Length>,
    family: FontFamilyList,
}

impl Font {
    pub fn try_new(
        style: Option<TextSlant>,
        variant: Option<FontVariant>,
        weight: Option<FontWeight>,
        stretch: Option<FontStretch>,
        size: Length,
        line_height: Option<Length>,
        family: FontFamilyList,
    ) -> Result<Self> {
        validate_font_size_length(&size)?;
        if let Some(line_height) = &line_height {
            validate_line_height_length(line_height)?;
        }
        family.validate()?;
        if family.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "font shorthand requires at least one font family",
            ));
        }
        let font = Self {
            style,
            variant,
            weight,
            stretch,
            size,
            line_height,
            family,
        };
        font.validate()?;
        Ok(font)
    }

    #[must_use]
    pub const fn style(&self) -> Option<TextSlant> {
        self.style
    }

    #[must_use]
    pub const fn variant(&self) -> Option<FontVariant> {
        self.variant
    }

    #[must_use]
    pub const fn weight(&self) -> Option<FontWeight> {
        self.weight
    }

    #[must_use]
    pub const fn stretch(&self) -> Option<FontStretch> {
        self.stretch
    }

    #[must_use]
    pub const fn size(&self) -> &Length {
        &self.size
    }

    #[must_use]
    pub const fn line_height(&self) -> Option<&Length> {
        self.line_height.as_ref()
    }

    #[must_use]
    pub const fn family(&self) -> &FontFamilyList {
        &self.family
    }

    pub fn validate(&self) -> Result<()> {
        validate_font_size_length(&self.size)?;
        if let Some(style) = self.style {
            validate_slant(style)?;
        }
        if let Some(line_height) = &self.line_height {
            validate_line_height_length(line_height)?;
        }
        self.family.validate()
    }
}
```

Add these helper functions in `src/value.rs` near existing validation helpers:

```rust
fn validate_font_size_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(_) | Length::Percent(_) | Length::Calc(_) => length.validate(),
        Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            "font-size accepts only font-size length values",
        )),
    }
}

fn validate_line_height_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(_) | Length::Percent(_) | Length::Normal | Length::Calc(_) => {
            length.validate()
        }
        Length::Auto | Length::Fill | Length::Fit | Length::MinContent | Length::MaxContent => {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "line-height accepts only line-height length values",
            ))
        }
    }
}
```

- [ ] **Step 4: Extend `Value` and validation**

In `src/value.rs`, add variants to `Value`:

```rust
FontWeight(FontWeight),
TextSlant(TextSlant),
FontStretch(FontStretch),
FontVariant(FontVariant),
FontFeatureSettings(FontFeatureSettings),
Font(Font),
```

Update `Value::interpolation()`:

```rust
Self::FontWeight(_) => Interpolation::Number,
Self::TextSlant(_)
| Self::FontStretch(_)
| Self::FontVariant(_)
| Self::FontFeatureSettings(_)
| Self::Font(_) => Interpolation::Discrete,
```

Update `Value::validate()`:

```rust
Self::FontWeight(_)
| Self::FontStretch(_)
| Self::FontVariant(_)
| Self::FontFeatureSettings(_) => Ok(()),
Self::TextSlant(value) => validate_slant(*value),
Self::Font(value) => value.validate(),
```

- [ ] **Step 5: Add font properties and metadata**

In `src/property.rs`, add `Font` and `FontStretch`, `FontVariant`, `FontFeatureSettings` to `Property`, then add them to `Property::ALL` near the existing font entries:

```rust
Font,
FontFamily,
FontSize,
FontWeight,
FontStyle,
FontStretch,
FontVariant,
FontFeatureSettings,
LineHeight,
```

Update `Property::is_canonical()`:

```rust
Self::Font | Self::TextDecoration | ...
```

Update `metadata()`:

```rust
Self::Font => Metadata::new(Value::Font(Font::try_new(
    None,
    None,
    None,
    None,
    Length::Px(16.0),
    None,
    FontFamilyList::new(["serif"]).unwrap(),
).unwrap()))
.inherited(true)
.impact(Impact::empty().text().layout()),
Self::FontWeight => Metadata::new(Value::FontWeight(FontWeight::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout())
    .interpolation(Interpolation::Number),
Self::FontStyle => Metadata::new(Value::TextSlant(TextSlant::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::FontStretch => Metadata::new(Value::FontStretch(FontStretch::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::FontVariant => Metadata::new(Value::FontVariant(FontVariant::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::FontFeatureSettings => Metadata::new(Value::FontFeatureSettings(
    FontFeatureSettings::default(),
))
.inherited(true)
.impact(Impact::empty().text().layout()),
```

Update `accepts()`:

```rust
Self::Font => matches!(value, Value::Font(_)),
Self::FontWeight => matches!(value, Value::FontWeight(_)),
Self::FontStyle => matches!(value, Value::TextSlant(_)),
Self::FontStretch => matches!(value, Value::FontStretch(_)),
Self::FontVariant => matches!(value, Value::FontVariant(_)),
Self::FontFeatureSettings => matches!(value, Value::FontFeatureSettings(_)),
```

Remove `FontWeight` and `FontStyle` from the legacy-keyword-only rejection arm.

- [ ] **Step 6: Add declaration front doors and shorthand lowering**

In `src/declaration.rs`, add imports for the new types.

Add builder methods:

```rust
pub fn try_font_family(self, family: FontFamilyList) -> Result<Self> {
    self.try_set(Property::FontFamily, Value::FontFamilyList(family))
}

pub fn try_line_height(self, line_height: Length) -> Result<Self> {
    self.try_set(Property::LineHeight, Value::Length(line_height))
}

pub fn font_weight(self, weight: FontWeight) -> Self {
    self.set(Property::FontWeight, Value::FontWeight(weight))
}

pub fn font_style(self, style: TextSlant) -> Self {
    self.set(Property::FontStyle, Value::TextSlant(style))
}

pub fn font_stretch(self, stretch: FontStretch) -> Self {
    self.set(Property::FontStretch, Value::FontStretch(stretch))
}

pub fn font_variant(self, variant: FontVariant) -> Self {
    self.set(Property::FontVariant, Value::FontVariant(variant))
}

pub fn try_font_feature_settings(self, settings: FontFeatureSettings) -> Result<Self> {
    self.try_set(
        Property::FontFeatureSettings,
        Value::FontFeatureSettings(settings),
    )
}

pub fn try_font(self, font: Font) -> Result<Self> {
    self.try_set(Property::Font, Value::Font(font))
}
```

Update `canonical_properties()`:

```rust
Property::Font => &[
    Property::FontStyle,
    Property::FontVariant,
    Property::FontWeight,
    Property::FontStretch,
    Property::FontSize,
    Property::LineHeight,
    Property::FontFamily,
],
```

Update `canonical_declarations()`:

```rust
(Property::Font, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::Font), Value::Keyword(keyword))
}
(Property::Font, Value::Font(font)) => font_declarations(font),
```

Add:

```rust
fn font_declarations(font: Font) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    if let Some(style) = font.style() {
        declarations.push(Declaration::new(Property::FontStyle, Value::TextSlant(style)));
    }
    if let Some(variant) = font.variant() {
        declarations.push(Declaration::new(
            Property::FontVariant,
            Value::FontVariant(variant),
        ));
    }
    if let Some(weight) = font.weight() {
        declarations.push(Declaration::new(
            Property::FontWeight,
            Value::FontWeight(weight),
        ));
    }
    if let Some(stretch) = font.stretch() {
        declarations.push(Declaration::new(
            Property::FontStretch,
            Value::FontStretch(stretch),
        ));
    }
    declarations.push(Declaration::new(
        Property::FontSize,
        Value::Length(font.size().clone()),
    ));
    if let Some(line_height) = font.line_height() {
        declarations.push(Declaration::new(
            Property::LineHeight,
            Value::Length(line_height.clone()),
        ));
    }
    declarations.push(Declaration::new(
        Property::FontFamily,
        Value::FontFamilyList(font.family().clone()),
    ));
    declarations
}
```

- [ ] **Step 7: Add hashing for new value variants**

In `src/declaration.rs`, add unique top-level hash tags after the existing Operation 8 tags:

```rust
Value::FontWeight(value) => {
    50u8.hash(state);
    value.hash(state);
}
Value::TextSlant(value) => {
    51u8.hash(state);
    hash_slant(*value, state);
}
Value::FontStretch(value) => {
    52u8.hash(state);
    value.hash(state);
}
Value::FontVariant(value) => {
    53u8.hash(state);
    value.hash(state);
}
Value::FontFeatureSettings(value) => {
    54u8.hash(state);
    value.hash(state);
}
Value::Font(value) => {
    55u8.hash(state);
    hash_font(value, state);
}
```

Add:

```rust
fn hash_font(value: &Font, state: &mut DefaultHasher) {
    value.style().map(|style| {
        let mut hasher = DefaultHasher::new();
        hash_slant(style, &mut hasher);
        hasher.finish()
    }).hash(state);
    value.variant().hash(state);
    value.weight().hash(state);
    value.stretch().hash(state);
    hash_length(value.size(), state);
    if let Some(line_height) = value.line_height() {
        true.hash(state);
        hash_length(line_height, state);
    } else {
        false.hash(state);
    }
    value.family().hash(state);
}
```

Run the discriminant audit after adding all Task 1 variants:

```sh
python3 - <<'PY'
import pathlib, re
text = pathlib.Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
nums = [int(m.group(1)) for m in re.finditer(r'Value::[^=]+=> \{\s*(\d+)u8\.hash\(state\);', block)]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(nums)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
```

- [ ] **Step 8: Add resolver getters and public reexports**

In `src/resolver.rs`, add:

```rust
#[must_use]
pub fn font_family(&self) -> &FontFamilyList {
    match self.get(Property::FontFamily) {
        Value::FontFamilyList(family) => family,
        _ => unreachable!("resolved font-family stores a font family list"),
    }
}

#[must_use]
pub fn line_height(&self) -> Length {
    match self.get(Property::LineHeight) {
        Value::Length(value) => value.clone(),
        _ => Length::Px(16.0),
    }
}

#[must_use]
pub fn font_weight(&self) -> FontWeight {
    match self.get(Property::FontWeight) {
        Value::FontWeight(value) => *value,
        _ => FontWeight::default(),
    }
}

#[must_use]
pub fn font_style(&self) -> TextSlant {
    match self.get(Property::FontStyle) {
        Value::TextSlant(value) => *value,
        _ => TextSlant::default(),
    }
}

#[must_use]
pub fn font_stretch(&self) -> FontStretch {
    match self.get(Property::FontStretch) {
        Value::FontStretch(value) => *value,
        _ => FontStretch::default(),
    }
}

#[must_use]
pub fn font_variant(&self) -> FontVariant {
    match self.get(Property::FontVariant) {
        Value::FontVariant(value) => *value,
        _ => FontVariant::default(),
    }
}

#[must_use]
pub fn font_feature_settings(&self) -> &FontFeatureSettings {
    match self.get(Property::FontFeatureSettings) {
        Value::FontFeatureSettings(value) => value,
        _ => unreachable!("resolved font-feature-settings stores feature settings"),
    }
}
```

In `src/lib.rs`, reexport:

```rust
Font, FontFeature, FontFeatureSettings, FontFeatureTag, FontFeatureValue,
FontStretch, FontVariant, FontWeight, FontWeightNumber,
```

- [ ] **Step 9: Add type-safety compile tests**

Create `tests/compile_fail/invalid_font_feature_tag_literal.rs`:

```rust
use surgeist_style::FontFeatureTag;

fn main() {
    let _tag = FontFeatureTag(String::from("kern"));
}
```

Create `tests/compile_fail/invalid_font_weight_number_literal.rs`:

```rust
use surgeist_style::FontWeightNumber;

fn main() {
    let _weight = FontWeightNumber(400);
}
```

Update `tests/type_safety.rs` to include these compile-fail files.

Run once to generate the stderr with trybuild:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

Then inspect the generated `.stderr` files and verify they fail because the fields are private, not because imports are missing.

- [ ] **Step 10: Update public construction example**

In `tests/compile_pass/typed_public_construction.rs`, add valid construction of:

```rust
FontFeatureSettings::features([FontFeature::new(
    FontFeatureTag::new("kern")?,
    Some(FontFeatureValue::On),
)])?;
Font::try_new(
    Some(TextSlant::Italic),
    Some(FontVariant::SmallCaps),
    Some(FontWeight::Bold),
    Some(FontStretch::Expanded),
    Length::Px(16.0),
    Some(Length::Percent(120.0)),
    FontFamilyList::new(["Inter", "serif"])?,
)?;
```

Use the corresponding `Declarations` builders so the public front doors compile.

- [ ] **Step 11: Run focused Task 1 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style font_shorthand_lowers_to_canonical_font_longhands
cargo test -p surgeist-style font_values_validate_css_facing_domains
cargo test -p surgeist-style font_shorthand_rejects_invalid_length_domains
cargo test -p surgeist-style resolved_font_getters_return_typed_values
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
import pathlib, re
text = pathlib.Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
nums = [int(m.group(1)) for m in re.finditer(r'Value::[^=]+=> \{\s*(\d+)u8\.hash\(state\);', block)]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(nums)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected:

- focused tests pass,
- type-safety tests pass,
- hash discriminants have no duplicates,
- dependency search has no matches,
- diff check passes,
- status shows only intended Task 1 files.

- [ ] **Step 12: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_font_feature_tag_literal.rs tests/compile_fail/invalid_font_feature_tag_literal.stderr tests/compile_fail/invalid_font_weight_number_literal.rs tests/compile_fail/invalid_font_weight_number_literal.stderr
git commit -m "style: add font property models"
```

---

### Task 2: Inline Text Metrics And Alignment

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_text_indent_literal.rs`
- Create: `tests/compile_fail/invalid_text_indent_literal.stderr`
- Create: `tests/compile_fail/invalid_text_length_wrapper_literal.rs`
- Create: `tests/compile_fail/invalid_text_length_wrapper_literal.stderr`

- [ ] **Step 1: Add failing tests for inline text value models**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn inline_text_properties_accept_typed_values() {
    let declarations = Declarations::new()
        .text_align_last(TextAlignLast::Justify)
        .try_text_indent(TextIndent::new(Length::Percent(12.5), true, false).unwrap())
        .unwrap()
        .try_vertical_align(VerticalAlign::try_length(Length::Px(-2.0)).unwrap())
        .unwrap()
        .try_letter_spacing(LetterSpacing::try_length(Length::Px(1.5)).unwrap())
        .unwrap()
        .text_transform(TextTransform::Uppercase);

    assert_eq!(
        declarations.get(Property::TextAlignLast),
        Some(&Value::TextAlignLast(TextAlignLast::Justify))
    );
    assert_eq!(
        declarations.get(Property::TextIndent),
        Some(&Value::TextIndent(
            TextIndent::new(Length::Percent(12.5), true, false).unwrap()
        ))
    );
    assert_eq!(
        declarations.get(Property::VerticalAlign),
        Some(&Value::VerticalAlign(
            VerticalAlign::try_length(Length::Px(-2.0)).unwrap()
        ))
    );
    assert_eq!(
        declarations.get(Property::LetterSpacing),
        Some(&Value::LetterSpacing(
            LetterSpacing::try_length(Length::Px(1.5)).unwrap()
        ))
    );
    assert_eq!(
        declarations.get(Property::TextTransform),
        Some(&Value::TextTransform(TextTransform::Uppercase))
    );
}

#[test]
fn inline_text_values_validate_length_domains() {
    assert!(TextIndent::new(Length::Auto, false, false).is_err());
    assert!(VerticalAlign::try_length(Length::Auto).is_err());
    assert!(LetterSpacing::try_length(Length::Percent(10.0)).is_err());
    assert!(LetterSpacing::try_length(Length::Normal).is_err());
    assert!(LetterSpacing::try_length(Length::Calc(
        CalcLength::sum(
            CalcLengthTerm::add(CalcLength::percent(50.0)),
            [CalcLengthTerm::add(CalcLength::px(1.0))]
        )
    ))
    .is_err());
}
```

Add to `src/resolver.rs` tests:

```rust
#[test]
fn resolved_inline_text_getters_return_typed_values() {
    let indent = TextIndent::new(Length::Px(8.0), false, true).unwrap();
    let style = resolve_single(
        Declarations::new()
            .text_align_last(TextAlignLast::Center)
            .try_text_indent(indent.clone())
            .unwrap()
            .vertical_align(VerticalAlign::Middle)
            .try_letter_spacing(LetterSpacing::try_length(Length::Px(1.0)).unwrap())
            .unwrap()
            .text_transform(TextTransform::Capitalize),
    );

    assert_eq!(style.text_align_last(), TextAlignLast::Center);
    assert_eq!(style.text_indent(), indent);
    assert_eq!(style.vertical_align(), VerticalAlign::Middle);
    assert_eq!(
        style.letter_spacing(),
        LetterSpacing::try_length(Length::Px(1.0)).unwrap()
    );
    assert_eq!(style.text_transform(), TextTransform::Capitalize);
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style inline_text_properties_accept_typed_values
cargo test -p surgeist-style inline_text_values_validate_length_domains
cargo test -p surgeist-style resolved_inline_text_getters_return_typed_values
```

Expected before implementation: compile failures for missing types, property variants, value variants, methods, and getters.

- [ ] **Step 3: Add inline text value models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextAlignLast {
    #[default]
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextIndent {
    length: Length,
    hanging: bool,
    each_line: bool,
}

impl TextIndent {
    pub fn new(length: Length, hanging: bool, each_line: bool) -> Result<Self> {
        validate_text_length(&length, "text-indent")?;
        Ok(Self {
            length,
            hanging,
            each_line,
        })
    }

    #[must_use]
    pub const fn length(&self) -> &Length {
        &self.length
    }

    #[must_use]
    pub const fn hanging(&self) -> bool {
        self.hanging
    }

    #[must_use]
    pub const fn each_line(&self) -> bool {
        self.each_line
    }
}

impl Default for TextIndent {
    fn default() -> Self {
        Self {
            length: Length::ZERO,
            hanging: false,
            each_line: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum VerticalAlign {
    #[default]
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
    Length(VerticalAlignLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerticalAlignLength(Length);

impl VerticalAlignLength {
    pub fn new(length: Length) -> Result<Self> {
        validate_text_length(&length, "vertical-align")?;
        Ok(Self(length))
    }

    #[must_use]
    pub const fn length(&self) -> &Length {
        &self.0
    }
}

impl VerticalAlign {
    pub fn try_length(length: Length) -> Result<Self> {
        Ok(Self::Length(VerticalAlignLength::new(length)?))
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Length(length) => validate_text_length(length.length(), "vertical-align"),
            Self::Baseline
            | Self::Sub
            | Self::Super
            | Self::TextTop
            | Self::TextBottom
            | Self::Middle
            | Self::Top
            | Self::Bottom => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum LetterSpacing {
    #[default]
    Normal,
    Length(LetterSpacingLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetterSpacingLength(Length);

impl LetterSpacingLength {
    pub fn new(length: Length) -> Result<Self> {
        validate_letter_spacing_length(&length)?;
        Ok(Self(length))
    }

    #[must_use]
    pub const fn length(&self) -> &Length {
        &self.0
    }
}

impl LetterSpacing {
    pub fn try_length(length: Length) -> Result<Self> {
        Ok(Self::Length(LetterSpacingLength::new(length)?))
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Normal => Ok(()),
            Self::Length(length) => validate_letter_spacing_length(length.length()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextTransform {
    #[default]
    None,
    Capitalize,
    Uppercase,
    Lowercase,
}
```

Add validation helpers:

```rust
fn validate_text_length(length: &Length, property_name: &str) -> Result<()> {
    match length {
        Length::Px(_) | Length::Percent(_) | Length::Calc(_) => length.validate(),
        Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property_name} accepts only text length values"),
        )),
    }
}

fn validate_letter_spacing_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(_) => length.validate(),
        Length::Calc(calc) if !calc.uses_percentage() => length.validate(),
        Length::Percent(_)
        | Length::Calc(_)
        | Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            "letter-spacing accepts only non-percentage length values",
        )),
    }
}
```

- [ ] **Step 4: Extend `Value`, properties, builders, and getters**

Add `Value` variants:

```rust
TextAlignLast(TextAlignLast),
TextIndent(TextIndent),
VerticalAlign(VerticalAlign),
LetterSpacing(LetterSpacing),
TextTransform(TextTransform),
```

Update validation:

```rust
Self::TextAlignLast(_) | Self::TextTransform(_) => Ok(()),
Self::TextIndent(value) => validate_text_length(value.length(), "text-indent"),
Self::VerticalAlign(value) => value.validate(),
Self::LetterSpacing(value) => value.validate(),
```

Add `Property` variants near existing text properties:

```rust
TextAlignLast,
TextIndent,
VerticalAlign,
LetterSpacing,
TextTransform,
```

Add metadata:

```rust
Self::TextAlignLast => Metadata::new(Value::TextAlignLast(TextAlignLast::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::TextIndent => Metadata::new(Value::TextIndent(TextIndent::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout())
    .interpolation(Interpolation::Length),
Self::VerticalAlign => Metadata::new(Value::VerticalAlign(VerticalAlign::default()))
    .impact(Impact::empty().text().layout()),
Self::LetterSpacing => Metadata::new(Value::LetterSpacing(LetterSpacing::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout())
    .interpolation(Interpolation::Length),
Self::TextTransform => Metadata::new(Value::TextTransform(TextTransform::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
```

Add `accepts()` arms:

```rust
Self::TextAlignLast => matches!(value, Value::TextAlignLast(_)),
Self::TextIndent => matches!(value, Value::TextIndent(_)),
Self::VerticalAlign => matches!(value, Value::VerticalAlign(_)),
Self::LetterSpacing => matches!(value, Value::LetterSpacing(_)),
Self::TextTransform => matches!(value, Value::TextTransform(_)),
```

In `src/declaration.rs`, add builders:

```rust
pub fn text_align_last(self, value: TextAlignLast) -> Self {
    self.set(Property::TextAlignLast, Value::TextAlignLast(value))
}

pub fn try_text_indent(self, value: TextIndent) -> Result<Self> {
    self.try_set(Property::TextIndent, Value::TextIndent(value))
}

pub fn vertical_align(self, value: VerticalAlign) -> Self {
    self.set(Property::VerticalAlign, Value::VerticalAlign(value))
}

pub fn try_vertical_align(self, value: VerticalAlign) -> Result<Self> {
    self.try_set(Property::VerticalAlign, Value::VerticalAlign(value))
}

pub fn try_letter_spacing(self, value: LetterSpacing) -> Result<Self> {
    self.try_set(Property::LetterSpacing, Value::LetterSpacing(value))
}

pub fn text_transform(self, value: TextTransform) -> Self {
    self.set(Property::TextTransform, Value::TextTransform(value))
}
```

In `src/resolver.rs`, add typed getters matching the tests.

In `src/lib.rs`, reexport:

```rust
LetterSpacing, LetterSpacingLength, TextAlignLast, TextIndent, TextTransform,
VerticalAlign, VerticalAlignLength,
```

- [ ] **Step 5: Add hashing and type-safety coverage**

In `src/declaration.rs`, add unique `hash_value` tags:

```rust
Value::TextAlignLast(value) => {
    56u8.hash(state);
    value.hash(state);
}
Value::TextIndent(value) => {
    57u8.hash(state);
    hash_length(value.length(), state);
    value.hanging().hash(state);
    value.each_line().hash(state);
}
Value::VerticalAlign(value) => {
    58u8.hash(state);
    hash_vertical_align(value, state);
}
Value::LetterSpacing(value) => {
    59u8.hash(state);
    hash_letter_spacing(value, state);
}
Value::TextTransform(value) => {
    60u8.hash(state);
    value.hash(state);
}
```

Create `tests/compile_fail/invalid_text_indent_literal.rs`:

```rust
use surgeist_style::{Length, TextIndent};

fn main() {
    let _indent = TextIndent {
        length: Length::ZERO,
        hanging: false,
        each_line: false,
    };
}
```

Create `tests/compile_fail/invalid_text_length_wrapper_literal.rs`:

```rust
use surgeist_style::{Length, LetterSpacingLength, VerticalAlignLength};

fn main() {
    let _vertical = VerticalAlignLength(Length::Px(1.0));
    let _letter = LetterSpacingLength(Length::Px(1.0));
}
```

Update `tests/type_safety.rs`, run `TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety`, and inspect the stderr.

- [ ] **Step 6: Run focused Task 2 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style inline_text_properties_accept_typed_values
cargo test -p surgeist-style inline_text_values_validate_length_domains
cargo test -p surgeist-style resolved_inline_text_getters_return_typed_values
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
import pathlib, re
text = pathlib.Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
nums = [int(m.group(1)) for m in re.finditer(r'Value::[^=]+=> \{\s*(\d+)u8\.hash\(state\);', block)]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(nums)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_text_indent_literal.rs tests/compile_fail/invalid_text_indent_literal.stderr tests/compile_fail/invalid_text_length_wrapper_literal.rs tests/compile_fail/invalid_text_length_wrapper_literal.stderr
git commit -m "style: add inline text property models"
```

---

### Task 3: Wrapping, Whitespace, Break, And Overflow Values

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add failing tests for wrapping and overflow values**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn text_flow_properties_accept_typed_values() {
    let declarations = Declarations::new()
        .text_wrap(TextWrap::Balance)
        .white_space(WhiteSpace::BreakSpaces)
        .word_break(WordBreak::BreakWord)
        .overflow_wrap(OverflowWrap::Anywhere)
        .text_overflow(TextOverflow::Ellipsis);

    assert_eq!(
        declarations.get(Property::TextWrap),
        Some(&Value::TextWrap(TextWrap::Balance))
    );
    assert_eq!(
        declarations.get(Property::WhiteSpace),
        Some(&Value::WhiteSpace(WhiteSpace::BreakSpaces))
    );
    assert_eq!(
        declarations.get(Property::WordBreak),
        Some(&Value::WordBreak(WordBreak::BreakWord))
    );
    assert_eq!(
        declarations.get(Property::OverflowWrap),
        Some(&Value::OverflowWrap(OverflowWrap::Anywhere))
    );
    assert_eq!(
        declarations.get(Property::TextOverflow),
        Some(&Value::TextOverflow(TextOverflow::Ellipsis))
    );
}
```

Add to `src/resolver.rs` tests:

```rust
#[test]
fn resolved_text_flow_getters_return_typed_values() {
    let style = resolve_single(
        Declarations::new()
            .text_wrap(TextWrap::Pretty)
            .white_space(WhiteSpace::PreWrap)
            .word_break(WordBreak::KeepAll)
            .overflow_wrap(OverflowWrap::BreakWord)
            .text_overflow(TextOverflow::Ellipsis),
    );

    assert_eq!(style.text_wrap(), TextWrap::Pretty);
    assert_eq!(style.white_space(), WhiteSpace::PreWrap);
    assert_eq!(style.word_break(), WordBreak::KeepAll);
    assert_eq!(style.overflow_wrap(), OverflowWrap::BreakWord);
    assert_eq!(style.text_overflow(), TextOverflow::Ellipsis);
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style text_flow_properties_accept_typed_values
cargo test -p surgeist-style resolved_text_flow_getters_return_typed_values
```

Expected before implementation: compile failures for missing value variants, `TextOverflow`, builder methods, and resolver getters.

- [ ] **Step 3: Replace legacy flow enums with CSS-facing style enums**

In `src/value.rs`, update the existing enums:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextWrap {
    #[default]
    Wrap,
    NoWrap,
    Balance,
    Pretty,
    Stable,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WhiteSpace {
    #[default]
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WordBreak {
    #[default]
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum OverflowWrap {
    #[default]
    Normal,
    BreakWord,
    Anywhere,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
}
```

Update `TextValue::default()` expectations to:

```rust
wrap: TextWrap::Wrap,
white_space: WhiteSpace::Normal,
word_break: WordBreak::Normal,
overflow_wrap: OverflowWrap::Normal,
```

Update `text_value_defaults_preserve_style_text_contract` to assert the new defaults.

- [ ] **Step 4: Add dedicated `Value` variants and property acceptance**

Add `Value` variants:

```rust
TextWrap(TextWrap),
WhiteSpace(WhiteSpace),
WordBreak(WordBreak),
OverflowWrap(OverflowWrap),
TextOverflow(TextOverflow),
```

Update `Value::validate()`:

```rust
Self::TextWrap(_)
| Self::WhiteSpace(_)
| Self::WordBreak(_)
| Self::OverflowWrap(_)
| Self::TextOverflow(_) => Ok(()),
```

Update `Value::interpolation()` to treat them as discrete.

In `src/property.rs`, replace legacy keyword metadata for these properties:

```rust
Self::TextWrap => Metadata::new(Value::TextWrap(TextWrap::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::WhiteSpace => Metadata::new(Value::WhiteSpace(WhiteSpace::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::WordBreak => Metadata::new(Value::WordBreak(WordBreak::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::OverflowWrap => Metadata::new(Value::OverflowWrap(OverflowWrap::default()))
    .inherited(true)
    .impact(Impact::empty().text().layout()),
Self::TextOverflow => Metadata::new(Value::TextOverflow(TextOverflow::default()))
    .impact(Impact::empty().text().layout()),
```

Update `accepts()`:

```rust
Self::TextWrap => matches!(value, Value::TextWrap(_)),
Self::WhiteSpace => matches!(value, Value::WhiteSpace(_)),
Self::WordBreak => matches!(value, Value::WordBreak(_)),
Self::OverflowWrap => matches!(value, Value::OverflowWrap(_)),
Self::TextOverflow => matches!(value, Value::TextOverflow(_)),
```

Remove these properties from the legacy-keyword-only rejection arm.

- [ ] **Step 5: Add declaration builders, resolver getters, hashing, and reexports**

In `src/declaration.rs`, add:

```rust
pub fn text_wrap(self, value: TextWrap) -> Self {
    self.set(Property::TextWrap, Value::TextWrap(value))
}

pub fn white_space(self, value: WhiteSpace) -> Self {
    self.set(Property::WhiteSpace, Value::WhiteSpace(value))
}

pub fn word_break(self, value: WordBreak) -> Self {
    self.set(Property::WordBreak, Value::WordBreak(value))
}

pub fn overflow_wrap(self, value: OverflowWrap) -> Self {
    self.set(Property::OverflowWrap, Value::OverflowWrap(value))
}

pub fn text_overflow(self, value: TextOverflow) -> Self {
    self.set(Property::TextOverflow, Value::TextOverflow(value))
}
```

Add `hash_value` tags:

```rust
Value::TextWrap(value) => {
    61u8.hash(state);
    value.hash(state);
}
Value::WhiteSpace(value) => {
    62u8.hash(state);
    value.hash(state);
}
Value::WordBreak(value) => {
    63u8.hash(state);
    value.hash(state);
}
Value::OverflowWrap(value) => {
    64u8.hash(state);
    value.hash(state);
}
Value::TextOverflow(value) => {
    65u8.hash(state);
    value.hash(state);
}
```

In `src/resolver.rs`, add getters matching the tests.

In `src/lib.rs`, reexport `TextOverflow`.

- [ ] **Step 6: Run focused Task 3 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style text_flow_properties_accept_typed_values
cargo test -p surgeist-style resolved_text_flow_getters_return_typed_values
cargo test -p surgeist-style text_value_defaults_preserve_style_text_contract
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
import pathlib, re
text = pathlib.Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
nums = [int(m.group(1)) for m in re.finditer(r'Value::[^=]+=> \{\s*(\d+)u8\.hash\(state\);', block)]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(nums)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: type text flow properties"
```

---

### Task 4: Text Decoration Longhands And Shorthand

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_text_decoration_line_literal.rs`
- Create: `tests/compile_fail/invalid_text_decoration_line_literal.stderr`
- Create: `tests/compile_fail/invalid_text_decoration_thickness_length_literal.rs`
- Create: `tests/compile_fail/invalid_text_decoration_thickness_length_literal.stderr`

- [ ] **Step 1: Add failing tests for text decoration models and lowering**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn text_decoration_shorthand_lowers_to_canonical_longhands() {
    let line = TextDecorationLine::try_new([
        TextDecorationLineComponent::Underline,
        TextDecorationLineComponent::LineThrough,
    ])
    .unwrap();
    let thickness = TextDecorationThickness::try_length(Length::Px(2.0)).unwrap();
    let decoration = TextDecoration::try_new(
        Some(line.clone()),
        Some(TextDecorationStyle::Wavy),
        Some(thickness.clone()),
    )
    .unwrap();

    let declarations = Declarations::new()
        .try_text_decoration(decoration)
        .unwrap();

    assert_eq!(declarations.get(Property::TextDecoration), None);
    assert_eq!(
        declarations.get(Property::TextDecorationLine),
        Some(&Value::TextDecorationLine(line))
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
fn text_decoration_models_validate_domains() {
    assert!(TextDecorationLine::none().is_none());
    assert!(
        TextDecorationLine::try_new([
            TextDecorationLineComponent::Underline,
            TextDecorationLineComponent::Underline,
        ])
        .is_err()
    );
    assert!(TextDecoration::try_new(None, None, None).is_err());
    assert!(TextDecorationThickness::try_length(Length::Px(0.0)).is_ok());
    assert!(TextDecorationThickness::try_length(Length::Percent(10.0)).is_ok());
    assert!(TextDecorationThickness::try_length(Length::Px(-1.0)).is_err());
    assert!(TextDecorationThickness::try_length(Length::Calc(CalcLength::sum(
        CalcLengthTerm::add(CalcLength::px(0.0)),
        [CalcLengthTerm::sub(CalcLength::px(1.0))]
    )))
    .is_err());
    assert!(TextDecorationThickness::try_length(Length::Auto).is_err());
}
```

Add to `src/authored.rs` tests:

```rust
#[test]
fn text_decoration_css_wide_keyword_expands_to_longhands() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::TextDecoration),
        CssWideKeyword::Unset,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();
    assert_eq!(canonical.get(Property::TextDecoration), None);
    assert_eq!(
        canonical.get(Property::TextDecorationLine),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
    );
    assert_eq!(
        canonical.get(Property::TextDecorationStyle),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
    );
    assert_eq!(
        canonical.get(Property::TextDecorationThickness),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
    );
}
```

Add to `src/resolver.rs` tests:

```rust
#[test]
fn resolved_text_decoration_getters_return_typed_values() {
    let line = TextDecorationLine::try_new([TextDecorationLineComponent::Overline]).unwrap();
    let thickness = TextDecorationThickness::FromFont;
    let style = resolve_single(
        Declarations::new()
            .try_text_decoration_line(line.clone())
            .unwrap()
            .text_decoration_style(TextDecorationStyle::Dashed)
            .try_text_decoration_thickness(thickness.clone())
            .unwrap(),
    );

    assert_eq!(style.text_decoration_line(), &line);
    assert_eq!(style.text_decoration_style(), TextDecorationStyle::Dashed);
    assert_eq!(style.text_decoration_thickness(), &thickness);
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style text_decoration_shorthand_lowers_to_canonical_longhands
cargo test -p surgeist-style text_decoration_models_validate_domains
cargo test -p surgeist-style text_decoration_css_wide_keyword_expands_to_longhands
cargo test -p surgeist-style resolved_text_decoration_getters_return_typed_values
```

Expected before implementation: compile failures for missing text decoration types, property variants, value variants, builders, canonical lowering, and getters.

- [ ] **Step 3: Add text decoration value models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextDecorationLineComponent {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextDecorationLine {
    components: Vec<TextDecorationLineComponent>,
    none: bool,
}

impl TextDecorationLine {
    pub fn try_new(
        components: impl IntoIterator<Item = TextDecorationLineComponent>,
    ) -> Result<Self> {
        let components = components.into_iter().collect::<Vec<_>>();
        if components.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "text-decoration-line requires at least one component",
            ));
        }
        if has_duplicate_decoration_line_components(&components) {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "text-decoration-line components must not repeat",
            ));
        }
        Ok(Self {
            components,
            none: false,
        })
    }

    #[must_use]
    pub const fn none() -> Self {
        Self {
            components: Vec::new(),
            none: true,
        }
    }

    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.none
    }

    #[must_use]
    pub fn components(&self) -> &[TextDecorationLineComponent] {
        &self.components
    }
}

impl Default for TextDecorationLine {
    fn default() -> Self {
        Self::none()
    }
}

fn has_duplicate_decoration_line_components(
    components: &[TextDecorationLineComponent],
) -> bool {
    components.iter().enumerate().any(|(index, component)| {
        components
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate == component)
    })
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextDecorationStyle {
    #[default]
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextDecorationThickness {
    #[default]
    Auto,
    FromFont,
    Length(TextDecorationThicknessLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextDecorationThicknessLength(Length);

impl TextDecorationThicknessLength {
    pub fn new(length: Length) -> Result<Self> {
        validate_text_decoration_thickness_length(&length)?;
        Ok(Self(length))
    }

    #[must_use]
    pub const fn length(&self) -> &Length {
        &self.0
    }
}

impl TextDecorationThickness {
    pub fn try_length(length: Length) -> Result<Self> {
        Ok(Self::Length(TextDecorationThicknessLength::new(length)?))
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Auto | Self::FromFont => Ok(()),
            Self::Length(length) => validate_text_decoration_thickness_length(length.length()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextDecoration {
    line: Option<TextDecorationLine>,
    style: Option<TextDecorationStyle>,
    thickness: Option<TextDecorationThickness>,
}

impl TextDecoration {
    pub fn try_new(
        line: Option<TextDecorationLine>,
        style: Option<TextDecorationStyle>,
        thickness: Option<TextDecorationThickness>,
    ) -> Result<Self> {
        if line.is_none() && style.is_none() && thickness.is_none() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "text-decoration shorthand requires at least one component",
            ));
        }
        let value = Self {
            line,
            style,
            thickness,
        };
        value.validate()?;
        Ok(value)
    }

    #[must_use]
    pub const fn line(&self) -> Option<&TextDecorationLine> {
        self.line.as_ref()
    }

    #[must_use]
    pub const fn style(&self) -> Option<TextDecorationStyle> {
        self.style
    }

    #[must_use]
    pub const fn thickness(&self) -> Option<&TextDecorationThickness> {
        self.thickness.as_ref()
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(thickness) = &self.thickness {
            thickness.validate()?;
        }
        Ok(())
    }
}
```

Add:

```rust
fn validate_text_decoration_thickness_length(length: &Length) -> Result<()> {
    match length {
        Length::Px(value) | Length::Percent(value) if *value >= 0.0 => length.validate(),
        Length::Calc(calc) if !calc_is_definitely_negative(calc) => length.validate(),
        Length::Px(_)
        | Length::Percent(_)
        | Length::Calc(_)
        | Length::Auto
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => Err(Error::new(
            ErrorCode::InvalidValue,
            "text-decoration-thickness accepts only non-negative thickness lengths",
        )),
    }
}
```

Add these helpers in `src/value.rs` next to the new text decoration thickness validation helper and call `calc_is_definitely_negative` from `validate_text_decoration_thickness_length`. This mirrors the sign-aware coefficient traversal used by the existing property-domain length validation, so subtraction is not missed:

```rust
use super::CalcOperator;

#[derive(Clone, Copy, Debug, Default)]
struct CalcCoefficients {
    px: f32,
    percent: f32,
}

fn calc_is_definitely_negative(calc: &CalcLength) -> bool {
    calc_coefficients(calc, 1.0).is_some_and(|coefficients| {
        coefficients.px < 0.0 && coefficients.percent <= 0.0
            || coefficients.px <= 0.0 && coefficients.percent < 0.0
    })
}

fn calc_coefficients(calc: &CalcLength, sign: f32) -> Option<CalcCoefficients> {
    match calc {
        CalcLength::Px(value) => Some(CalcCoefficients {
            px: sign * *value,
            percent: 0.0,
        }),
        CalcLength::Percent(value) => Some(CalcCoefficients {
            px: 0.0,
            percent: sign * *value,
        }),
        CalcLength::Sum(terms) => {
            let mut total = CalcCoefficients::default();
            for term in terms {
                let sign = match term.operator() {
                    CalcOperator::Add => sign,
                    CalcOperator::Sub => -sign,
                };
                let coefficients = calc_coefficients(term.value(), sign)?;
                total.px += coefficients.px;
                total.percent += coefficients.percent;
            }
            Some(total)
        }
    }
}
```

- [ ] **Step 4: Add properties, values, builders, lowering, getters, and reexports**

Add `Property` variants:

```rust
TextDecorationLine,
TextDecorationStyle,
TextDecorationThickness,
```

Keep `TextDecoration` and mark it non-canonical.

Add `Value` variants:

```rust
TextDecoration(TextDecoration),
TextDecorationLine(TextDecorationLine),
TextDecorationStyle(TextDecorationStyle),
TextDecorationThickness(TextDecorationThickness),
```

Replace legacy `Property::TextDecoration` keyword-only behavior with:

```rust
Self::TextDecoration => Metadata::new(Value::TextDecoration(
    TextDecoration::try_new(Some(TextDecorationLine::default()), None, None).unwrap(),
))
.inherited(false)
.impact(Impact::empty().text().layout()),
Self::TextDecorationLine => Metadata::new(Value::TextDecorationLine(
    TextDecorationLine::default(),
))
.impact(Impact::empty().text().layout()),
Self::TextDecorationStyle => Metadata::new(Value::TextDecorationStyle(
    TextDecorationStyle::default(),
))
.impact(Impact::empty().text().layout()),
Self::TextDecorationThickness => Metadata::new(Value::TextDecorationThickness(
    TextDecorationThickness::default(),
))
.impact(Impact::empty().text().layout())
.interpolation(Interpolation::Length),
```

Add canonical lowering:

```rust
Property::TextDecoration => &[
    Property::TextDecorationLine,
    Property::TextDecorationStyle,
    Property::TextDecorationThickness,
],
```

```rust
(Property::TextDecoration, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::TextDecoration),
    Value::Keyword(keyword),
),
(Property::TextDecoration, Value::TextDecoration(value)) => {
    text_decoration_declarations(value)
}
```

```rust
fn text_decoration_declarations(value: TextDecoration) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    if let Some(line) = value.line() {
        declarations.push(Declaration::new(
            Property::TextDecorationLine,
            Value::TextDecorationLine(line.clone()),
        ));
    }
    if let Some(style) = value.style() {
        declarations.push(Declaration::new(
            Property::TextDecorationStyle,
            Value::TextDecorationStyle(style),
        ));
    }
    if let Some(thickness) = value.thickness() {
        declarations.push(Declaration::new(
            Property::TextDecorationThickness,
            Value::TextDecorationThickness(thickness.clone()),
        ));
    }
    declarations
}
```

Add declaration builders:

```rust
pub fn try_text_decoration(self, value: TextDecoration) -> Result<Self> {
    self.try_set(Property::TextDecoration, Value::TextDecoration(value))
}

pub fn try_text_decoration_line(self, value: TextDecorationLine) -> Result<Self> {
    self.try_set(
        Property::TextDecorationLine,
        Value::TextDecorationLine(value),
    )
}

pub fn text_decoration_style(self, value: TextDecorationStyle) -> Self {
    self.set(
        Property::TextDecorationStyle,
        Value::TextDecorationStyle(value),
    )
}

pub fn try_text_decoration_thickness(
    self,
    value: TextDecorationThickness,
) -> Result<Self> {
    self.try_set(
        Property::TextDecorationThickness,
        Value::TextDecorationThickness(value),
    )
}
```

Add resolver getters matching the tests.

In `src/lib.rs`, reexport:

```rust
TextDecoration, TextDecorationLine, TextDecorationLineComponent,
TextDecorationStyle, TextDecorationThickness, TextDecorationThicknessLength,
```

- [ ] **Step 5: Add hashing and type-safety coverage**

Add `hash_value` tags:

```rust
Value::TextDecoration(value) => {
    66u8.hash(state);
    hash_text_decoration(value, state);
}
Value::TextDecorationLine(value) => {
    67u8.hash(state);
    value.hash(state);
}
Value::TextDecorationStyle(value) => {
    68u8.hash(state);
    value.hash(state);
}
Value::TextDecorationThickness(value) => {
    69u8.hash(state);
    hash_text_decoration_thickness(value, state);
}
```

Create `tests/compile_fail/invalid_text_decoration_line_literal.rs`:

```rust
use surgeist_style::TextDecorationLine;

fn main() {
    let _line = TextDecorationLine {
        components: Vec::new(),
        none: true,
    };
}
```

Create `tests/compile_fail/invalid_text_decoration_thickness_length_literal.rs`:

```rust
use surgeist_style::{Length, TextDecorationThicknessLength};

fn main() {
    let _thickness = TextDecorationThicknessLength(Length::Px(1.0));
}
```

Update `tests/type_safety.rs`, run `TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety`, and inspect the stderr.

- [ ] **Step 6: Run focused Task 4 checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style text_decoration_shorthand_lowers_to_canonical_longhands
cargo test -p surgeist-style text_decoration_models_validate_domains
cargo test -p surgeist-style text_decoration_css_wide_keyword_expands_to_longhands
cargo test -p surgeist-style resolved_text_decoration_getters_return_typed_values
cargo test -p surgeist-style --test type_safety
python3 - <<'PY'
import pathlib, re
text = pathlib.Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start)
block = text[start:end]
nums = [int(m.group(1)) for m in re.finditer(r'Value::[^=]+=> \{\s*(\d+)u8\.hash\(state\);', block)]
dups = sorted({n for n in nums if nums.count(n) > 1})
print(f'arms={len(nums)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

- [ ] **Step 7: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/authored.rs src/lib.rs tests/type_safety.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_text_decoration_line_literal.rs tests/compile_fail/invalid_text_decoration_line_literal.stderr tests/compile_fail/invalid_text_decoration_thickness_length_literal.rs tests/compile_fail/invalid_text_decoration_thickness_length_literal.stderr
git commit -m "style: lower text decoration properties"
```

---

### Task 5: Integration, Public Surface, And Ledger Rebase

**Files:**
- Modify: `src/authored.rs`
- Modify: `src/resolver.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Add Operation 9 resolved smoke test**

In `src/resolver.rs`, add:

```rust
#[test]
fn text_operation_nine_values_resolve_together() {
    let feature_settings = FontFeatureSettings::features([FontFeature::new(
        FontFeatureTag::new("liga").unwrap(),
        Some(FontFeatureValue::Off),
    )])
    .unwrap();
    let indent = TextIndent::new(Length::Px(10.0), true, true).unwrap();
    let decoration_line =
        TextDecorationLine::try_new([TextDecorationLineComponent::Underline]).unwrap();
    let decoration_thickness =
        TextDecorationThickness::try_length(Length::Percent(12.0)).unwrap();

    let style = resolve_single(
        Declarations::new()
            .try_font_family(FontFamilyList::new(["Inter", "serif"]).unwrap())
            .unwrap()
            .try_font_size(Length::Px(18.0))
            .unwrap()
            .try_line_height(Length::Percent(130.0))
            .unwrap()
            .font_weight(FontWeight::Number(FontWeightNumber::new(625).unwrap()))
            .font_style(TextSlant::Italic)
            .font_stretch(FontStretch::SemiExpanded)
            .font_variant(FontVariant::SmallCaps)
            .try_font_feature_settings(feature_settings.clone())
            .unwrap()
            .text_align_last(TextAlignLast::End)
            .try_text_indent(indent.clone())
            .unwrap()
            .vertical_align(VerticalAlign::TextTop)
            .try_letter_spacing(LetterSpacing::try_length(Length::Px(0.5)).unwrap())
            .unwrap()
            .text_wrap(TextWrap::Stable)
            .white_space(WhiteSpace::PreLine)
            .word_break(WordBreak::BreakAll)
            .overflow_wrap(OverflowWrap::Anywhere)
            .text_overflow(TextOverflow::Ellipsis)
            .try_text_decoration_line(decoration_line.clone())
            .unwrap()
            .text_decoration_style(TextDecorationStyle::Dotted)
            .try_text_decoration_thickness(decoration_thickness.clone())
            .unwrap()
            .text_transform(TextTransform::Lowercase),
    );

    assert_eq!(style.font_size(), Length::Px(18.0));
    assert_eq!(style.line_height(), Length::Percent(130.0));
    assert_eq!(
        style.font_weight(),
        FontWeight::Number(FontWeightNumber::new(625).unwrap())
    );
    assert_eq!(style.font_style(), TextSlant::Italic);
    assert_eq!(style.font_stretch(), FontStretch::SemiExpanded);
    assert_eq!(style.font_variant(), FontVariant::SmallCaps);
    assert_eq!(style.font_feature_settings(), &feature_settings);
    assert_eq!(style.text_align_last(), TextAlignLast::End);
    assert_eq!(style.text_indent(), indent);
    assert_eq!(style.vertical_align(), VerticalAlign::TextTop);
    assert_eq!(
        style.letter_spacing(),
        LetterSpacing::try_length(Length::Px(0.5)).unwrap()
    );
    assert_eq!(style.text_wrap(), TextWrap::Stable);
    assert_eq!(style.white_space(), WhiteSpace::PreLine);
    assert_eq!(style.word_break(), WordBreak::BreakAll);
    assert_eq!(style.overflow_wrap(), OverflowWrap::Anywhere);
    assert_eq!(style.text_overflow(), TextOverflow::Ellipsis);
    assert_eq!(style.text_decoration_line(), &decoration_line);
    assert_eq!(style.text_decoration_style(), TextDecorationStyle::Dotted);
    assert_eq!(style.text_decoration_thickness(), &decoration_thickness);
    assert_eq!(style.text_transform(), TextTransform::Lowercase);
}
```

- [ ] **Step 2: Add authored CSS-wide integration coverage**

In `src/authored.rs`, add:

```rust
#[test]
fn text_shorthands_expand_css_wide_keywords_to_canonical_longhands() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Font),
        CssWideKeyword::RevertLayer,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::TextDecoration),
        CssWideKeyword::Initial,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();
    assert_eq!(canonical.get(Property::Font), None);
    for property in [
        Property::FontStyle,
        Property::FontVariant,
        Property::FontWeight,
        Property::FontStretch,
        Property::FontSize,
        Property::LineHeight,
        Property::FontFamily,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(
                CssWideKeyword::RevertLayer
            ))
        );
    }

    assert_eq!(canonical.get(Property::TextDecoration), None);
    for property in [
        Property::TextDecorationLine,
        Property::TextDecorationStyle,
        Property::TextDecorationThickness,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Initial))
        );
    }
}
```

- [ ] **Step 3: Rebase the property coverage ledger**

Update `plans/2026-07-05-css-property-coverage-ledger.md` only for rows affected by Operation 9:

- Change `Font` from `New shorthand lowering needed` to `Existing style shorthand`.
- Change `FontWeight`, `FontStyle`, `FontStretch`, `FontVariant`, `LetterSpacing`, `TextAlignLast`, `TextIndent`, `VerticalAlign`, `TextWrap`, `WhiteSpace`, `WordBreak`, `OverflowWrap`, `TextOverflow`, `TextDecorationLine`, `TextDecorationStyle`, `TextDecorationThickness`, and `TextTransform` from `New style property needed` to `Existing style property`.
- Change `FontFeatureSettings` from `Symbolic style data needed` to `Existing style property`, with a lowering note that feature tags are preserved as symbolic style-owned data for later shaping.
- Change `TextDecoration` from `New shorthand lowering needed` to `Existing style shorthand`, with a note that Operation 9 lowers line/style/thickness and `TextDecorationColor` remains Operation 10.
- Leave `TextDecorationColor` in Operation 10.
- Update the Family Rollup `Text and font` row so it no longer claims Operation 9 gaps are missing.
- Update Next Sequence Context so Operation 10 paint/color/effects comes next.

Run the ledger consistency check:

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

- [ ] **Step 4: Run full crate checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected:

- formatting passes,
- tests pass,
- clippy passes,
- dependency search has no matches,
- diff check passes,
- status shows only intended Operation 9 files before commit.

- [ ] **Step 5: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/authored.rs src/resolver.rs tests/compile_pass/typed_public_construction.rs plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: integrate text property coverage"
```

---

## Final Verification

After all task commits are complete, run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected final state:

- no direct `surgeist-css` or `surgeist-text` dependency,
- clean formatting,
- all tests pass,
- clippy has no warnings,
- working tree is clean except the branch being ahead by the new task commits.

## Final Holistic Review Prompt

Use a clean-context reviewer with this prompt:

```text
You are a final holistic reviewer for surgeist-style Operation 9 text-facing property family expansion. Do not edit files.

Repo: /Users/codex/Development/surgeist-style

Read:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger.md
- plans/2026-07-05-text-facing-property-families-implementation.md
- read-only /Users/codex/Development/surgeist-css/src/syntax.rs

Review the completed implementation against:
- style owns typed text/font receiving and resolved models;
- font and text-decoration shorthands canonicalize into longhands;
- broad legacy keyword placeholders are not left as the only path for Operation 9 properties;
- font-feature-settings preserve symbolic feature data without invoking shaping;
- style does not depend on surgeist-css or surgeist-text;
- no style-to-text adapter or workaround lowering layer was added;
- symbolic lengths and calc values remain symbolic;
- public APIs have front doors and invalid states are hard to construct;
- TextDecorationColor remains honestly deferred to Operation 10;
- Operation 9 ledger rows were rebased honestly and Operation 10 remains the next plan;
- implementation follows the Rust modeling guide.

Run:
- cargo fmt --check
- cargo test -p surgeist-style
- cargo clippy -p surgeist-style --all-targets -- -D warnings
- rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
- git diff --check
- git status --short --branch

Report findings first with file/line references. If clean, say clean and include commands run.
```

## This Will Come Next

After Operation 9 lands and reviewers are clean, write the next sequential implementation plan for Operation 10: paint/color/effects computed property families.

Operation 10 should start from the rebased ledger and cover:

- symbolic color values and `currentColor`,
- text-decoration color,
- background color/image/position/size/repeat/origin/clip/attachment and background shorthand lowering,
- side-specific border color and style,
- border, border-side, and border-radius expansion,
- CSS outline properties distinct from focus outline,
- box decoration break,
- filters, backdrop filters, clip paths, masks, and individual transform properties,
- interaction values such as `user-select` when the ledger keeps them with paint/effects.

Operation 10 must keep renderer resource loading, backend capabilities, image decoding, path tessellation, and render command generation outside `surgeist-style`. Symbolic paint/effect values should remain typed style data until render-facing crates own realization.
