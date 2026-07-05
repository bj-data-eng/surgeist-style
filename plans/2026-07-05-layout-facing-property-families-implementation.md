# Layout-Facing Property Families Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Operation 8 from the CSS surface sequence by expanding `surgeist-style` layout-facing computed property families with style-owned, type-safe models and canonical lowering.

**Architecture:** Keep `surgeist-style` independent from `surgeist-css`; root will lower CSS parser syntax into the style-owned APIs introduced here. Aggregates such as `margin`, `padding`, `inset`, `border-width`, `place-*`, and `flex` remain style shorthands that canonicalize into longhands before resolution. Semantic numeric layout values get dedicated style-owned newtypes or enums instead of sharing `Value::Number`.

**Tech Stack:** Rust 2024, `surgeist-style`, crate-local unit tests, `trybuild` compile-fail/compile-pass tests, `cargo fmt`, `cargo test -p surgeist-style`, `cargo clippy -p surgeist-style --all-targets -- -D warnings`.

---

## Source Context

- Operation sequence: `plans/2026-07-05-css-surface-style-operations-sequence.md`
- CSS property ledger: `plans/2026-07-05-css-property-coverage-ledger.md`
- Rust modeling guide: `guidance/surgeist-rust-modeling-guide.md`
- Read-only CSS source snapshot: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Current style model files:
  - `src/property.rs`
  - `src/value.rs`
  - `src/declaration.rs`
  - `src/resolver.rs`
  - `src/authored.rs`
  - `src/lib.rs`
  - `tests/type_safety.rs`
  - `tests/compile_pass/typed_public_construction.rs`

## Boundaries

- Do not add a `surgeist-css` dependency to `Cargo.toml`.
- Do not add a style-to-layout adapter in this crate.
- Do not add compatibility aliases for old broad numeric contracts.
- Do not add raw CSS parser types, CSS source locations, or parser dispatch to style.
- Preserve `Length::Calc` and other symbolic length values; this pass must not resolve percentages, calc values, or layout bases.
- Keep generated content, text shaping, paint resources, timing/keyframes, and general cache policy outside this pass.
- Worker commits are not allowed. The coordinator commits each clean task after worker/reviewer reconciliation.

## Operation 8 Coverage

The ledger currently marks 78 CSS properties for `Operation 8 layout-facing properties`.

This plan must make the following ledger gaps style-owned and type-safe:

| Gap | Style result |
| --- | --- |
| `top`, `right`, `bottom`, `left` | canonical inset side longhands |
| `margin-top`, `margin-right`, `margin-bottom`, `margin-left` | canonical margin side longhands |
| `padding-top`, `padding-right`, `padding-bottom`, `padding-left` | canonical padding side longhands |
| `border-top-width`, `border-right-width`, `border-bottom-width`, `border-left-width` | canonical border-width side longhands |
| `position: static/fixed/sticky` | `LayoutPosition` variants |
| `z-index: auto` and integer z-index | style-owned `ZIndex` enum |
| `content-visibility` | style-owned `ContentVisibility` enum |
| `scrollbar-width: auto/thin/none` | style-owned `ScrollbarWidth` enum |
| `order` | style-owned `Order` newtype |
| `flex-grow` and `flex-shrink` | style-owned `FlexFactor` newtype |
| `aspect-ratio` | style-owned `AspectRatio` enum/newtype contract |
| `flex` | style-owned shorthand lowering to grow, shrink, and basis |
| `place-content`, `place-items`, `place-self` | style-owned shorthand lowering to align/justify longhands |
| `justify-tracks`, `align-tracks` | style-owned track-alignment longhands |

## File Structure

- `src/value.rs`
  - Add new layout value types: `ContentVisibility`, `ScrollbarWidth`, `ZIndex`, `Order`, `FlexFactor`, `AspectRatio`, `Flex`, `PlaceContentAlignment`, and `PlaceItemsAlignment`.
  - Extend `LayoutPosition` with `Static`, `Fixed`, and `Sticky`.
  - Extend `Value` with dedicated variants for the new semantic layout values.
  - Keep constructor invariants on newtypes with private fields.
- `src/property.rs`
  - Add new property variants and metadata.
  - Mark aggregate edge properties and new shorthand properties non-canonical.
  - Validate property/value compatibility and domain constraints.
- `src/declaration.rs`
  - Add typed front-door builders.
  - Expand shorthands into canonical longhand declarations.
  - Hash new value variants.
- `src/resolver.rs`
  - Keep `Resolved::new()` populated with every canonical property.
  - Add aggregate edge getters that assemble side longhands.
  - Add typed getters for new layout values.
- `src/authored.rs`
  - Ensure CSS-wide keyword expansion for new shorthand/longhand properties remains canonical and does not write aggregate shorthands into the resolved store.
- `src/lib.rs`
  - Reexport new public style-owned layout types.
- `tests/compile_pass/typed_public_construction.rs`
  - Add public construction examples for new valid front doors.
- `tests/compile_fail/*.rs`
  - Add type-safety failures for private newtype fields.
- `plans/2026-07-05-css-property-coverage-ledger.md`
  - Rebase Operation 8 rows after implementation so Operation 9 starts from an honest ledger.

---

### Task 1: Canonical Edge Side Longhands

**Files:**
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/authored.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add failing tests for edge shorthand canonicalization**

Add unit tests to `src/declaration.rs` under the existing `#[cfg(test)]` module:

```rust
#[test]
fn edge_shorthands_lower_to_side_longhands() {
    let edges = Edges::new(
        Length::Px(1.0),
        Length::Px(2.0),
        Length::Px(3.0),
        Length::Px(4.0),
    );

    let declarations = Declarations::new().try_margin(edges.clone()).unwrap();
    assert_eq!(declarations.get(Property::Margin), None);
    assert_eq!(declarations.get(Property::MarginTop), Some(&Value::Length(edges.top.clone())));
    assert_eq!(declarations.get(Property::MarginRight), Some(&Value::Length(edges.right.clone())));
    assert_eq!(declarations.get(Property::MarginBottom), Some(&Value::Length(edges.bottom.clone())));
    assert_eq!(declarations.get(Property::MarginLeft), Some(&Value::Length(edges.left.clone())));

    let declarations = Declarations::new().try_padding(edges.clone()).unwrap();
    assert_eq!(declarations.get(Property::Padding), None);
    assert_eq!(declarations.get(Property::PaddingTop), Some(&Value::Length(edges.top.clone())));
    assert_eq!(declarations.get(Property::PaddingRight), Some(&Value::Length(edges.right.clone())));
    assert_eq!(declarations.get(Property::PaddingBottom), Some(&Value::Length(edges.bottom.clone())));
    assert_eq!(declarations.get(Property::PaddingLeft), Some(&Value::Length(edges.left.clone())));

    let declarations = Declarations::new().try_border_width(edges.clone()).unwrap();
    assert_eq!(declarations.get(Property::BorderWidth), None);
    assert_eq!(declarations.get(Property::BorderTopWidth), Some(&Value::Length(edges.top.clone())));
    assert_eq!(declarations.get(Property::BorderRightWidth), Some(&Value::Length(edges.right.clone())));
    assert_eq!(declarations.get(Property::BorderBottomWidth), Some(&Value::Length(edges.bottom.clone())));
    assert_eq!(declarations.get(Property::BorderLeftWidth), Some(&Value::Length(edges.left.clone())));

    let declarations = Declarations::new().try_inset(edges.clone()).unwrap();
    assert_eq!(declarations.get(Property::Inset), None);
    assert_eq!(declarations.get(Property::Top), Some(&Value::Length(edges.top)));
    assert_eq!(declarations.get(Property::Right), Some(&Value::Length(edges.right)));
    assert_eq!(declarations.get(Property::Bottom), Some(&Value::Length(edges.bottom)));
    assert_eq!(declarations.get(Property::Left), Some(&Value::Length(edges.left)));
}
```

Add resolver tests to `src/resolver.rs`:

```rust
#[test]
fn resolved_edge_getters_assemble_side_longhands() {
    let style = resolve_single(
        Declarations::new()
            .try_margin_top(Length::Px(2.0)).unwrap()
            .try_margin_right(Length::Px(4.0)).unwrap()
            .try_padding_bottom(Length::Px(6.0)).unwrap()
            .try_border_left_width(Length::Px(8.0)).unwrap(),
    );

    assert_eq!(style.margin_edges().top, Length::Px(2.0));
    assert_eq!(style.margin_edges().right, Length::Px(4.0));
    assert_eq!(style.margin_edges().bottom, Length::ZERO);
    assert_eq!(style.margin_edges().left, Length::ZERO);

    assert_eq!(style.padding_edges().top, Length::ZERO);
    assert_eq!(style.padding_edges().bottom, Length::Px(6.0));

    assert_eq!(style.border_width_edges().left, Length::Px(8.0));
    assert_eq!(style.inset_edges().top, Length::Auto);
}
```

Use the local resolver test helper already present in `src/resolver.rs`; if no helper accepts raw declarations, add a crate-private test helper in the test module only:

```rust
fn resolve_single(declarations: Declarations) -> Resolved {
    resolve_child(
        Sheet::new().rule(Selector::tag("button").unwrap(), declarations),
        None,
    )
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style edge_shorthands_lower_to_side_longhands resolved_edge_getters_assemble_side_longhands
```

Expected before implementation: compile failures for missing `Property` variants and missing `Declarations`/`Resolved` methods.

- [ ] **Step 3: Add side longhand properties**

Update `src/property.rs`:

```rust
pub enum Property {
    Display,
    BoxSizing,
    Position,
    Inset,
    Top,
    Right,
    Bottom,
    Left,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MinSize,
    MaxWidth,
    MaxHeight,
    MaxSize,
    AspectRatio,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Overflow,
    OverflowX,
    OverflowY,
    ScrollbarWidth,
    ZIndex,
    // existing variants continue here
    BorderWidth,
    BorderTopWidth,
    BorderRightWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    // existing variants continue here
}
```

Insert every new variant into `Property::ALL` in the same order as the enum.

Update `Property::is_canonical()` so these aggregate properties are shorthands:

```rust
Self::Inset
    | Self::Margin
    | Self::Padding
    | Self::BorderWidth
    | Self::Gap
    | Self::MinSize
    | Self::MaxSize
    | Self::Overflow
    | Self::Align
    | Self::Justify
    | Self::GridTemplate
    | Self::Grid
    | Self::GridRow
    | Self::GridColumn
    | Self::GridArea
```

- [ ] **Step 4: Add metadata and validation for edge side longhands**

In `Property::metadata()`, add side defaults:

```rust
Self::Top | Self::Right | Self::Bottom | Self::Left => {
    Metadata::new(Value::Length(Length::Auto))
        .impact(Impact::empty().layout())
        .interpolation(Interpolation::Length)
}
Self::MarginTop | Self::MarginRight | Self::MarginBottom | Self::MarginLeft => {
    Metadata::new(Value::Length(Length::ZERO))
        .impact(Impact::empty().layout())
        .interpolation(Interpolation::Length)
}
Self::PaddingTop | Self::PaddingRight | Self::PaddingBottom | Self::PaddingLeft => {
    Metadata::new(Value::Length(Length::ZERO))
        .impact(Impact::empty().layout())
        .interpolation(Interpolation::Length)
}
Self::BorderTopWidth
| Self::BorderRightWidth
| Self::BorderBottomWidth
| Self::BorderLeftWidth => Metadata::new(Value::Length(Length::ZERO))
    .impact(Impact::empty().layout().paint())
    .interpolation(Interpolation::Length),
```

In `Property::accepts()`, make side longhands accept only `Value::Length(_)`.

In `Property::validate_domain()`, enforce:

```rust
(
    Self::PaddingTop
    | Self::PaddingRight
    | Self::PaddingBottom
    | Self::PaddingLeft
    | Self::BorderTopWidth
    | Self::BorderRightWidth
    | Self::BorderBottomWidth
    | Self::BorderLeftWidth,
    Value::Length(length),
) => {
    validate_normal_length_scope(length, self)?;
    validate_auto_length_scope(length, self)?;
    validate_non_negative_length(length, self)
}
(
    Self::MarginTop | Self::MarginRight | Self::MarginBottom | Self::MarginLeft,
    Value::Length(length),
) => validate_normal_length_scope(length, self),
(
    Self::Top | Self::Right | Self::Bottom | Self::Left,
    Value::Length(length),
) => validate_normal_length_scope(length, self),
```

Add a helper beside `validate_normal_length_scope()`:

```rust
fn validate_auto_length_scope(length: &Length, property: Property) -> Result<()> {
    if matches!(length, Length::Auto)
        && matches!(
            property,
            Property::PaddingTop
                | Property::PaddingRight
                | Property::PaddingBottom
                | Property::PaddingLeft
                | Property::BorderTopWidth
                | Property::BorderRightWidth
                | Property::BorderBottomWidth
                | Property::BorderLeftWidth
        )
    {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} does not accept auto length"),
        ));
    }
    Ok(())
}
```

- [ ] **Step 5: Lower aggregate edges into side declarations**

In `src/declaration.rs`, update `canonical_properties()`:

```rust
Property::Inset => vec![Property::Top, Property::Right, Property::Bottom, Property::Left],
Property::Margin => vec![
    Property::MarginTop,
    Property::MarginRight,
    Property::MarginBottom,
    Property::MarginLeft,
],
Property::Padding => vec![
    Property::PaddingTop,
    Property::PaddingRight,
    Property::PaddingBottom,
    Property::PaddingLeft,
],
Property::BorderWidth => vec![
    Property::BorderTopWidth,
    Property::BorderRightWidth,
    Property::BorderBottomWidth,
    Property::BorderLeftWidth,
],
```

In `canonical_declarations()`, add edge expansion before the final fallback:

```rust
(Property::Inset, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::Inset), Value::Keyword(keyword))
}
(Property::Inset, Value::Edges(edges)) => edge_declarations(
    edges,
    Property::Top,
    Property::Right,
    Property::Bottom,
    Property::Left,
),
(Property::Margin, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::Margin), Value::Keyword(keyword))
}
(Property::Margin, Value::Edges(edges)) => edge_declarations(
    edges,
    Property::MarginTop,
    Property::MarginRight,
    Property::MarginBottom,
    Property::MarginLeft,
),
(Property::Padding, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::Padding), Value::Keyword(keyword))
}
(Property::Padding, Value::Edges(edges)) => edge_declarations(
    edges,
    Property::PaddingTop,
    Property::PaddingRight,
    Property::PaddingBottom,
    Property::PaddingLeft,
),
(Property::BorderWidth, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::BorderWidth), Value::Keyword(keyword))
}
(Property::BorderWidth, Value::Edges(edges)) => edge_declarations(
    edges,
    Property::BorderTopWidth,
    Property::BorderRightWidth,
    Property::BorderBottomWidth,
    Property::BorderLeftWidth,
),
```

Add the helper:

```rust
fn edge_declarations(
    edges: Edges,
    top: Property,
    right: Property,
    bottom: Property,
    left: Property,
) -> Vec<Declaration> {
    vec![
        Declaration::new(top, Value::Length(edges.top)),
        Declaration::new(right, Value::Length(edges.right)),
        Declaration::new(bottom, Value::Length(edges.bottom)),
        Declaration::new(left, Value::Length(edges.left)),
    ]
}
```

- [ ] **Step 6: Validate canonical longhand output before storage**

Update `Declarations::try_insert()` in `src/declaration.rs` so shorthand inputs cannot bypass longhand domain rules:

```rust
pub fn try_insert(&mut self, property: Property, value: Value) -> Result<&mut Self> {
    property.validate_value(&value)?;
    let declarations = canonical_declarations(property, value);
    for declaration in &declarations {
        declaration.property.validate_value(&declaration.value)?;
    }
    self.insert_validated(declarations);
    Ok(self)
}
```

Rename the private insertion helper so the validation boundary is explicit:

```rust
fn insert(&mut self, property: Property, value: Value) -> &mut Self {
    self.insert_validated(canonical_declarations(property, value));
    self
}

fn insert_validated(&mut self, declarations: Vec<Declaration>) -> &mut Self {
    for declaration in declarations {
        self.insert_canonical(declaration.property, declaration.value);
    }
    self
}
```

Keep `insert` private. Public callers that can mix arbitrary `Property` and `Value` must use `try_insert` or `try_set`; property/value mismatches remain runtime validation failures at this broad front door.

Add unit tests to `src/declaration.rs`:

```rust
#[test]
fn edge_shorthands_validate_canonical_longhand_domains() {
    assert!(
        Declarations::new()
            .try_padding(Edges::all(Length::Auto))
            .is_err()
    );
    assert!(
        Declarations::new()
            .try_border_width(Edges::all(Length::Normal))
            .is_err()
    );
    assert!(
        Declarations::new()
            .try_margin(Edges::all(Length::Normal))
            .is_err()
    );
    assert!(
        Declarations::new()
            .try_set(Property::PaddingTop, Value::Color(Color::BLACK))
            .is_err()
    );
}
```

- [ ] **Step 7: Add edge side front doors and aggregate getters**

In `Declarations`, add:

```rust
pub fn try_inset(self, edges: Edges) -> Result<Self> {
    self.try_set(Property::Inset, Value::Edges(edges))
}

pub fn try_inset_top(self, value: Length) -> Result<Self> {
    self.try_set(Property::Top, Value::Length(value))
}

pub fn try_inset_right(self, value: Length) -> Result<Self> {
    self.try_set(Property::Right, Value::Length(value))
}

pub fn try_inset_bottom(self, value: Length) -> Result<Self> {
    self.try_set(Property::Bottom, Value::Length(value))
}

pub fn try_inset_left(self, value: Length) -> Result<Self> {
    self.try_set(Property::Left, Value::Length(value))
}

pub fn try_margin_top(self, value: Length) -> Result<Self> {
    self.try_set(Property::MarginTop, Value::Length(value))
}

pub fn try_margin_right(self, value: Length) -> Result<Self> {
    self.try_set(Property::MarginRight, Value::Length(value))
}

pub fn try_margin_bottom(self, value: Length) -> Result<Self> {
    self.try_set(Property::MarginBottom, Value::Length(value))
}

pub fn try_margin_left(self, value: Length) -> Result<Self> {
    self.try_set(Property::MarginLeft, Value::Length(value))
}

pub fn try_padding_top(self, value: Length) -> Result<Self> {
    self.try_set(Property::PaddingTop, Value::Length(value))
}

pub fn try_padding_right(self, value: Length) -> Result<Self> {
    self.try_set(Property::PaddingRight, Value::Length(value))
}

pub fn try_padding_bottom(self, value: Length) -> Result<Self> {
    self.try_set(Property::PaddingBottom, Value::Length(value))
}

pub fn try_padding_left(self, value: Length) -> Result<Self> {
    self.try_set(Property::PaddingLeft, Value::Length(value))
}

pub fn try_border_top_width(self, value: Length) -> Result<Self> {
    self.try_set(Property::BorderTopWidth, Value::Length(value))
}

pub fn try_border_right_width(self, value: Length) -> Result<Self> {
    self.try_set(Property::BorderRightWidth, Value::Length(value))
}

pub fn try_border_bottom_width(self, value: Length) -> Result<Self> {
    self.try_set(Property::BorderBottomWidth, Value::Length(value))
}

pub fn try_border_left_width(self, value: Length) -> Result<Self> {
    self.try_set(Property::BorderLeftWidth, Value::Length(value))
}
```

In `Resolved`, assemble aggregates from canonical sides:

```rust
pub fn inset_edges(&self) -> Edges {
    Edges::new(
        self.length_or(Property::Top, Length::Auto),
        self.length_or(Property::Right, Length::Auto),
        self.length_or(Property::Bottom, Length::Auto),
        self.length_or(Property::Left, Length::Auto),
    )
}

pub fn margin_edges(&self) -> Edges {
    Edges::new(
        self.length_or(Property::MarginTop, Length::ZERO),
        self.length_or(Property::MarginRight, Length::ZERO),
        self.length_or(Property::MarginBottom, Length::ZERO),
        self.length_or(Property::MarginLeft, Length::ZERO),
    )
}

pub fn padding_edges(&self) -> Edges {
    Edges::new(
        self.length_or(Property::PaddingTop, Length::ZERO),
        self.length_or(Property::PaddingRight, Length::ZERO),
        self.length_or(Property::PaddingBottom, Length::ZERO),
        self.length_or(Property::PaddingLeft, Length::ZERO),
    )
}

pub fn border_width_edges(&self) -> Edges {
    Edges::new(
        self.length_or(Property::BorderTopWidth, Length::ZERO),
        self.length_or(Property::BorderRightWidth, Length::ZERO),
        self.length_or(Property::BorderBottomWidth, Length::ZERO),
        self.length_or(Property::BorderLeftWidth, Length::ZERO),
    )
}

fn length_or(&self, property: Property, fallback: Length) -> Length {
    match self.get(property) {
        Value::Length(value) => value.clone(),
        _ => fallback,
    }
}
```

- [ ] **Step 8: Add compile-pass and runtime validation coverage**

Add to `tests/compile_pass/typed_public_construction.rs`:

```rust
let declarations = Declarations::new()
    .try_inset_top(Length::Auto)?
    .try_margin_left(Length::Px(-4.0))?
    .try_padding_right(Length::Px(8.0))?
    .try_border_bottom_width(Length::Px(2.0))?;
assert_eq!(declarations.len(), 4);
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
cargo test -p surgeist-style edge_shorthands_lower_to_side_longhands edge_shorthands_validate_canonical_longhand_domains resolved_edge_getters_assemble_side_longhands
```

No new compile-fail property/value mismatch fixture is needed for side longhands because the public `try_set(Property, Value)` API intentionally validates broad property/value pairs at runtime. The compile-pass fixture proves the typed front doors; the unit test proves broad mismatches are rejected.

- [ ] **Step 9: Commit after worker/reviewer clean**

Coordinator runs:

```sh
cargo fmt --check
cargo test -p surgeist-style edge_shorthands_lower_to_side_longhands edge_shorthands_validate_canonical_longhand_domains resolved_edge_getters_assemble_side_longhands
cargo test -p surgeist-style --test type_safety
git diff --check
git status --short --branch
```

After a scoped reviewer is clean, commit:

```sh
git add src/property.rs src/declaration.rs src/resolver.rs src/authored.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: add layout edge longhands"
```

---

### Task 2: Type-Safe Core Layout Values

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_layout_value_newtype_literal.rs`
- Create: `tests/compile_fail/invalid_layout_value_newtype_literal.stderr`

- [ ] **Step 1: Add failing tests for typed layout values**

Add unit tests to `src/property.rs`:

```rust
#[test]
fn core_layout_properties_accept_typed_values() {
    Property::Position
        .validate_value(&Value::Position(LayoutPosition::Fixed))
        .unwrap();
    Property::ZIndex
        .validate_value(&Value::ZIndex(ZIndex::Auto))
        .unwrap();
    Property::ZIndex
        .validate_value(&Value::ZIndex(ZIndex::integer(-2)))
        .unwrap();
    Property::ScrollbarWidth
        .validate_value(&Value::ScrollbarWidth(ScrollbarWidth::Thin))
        .unwrap();
    Property::ContentVisibility
        .validate_value(&Value::ContentVisibility(ContentVisibility::Auto))
        .unwrap();
    Property::AspectRatio
        .validate_value(&Value::AspectRatio(AspectRatio::ratio(16.0 / 9.0).unwrap()))
        .unwrap();
    Property::Order
        .validate_value(&Value::Order(Order::new(-2)))
        .unwrap();
    Property::FlexGrow
        .validate_value(&Value::FlexFactor(FlexFactor::new(2.0).unwrap()))
        .unwrap();
}

#[test]
fn semantic_numbers_are_not_interchangeable() {
    assert!(Property::ZIndex.validate_value(&Value::Number(1.0)).is_err());
    assert!(Property::AspectRatio.validate_value(&Value::Number(1.0)).is_err());
    assert!(Property::FlexGrow.validate_value(&Value::Number(1.0)).is_err());
    assert!(Property::Order.validate_value(&Value::Number(1.0)).is_err());
    assert!(Property::ScrollbarWidth.validate_value(&Value::Number(1.0)).is_err());
}
```

Add resolver tests to `src/resolver.rs`:

```rust
#[test]
fn resolved_core_layout_getters_return_typed_values() {
    let style = resolve_single(
        Declarations::new()
            .position(LayoutPosition::Sticky)
            .z_index(ZIndex::integer(3))
            .scrollbar_width(ScrollbarWidth::None)
            .content_visibility(ContentVisibility::Hidden)
            .order(Order::new(-1))
            .try_aspect_ratio(AspectRatio::ratio(2.0).unwrap()).unwrap(),
    );

    assert_eq!(style.position(), LayoutPosition::Sticky);
    assert_eq!(style.z_index(), ZIndex::integer(3));
    assert_eq!(style.scrollbar_width(), ScrollbarWidth::None);
    assert_eq!(style.content_visibility(), ContentVisibility::Hidden);
    assert_eq!(style.order(), Order::new(-1));
    assert_eq!(style.aspect_ratio(), AspectRatio::ratio(2.0).unwrap());
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style core_layout_properties_accept_typed_values semantic_numbers_are_not_interchangeable resolved_core_layout_getters_return_typed_values
```

Expected before implementation: missing types, missing `Value` variants, and missing methods.

- [ ] **Step 3: Add style-owned value types**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ContentVisibility {
    #[default]
    Visible,
    Hidden,
    Auto,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScrollbarWidth {
    #[default]
    Auto,
    Thin,
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ZIndex {
    #[default]
    Auto,
    Integer(i32),
}

impl ZIndex {
    #[must_use]
    pub const fn integer(value: i32) -> Self {
        Self::Integer(value)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Order(i32);

impl Order {
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexFactor(f32);

impl FlexFactor {
    pub fn new(value: f32) -> Result<Self> {
        validate_non_negative(value, "flex factor")?;
        Ok(Self(value))
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    #[must_use]
    pub const fn one() -> Self {
        Self(1.0)
    }

    #[must_use]
    pub const fn get(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AspectRatio(AspectRatioKind);

#[derive(Clone, Copy, Debug, PartialEq)]
enum AspectRatioKind {
    Auto,
    Ratio(f32),
}

impl AspectRatio {
    pub const AUTO: Self = Self(AspectRatioKind::Auto);

    pub fn ratio(value: f32) -> Result<Self> {
        validate_finite(value, "aspect ratio")?;
        if value <= 0.0 {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "aspect ratio must be positive",
            ));
        }
        Ok(Self(AspectRatioKind::Ratio(value)))
    }

    #[must_use]
    pub const fn as_ratio(self) -> Option<f32> {
        match self.0 {
            AspectRatioKind::Auto => None,
            AspectRatioKind::Ratio(value) => Some(value),
        }
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::AUTO
    }
}
```

Extend `LayoutPosition`:

```rust
pub enum LayoutPosition {
    Static,
    #[default]
    Relative,
    Absolute,
    Fixed,
    Sticky,
}
```

The default remains `Relative` in this pass to avoid mixing explicit CSS property support with a global default-policy migration.

- [ ] **Step 4: Add dedicated `Value` variants**

Update `Value`:

```rust
ZIndex(ZIndex),
ScrollbarWidth(ScrollbarWidth),
ContentVisibility(ContentVisibility),
Order(Order),
FlexFactor(FlexFactor),
AspectRatio(AspectRatio),
```

Update `Value::interpolation()`:

```rust
Self::FlexFactor(_) | Self::AspectRatio(_) => Interpolation::Number,
Self::ZIndex(_)
| Self::ScrollbarWidth(_)
| Self::ContentVisibility(_)
| Self::Order(_) => Interpolation::Discrete,
```

Update `Value::validate()`:

```rust
Self::FlexFactor(value) => value.validate(),
Self::AspectRatio(value) => value.validate(),
Self::ZIndex(_)
| Self::ScrollbarWidth(_)
| Self::ContentVisibility(_)
| Self::Order(_) => Ok(()),
```

Add methods:

```rust
impl FlexFactor {
    pub fn validate(self) -> Result<()> {
        validate_non_negative(self.0, "flex factor")
    }
}

impl AspectRatio {
    pub fn validate(self) -> Result<()> {
        match self.as_ratio() {
            Some(value) => Self::ratio(value).map(|_| ()),
            None => Ok(()),
        }
    }
}
```

- [ ] **Step 5: Add new properties and typed metadata**

In `src/property.rs`, add `ContentVisibility` and `Order` to `Property`, `Property::ALL`, and `Property::is_canonical()`.

Update metadata:

```rust
Self::ZIndex => Metadata::new(Value::ZIndex(ZIndex::default()))
    .impact(Impact::empty().layout().paint()),
Self::ScrollbarWidth => Metadata::new(Value::ScrollbarWidth(ScrollbarWidth::default()))
    .impact(Impact::empty().layout()),
Self::ContentVisibility => Metadata::new(Value::ContentVisibility(ContentVisibility::default()))
    .impact(Impact::empty().layout().paint()),
Self::Order => Metadata::new(Value::Order(Order::default()))
    .impact(Impact::empty().layout()),
Self::FlexGrow => Metadata::new(Value::FlexFactor(FlexFactor::zero()))
    .impact(Impact::empty().layout())
    .interpolation(Interpolation::Number),
Self::FlexShrink => Metadata::new(Value::FlexFactor(FlexFactor::one()))
    .impact(Impact::empty().layout())
    .interpolation(Interpolation::Number),
Self::AspectRatio => Metadata::new(Value::AspectRatio(AspectRatio::default()))
    .impact(Impact::empty().layout())
    .interpolation(Interpolation::Number),
```

Update `Property::accepts()`:

```rust
Self::ZIndex => matches!(value, Value::ZIndex(_)),
Self::ScrollbarWidth => matches!(value, Value::ScrollbarWidth(_)),
Self::ContentVisibility => matches!(value, Value::ContentVisibility(_)),
Self::Order => matches!(value, Value::Order(_)),
Self::FlexGrow | Self::FlexShrink => matches!(value, Value::FlexFactor(_)),
Self::AspectRatio => matches!(value, Value::AspectRatio(_)),
```

Remove these properties from the old `Value::Number(_)` acceptance arm. Keep `Value::Number` for opacity and transition durations until the timing pass replaces those contracts.

- [ ] **Step 6: Add declaration front doors and resolved getters**

In `Declarations`, add:

```rust
#[must_use]
pub fn position(self, position: LayoutPosition) -> Self {
    self.set(Property::Position, Value::Position(position))
}

#[must_use]
pub fn z_index(self, z_index: ZIndex) -> Self {
    self.set(Property::ZIndex, Value::ZIndex(z_index))
}

#[must_use]
pub fn scrollbar_width(self, value: ScrollbarWidth) -> Self {
    self.set(Property::ScrollbarWidth, Value::ScrollbarWidth(value))
}

#[must_use]
pub fn content_visibility(self, value: ContentVisibility) -> Self {
    self.set(Property::ContentVisibility, Value::ContentVisibility(value))
}

#[must_use]
pub fn order(self, order: Order) -> Self {
    self.set(Property::Order, Value::Order(order))
}

pub fn try_flex_grow(self, value: FlexFactor) -> Result<Self> {
    self.try_set(Property::FlexGrow, Value::FlexFactor(value))
}

pub fn try_flex_shrink(self, value: FlexFactor) -> Result<Self> {
    self.try_set(Property::FlexShrink, Value::FlexFactor(value))
}

pub fn try_aspect_ratio(self, value: AspectRatio) -> Result<Self> {
    self.try_set(Property::AspectRatio, Value::AspectRatio(value))
}
```

In `Resolved`, add typed getters:

```rust
pub fn position(&self) -> LayoutPosition {
    match self.get(Property::Position) {
        Value::Position(value) => *value,
        _ => LayoutPosition::default(),
    }
}

pub fn z_index(&self) -> ZIndex {
    match self.get(Property::ZIndex) {
        Value::ZIndex(value) => *value,
        _ => ZIndex::default(),
    }
}

pub fn scrollbar_width(&self) -> ScrollbarWidth {
    match self.get(Property::ScrollbarWidth) {
        Value::ScrollbarWidth(value) => *value,
        _ => ScrollbarWidth::default(),
    }
}

pub fn content_visibility(&self) -> ContentVisibility {
    match self.get(Property::ContentVisibility) {
        Value::ContentVisibility(value) => *value,
        _ => ContentVisibility::default(),
    }
}

pub fn order(&self) -> Order {
    match self.get(Property::Order) {
        Value::Order(value) => *value,
        _ => Order::default(),
    }
}

pub fn flex_grow(&self) -> FlexFactor {
    match self.get(Property::FlexGrow) {
        Value::FlexFactor(value) => *value,
        _ => FlexFactor::zero(),
    }
}

pub fn flex_shrink(&self) -> FlexFactor {
    match self.get(Property::FlexShrink) {
        Value::FlexFactor(value) => *value,
        _ => FlexFactor::one(),
    }
}

pub fn aspect_ratio(&self) -> AspectRatio {
    match self.get(Property::AspectRatio) {
        Value::AspectRatio(value) => *value,
        _ => AspectRatio::default(),
    }
}
```

- [ ] **Step 7: Hash and export the new value types**

Update `declaration::hash_value()` and `value_kind()` for every new `Value` variant. Use stable discriminants and hash exposed primitive payloads through getters:

```rust
Value::ZIndex(value) => {
    41u8.hash(state);
    match value {
        ZIndex::Auto => 0u8.hash(state),
        ZIndex::Integer(value) => {
            1u8.hash(state);
            value.hash(state);
        }
    }
}
Value::Order(value) => {
    42u8.hash(state);
    value.get().hash(state);
}
Value::FlexFactor(value) => {
    43u8.hash(state);
    value.get().to_bits().hash(state);
}
Value::AspectRatio(value) => {
    44u8.hash(state);
    match value.as_ratio() {
        Some(ratio) => {
            1u8.hash(state);
            ratio.to_bits().hash(state);
        }
        None => 0u8.hash(state),
    }
}
```

Use unique discriminants that do not collide with existing arms in the local function. Before committing this task, run:

```sh
python3 - <<'PY'
from pathlib import Path
import re
lines = Path('src/declaration.rs').read_text().splitlines()
values = []
pending_value_arm = False
for line in lines:
    if 'Value::' in line and '=>' in line:
        pending_value_arm = True
    if pending_value_arm:
        match = re.search(r'(\d+)u8\.hash\(state\)', line)
        if match:
            values.append(int(match.group(1)))
            pending_value_arm = False
duplicates = sorted({value for value in values if values.count(value) > 1})
print(f'top_level_value_discriminants={sorted(values)}')
print(f'duplicates={duplicates}')
raise SystemExit(0 if not duplicates else 1)
PY
```

Update `src/lib.rs` reexports:

```rust
AspectRatio, ContentVisibility, FlexFactor, Order, ScrollbarWidth, ZIndex,
```

- [ ] **Step 8: Add type-safety compile tests**

Add to `tests/compile_pass/typed_public_construction.rs`:

```rust
let declarations = Declarations::new()
    .position(LayoutPosition::Fixed)
    .z_index(ZIndex::integer(-1))
    .scrollbar_width(ScrollbarWidth::Thin)
    .content_visibility(ContentVisibility::Auto)
    .order(Order::new(2))
    .try_flex_grow(FlexFactor::new(2.0)?)?
    .try_flex_shrink(FlexFactor::new(0.0)?)?
    .try_aspect_ratio(AspectRatio::ratio(16.0 / 9.0)?)?;
assert_eq!(declarations.len(), 8);
```

Create `tests/compile_fail/invalid_layout_value_newtype_literal.rs`:

```rust
use surgeist_style::{AspectRatio, FlexFactor, Order};

fn main() {
    let _order = Order(1);
    let _grow = FlexFactor(1.0);
    let _ratio = AspectRatio(0.0);
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
cargo test -p surgeist-style core_layout_properties_accept_typed_values semantic_numbers_are_not_interchangeable resolved_core_layout_getters_return_typed_values
```

- [ ] **Step 9: Commit after worker/reviewer clean**

Coordinator runs:

```sh
cargo fmt --check
cargo test -p surgeist-style core_layout_properties_accept_typed_values semantic_numbers_are_not_interchangeable resolved_core_layout_getters_return_typed_values
cargo test -p surgeist-style --test type_safety
git diff --check
git status --short --branch
```

After a scoped reviewer is clean, commit:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_layout_value_newtype_literal.rs tests/compile_fail/invalid_layout_value_newtype_literal.stderr
git commit -m "style: type layout values"
```

---

### Task 3: Flex, Place, And Track Alignment Shorthands

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add failing shorthand tests**

Add to `src/declaration.rs` tests:

```rust
#[test]
fn flex_shorthand_lowers_to_grow_shrink_and_basis() {
    let declarations = Declarations::new().try_flex(Flex::none()).unwrap();
    assert_eq!(declarations.get(Property::Flex), None);
    assert_eq!(declarations.get(Property::FlexGrow), Some(&Value::FlexFactor(FlexFactor::zero())));
    assert_eq!(declarations.get(Property::FlexShrink), Some(&Value::FlexFactor(FlexFactor::zero())));
    assert_eq!(declarations.get(Property::FlexBasis), Some(&Value::Length(Length::Auto)));

    let declarations = Declarations::new().try_flex(Flex::auto()).unwrap();
    assert_eq!(declarations.get(Property::FlexGrow), Some(&Value::FlexFactor(FlexFactor::one())));
    assert_eq!(declarations.get(Property::FlexShrink), Some(&Value::FlexFactor(FlexFactor::one())));
    assert_eq!(declarations.get(Property::FlexBasis), Some(&Value::Length(Length::Auto)));

    let declarations = Declarations::new()
        .try_flex(Flex::components(
            FlexFactor::new(2.0).unwrap(),
            None,
            None,
        ))
        .unwrap();
    assert_eq!(declarations.get(Property::FlexGrow), Some(&Value::FlexFactor(FlexFactor::new(2.0).unwrap())));
    assert_eq!(declarations.get(Property::FlexShrink), Some(&Value::FlexFactor(FlexFactor::one())));
    assert_eq!(declarations.get(Property::FlexBasis), Some(&Value::Length(Length::ZERO)));
}

#[test]
fn flex_shorthand_validates_canonical_basis_domain() {
    assert!(
        Declarations::new()
            .try_flex(Flex::components(
                FlexFactor::one(),
                None,
                Some(Length::Normal),
            ))
            .is_err()
    );
    assert!(
        Declarations::new()
            .try_flex(Flex::components(
                FlexFactor::one(),
                None,
                Some(Length::Px(-1.0)),
            ))
            .is_err()
    );
}

#[test]
fn place_shorthands_lower_to_axis_longhands() {
    let declarations = Declarations::new()
        .place_content(PlaceContentAlignment::new(AlignContent::Center, AlignContent::SpaceBetween));
    assert_eq!(declarations.get(Property::PlaceContent), None);
    assert_eq!(declarations.get(Property::AlignContent), Some(&Value::AlignContent(AlignContent::Center)));
    assert_eq!(declarations.get(Property::JustifyContent), Some(&Value::AlignContent(AlignContent::SpaceBetween)));

    let declarations = Declarations::new()
        .place_items(PlaceItemsAlignment::new(AlignItems::Start, AlignItems::Stretch))
        .place_self(PlaceItemsAlignment::new(AlignItems::End, AlignItems::Center));
    assert_eq!(declarations.get(Property::AlignItems), Some(&Value::AlignItems(AlignItems::Start)));
    assert_eq!(declarations.get(Property::JustifyItems), Some(&Value::AlignItems(AlignItems::Stretch)));
    assert_eq!(declarations.get(Property::AlignSelf), Some(&Value::AlignItems(AlignItems::End)));
    assert_eq!(declarations.get(Property::JustifySelf), Some(&Value::AlignItems(AlignItems::Center)));
}
```

Add to `src/property.rs` tests:

```rust
#[test]
fn track_alignment_uses_content_alignment_value() {
    Property::AlignTracks
        .validate_value(&Value::AlignContent(AlignContent::Center))
        .unwrap();
    Property::JustifyTracks
        .validate_value(&Value::AlignContent(AlignContent::SpaceAround))
        .unwrap();
    assert!(Property::AlignTracks.validate_value(&Value::AlignItems(AlignItems::Center)).is_err());
    assert!(
        Declarations::new()
            .try_set(Property::AlignTracks, Value::AlignItems(AlignItems::Center))
            .is_err()
    );
}
```

- [ ] **Step 2: Run focused tests and confirm they fail**

Run:

```sh
cargo test -p surgeist-style flex_shorthand_lowers_to_grow_shrink_and_basis flex_shorthand_validates_canonical_basis_domain place_shorthands_lower_to_axis_longhands track_alignment_uses_content_alignment_value
```

Expected before implementation: missing `Flex`, `PlaceContentAlignment`, `PlaceItemsAlignment`, and property variants.

- [ ] **Step 3: Add shorthand value types**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Flex {
    None,
    Auto,
    Components {
        grow: FlexFactor,
        shrink: Option<FlexFactor>,
        basis: Option<Length>,
    },
}

impl Flex {
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    #[must_use]
    pub const fn auto() -> Self {
        Self::Auto
    }

    #[must_use]
    pub const fn components(
        grow: FlexFactor,
        shrink: Option<FlexFactor>,
        basis: Option<Length>,
    ) -> Self {
        Self::Components { grow, shrink, basis }
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::None | Self::Auto => Ok(()),
            Self::Components { grow, shrink, basis } => {
                grow.validate()?;
                if let Some(shrink) = shrink {
                    shrink.validate()?;
                }
                if let Some(basis) = basis {
                    basis.validate()?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlaceContentAlignment {
    first: AlignContent,
    second: AlignContent,
}

impl PlaceContentAlignment {
    #[must_use]
    pub const fn new(first: AlignContent, second: AlignContent) -> Self {
        Self { first, second }
    }

    #[must_use]
    pub const fn all(value: AlignContent) -> Self {
        Self::new(value, value)
    }

    #[must_use]
    pub const fn first(self) -> AlignContent {
        self.first
    }

    #[must_use]
    pub const fn second(self) -> AlignContent {
        self.second
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlaceItemsAlignment {
    first: AlignItems,
    second: AlignItems,
}

impl PlaceItemsAlignment {
    #[must_use]
    pub const fn new(first: AlignItems, second: AlignItems) -> Self {
        Self { first, second }
    }

    #[must_use]
    pub const fn all(value: AlignItems) -> Self {
        Self::new(value, value)
    }

    #[must_use]
    pub const fn first(self) -> AlignItems {
        self.first
    }

    #[must_use]
    pub const fn second(self) -> AlignItems {
        self.second
    }
}
```

Add `Value::Flex(Flex)`, `Value::PlaceContentAlignment(PlaceContentAlignment)`, and `Value::PlaceItemsAlignment(PlaceItemsAlignment)`.

- [ ] **Step 4: Add properties and validation**

In `src/property.rs`, add:

```rust
PlaceContent,
PlaceItems,
PlaceSelf,
Order,
Flex,
JustifyTracks,
AlignTracks,
```

Insert the variants in `Property::ALL`.

Mark these as non-canonical:

```rust
Self::PlaceContent | Self::PlaceItems | Self::PlaceSelf | Self::Flex
```

Keep `AlignTracks` and `JustifyTracks` canonical.

Metadata:

```rust
Self::PlaceContent => Metadata::new(Value::PlaceContentAlignment(PlaceContentAlignment::all(AlignContent::default())))
    .impact(Impact::empty().layout()),
Self::PlaceItems | Self::PlaceSelf => Metadata::new(Value::PlaceItemsAlignment(PlaceItemsAlignment::all(AlignItems::default())))
    .impact(Impact::empty().layout()),
Self::Flex => Metadata::new(Value::Flex(Flex::none()))
    .impact(Impact::empty().layout()),
Self::AlignTracks | Self::JustifyTracks => {
    Metadata::new(Value::AlignContent(AlignContent::default()))
        .impact(Impact::empty().layout())
}
```

Acceptance:

```rust
Self::PlaceContent => matches!(value, Value::PlaceContentAlignment(_)),
Self::PlaceItems | Self::PlaceSelf => matches!(value, Value::PlaceItemsAlignment(_)),
Self::Flex => matches!(value, Value::Flex(_)),
Self::AlignTracks | Self::JustifyTracks => matches!(value, Value::AlignContent(_)),
```

- [ ] **Step 5: Add canonical shorthand lowering**

Update `canonical_properties()`:

```rust
Property::PlaceContent => vec![Property::AlignContent, Property::JustifyContent],
Property::PlaceItems => vec![Property::AlignItems, Property::JustifyItems],
Property::PlaceSelf => vec![Property::AlignSelf, Property::JustifySelf],
Property::Flex => vec![Property::FlexGrow, Property::FlexShrink, Property::FlexBasis],
```

Update `canonical_declarations()`:

```rust
(Property::PlaceContent, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::PlaceContent),
    Value::Keyword(keyword),
),
(Property::PlaceContent, Value::PlaceContentAlignment(value)) => vec![
    Declaration::new(Property::AlignContent, Value::AlignContent(value.first())),
    Declaration::new(Property::JustifyContent, Value::AlignContent(value.second())),
],
(Property::PlaceItems, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::PlaceItems),
    Value::Keyword(keyword),
),
(Property::PlaceItems, Value::PlaceItemsAlignment(value)) => vec![
    Declaration::new(Property::AlignItems, Value::AlignItems(value.first())),
    Declaration::new(Property::JustifyItems, Value::AlignItems(value.second())),
],
(Property::PlaceSelf, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::PlaceSelf),
    Value::Keyword(keyword),
),
(Property::PlaceSelf, Value::PlaceItemsAlignment(value)) => vec![
    Declaration::new(Property::AlignSelf, Value::AlignItems(value.first())),
    Declaration::new(Property::JustifySelf, Value::AlignItems(value.second())),
],
(Property::Flex, Value::Keyword(keyword)) => {
    same_value_declarations(canonical_properties(Property::Flex), Value::Keyword(keyword))
}
(Property::Flex, Value::Flex(value)) => flex_declarations(value),
```

Add:

```rust
fn flex_declarations(value: Flex) -> Vec<Declaration> {
    let (grow, shrink, basis) = match value {
        Flex::None => (FlexFactor::zero(), FlexFactor::zero(), Length::Auto),
        Flex::Auto => (FlexFactor::one(), FlexFactor::one(), Length::Auto),
        Flex::Components { grow, shrink, basis } => {
            (grow, shrink.unwrap_or_else(FlexFactor::one), basis.unwrap_or(Length::ZERO))
        }
    };

    vec![
        Declaration::new(Property::FlexGrow, Value::FlexFactor(grow)),
        Declaration::new(Property::FlexShrink, Value::FlexFactor(shrink)),
        Declaration::new(Property::FlexBasis, Value::Length(basis)),
    ]
}
```

- [ ] **Step 6: Add declaration front doors and resolved getters**

In `Declarations`, add:

```rust
pub fn try_flex(self, value: Flex) -> Result<Self> {
    self.try_set(Property::Flex, Value::Flex(value))
}

#[must_use]
pub fn place_content(self, value: PlaceContentAlignment) -> Self {
    self.set(Property::PlaceContent, Value::PlaceContentAlignment(value))
}

#[must_use]
pub fn place_items(self, value: PlaceItemsAlignment) -> Self {
    self.set(Property::PlaceItems, Value::PlaceItemsAlignment(value))
}

#[must_use]
pub fn place_self(self, value: PlaceItemsAlignment) -> Self {
    self.set(Property::PlaceSelf, Value::PlaceItemsAlignment(value))
}

#[must_use]
pub fn align_tracks(self, value: AlignContent) -> Self {
    self.set(Property::AlignTracks, Value::AlignContent(value))
}

#[must_use]
pub fn justify_tracks(self, value: AlignContent) -> Self {
    self.set(Property::JustifyTracks, Value::AlignContent(value))
}
```

In `Resolved`, add:

```rust
pub fn align_tracks(&self) -> AlignContent {
    match self.get(Property::AlignTracks) {
        Value::AlignContent(value) => *value,
        _ => AlignContent::default(),
    }
}

pub fn justify_tracks(&self) -> AlignContent {
    match self.get(Property::JustifyTracks) {
        Value::AlignContent(value) => *value,
        _ => AlignContent::default(),
    }
}
```

- [ ] **Step 7: Hash, export, and compile-test the shorthands**

Update `hash_value()` and `value_kind()` for `Value::Flex`, `Value::PlaceContentAlignment`, and `Value::PlaceItemsAlignment`.

Update `src/lib.rs` reexports:

```rust
Flex, PlaceContentAlignment, PlaceItemsAlignment,
```

Add to `tests/compile_pass/typed_public_construction.rs`:

```rust
let declarations = Declarations::new()
    .try_flex(Flex::components(FlexFactor::new(2.0)?, None, Some(Length::Px(10.0))))?
    .place_content(PlaceContentAlignment::all(AlignContent::Center))
    .place_items(PlaceItemsAlignment::new(AlignItems::Start, AlignItems::Stretch))
    .place_self(PlaceItemsAlignment::all(AlignItems::Center))
    .align_tracks(AlignContent::SpaceBetween)
    .justify_tracks(AlignContent::SpaceAround);
assert_eq!(declarations.len(), 11);
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
cargo test -p surgeist-style flex_shorthand_lowers_to_grow_shrink_and_basis flex_shorthand_validates_canonical_basis_domain place_shorthands_lower_to_axis_longhands track_alignment_uses_content_alignment_value
```

Do not add a compile-fail fixture for `Property::AlignTracks` paired with `Value::AlignItems`; the public broad API is `try_set(Property, Value)`, so this invariant is a runtime validation failure unless a later API plan removes that broad front door.

- [ ] **Step 8: Commit after worker/reviewer clean**

Coordinator runs:

```sh
cargo fmt --check
cargo test -p surgeist-style flex_shorthand_lowers_to_grow_shrink_and_basis flex_shorthand_validates_canonical_basis_domain place_shorthands_lower_to_axis_longhands track_alignment_uses_content_alignment_value
cargo test -p surgeist-style --test type_safety
git diff --check
git status --short --branch
```

After a scoped reviewer is clean, commit:

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: lower layout shorthands"
```

---

### Task 4: Integration, Resolver Surface, And Ledger Rebase

**Files:**
- Modify: `src/authored.rs`
- Modify: `src/invalidation.rs`
- Modify: `src/resolver.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Add integration tests for authored CSS-wide expansion**

In `src/authored.rs`, add:

```rust
#[test]
fn new_layout_shorthands_expand_css_wide_keywords_to_canonical_longhands() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Flex),
        CssWideKeyword::RevertLayer,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::PlaceContent),
        CssWideKeyword::Unset,
    ));
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::Margin),
        CssWideKeyword::Initial,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();
    assert_eq!(canonical.get(Property::Flex), None);
    assert_eq!(
        canonical.get(Property::FlexGrow),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
    );
    assert_eq!(
        canonical.get(Property::FlexShrink),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
    );
    assert_eq!(
        canonical.get(Property::FlexBasis),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
    );
    assert_eq!(canonical.get(Property::PlaceContent), None);
    assert_eq!(
        canonical.get(Property::AlignContent),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
    );
    assert_eq!(
        canonical.get(Property::JustifyContent),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
    );
    assert_eq!(
        canonical.get(Property::MarginTop),
        Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Initial))
    );
}
```

- [ ] **Step 2: Add final resolved smoke test**

In `src/resolver.rs`, add:

```rust
#[test]
fn layout_operation_eight_values_resolve_together() {
    let style = resolve_single(
        Declarations::new()
            .display(Display::Grid)
            .position(LayoutPosition::Fixed)
            .try_inset(Edges::all(Length::Px(3.0))).unwrap()
            .try_margin_left(Length::Px(-2.0)).unwrap()
            .try_padding(Edges::all(Length::Px(4.0))).unwrap()
            .try_border_width(Edges::all(Length::Px(1.0))).unwrap()
            .z_index(ZIndex::integer(7))
            .scrollbar_width(ScrollbarWidth::Thin)
            .content_visibility(ContentVisibility::Auto)
            .order(Order::new(5))
            .try_flex(Flex::auto()).unwrap()
            .place_content(PlaceContentAlignment::all(AlignContent::Center))
            .align_tracks(AlignContent::SpaceEvenly),
    );

    assert_eq!(style.display(), Display::Grid);
    assert_eq!(style.position(), LayoutPosition::Fixed);
    assert_eq!(style.inset_edges().left, Length::Px(3.0));
    assert_eq!(style.margin_edges().left, Length::Px(-2.0));
    assert_eq!(style.padding_edges().right, Length::Px(4.0));
    assert_eq!(style.border_width_edges().top, Length::Px(1.0));
    assert_eq!(style.z_index(), ZIndex::integer(7));
    assert_eq!(style.scrollbar_width(), ScrollbarWidth::Thin);
    assert_eq!(style.content_visibility(), ContentVisibility::Auto);
    assert_eq!(style.order(), Order::new(5));
    assert_eq!(style.flex_grow(), FlexFactor::one());
    assert_eq!(style.flex_shrink(), FlexFactor::one());
    assert_eq!(style.align_tracks(), AlignContent::SpaceEvenly);
}
```

Use the existing `Resolved::display()` getter in this smoke test; do not add a duplicate method.

- [ ] **Step 3: Verify invalidation stays property-driven**

Run:

```sh
cargo test -p surgeist-style invalidation
```

If tests fail because `Property::ALL` gained canonical properties, update invalidation assertions to count properties through `Property::ALL.iter().filter(|property| property.is_canonical())` instead of hard-coded totals.

- [ ] **Step 4: Rebase the property coverage ledger**

Update `plans/2026-07-05-css-property-coverage-ledger.md` only for Operation 8 rows affected by this implementation:

- Change side longhand rows from `New style property needed` to `Existing style property`.
- Change `Flex`, `PlaceContent`, `PlaceItems`, and `PlaceSelf` from `New shorthand lowering needed` to `Existing style shorthand`.
- Change `ContentVisibility`, `Order`, `JustifyTracks`, and `AlignTracks` from `New style property needed` to `Existing style property`.
- Change `ZIndex`, `AspectRatio`, `ScrollbarWidth`, `FlexGrow`, and `FlexShrink` targets away from `Value::Number` and toward their style-owned typed values.
- Leave Operation 9, 10, 11, 12, and 14 rows unchanged.
- Update the Family Rollup missing-support cells for Operation 8 families so they no longer claim the implemented gaps are missing.
- Update the Next Sequence Context to say Operation 9 text-facing properties come next.

Run a ledger consistency check:

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

- [ ] **Step 5: Run full crate checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected:

- formatting passes,
- tests pass,
- clippy passes,
- the `surgeist_css|surgeist-css` search has no matches,
- diff check passes,
- status shows only intended Operation 8 files before commit.

- [ ] **Step 6: Commit after worker/reviewer clean**

After a scoped reviewer is clean, coordinator commits:

```sh
git add src/authored.rs src/invalidation.rs src/resolver.rs tests/compile_pass/typed_public_construction.rs plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: integrate layout property coverage"
```

---

## Final Verification

After all task commits are complete, run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected final state:

- no direct `surgeist-css` dependency,
- clean formatting,
- all tests pass,
- clippy has no warnings,
- working tree is clean except the branch being ahead by the new task commits.

## Final Holistic Review Prompt

Use a clean-context reviewer with this prompt:

```text
You are a final holistic reviewer for surgeist-style Operation 8 layout-facing property family expansion. Do not edit files.

Repo: /Users/codex/Development/surgeist-style

Read:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger.md
- plans/2026-07-05-layout-facing-property-families-implementation.md
- read-only /Users/codex/Development/surgeist-css/src/syntax.rs

Review the completed implementation against:
- style owns typed layout-facing receiving and resolved models;
- aggregate edge properties, place shorthands, flex shorthand, and grid shorthand behavior canonicalize into longhands;
- semantic numeric values are not left as broad `Value::Number` contracts for z-index, order, flex grow/shrink, aspect ratio, or scrollbar width;
- `surgeist-style` does not depend on `surgeist-css`;
- no style-to-layout adapter or workaround lowering layer was added;
- symbolic lengths and calc values remain symbolic;
- public APIs have front doors and invalid states are hard to construct;
- Operation 8 ledger rows were rebased honestly and Operation 9 remains the next plan.

Run:
- cargo fmt --check
- cargo test -p surgeist-style
- cargo clippy -p surgeist-style --all-targets -- -D warnings
- rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
- git diff --check
- git status --short --branch

Report findings first with file/line references. If clean, say clean and include commands run.
```

## This Will Come Next

After Operation 8 lands and reviewers are clean, write the next sequential implementation plan for Operation 9: text-facing computed property families.

Operation 9 should start from the rebased ledger and cover:

- font family and font shorthand,
- font size, line height, weight, slant/style, stretch/width,
- font variant and feature settings,
- letter spacing,
- text alignment and text-align-last,
- text indent and vertical alignment,
- text wrap, white space, word break, overflow wrap, and text overflow,
- text decoration line/color/style/thickness,
- text transform.

Operation 9 must keep font loading, shaping, glyph layout, and text backend contracts outside `surgeist-style`. It should keep values authored/resolved-style-facing and avoid reintroducing `surgeist-text` or any CSS parser dependency.
