# Conditions, Layers, And Scope Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add style-owned media/container condition models, cascade layer registration, and scoped rule matching inputs for Operation 13.

**Architecture:** `surgeist-style` receives root-lowered condition, layer, and scope data as style-owned models. The resolver should evaluate rules against caller-provided environment/container facts and scoped selector anchors, while leaving CSS parsing, source diagnostics, host environment discovery, network/filesystem imports, and broad Operation 14 cache/invalidation redesign outside this crate. Layer order remains an explicit style cascade dimension that outranks source order in the existing `RulePrecedence` ordering.

**Tech Stack:** Rust, `surgeist-style`, crate-local tests, trybuild compile-fail public API tests.

---

## Context

Read before implementing:

- `AGENTS.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-property-coverage-ledger.md`
- Current code:
  - `src/condition.rs`
  - `src/precedence.rs`
  - `src/sheet.rs`
  - `src/resolver.rs`
  - `src/selector.rs`
  - `src/lib.rs`
  - `tests/compile_pass/typed_public_construction.rs`

Read-only CSS surface reference:

- `/Users/codex/Development/surgeist-css/src/syntax.rs`
- `/Users/codex/Development/surgeist-css/src/parser/queries.rs`

Do not add a `surgeist-css` dependency. The CSS files are only evidence for the authored syntax surface root will lower.

## Boundaries

This operation owns:

- media query lists, typed media queries, media condition trees, media feature facts, and media environment facts;
- container names, container condition trees, container feature facts, container style custom-property facts, and queryable container facts;
- layer names, layer statements, named/anonymous layer registration, and layer-block rule APIs that produce `LayerOrder`;
- scope root/limit selector lists and scoped rule matching contracts;
- focused cache-key hashing for any new environment/container/scope facts used by the current resolver, so existing resolver caching stays correct.

This operation does not own:

- CSS parsing or CSS source diagnostics;
- `@import` loading, network, or filesystem policy;
- final Operation 14 invalidation/cache redesign;
- `!important`;
- style-to-root, style-to-layout, or style-to-render adapters;
- host environment discovery;
- container size/layout computation;
- broad compatibility aliases for replaced `Viewport`/`Container` names.

Breaking API changes are allowed. Do not preserve `Viewport`/`Container` as compatibility aliases if they no longer name the right domain.

## Current State

- `Condition` is currently a broad two-variant enum over `Viewport` and `Container`.
- `Viewport` and `Container` store optional raw `f32` width/height/min/max values.
- `Rule::when` stores a flat `Vec<Condition>`, and `Condition::matches_all` treats that vector as conjunction.
- `Context` stores `viewport: Viewport` and `container: Option<Container>`.
- `RulePrecedence` already orders by `LayerOrder`, then specificity, then `SourceOrder`.
- `LayerOrder::default()` is the unlayered layer order.
- Selector matching already has root/scope facts through `SelectorMatchContext::with_root` and `with_scope`.
- There is no style-owned named layer registry and no `RuleScope` stored on `Rule`.

## Desired Model

Use these names unless a worker finds a concrete compile conflict:

- `MediaEnvironment`
- `MediaQueryList`
- `MediaQuery`
- `TypedMediaQuery`
- `MediaQueryModifier`
- `MediaType`
- `MediaCondition`
- `MediaConditionList`
- `MediaFeatureQuery`
- `QueryComparison`
- `RangeFeature<T>`
- `QueryLength`
- `QueryLengthUnit`
- `QueryLengthBasis`
- `Resolution`
- `ResolutionUnit`
- `Ratio`
- `NonNegativeInteger`
- `Orientation`
- `ColorSchemePreference`
- `ReducedMotionPreference`
- `ReducedTransparencyPreference`
- `ContrastPreference`
- `ForcedColorsMode`
- `HoverCapability`
- `PointerCapability`
- `DisplayMode`
- `ContainerName`
- `ContainerFacts`
- `ContainerCondition`
- `ContainerConditionList`
- `ContainerFeatureQuery`
- `ContainerStyleQuery`
- `StyleLayerName`
- `StyleLayerNameList`
- `LayerStatement`
- `LayerBlock`
- `LayerRegistry`
- `RuleScope`
- `ScopeSelectorList`

Keep public fields private for invariant-bearing types. Use checked constructors for nonempty lists, finite numbers, non-negative dimensions, positive resolution, and nonempty names.

---

### Task 1: Add Media Query And Environment Models

**Files:**

- Modify: `src/condition.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_media_query_list_empty_literal.rs`
- Create: `tests/compile_fail/invalid_media_query_list_empty_literal.stderr`
- Create: `tests/compile_fail/invalid_query_length_literal.rs`
- Create: `tests/compile_fail/invalid_query_length_literal.stderr`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: clean except prior committed work.

- [ ] **Step 2: Add media primitive tests first**

Add focused tests in `src/condition.rs`:

```rust
#[test]
fn media_query_lists_match_environment_facts() {
    let query = MediaQueryList::try_new([MediaQuery::Condition(MediaCondition::Feature(
        MediaFeatureQuery::Width(RangeFeature::new(
            Some(QueryComparison::GreaterThanOrEqual),
            QueryLength::try_new(640.0, QueryLengthUnit::Px).unwrap(),
        )),
    ))])
    .unwrap();

    let matching = MediaEnvironment::new()
        .media_type(MediaType::Screen)
        .width(QueryLength::try_new(800.0, QueryLengthUnit::Px).unwrap())
        .height(QueryLength::try_new(600.0, QueryLengthUnit::Px).unwrap());
    let too_small = MediaEnvironment::new()
        .media_type(MediaType::Screen)
        .width(QueryLength::try_new(320.0, QueryLengthUnit::Px).unwrap());

    assert!(query.matches(&matching));
    assert!(!query.matches(&too_small));
}

#[test]
fn typed_media_queries_support_modifiers_types_and_conditions() {
    let screen = MediaQuery::Typed(TypedMediaQuery::new(
        None,
        MediaType::Screen,
        Some(MediaCondition::Feature(MediaFeatureQuery::Orientation(
            Orientation::Landscape,
        ))),
    ));
    let not_print = MediaQuery::Typed(TypedMediaQuery::new(
        Some(MediaQueryModifier::Not),
        MediaType::Print,
        None,
    ));
    let only_screen = MediaQuery::Typed(TypedMediaQuery::new(
        Some(MediaQueryModifier::Only),
        MediaType::Screen,
        None,
    ));
    let list = MediaQueryList::try_new([screen, not_print, only_screen]).unwrap();
    let environment = MediaEnvironment::new()
        .media_type(MediaType::Screen)
        .width(QueryLength::try_new(800.0, QueryLengthUnit::Px).unwrap())
        .height(QueryLength::try_new(400.0, QueryLengthUnit::Px).unwrap());

    assert!(list.matches(&environment));
}

#[test]
fn media_condition_lists_require_two_conditions() {
    assert!(MediaConditionList::try_new([MediaCondition::Feature(
        MediaFeatureQuery::Hover(HoverCapability::Hover),
    )])
    .is_err());

    let conditions = MediaConditionList::try_new([
        MediaCondition::Feature(MediaFeatureQuery::Hover(HoverCapability::Hover)),
        MediaCondition::Not(Box::new(MediaCondition::Feature(
            MediaFeatureQuery::ForcedColors(ForcedColorsMode::Active),
        ))),
    ])
    .unwrap();

    let environment = MediaEnvironment::new()
        .hover(HoverCapability::Hover)
        .forced_colors(ForcedColorsMode::None);
    assert!(MediaCondition::And(conditions).matches(&environment));
}

#[test]
fn query_lengths_preserve_css_units_and_require_basis_for_resolution() {
    let query = QueryLength::try_new(40.0, QueryLengthUnit::Rem).unwrap();
    let without_basis = MediaEnvironment::new()
        .width(QueryLength::try_new(800.0, QueryLengthUnit::Px).unwrap());
    let with_basis = without_basis.clone().with_length_basis(
        QueryLengthBasis::new().root_font_size(QueryLength::try_new(20.0, QueryLengthUnit::Px).unwrap()),
    );

    assert_eq!(query.unit(), QueryLengthUnit::Rem);
    assert_eq!(query.to_css_px(with_basis.length_basis()), Some(800.0));
    assert_eq!(query.to_css_px(without_basis.length_basis()), None);
}
```

Run:

```sh
cargo test -p surgeist-style media_query_lists
```

Expected: fail because the types do not exist yet.

- [ ] **Step 3: Replace raw viewport model with media-owned types**

In `src/condition.rs`, add the media model below alongside the current
`Viewport` and raw `Container` models. Do not remove `Viewport` in Task 1:
`src/resolver.rs` and `src/sheet.rs` still use it until Task 3. Keep the old raw
`Container` path compiling temporarily in Task 1; Task 2 adds
`ContainerCondition` and `ContainerFacts`, and Task 3 removes the old
resolver-facing API. Resolver wiring happens in Task 3.

Implementation shape:

```rust
use crate::{Error, ErrorCode, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    Media(MediaQueryList),
    Container(Container),
}

impl Condition {
    #[must_use]
    pub fn media(query: MediaQueryList) -> Self {
        Self::Media(query)
    }

    #[must_use]
    pub fn is_media(&self) -> bool {
        matches!(self, Self::Media(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaQueryList {
    queries: Vec<MediaQuery>,
}

impl MediaQueryList {
    pub fn try_new(queries: impl IntoIterator<Item = MediaQuery>) -> Result<Self> {
        let queries = queries.into_iter().collect::<Vec<_>>();
        if queries.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "media query list cannot be empty",
            ));
        }
        Ok(Self { queries })
    }

    #[must_use]
    pub fn queries(&self) -> &[MediaQuery] {
        &self.queries
    }

    #[must_use]
    pub fn matches(&self, environment: &MediaEnvironment) -> bool {
        self.queries.iter().any(|query| query.matches(environment))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaQuery {
    Condition(MediaCondition),
    Typed(TypedMediaQuery),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedMediaQuery {
    modifier: Option<MediaQueryModifier>,
    media_type: MediaType,
    condition: Option<MediaCondition>,
}

impl TypedMediaQuery {
    #[must_use]
    pub const fn new(
        modifier: Option<MediaQueryModifier>,
        media_type: MediaType,
        condition: Option<MediaCondition>,
    ) -> Self {
        Self {
            modifier,
            media_type,
            condition,
        }
    }

    #[must_use]
    pub const fn modifier(&self) -> Option<MediaQueryModifier> {
        self.modifier
    }

    #[must_use]
    pub const fn media_type(&self) -> MediaType {
        self.media_type
    }

    #[must_use]
    pub const fn condition(&self) -> Option<&MediaCondition> {
        self.condition.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MediaQueryModifier {
    Not,
    Only,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MediaType {
    #[default]
    All,
    Screen,
    Print,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaCondition {
    Feature(MediaFeatureQuery),
    Not(Box<MediaCondition>),
    And(MediaConditionList),
    Or(MediaConditionList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaConditionList {
    conditions: Vec<MediaCondition>,
}

impl MediaConditionList {
    pub fn try_new(conditions: impl IntoIterator<Item = MediaCondition>) -> Result<Self> {
        let conditions = conditions.into_iter().collect::<Vec<_>>();
        if conditions.len() < 2 {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "media condition list requires at least two conditions",
            ));
        }
        Ok(Self { conditions })
    }

    #[must_use]
    pub fn conditions(&self) -> &[MediaCondition] {
        &self.conditions
    }
}
```

Keep the existing `Viewport` and `Container` structs below this point until Task
3. They are temporary compile bridges inside the implementation sequence.

Continue the same file with range/fact types:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QueryComparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeFeature<T> {
    comparison: Option<QueryComparison>,
    value: T,
}

impl<T> RangeFeature<T> {
    #[must_use]
    pub const fn new(comparison: Option<QueryComparison>, value: T) -> Self {
        Self { comparison, value }
    }

    #[must_use]
    pub const fn comparison(&self) -> Option<QueryComparison> {
        self.comparison
    }

    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryLength {
    value: f32,
    unit: QueryLengthUnit,
}

impl QueryLength {
    pub fn try_new(value: f32, unit: QueryLengthUnit) -> Result<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "query length must be finite and non-negative",
            ));
        }
        Ok(Self { value, unit })
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }

    #[must_use]
    pub const fn unit(self) -> QueryLengthUnit {
        self.unit
    }

    #[must_use]
    pub fn to_css_px(self, basis: &QueryLengthBasis) -> Option<f32> {
        basis.resolve(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QueryLengthUnit {
    Px,
    Em,
    Rem,
    Ex,
    Rex,
    Cap,
    Rcap,
    Ch,
    Rch,
    Ic,
    Ric,
    Lh,
    Rlh,
    Vw,
    Vh,
    Vi,
    Vb,
    Vmin,
    Vmax,
    Svw,
    Svh,
    Svi,
    Svb,
    Svmin,
    Svmax,
    Lvw,
    Lvh,
    Lvi,
    Lvb,
    Lvmin,
    Lvmax,
    Dvw,
    Dvh,
    Dvi,
    Dvb,
    Dvmin,
    Dvmax,
    Cqw,
    Cqh,
    Cqi,
    Cqb,
    Cqmin,
    Cqmax,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt,
}
```

Do not narrow query lengths to `Px`. Root may lower any length unit accepted by
`surgeist-css`; style must preserve the unit and resolve it only from explicit
facts.

Add `QueryLengthBasis`:

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueryLengthBasis {
    em: Option<QueryLength>,
    rem: Option<QueryLength>,
    ex: Option<QueryLength>,
    rex: Option<QueryLength>,
    cap: Option<QueryLength>,
    rcap: Option<QueryLength>,
    ch: Option<QueryLength>,
    rch: Option<QueryLength>,
    ic: Option<QueryLength>,
    ric: Option<QueryLength>,
    lh: Option<QueryLength>,
    rlh: Option<QueryLength>,
    viewport_width: Option<QueryLength>,
    viewport_height: Option<QueryLength>,
    small_viewport_width: Option<QueryLength>,
    small_viewport_height: Option<QueryLength>,
    large_viewport_width: Option<QueryLength>,
    large_viewport_height: Option<QueryLength>,
    dynamic_viewport_width: Option<QueryLength>,
    dynamic_viewport_height: Option<QueryLength>,
    container_width: Option<QueryLength>,
    container_height: Option<QueryLength>,
    container_inline_size: Option<QueryLength>,
    container_block_size: Option<QueryLength>,
}
```

Add these exact builders and getters for each basis field:

```rust
impl QueryLengthBasis {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn root_font_size(mut self, value: QueryLength) -> Self {
        self.rem = Some(value);
        self
    }

    #[must_use]
    pub fn font_size(mut self, value: QueryLength) -> Self {
        self.em = Some(value);
        self
    }

    #[must_use] pub fn ex_size(mut self, value: QueryLength) -> Self { self.ex = Some(value); self }
    #[must_use] pub fn cap_size(mut self, value: QueryLength) -> Self { self.cap = Some(value); self }
    #[must_use] pub fn ch_size(mut self, value: QueryLength) -> Self { self.ch = Some(value); self }
    #[must_use] pub fn ic_size(mut self, value: QueryLength) -> Self { self.ic = Some(value); self }
    #[must_use] pub fn line_height(mut self, value: QueryLength) -> Self { self.lh = Some(value); self }
    #[must_use] pub fn root_ex_size(mut self, value: QueryLength) -> Self { self.rex = Some(value); self }
    #[must_use] pub fn root_cap_size(mut self, value: QueryLength) -> Self { self.rcap = Some(value); self }
    #[must_use] pub fn root_ch_size(mut self, value: QueryLength) -> Self { self.rch = Some(value); self }
    #[must_use] pub fn root_ic_size(mut self, value: QueryLength) -> Self { self.ric = Some(value); self }
    #[must_use] pub fn root_line_height(mut self, value: QueryLength) -> Self { self.rlh = Some(value); self }
    #[must_use] pub fn viewport_width(mut self, value: QueryLength) -> Self { self.viewport_width = Some(value); self }
    #[must_use] pub fn viewport_height(mut self, value: QueryLength) -> Self { self.viewport_height = Some(value); self }
    #[must_use] pub fn small_viewport_width(mut self, value: QueryLength) -> Self { self.small_viewport_width = Some(value); self }
    #[must_use] pub fn small_viewport_height(mut self, value: QueryLength) -> Self { self.small_viewport_height = Some(value); self }
    #[must_use] pub fn large_viewport_width(mut self, value: QueryLength) -> Self { self.large_viewport_width = Some(value); self }
    #[must_use] pub fn large_viewport_height(mut self, value: QueryLength) -> Self { self.large_viewport_height = Some(value); self }
    #[must_use] pub fn dynamic_viewport_width(mut self, value: QueryLength) -> Self { self.dynamic_viewport_width = Some(value); self }
    #[must_use] pub fn dynamic_viewport_height(mut self, value: QueryLength) -> Self { self.dynamic_viewport_height = Some(value); self }
    #[must_use] pub fn container_width(mut self, value: QueryLength) -> Self { self.container_width = Some(value); self }
    #[must_use] pub fn container_height(mut self, value: QueryLength) -> Self { self.container_height = Some(value); self }
    #[must_use] pub fn container_inline_size(mut self, value: QueryLength) -> Self { self.container_inline_size = Some(value); self }
    #[must_use] pub fn container_block_size(mut self, value: QueryLength) -> Self { self.container_block_size = Some(value); self }

    #[must_use] pub const fn font_size_basis(&self) -> Option<QueryLength> { self.em }
    #[must_use] pub const fn root_font_size_basis(&self) -> Option<QueryLength> { self.rem }
    #[must_use] pub const fn viewport_width_basis(&self) -> Option<QueryLength> { self.viewport_width }
    #[must_use] pub const fn viewport_height_basis(&self) -> Option<QueryLength> { self.viewport_height }
    #[must_use] pub const fn container_inline_size_basis(&self) -> Option<QueryLength> { self.container_inline_size }
}
```

Add the remaining getters using the same pattern:

```rust
ex_size_basis, cap_size_basis, ch_size_basis, ic_size_basis, line_height_basis,
root_ex_size_basis, root_cap_size_basis, root_ch_size_basis, root_ic_size_basis,
root_line_height_basis, small_viewport_width_basis, small_viewport_height_basis,
large_viewport_width_basis, large_viewport_height_basis, dynamic_viewport_width_basis,
dynamic_viewport_height_basis, container_width_basis, container_height_basis,
container_block_size_basis
```

Use conventional absolute CSS conversions for absolute units:

- `1in = 96px`
- `1cm = 96px / 2.54`
- `1mm = 96px / 25.4`
- `1q = 96px / 101.6`
- `1pc = 16px`
- `1pt = 96px / 72`

Relative unit resolution:

- `em`, `ex`, `cap`, `ch`, `ic`, `lh` use their corresponding non-root basis.
- `rem`, `rex`, `rcap`, `rch`, `ric`, `rlh` use their corresponding root basis.
- `vw`, `vh`, `vi`, `vb`, `vmin`, `vmax` use current viewport width/height. In this pass `vi` maps to width and `vb` maps to height because writing-mode-aware media query axes are not modeled yet; document that root may provide already-normalized dimensions if it needs another writing mode.
- `sv*`, `lv*`, `dv*` use small, large, and dynamic viewport bases.
- `cq*` use container bases.
- If a required basis is missing or recursively cannot resolve to px, return `None`.

Range matching must use `Option<f32>` conversion:

- Convert both query value and environment fact to px with the same basis.
- Missing conversion means the feature does not match.
- Do not silently compare values with incompatible unresolved units.

Add equivalent private-field checked types:

- `Resolution { value: f32, unit: ResolutionUnit }`, positive finite, `to_dppx`.
- `Ratio { numerator: f32, denominator: f32 }`, numerator finite and `>= 0.0`, denominator finite and `> 0.0`, `value`.
- `NonNegativeInteger { value: u32 }`.

Specify `ResolutionUnit` exactly:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolutionUnit {
    Dpi,
    Dpcm,
    Dppx,
}
```

`Resolution::to_dppx` must use:

- `Dppx`: `value`
- `Dpi`: `value / 96.0`
- `Dpcm`: `value * 2.54 / 96.0`

Add closed media value enums:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Orientation { Portrait, Landscape }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorSchemePreference { Light, Dark }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReducedMotionPreference { Reduce, NoPreference }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReducedTransparencyPreference { Reduce, NoPreference }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ContrastPreference { NoPreference, More, Less, Custom }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ForcedColorsMode { None, Active }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HoverCapability { None, Hover }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PointerCapability { None, Coarse, Fine }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DisplayMode { Fullscreen, Standalone, MinimalUi, Browser, PictureInPicture }
```

Add `MediaFeatureQuery` and `MediaEnvironment`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum MediaFeatureQuery {
    Width(RangeFeature<QueryLength>),
    Height(RangeFeature<QueryLength>),
    Resolution(RangeFeature<Resolution>),
    Color(RangeFeature<NonNegativeInteger>),
    Monochrome(RangeFeature<NonNegativeInteger>),
    Orientation(Orientation),
    PrefersColorScheme(ColorSchemePreference),
    PrefersReducedMotion(ReducedMotionPreference),
    PrefersReducedTransparency(ReducedTransparencyPreference),
    PrefersContrast(ContrastPreference),
    ForcedColors(ForcedColorsMode),
    Hover(HoverCapability),
    AnyHover(HoverCapability),
    Pointer(PointerCapability),
    AnyPointer(PointerCapability),
    DisplayMode(DisplayMode),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaEnvironment {
    media_type: MediaType,
    width: Option<QueryLength>,
    height: Option<QueryLength>,
    length_basis: QueryLengthBasis,
    resolution: Option<Resolution>,
    color: Option<NonNegativeInteger>,
    monochrome: Option<NonNegativeInteger>,
    orientation: Option<Orientation>,
    prefers_color_scheme: Option<ColorSchemePreference>,
    prefers_reduced_motion: Option<ReducedMotionPreference>,
    prefers_reduced_transparency: Option<ReducedTransparencyPreference>,
    prefers_contrast: Option<ContrastPreference>,
    forced_colors: Option<ForcedColorsMode>,
    hover: Option<HoverCapability>,
    any_hover: Option<HoverCapability>,
    pointer: Option<PointerCapability>,
    any_pointer: Option<PointerCapability>,
    display_mode: Option<DisplayMode>,
}
```

Builder methods should consume and return `Self`:

```rust
impl MediaEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn media_type(mut self, media_type: MediaType) -> Self {
        self.media_type = media_type;
        self
    }

    #[must_use]
    pub const fn width(mut self, width: QueryLength) -> Self {
        self.width = Some(width);
        self
    }
}
```

Add these exact builders:

```rust
media_type, width, height, with_length_basis, resolution, color, monochrome,
orientation, prefers_color_scheme, prefers_reduced_motion,
prefers_reduced_transparency, prefers_contrast, forced_colors, hover,
any_hover, pointer, any_pointer, display_mode
```

Add these exact getters:

```rust
media_type, width, height, length_basis, resolution, color, monochrome,
orientation_fact, prefers_color_scheme, prefers_reduced_motion,
prefers_reduced_transparency, prefers_contrast, forced_colors, hover,
any_hover, pointer, any_pointer, display_mode
```

Use `orientation_fact()` for the explicit stored orientation getter; keep
`orientation()` available for the derived value that may fall back to
width/height.

Matching semantics:

- `MediaQueryList` is OR over queries.
- `Condition` queries are evaluated directly.
- typed media query `Only` behaves like no modifier.
- typed media query `Not` negates media-type + optional condition.
- `MediaType::All` matches every environment.
- `MediaType::Screen` and `Print` match only the environment media type.
- `MediaCondition::And` requires all children.
- `MediaCondition::Or` requires any child.
- `MediaCondition::Not` negates one child.
- Range feature with `None` comparison is a presence/equality check: it matches if the environment has that fact and equals the query value after basis-driven unit normalization.
- Range feature comparison variants use normalized values.
- Missing environment facts do not match feature queries.
- Orientation may be explicit from facts; if absent and both width/height exist, derive `Landscape` when width >= height, otherwise `Portrait`.

- [ ] **Step 4: Add compile-fail tests**

Create `tests/compile_fail/invalid_media_query_list_empty_literal.rs`:

```rust
use surgeist_style::MediaQueryList;

fn main() {
    let _list = MediaQueryList { queries: Vec::new() };
}
```

Create `tests/compile_fail/invalid_query_length_literal.rs`:

```rust
use surgeist_style::{QueryLength, QueryLengthUnit};

fn main() {
    let _length = QueryLength {
        value: 1.0,
        unit: QueryLengthUnit::Px,
    };
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_media_query_list_empty_literal.stderr tests/compile_fail/invalid_query_length_literal.stderr
```

Expected: stderr files show private-field errors.

- [ ] **Step 5: Reexport media types**

In `src/lib.rs`, keep the existing `Viewport` and `Container` reexports for the
temporary Task 1/2 bridge, and add the new media types:

```rust
pub use condition::{
    ColorSchemePreference, Condition, ContrastPreference, DisplayMode, ForcedColorsMode,
    HoverCapability, MediaCondition, MediaConditionList, MediaEnvironment, MediaFeatureQuery,
    MediaQuery, MediaQueryList, MediaQueryModifier, MediaType, NonNegativeInteger, Orientation,
    PointerCapability, QueryComparison, QueryLength, QueryLengthBasis, QueryLengthUnit, RangeFeature,
    ReducedMotionPreference, ReducedTransparencyPreference, Resolution, ResolutionUnit,
};
```

Task 3 removes the old `Viewport` and raw `Container` public reexports.

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo test -p surgeist-style media_query
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/condition.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_media_query_list_empty_literal.rs tests/compile_fail/invalid_media_query_list_empty_literal.stderr tests/compile_fail/invalid_query_length_literal.rs tests/compile_fail/invalid_query_length_literal.stderr
git commit -m "style: add media query facts"
```

---

### Task 2: Add Container Query And Container Fact Models

**Files:**

- Modify: `src/condition.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_container_name_literal.rs`
- Create: `tests/compile_fail/invalid_container_name_literal.stderr`
- Create: `tests/compile_fail/invalid_container_condition_list_literal.rs`
- Create: `tests/compile_fail/invalid_container_condition_list_literal.stderr`

- [ ] **Step 1: Add container tests first**

Add tests in `src/condition.rs`:

```rust
#[test]
fn container_conditions_match_named_container_facts() {
    let name = ContainerName::try_new("sidebar").unwrap();
    let condition = ContainerCondition::Feature(ContainerFeatureQuery::InlineSize(
        RangeFeature::new(
            Some(QueryComparison::GreaterThanOrEqual),
            QueryLength::try_new(320.0, QueryLengthUnit::Px).unwrap(),
        ),
    ));
    let facts = ContainerFacts::new()
        .name(name.clone())
        .inline_size(QueryLength::try_new(400.0, QueryLengthUnit::Px).unwrap())
        .block_size(QueryLength::try_new(600.0, QueryLengthUnit::Px).unwrap());

    assert!(condition.matches(&facts));
    assert_eq!(facts.name(), Some(&name));
}

#[test]
fn container_style_queries_match_custom_property_facts() {
    let name = CustomPropertyName::try_new("--theme").unwrap();
    let value = AuthoredTokens::new("dark");
    let facts = ContainerFacts::new().custom_property(name.clone(), value.clone());

    assert!(ContainerCondition::Style(ContainerStyleQuery::CustomPropertyPresence(
        name.clone(),
    ))
    .matches(&facts));
    assert!(ContainerCondition::Style(ContainerStyleQuery::CustomPropertyValue {
        name,
        value,
    })
    .matches(&facts));
}

#[test]
fn container_conditions_require_two_children_for_and_or() {
    assert!(ContainerConditionList::try_new([ContainerCondition::Feature(
        ContainerFeatureQuery::Orientation(Orientation::Portrait),
    )])
    .is_err());
}
```

Run:

```sh
cargo test -p surgeist-style container_conditions
```

Expected: fail until implementation exists.

- [ ] **Step 2: Add container name and facts**

In `src/condition.rs`, add:

```rust
use std::collections::BTreeMap;
use crate::{AuthoredTokens, CustomPropertyName};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContainerName {
    value: String,
}

impl ContainerName {
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        validate_condition_ident(&value, "container name")?;
        if matches!(
            value.to_ascii_lowercase().as_str(),
            "none" | "and" | "or" | "not" | "style"
        ) {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "container name is reserved",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
```

`validate_condition_ident` should be style-owned and intentionally narrower than CSS parsing:

```rust
fn validate_condition_ident(value: &str, label: &'static str) -> Result<()> {
    if value.is_empty() || value.contains('\0') {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{label} cannot be empty or contain U+0000"),
        ));
    }
    if matches!(
        value.to_ascii_lowercase().as_str(),
        "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    ) {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{label} cannot be a CSS-wide keyword"),
        ));
    }
    Ok(())
}
```

Do not use `cssparser` in style.

Add `ContainerFacts`:

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerFacts {
    name: Option<ContainerName>,
    width: Option<QueryLength>,
    height: Option<QueryLength>,
    inline_size: Option<QueryLength>,
    block_size: Option<QueryLength>,
    length_basis: QueryLengthBasis,
    aspect_ratio: Option<Ratio>,
    orientation: Option<Orientation>,
    custom_properties: BTreeMap<CustomPropertyName, AuthoredTokens>,
}
```

Add these exact builders:

```rust
name, width, height, inline_size, block_size, with_length_basis, aspect_ratio,
orientation, custom_property
```

Add these exact getters:

```rust
name, width, height, inline_size, block_size, length_basis, aspect_ratio,
orientation_fact, custom_property, custom_properties
```

Use `orientation_fact()` for the explicit stored orientation getter; keep
`orientation()` available for the derived value that may fall back to
width/height. `width` and `height` are physical facts. `inline_size` and
`block_size` are root-provided logical facts; do not infer writing mode in style
in this pass. When matching container length range features, use the container
fact's `QueryLengthBasis`; missing bases make the feature not match.

- [ ] **Step 3: Add container condition tree**

Add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum ContainerCondition {
    Feature(ContainerFeatureQuery),
    Style(ContainerStyleQuery),
    Not(Box<ContainerCondition>),
    And(ContainerConditionList),
    Or(ContainerConditionList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContainerConditionList {
    conditions: Vec<ContainerCondition>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContainerFeatureQuery {
    Width(RangeFeature<QueryLength>),
    Height(RangeFeature<QueryLength>),
    InlineSize(RangeFeature<QueryLength>),
    BlockSize(RangeFeature<QueryLength>),
    AspectRatio(RangeFeature<Ratio>),
    Orientation(Orientation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContainerStyleQuery {
    CustomPropertyPresence(CustomPropertyName),
    CustomPropertyValue {
        name: CustomPropertyName,
        value: AuthoredTokens,
    },
}
```

Matching semantics:

- `Not`, `And`, `Or` mirror media conditions.
- Missing facts do not match feature queries.
- If `orientation` is absent and `width`/`height` are present, derive orientation from width >= height.
- `Style(CustomPropertyPresence)` matches if `ContainerFacts` contains the name.
- `Style(CustomPropertyValue)` matches exact `AuthoredTokens` equality.

Add `Condition::container(condition: ContainerCondition) -> Self` and update `Condition::is_container`.

Remove old `Container` from public exports and internal use where this task can do so without resolver changes. If resolver still needs a compiling temporary field before Task 3, keep it private only and remove in Task 3.

- [ ] **Step 4: Add compile-fail tests**

Create `tests/compile_fail/invalid_container_name_literal.rs`:

```rust
use surgeist_style::ContainerName;

fn main() {
    let _name = ContainerName {
        value: String::new(),
    };
}
```

Create `tests/compile_fail/invalid_container_condition_list_literal.rs`:

```rust
use surgeist_style::ContainerConditionList;

fn main() {
    let _list = ContainerConditionList {
        conditions: Vec::new(),
    };
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_container_name_literal.stderr tests/compile_fail/invalid_container_condition_list_literal.stderr
```

Expected: private-field failures.

- [ ] **Step 5: Update compile-pass public construction**

In `tests/compile_pass/typed_public_construction.rs`, add construction that uses:

```rust
let media = MediaQueryList::try_new([MediaQuery::Condition(MediaCondition::Feature(
    MediaFeatureQuery::Width(RangeFeature::new(
        Some(QueryComparison::GreaterThanOrEqual),
        QueryLength::try_new(640.0, QueryLengthUnit::Px)?,
    )),
))])?;

let container_name = ContainerName::try_new("sidebar")?;
let container_condition = ContainerCondition::And(ContainerConditionList::try_new([
    ContainerCondition::Feature(ContainerFeatureQuery::InlineSize(RangeFeature::new(
        Some(QueryComparison::GreaterThanOrEqual),
        QueryLength::try_new(320.0, QueryLengthUnit::Px)?,
    ))),
    ContainerCondition::Style(ContainerStyleQuery::CustomPropertyPresence(
        CustomPropertyName::try_new("--theme")?,
    )),
])?);

let container_facts = ContainerFacts::new()
    .name(container_name)
    .inline_size(QueryLength::try_new(400.0, QueryLengthUnit::Px)?)
    .with_length_basis(QueryLengthBasis::new().container_inline_size(
        QueryLength::try_new(400.0, QueryLengthUnit::Px)?,
    ))
    .custom_property(CustomPropertyName::try_new("--theme")?, AuthoredTokens::new("dark"));

let _ = (media, container_condition, container_facts);
```

- [ ] **Step 6: Reexport container types**

In `src/lib.rs`, export:

```rust
ContainerCondition, ContainerConditionList, ContainerFacts, ContainerFeatureQuery,
ContainerName, ContainerStyleQuery,
```

Remove `Container` from public exports.

- [ ] **Step 7: Run focused checks**

```sh
cargo test -p surgeist-style container_conditions
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 8: Commit after worker/reviewer clean**

```sh
git add src/condition.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_container_name_literal.rs tests/compile_fail/invalid_container_name_literal.stderr tests/compile_fail/invalid_container_condition_list_literal.rs tests/compile_fail/invalid_container_condition_list_literal.stderr
git commit -m "style: add container query facts"
```

---

### Task 3: Wire Conditions Through Rules, Resolver, And Cache Keys

**Files:**

- Modify: `src/condition.rs`
- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Update: affected condition/resolver/sheet tests

- [ ] **Step 1: Add resolver-facing condition tests**

Add tests in `src/resolver.rs`:

```rust
#[test]
fn media_condition_rules_match_environment_facts() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let query = MediaQueryList::try_new([MediaQuery::Condition(MediaCondition::Feature(
        MediaFeatureQuery::Width(RangeFeature::new(
            Some(QueryComparison::GreaterThanOrEqual),
            QueryLength::try_new(640.0, QueryLengthUnit::Px).unwrap(),
        )),
    ))])
    .unwrap();
    let sheet = Sheet::new().conditional_rule(
        Selector::tag("button").unwrap(),
        Declarations::new()
            .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
            .unwrap(),
        [Condition::media(query)],
    );
    let mut resolver = Resolver::new(sheet);

    let matching = resolver
        .resolve(Context::new(&tree, 0).media_environment(
            MediaEnvironment::new().width(QueryLength::try_new(800.0, QueryLengthUnit::Px).unwrap()),
        ))
        .unwrap();
    let non_matching = resolver
        .resolve(Context::new(&tree, 0).media_environment(
            MediaEnvironment::new().width(QueryLength::try_new(320.0, QueryLengthUnit::Px).unwrap()),
        ))
        .unwrap();

    assert_eq!(matching.text_color(), &StyleColor::rgba(Color::raw_rgba(1.0, 0.0, 0.0, 1.0)));
    assert_eq!(non_matching.text_color(), &StyleColor::rgba(Color::BLACK));
}

#[test]
fn container_condition_rules_match_container_facts() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let condition = ContainerCondition::Feature(ContainerFeatureQuery::Width(RangeFeature::new(
        Some(QueryComparison::GreaterThanOrEqual),
        QueryLength::try_new(300.0, QueryLengthUnit::Px).unwrap(),
    )));
    let sheet = Sheet::new().conditional_rule(
        Selector::tag("button").unwrap(),
        Declarations::new()
            .try_concrete_text_color(Color::raw_rgba(0.0, 1.0, 0.0, 1.0))
            .unwrap(),
        [Condition::container(condition)],
    );
    let mut resolver = Resolver::new(sheet);

    let matching = resolver
        .resolve(Context::new(&tree, 0).container_facts(
            ContainerFacts::new().width(QueryLength::try_new(320.0, QueryLengthUnit::Px).unwrap()),
        ))
        .unwrap();
    let missing = resolver.resolve(Context::new(&tree, 0)).unwrap();

    assert_eq!(matching.text_color(), &StyleColor::rgba(Color::raw_rgba(0.0, 1.0, 0.0, 1.0)));
    assert_eq!(missing.text_color(), &StyleColor::rgba(Color::BLACK));
}
```

Run:

```sh
cargo test -p surgeist-style condition_rules_match
```

Expected: fail until resolver wiring is changed.

- [ ] **Step 2: Update `Condition` matching front door**

In `src/condition.rs`, implement:

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConditionFacts {
    media: MediaEnvironment,
    container: Option<ContainerFacts>,
}

impl ConditionFacts {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn media(mut self, media: MediaEnvironment) -> Self {
        self.media = media;
        self
    }

    #[must_use]
    pub fn container(mut self, container: ContainerFacts) -> Self {
        self.container = Some(container);
        self
    }

    #[must_use]
    pub const fn media_environment(&self) -> &MediaEnvironment {
        &self.media
    }

    #[must_use]
    pub const fn container_facts(&self) -> Option<&ContainerFacts> {
        self.container.as_ref()
    }
}

impl Condition {
    #[must_use]
    pub fn matches(&self, facts: &ConditionFacts) -> bool {
        match self {
            Self::Media(query) => query.matches(facts.media_environment()),
            Self::Container(condition) => facts
                .container_facts()
                .is_some_and(|container| condition.matches(container)),
        }
    }

    #[must_use]
    pub fn matches_all(conditions: &[Self], facts: &ConditionFacts) -> bool {
        conditions.iter().all(|condition| condition.matches(facts))
    }
}
```

- [ ] **Step 3: Update `Context`**

In `src/resolver.rs`, replace:

```rust
pub viewport: Viewport,
pub container: Option<Container>,
```

with:

```rust
condition_facts: ConditionFacts,
```

Add builder/getter methods:

```rust
#[must_use]
pub const fn condition_facts(mut self, facts: ConditionFacts) -> Self {
    self.condition_facts = facts;
    self
}

#[must_use]
pub fn media_environment(mut self, media: MediaEnvironment) -> Self {
    self.condition_facts = self.condition_facts.media(media);
    self
}

#[must_use]
pub fn container_facts(mut self, container: ContainerFacts) -> Self {
    self.condition_facts = self.condition_facts.container(container);
    self
}

#[must_use]
pub const fn conditions(&self) -> &ConditionFacts {
    &self.condition_facts
}
```

In `src/lib.rs`, add `ConditionFacts` to the `condition` reexports in this task.

In `tests/compile_pass/typed_public_construction.rs`, add:

```rust
let condition_facts = ConditionFacts::new()
    .media(MediaEnvironment::new().width(QueryLength::try_new(800.0, QueryLengthUnit::Px)?))
    .container(ContainerFacts::new().width(QueryLength::try_new(320.0, QueryLengthUnit::Px)?));
assert!(condition_facts.container_facts().is_some());
```

Remove `viewport(...)` and `container(...)` public builders; no compatibility aliases.

- [ ] **Step 4: Update resolver matching and cache key**

In `Resolver::resolve`, replace:

```rust
if !Condition::matches_all(rule.conditions(), context.viewport, context.container) {
    continue;
}
```

with:

```rust
if !Condition::matches_all(rule.conditions(), context.conditions()) {
    continue;
}
```

In `cache_key`, replace `viewport.cache_values()` and `container.cache_values()` with stable hash helpers:

```rust
hash_condition_facts(context.conditions(), &mut hasher);
```

Implement `Hash` manually or derive `Hash` for fact types where valid. Do not hash raw `f32`; hash `to_bits()` for float-bearing private values. Keep this scoped cache correctness update; do not add Operation 14 invalidation APIs.

- [ ] **Step 5: Update sheet condition invalidation names**

In `src/sheet.rs`, rename:

- `viewport_change()` -> `media_condition_change()`
- `container_change()` -> `container_condition_change()`

Update implementation to use `Condition::is_media` / `Condition::is_container`. Do not add broad environment-delta invalidation yet.

Add tests proving these methods include affected properties for conditional rules.

- [ ] **Step 6: Update all current tests and public construction**

Search:

```sh
rg -n "Viewport|Container::|\\.viewport\\(|\\.container\\(|viewport_change|container_change" src tests
```

Expected after edits:

- no `Viewport`;
- no old `Container` raw fact type;
- no `Context::viewport` or `Context::container`;
- only `ContainerName`, `ContainerFacts`, `ContainerCondition`, and related names remain.

- [ ] **Step 7: Run checks**

```sh
cargo test -p surgeist-style condition
cargo test -p surgeist-style resolver
cargo test -p surgeist-style sheet
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 8: Commit after worker/reviewer clean**

```sh
git add src/condition.rs src/sheet.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: wire condition facts into resolver"
```

---

### Task 4: Add Cascade Layer Names, Registration, And Sheet APIs

**Files:**

- Modify: `src/precedence.rs`
- Modify: `src/sheet.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_style_layer_name_literal.rs`
- Create: `tests/compile_fail/invalid_style_layer_name_literal.stderr`
- Create: `tests/compile_fail/invalid_style_layer_name_list_empty_literal.rs`
- Create: `tests/compile_fail/invalid_style_layer_name_list_empty_literal.stderr`

- [ ] **Step 1: Add layer tests first**

In `src/sheet.rs`, add tests:

```rust
#[test]
fn layer_statements_register_named_layers_in_order() {
    let mut sheet = Sheet::new();
    let reset = StyleLayerName::try_new(["reset"]).unwrap();
    let theme = StyleLayerName::try_new(["theme"]).unwrap();
    sheet.declare_layers(StyleLayerNameList::try_new([reset.clone(), theme.clone()]).unwrap());

    assert_eq!(sheet.layer_order(&reset), Some(LayerOrder::new(1)));
    assert_eq!(sheet.layer_order(&theme), Some(LayerOrder::new(2)));
}

#[test]
fn named_layer_rules_use_registered_layer_order() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let base = StyleLayerName::try_new(["base"]).unwrap();
    let theme = StyleLayerName::try_new(["theme"]).unwrap();
    let mut sheet = Sheet::new();
    sheet.declare_layers(StyleLayerNameList::try_new([base.clone(), theme.clone()]).unwrap());
    sheet
        .push_layer_rule(
            base,
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
                .unwrap(),
        )
        .unwrap();
    sheet
        .push_layer_rule(
            theme,
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(0.0, 0.0, 1.0, 1.0))
                .unwrap(),
        )
        .unwrap();

    let mut resolver = Resolver::new(sheet);
    let resolved = resolver.resolve(Context::new(&tree, 0)).unwrap();

    assert_eq!(resolved.text_color(), &StyleColor::rgba(Color::raw_rgba(0.0, 0.0, 1.0, 1.0)));
}

#[test]
fn anonymous_layer_blocks_get_fresh_order() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let mut sheet = Sheet::new();
    let first = sheet.register_anonymous_layer();
    let second = sheet.register_anonymous_layer();
    sheet
        .push_layer_order_rule(
            first,
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
                .unwrap(),
        )
        .unwrap();
    sheet
        .push_layer_order_rule(
            second,
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(0.0, 1.0, 0.0, 1.0))
                .unwrap(),
        )
        .unwrap();

    assert!(second > first);
    let mut resolver = Resolver::new(sheet);
    let resolved = resolver.resolve(Context::new(&tree, 0)).unwrap();
    assert_eq!(resolved.text_color(), &StyleColor::rgba(Color::raw_rgba(0.0, 1.0, 0.0, 1.0)));
}
```

Run:

```sh
cargo test -p surgeist-style layer_
```

Expected: fail until implementation exists.

- [ ] **Step 2: Add style-owned layer names**

In `src/precedence.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StyleLayerName {
    components: Vec<String>,
}

impl StyleLayerName {
    pub fn try_new(components: impl IntoIterator<Item = impl Into<String>>) -> Result<Self> {
        let components = components.into_iter().map(Into::into).collect::<Vec<_>>();
        if components.is_empty() {
            return Err(Error::new(ErrorCode::InvalidValue, "layer name cannot be empty"));
        }
        for component in &components {
            validate_layer_component(component)?;
        }
        Ok(Self { components })
    }

    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleLayerNameList {
    names: Vec<StyleLayerName>,
}
```

Validation:

- component cannot be empty;
- component cannot contain U+0000;
- component cannot be CSS-wide keyword;
- no `cssparser` dependency;
- do not trim or normalize.

Add `StyleLayerNameList::try_new`, `names()`.

- [ ] **Step 3: Add layer statement/block model and registry**

In `src/precedence.rs`, add:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayerStatement {
    names: StyleLayerNameList,
}

impl LayerStatement {
    #[must_use]
    pub const fn new(names: StyleLayerNameList) -> Self {
        Self { names }
    }

    #[must_use]
    pub const fn names(&self) -> &StyleLayerNameList {
        &self.names
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayerBlock {
    Named(StyleLayerName),
    Anonymous,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LayerRegistry {
    named: BTreeMap<StyleLayerName, LayerOrder>,
    next_order: u32,
}
```

Layer order contract:

- unlayered rules keep `LayerOrder::default()` / `LayerOrder::new(0)`;
- named and anonymous layer orders start at `LayerOrder::new(1)`;
- later/higher layer order outranks earlier layer order and all source order ties, matching existing `RulePrecedence` ordering;
- first registration of a named layer fixes its order;
- subsequent statement or block registration of an existing name returns the existing order;
- anonymous layer blocks always allocate a fresh order.

Add methods:

```rust
impl LayerRegistry {
    #[must_use]
    pub fn new() -> Self;
    pub fn declare(&mut self, statement: &LayerStatement) -> Vec<(StyleLayerName, LayerOrder)>;
    pub fn register_named(&mut self, name: StyleLayerName) -> LayerOrder;
    pub fn register_anonymous(&mut self) -> LayerOrder;
    #[must_use]
    pub fn order(&self, name: &StyleLayerName) -> Option<LayerOrder>;
}
```

- [ ] **Step 4: Add layer APIs on `Sheet`**

Add `layers: LayerRegistry` to `Sheet`.

Add:

```rust
pub fn declare_layers(&mut self, names: StyleLayerNameList) -> &mut Self;
pub fn layer_order(&self, name: &StyleLayerName) -> Option<LayerOrder>;
pub fn register_anonymous_layer(&mut self) -> LayerOrder;
pub fn push_layer_order_rule(
    &mut self,
    layer_order: LayerOrder,
    selector: Selector,
    declarations: Declarations,
) -> Result<&mut Self>;
pub fn push_layer_rule(
    &mut self,
    layer: StyleLayerName,
    selector: Selector,
    declarations: Declarations,
) -> Result<&mut Self>;
pub fn push_authored_layer_rule(
    &mut self,
    layer: StyleLayerName,
    selector: Selector,
    declarations: AuthoredDeclarations,
    source_order: SourceOrder,
) -> Result<&mut Self>;
pub fn push_authored_layer_order_rule(
    &mut self,
    layer_order: LayerOrder,
    selector: Selector,
    declarations: AuthoredDeclarations,
    source_order: SourceOrder,
) -> Result<&mut Self>;
```

Implementation notes:

- `register_anonymous_layer` returns a fresh `LayerOrder` for one anonymous block. Root should call it once per anonymous `@layer { ... }` block, then call `push_layer_order_rule` or `push_authored_layer_order_rule` for each lowered rule in that block.
- `push_layer_order_rule` sets precedence with the supplied `LayerOrder`, current source order, and selector specificity. It must reject `LayerOrder::default()` with `ErrorCode::InvalidValue` so callers use unlayered APIs for unlayered rules.
- `push_layer_rule` registers the layer name if needed, sets precedence with that `LayerOrder`, current source order, and selector specificity.
- `push_authored_layer_rule` converts authored declarations and preserves the supplied `SourceOrder`, but computes layer order from the registry and selector specificity.
- `push_authored_layer_order_rule` converts authored declarations, preserves the supplied `SourceOrder`, and uses the supplied non-default `LayerOrder`.
- Do not add a parser-like nested `Vec<CssRule>` model to style. Root should flatten layer blocks into sheet APIs or call the block registration APIs while lowering.
- Do not alter existing unlayered `push_rule` behavior.

- [ ] **Step 5: Add compile-fail tests**

Create `tests/compile_fail/invalid_style_layer_name_literal.rs`:

```rust
use surgeist_style::StyleLayerName;

fn main() {
    let _name = StyleLayerName {
        components: Vec::new(),
    };
}
```

Create `tests/compile_fail/invalid_style_layer_name_list_empty_literal.rs`:

```rust
use surgeist_style::StyleLayerNameList;

fn main() {
    let _list = StyleLayerNameList { names: Vec::new() };
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
```

Expected: private-field stderr files are generated/updated.

- [ ] **Step 6: Update public exports and compile-pass**

In `src/lib.rs`, export:

```rust
LayerBlock, LayerRegistry, LayerStatement, StyleLayerName, StyleLayerNameList,
```

In `tests/compile_pass/typed_public_construction.rs`, add:

```rust
let base_layer = StyleLayerName::try_new(["base"])?;
let theme_layer = StyleLayerName::try_new(["theme", "buttons"])?;
let layers = StyleLayerNameList::try_new([base_layer.clone(), theme_layer.clone()])?;
let mut sheet = Sheet::new();
sheet.declare_layers(layers);
sheet.push_layer_rule(base_layer, Selector::tag("button")?, Declarations::new())?;
assert!(sheet.layer_order(&theme_layer).is_some());
```

- [ ] **Step 7: Run focused checks**

```sh
cargo test -p surgeist-style layer_
cargo test -p surgeist-style precedence
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 8: Commit after worker/reviewer clean**

```sh
git add src/precedence.rs src/sheet.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_style_layer_name_literal.rs tests/compile_fail/invalid_style_layer_name_literal.stderr tests/compile_fail/invalid_style_layer_name_list_empty_literal.rs tests/compile_fail/invalid_style_layer_name_list_empty_literal.stderr
git commit -m "style: add cascade layer registry"
```

---

### Task 5: Add Scoped Rule Model And Resolver Matching

**Files:**

- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_scope_selector_list_empty_literal.rs`
- Create: `tests/compile_fail/invalid_scope_selector_list_empty_literal.stderr`
- Create: `tests/compile_fail/invalid_rule_scope_literal.rs`
- Create: `tests/compile_fail/invalid_rule_scope_literal.stderr`

- [ ] **Step 1: Add scope tests first**

In `src/resolver.rs`, add:

```rust
#[test]
fn scoped_rules_apply_inside_root_until_limit() {
    let tree = TestTree::new(vec![
        TestNode::new(0, "root").children([1]),
        TestNode::new(1, "section").class("card").children([2, 3]),
        TestNode::new(2, "button"),
        TestNode::new(3, "footer").class("limit").children([4]),
        TestNode::new(4, "button"),
    ]);
    let scope = RuleScope::try_new(
        Some(ScopeSelectorList::try_new([Selector::class("card").unwrap()]).unwrap()),
        Some(ScopeSelectorList::try_new([Selector::class("limit").unwrap()]).unwrap()),
    )
    .unwrap();
    let mut sheet = Sheet::new();
    sheet
        .push_scoped_rule(
            scope,
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
                .unwrap(),
        )
        .unwrap();
    let mut resolver = Resolver::new(sheet);

    let inside = resolver.resolve(Context::new(&tree, 2)).unwrap();
    let outside_limit = resolver.resolve(Context::new(&tree, 4)).unwrap();

    assert_eq!(inside.text_color(), &StyleColor::rgba(Color::raw_rgba(1.0, 0.0, 0.0, 1.0)));
    assert_eq!(outside_limit.text_color(), &StyleColor::rgba(Color::BLACK));
}

#[test]
fn scope_anchor_pseudo_class_uses_matched_rule_scope_root() {
    let tree = TestTree::new(vec![
        TestNode::new(0, "root").children([1]),
        TestNode::new(1, "section").class("card").children([2]),
        TestNode::new(2, "button"),
    ]);
    let scope = RuleScope::try_new(
        Some(ScopeSelectorList::try_new([Selector::class("card").unwrap()]).unwrap()),
        None,
    )
    .unwrap();
    let selector = Selector::complex([
        ComplexSelectorPart::root(Selector::compound().scope_anchor()),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().tag("button").unwrap(),
        ),
    ])
    .unwrap();
    let mut sheet = Sheet::new();
    sheet
        .push_scoped_rule(
            scope,
            selector,
            Declarations::new()
                .try_concrete_text_color(Color::raw_rgba(0.0, 1.0, 0.0, 1.0))
                .unwrap(),
        )
        .unwrap();
    let mut resolver = Resolver::new(sheet);

    let resolved = resolver.resolve(Context::new(&tree, 2)).unwrap();

    assert_eq!(resolved.text_color(), &StyleColor::rgba(Color::raw_rgba(0.0, 1.0, 0.0, 1.0)));
}
```

Run:

```sh
cargo test -p surgeist-style scoped_rules
```

Expected: fail until model and resolver wiring exist.

- [ ] **Step 2: Add scope data types**

In `src/sheet.rs` or a new focused `src/scope.rs` if the file becomes too large, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ScopeSelectorList {
    selectors: Vec<Selector>,
}

impl ScopeSelectorList {
    pub fn try_new(selectors: impl IntoIterator<Item = Selector>) -> Result<Self> {
        let selectors = selectors.into_iter().collect::<Vec<_>>();
        if selectors.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidSelector,
                "scope selector list cannot be empty",
            ));
        }
        Ok(Self { selectors })
    }

    #[must_use]
    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuleScope {
    roots: Option<ScopeSelectorList>,
    limits: Option<ScopeSelectorList>,
}
```

Add:

```rust
impl RuleScope {
    pub fn try_new(
        roots: Option<ScopeSelectorList>,
        limits: Option<ScopeSelectorList>,
    ) -> Result<Self> {
        Ok(Self { roots, limits })
    }

    #[must_use]
    pub const fn roots(&self) -> Option<&ScopeSelectorList> {
        self.roots.as_ref()
    }

    #[must_use]
    pub const fn limits(&self) -> Option<&ScopeSelectorList> {
        self.limits.as_ref()
    }
}
```

Validation notes:

- `ScopeSelectorList` is nonempty.
- Selectors are already style-owned and validated.
- Do not permit pseudo-element scoped roots in style if a direct check exists; if not, state in code comments that root lowering is responsible for excluding pseudo-elements until selector exposes this predicate.

- [ ] **Step 3: Store optional scope on `Rule`**

Add `scope: Option<RuleScope>` to `Rule`.

Default `Rule` constructors set `scope: None`.

Add:

```rust
impl Rule {
    #[must_use]
    pub fn scoped(mut self, scope: RuleScope) -> Self {
        self.scope = Some(scope);
        self
    }

    #[must_use]
    pub const fn scope(&self) -> Option<&RuleScope> {
        self.scope.as_ref()
    }
}
```

Add `Sheet` APIs:

```rust
pub fn push_scoped_rule(
    &mut self,
    scope: RuleScope,
    selector: Selector,
    declarations: Declarations,
) -> Result<&mut Self>;

pub fn push_authored_scoped_rule(
    &mut self,
    scope: RuleScope,
    selector: Selector,
    declarations: AuthoredDeclarations,
    precedence: RulePrecedence,
) -> Result<&mut Self>;
```

`push_scoped_rule` should assign source order exactly like `push_rule` and preserve selector specificity.

- [ ] **Step 4: Implement scope matching helper**

In `src/resolver.rs`, add a helper:

```rust
fn matching_scope_anchor<T: Tree>(
    scope: &RuleScope,
    tree: &T,
    subject: T::Id,
    traversal: Traversal,
    explicit_root: Option<T::Id>,
) -> Result<Option<T::Id>>
where
    T::Id: Copy + Eq,
```

Semantics:

- Walk from `subject` through ancestors using `tree.parent(id, traversal)`.
- A root candidate matches if:
  - `scope.roots()` is `None` and the candidate is `explicit_root` when provided, or the topmost ancestor when not provided;
  - otherwise any root selector matches the candidate.
- A limit candidate blocks the scope if any limit selector matches a node strictly between the root and subject, or the subject itself. This means descendants inside a limit subtree are outside the scope.
- Return the nearest matching root that is not blocked by a limit.
- Use `SelectorMatchContext::new(candidate, traversal)` plus explicit root when matching root/limit selectors.

- [ ] **Step 5: Wire resolver selector context**

In `Resolver::resolve`, before selector matching:

```rust
let mut selector_context = SelectorMatchContext::new(context.node, context.traversal);
if let Some(root) = context.selector_root {
    selector_context = selector_context.with_root(root);
}
if let Some(scope) = context.selector_scope {
    selector_context = selector_context.with_scope(scope);
}
if let Some(rule_scope) = rule.scope() {
    let Some(anchor) = matching_scope_anchor(
        rule_scope,
        context.tree,
        context.node,
        context.traversal,
        context.selector_root,
    )? else {
        continue;
    };
    selector_context = selector_context.with_scope(anchor);
}
```

Rule-local scope should override `Context::selector_scope` for the actual scoped rule anchor. Keep `Context::selector_scope` for callers resolving already-scoped subtrees manually.

- [ ] **Step 6: Update cache key narrowly**

Because scoped rules depend on `selector_root` and the tree path, do not implement Operation 14 broad cache invalidation here. Existing cache key already includes sheet version, tree version, node identity, traversal, selector root, and selector scope. Add a test documenting that `RuleScope` behavior depends on tree version and node identity; do not add a new cache axis unless a failing test shows current cache keys are insufficient.

- [ ] **Step 7: Add compile-fail tests**

Create `tests/compile_fail/invalid_scope_selector_list_empty_literal.rs`:

```rust
use surgeist_style::ScopeSelectorList;

fn main() {
    let _list = ScopeSelectorList {
        selectors: Vec::new(),
    };
}
```

Create `tests/compile_fail/invalid_rule_scope_literal.rs`:

```rust
use surgeist_style::RuleScope;

fn main() {
    let _scope = RuleScope {
        roots: None,
        limits: None,
    };
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
```

Expected: private-field stderr files are generated/updated.

- [ ] **Step 8: Reexport and update compile-pass**

In `src/lib.rs`, export:

```rust
RuleScope, ScopeSelectorList,
```

In `tests/compile_pass/typed_public_construction.rs`, add:

```rust
let scope = RuleScope::try_new(
    Some(ScopeSelectorList::try_new([Selector::class("card")?])?),
    Some(ScopeSelectorList::try_new([Selector::class("limit")?])?),
)?;
let mut sheet = Sheet::new();
sheet.push_scoped_rule(scope, Selector::tag("button")?, Declarations::new())?;
```

- [ ] **Step 9: Run focused checks**

```sh
cargo test -p surgeist-style scoped_rules
cargo test -p surgeist-style scope
cargo test -p surgeist-style resolver
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 10: Commit after worker/reviewer clean**

```sh
git add src/sheet.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_scope_selector_list_empty_literal.rs tests/compile_fail/invalid_scope_selector_list_empty_literal.stderr tests/compile_fail/invalid_rule_scope_literal.rs tests/compile_fail/invalid_rule_scope_literal.stderr
git commit -m "style: add scoped rule matching"
```

---

### Task 6: Rebase Ledgers And Record Next Context

**Files:**

- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md` only if Operation 13 changes property-row next context text

- [ ] **Step 1: Rebase Operation 13 condition/layer/scope ledger rows**

Update only the relevant rows in `plans/2026-07-05-css-surface-style-ledger.md`:

- `CssRule::LayerStatement`: existing style model, target `LayerStatement` + `LayerRegistry`.
- `CssRule::LayerBlock`: existing style model, target `LayerBlock` + `LayerRegistry` + sheet layer rule APIs.
- `CssRule::Media`: existing style model, target `Condition::Media(MediaQueryList)`.
- `CssRule::Container`: existing style model, target `Condition::Container(ContainerCondition)` + `ContainerFacts`.
- `CssRule::Scope`: existing style model, target `RuleScope` + `ScopeSelectorList`.
- `:scope`: existing style model, target `SelectorMatchContext::with_scope` plus `RuleScope` resolver matching.
- Condition, layer, scope, and environment rows from the condition/layer/scope ledger should reflect the new style-owned models.

Do not mark Operation 14 cache/invalidation rows complete. Keep cache/invalidation generalization honestly deferred.

- [ ] **Step 2: Update next context**

Add or update trailing context saying:

```markdown
After Operation 13 lands, the next implementation plan should cover Operation 14: resolver cache keys, invalidation, and diagnostic reporting integration over the now-real authored/property/selector/condition/layer/scope models.
```

- [ ] **Step 3: Run ledger searches**

Run:

```sh
rg -n "Operation 13|Operation 14|Media query|Container condition|LayerStatement|LayerBlock|RuleScope|ScopeSelectorList|LayerRegistry|Condition::Media|Condition::Container" plans
```

Expected: Operation 13 rows point to implemented models and Operation 14 remains the next context.

- [ ] **Step 4: Run checks**

```sh
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 5: Commit after worker/reviewer clean**

```sh
git add plans/2026-07-05-css-surface-style-ledger.md plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: rebase condition layer scope ledger"
```

If `plans/2026-07-05-css-property-coverage-ledger.md` is unchanged, omit it from `git add`.

---

## Final Verification

Run after all task commits and final holistic review:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
! rg -n "Viewport|Container::|\\.viewport\\(|\\.container\\(" src tests
git diff --check
git status --short --branch
```

Expected:

- full tests pass;
- clippy passes with `-D warnings`;
- no direct `surgeist-css` or `surgeist-text` coupling;
- no stale `Viewport`/old `Container` API references;
- status is clean except expected unpushed commits.

Assign a final holistic clean-context reviewer against:

- this plan;
- `AGENTS.md`;
- `guidance/surgeist-rust-modeling-guide.md`;
- the full implementation diff.

Reviewer must explicitly check:

- style-owned models, not CSS wrappers;
- invariant-bearing types have private fields and checked constructors;
- media/container/layer/scope models do not sneak in host querying, CSS parsing, or import loading;
- condition matching and scoped rule matching are covered by meaningful tests;
- layer ordering is concrete and matches the project decision that layer order outranks source order;
- Operation 14 remains deferred except for cache-key correctness required by current resolver behavior.

## This Comes Next

After this Operation 13 plan is implemented and reviewed clean, write the Operation 14 implementation plan for resolver cache keys, invalidation, and diagnostic reporting integration. Operation 14 should use the real models from Operations 8-13 instead of guessing generic invalidation axes early.
