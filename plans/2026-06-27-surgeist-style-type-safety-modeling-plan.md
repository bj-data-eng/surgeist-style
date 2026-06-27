# Surgeist Style Type Safety Modeling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `surgeist-style` type safe by moving invariants into semantic public types, typed declaration front doors, and explicit conversion boundaries.

**Architecture:** The primary public authoring API becomes property-specific typed construction. Generic `Property` plus `Value` construction remains only as an explicitly fallible parser/interop escape hatch, not the ergonomic path. Public API breakage is acceptable where it removes ordinary construction of invalid style values.

**Tech Stack:** Rust 2024, crate-local unit tests, `trybuild` compile-fail tests, `surgeist-layout`, `surgeist-retained`, `surgeist-text`, and the existing API artifact generator.

---

## Modeling Decisions

The implementation must satisfy these decisions from the user and reviewers:

- Type safety means invalid style states are not ordinary public values.
- Runtime validation alone is not enough for mismatched property/value pairs.
- `Value` must not remain an unexamined public transport bag.
- Every `lib.rs` reexport owned by this crate must be audited, not only `Property` and `Value`.
- Raw primitive payloads are allowed only when the primitive has no style-owned invariant or unit meaning.
- String and list payloads need semantic wrappers when style owns name/list validation.
- Authored, normalized, and resolved phases must either have distinct types or a written per-domain rationale that their invariants are identical.
- Calc must remain symbolic through style and lower through `lower_with_store` when the target layout domain supports calc.

## Expected File Structure

- Modify: `src/value.rs`
  - Add semantic units, numeric values, names, non-empty list wrappers, grid/subgrid wrappers, text/list wrappers, and private-field composite types.
- Modify: `src/calc.rs`
  - Replace raw calc units and empty public sums with typed units and non-empty/fallible sum construction.
- Modify: `src/declaration.rs`
  - Add typed declaration front doors and demote generic property/value construction to explicit fallible interop.
- Modify: `src/property.rs`
  - Convert property metadata and validation to typed domains; make metadata read-only from public API.
- Modify: `src/resolver.rs`
  - Decide and implement resolved phase front doors.
- Modify: `src/adapters/layout.rs`
  - Lower semantic style values into layout types through narrow helpers.
- Modify: `src/error.rs`
  - Add structured error detail or semantic error constructors that tests can inspect without relying primarily on prose substrings.
- Modify: `src/lib.rs`
  - Reexport only intentional front-door types.
- Modify: `api/public-api.txt`
  - Refresh after intentional API changes.
- Create/modify: `tests/type_safety.rs`, `tests/compile_fail/*.rs`, `tests/compile_pass/*.rs`
  - Prove invalid public construction does not compile.
- Create: `plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md`
  - Source-derived public-surface, property/value, primitive-payload, and phase inventory.
- Create: `plans/2026-06-27-surgeist-style-type-safety-api-report.md`
  - Cross-crate readiness report.

## Non-Goals

- Do not edit sibling crates from this repo.
- Do not change CSS parser behavior in `surgeist-css`; report required parser-lowering updates in the final API report.
- Do not move layout, text shaping, rendering, or retained-tree ownership into style.
- Do not preserve unchecked public convenience APIs when they conflict with type safety.

## Task 1: Add Type Safety Regression Harness

**Files:**
- Modify: `Cargo.toml`
- Create: `tests/type_safety.rs`
- Create: `tests/compile_fail/invalid_public_construction.rs`
- Create: `tests/compile_fail/property_value_mismatch.rs`
- Create: `tests/compile_fail/invalid_names_and_lists.rs`
- Create: `tests/compile_fail/invalid_calc_construction.rs`
- Create: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add `trybuild`**

Add:

```toml
[dev-dependencies]
trybuild = "1"
```

- [ ] **Step 2: Add the test runner**

Create `tests/type_safety.rs`:

```rust
#[test]
fn public_type_safety_contract() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/compile_fail/*.rs");
    tests.pass("tests/compile_pass/*.rs");
}
```

- [ ] **Step 3: Add invalid construction fixtures**

Create `tests/compile_fail/invalid_public_construction.rs`:

```rust,compile_fail
use surgeist_style::{
    Declaration, GridLine, GridTrackComponent, Property, TrackRepeat, TrackRepeatCount, Value,
};

fn main() {
    let _declaration = Declaration {
        property: Property::Opacity,
        value: Value::Number(2.0),
    };

    let _line = GridLine::Line(0);

    let _repeat = TrackRepeat {
        count: TrackRepeatCount::Count(0),
        components: Vec::<GridTrackComponent>::new(),
    };
}
```

Create `tests/compile_fail/property_value_mismatch.rs`:

```rust,compile_fail
use surgeist_style::{Color, Declarations, Property, Value};

fn main() {
    let mut declarations = Declarations::new();
    let _ = declarations.insert(Property::Width, Value::Color(Color::BLACK));
}
```

Create `tests/compile_fail/invalid_names_and_lists.rs`:

```rust,compile_fail
use surgeist_style::{
    GridLine, GridTrackComponent, SubgridLineNameComponent, SubgridLineNameRepeatCount, Value,
};

fn main() {
    let _line = GridLine::BareIdent(String::new());
    let _names = GridTrackComponent::LineNames(vec![String::new()]);
    let _subgrid = SubgridLineNameComponent::Repeat {
        count: SubgridLineNameRepeatCount::Count(0),
        line_name_sets: vec![],
    };
    let _strings = Value::StringList(vec![String::new()]);
}
```

Create `tests/compile_fail/invalid_typed_names_after_replacement.rs` after Task 5 introduces semantic name/list types:

```rust,compile_fail
use surgeist_style::{FontFamilyList, GridLineName};

fn main() {
    let _line_name = GridLineName("");
    let _families = FontFamilyList(vec![]);
}
```

This second fixture must fail because the new wrappers have private fields or because invalid construction is only available through fallible constructors. It must not fail from an unresolved import.

Create `tests/compile_fail/invalid_calc_construction.rs`:

```rust,compile_fail
use surgeist_style::{CalcLength, CalcLengthTerm};

fn main() {
    let _empty = CalcLength::Sum(Vec::<CalcLengthTerm>::new());
    let _term = CalcLengthTerm {
        operator: surgeist_style::CalcOperator::Add,
        value: Box::new(CalcLength::Px(1.0)),
    };
}
```

Expected before implementation: these compile-fail fixtures unexpectedly compile in several places, proving the current public API is too loose.

- [ ] **Step 4: Add typed compile-pass fixture**

Create `tests/compile_pass/typed_public_construction.rs`:

```rust
use surgeist_style::{
    Color, CssPx, Declarations, DimensionLength, Opacity, TypedDeclaration,
};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?));
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::text_color(Color::try_rgba(0.0, 0.0, 0.0, 1.0)?);

    let declarations = Declarations::from_typed([width, opacity, color])?;
    assert_eq!(declarations.len(), 3);
    Ok(())
}
```

Expected before implementation: fails because the typed front-door API does not exist.

- [ ] **Step 5: Run the harness**

```sh
cargo test -p surgeist-style --test type_safety
```

Expected before implementation: FAIL for the expected compile-fail/pass reasons.

## Task 2: Produce A Complete Public Surface And Domain Inventory

**Files:**
- Create: `plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md`
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`

- [ ] **Step 1: Write the source-derived inventory**

Create `plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md` with one row for every `Property::ALL` entry:

```markdown
# surgeist-style Type Safety Domain Inventory

| Property | Authored Type | Normalized Declaration Type | Resolved Type | Public Builder | Notes |
| --- | --- | --- | --- | --- | --- |
| Display | `Display` | `Display` | `Display` | `TypedDeclaration::display` | Closed enum; same invariants across phases. |
| Width | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::width` | Does not accept color, text, or arbitrary length domains. |
| Opacity | `Opacity` | `Opacity` | `Opacity` | `TypedDeclaration::opacity` | Unit interval. |
```

The actual file must include every property in `Property::ALL`. No row may say "same as existing" without naming the type.

- [ ] **Step 2: Add a `Value` variant inventory section**

Append:

```markdown
## Value Variant Decisions

| Variant | Keep Public? | Single Semantic Domain | Replacement/Reason |
| --- | --- | --- | --- |
| `Number` | No | No | Replace with `Opacity`, `AspectRatio`, `DurationSeconds`, `NonNegativeNumber`, `ZIndex`, or property-specific types. |
| `StringList` | No | No | Replace with `FontFamilyList`, `AnimationNameList`, or other named containers. |
| `Length` | Only if justified | Maybe | Prefer context wrappers such as `DimensionLength`, `EdgeLength`, `GapLength`, `FontSizeLength`. |
```

The implementation must update this table with every surviving `Value` variant and explain why it is not a broad transport bag.

- [ ] **Step 3: Add a public reexport surface section**

Append a table for every `surgeist-style` owned public reexport from `src/lib.rs`:

```markdown
## Public Reexport Surface Decisions

| Reexport | Owning File | Invariant/Unit Risk | Action |
| --- | --- | --- | --- |
| `Viewport` | `src/condition.rs` | Raw `f32` width/height query values | Replace with semantic viewport/container size values or document finite-only query boundary. |
| `Container` | `src/condition.rs` | Raw `f32` width/height query values | Same as `Viewport`. |
| `Selector` | `src/selector.rs` | `Complex(Vec<Part>)` direct invalid construction | Keep fields/variants private or require fallible constructors. |
| `Invalidation` / `Scope` / `Change` | `src/invalidation.rs` | Public boolean bit structs | Keep only if any bool combination is semantically valid; otherwise add constructors/builders. |
| `Metadata` / `Impact` | `src/property.rs` | Public mutable fields and arbitrary defaults | Make read-only/private-field. |
```

The actual table must include every reexport in `src/lib.rs` that is owned by this crate. Reexports from sibling crates such as `StateFlag` and text enums may be listed as external-owned with no local modeling change.

- [ ] **Step 4: Add a raw primitive payload and setter inventory**

Append:

```markdown
## Public Primitive Payload And Setter Decisions

| Public Path | Primitive | Meaning | Action |
| --- | --- | --- | --- |
| `Length::Px(f32)` | `f32` | CSS px | Replace payload with `CssPx`. |
| `Length::Percent(f32)` | `f32` | CSS percent | Replace payload with `Percent`. |
| `MaxTrackSizing::Flex(f32)` | `f32` | fractional track unit | Replace payload with `Fr`. |
| `GridFlowTolerance::Percent(f32)` | `f32` | percentage tolerance | Replace payload with `Percent` or tolerance-specific wrapper. |
| `Shadow::soft(alpha: f32)` | `f32` | alpha channel | Replace with `Opacity` or `UnitInterval`. |
| `Dash::new/density/phase(f32)` | `f32` | dash density/phase | Replace with semantic wrappers or fallible setters. |
| `Viewport::new(width: f32, height: f32)` | `f32` | viewport dimensions | Replace with finite non-negative size wrappers or fallible constructors. |
| `Container::new(width: f32, height: f32)` | `f32` | container dimensions | Replace with finite non-negative size wrappers or fallible constructors. |
```

The actual inventory must be produced by searching public `f32`, integer, `String`, and `Vec` payloads/setters in `src/`. A worker may not claim Task 4 or Task 8 complete while an unclassified public primitive path remains.

- [ ] **Step 5: Add a phase-boundary decision before implementation**

Append:

```markdown
## Phase Boundary Decisions

`Declarations` stores normalized typed declarations. Authoring helpers produce `TypedDeclaration`.
`Resolved` may store internal canonical values, but its public front doors are typed accessors.
`Resolved::get(Property) -> &Value` is not the primary public API; either make it crate-private or document it as a fallible/inspection escape hatch after the `Value` inventory is complete.
```

Choose the resolved storage option in this section before changing code:

```markdown
Chosen option:
- [ ] Option A: introduce `ResolvedValue` / `ResolvedStyleValue` and keep authored `Value` out of `Resolved`.
- [ ] Option B: keep one internal canonical value type and make raw `Value` inspection private or explicitly secondary.
```

- [ ] **Step 6: Update code according to the completed inventory**

Implement only inventory-backed domains. Do not add a new public type or keep an old public variant unless it appears in the inventory. Do not begin implementation before the public reexport, primitive payload, and phase-boundary sections are complete.

- [ ] **Step 7: Run inventory checks**

```sh
cargo test -p surgeist-style property
cargo test -p surgeist-style declaration
```

Expected: pass after code updates for the inventory phase.

- [ ] **Step 8: Commit**

```sh
git add plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md src/value.rs src/property.rs src/declaration.rs src/resolver.rs
git commit -m "style: inventory typed style domains"
```

## Task 3: Replace Generic Declaration Front Doors With Typed Declarations

**Files:**
- Modify: `src/declaration.rs`
- Modify: `src/property.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_fail/property_value_mismatch.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add `TypedDeclaration`**

In `src/declaration.rs`, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct TypedDeclaration {
    property: Property,
    value: Value,
}

impl TypedDeclaration {
    #[must_use]
    pub(crate) const fn new_unchecked(property: Property, value: Value) -> Self {
        Self { property, value }
    }

    #[must_use]
    pub const fn property(&self) -> Property {
        self.property
    }

    pub(crate) const fn value(&self) -> &Value {
        &self.value
    }
}
```

`TypedDeclaration::value` is crate-private. If a public raw-value inspection accessor is later required, it must be named as inspection/interop in the domain inventory and API report, not presented as the authoring front door.

- [ ] **Step 2: Add property-specific typed builders**

Add builders for every inventory row. Example shape:

```rust
impl TypedDeclaration {
    #[must_use]
    pub fn display(value: Display) -> Self {
        Self::new_unchecked(Property::Display, Value::Display(value))
    }

    #[must_use]
    pub fn width(value: DimensionLength) -> Self {
        Self::new_unchecked(Property::Width, Value::Dimension(value))
    }

    #[must_use]
    pub fn opacity(value: Opacity) -> Self {
        Self::new_unchecked(Property::Opacity, Value::Opacity(value))
    }
}
```

Do not expose `TypedDeclaration::new(Property, Value)`.

- [ ] **Step 3: Demote generic construction to an explicit fallible escape hatch**

Make `Declaration` private-field and keep only explicit fallible conversion:

```rust
impl Declaration {
    pub fn try_from_raw(property: Property, value: Value) -> Result<Self> {
        property.validate_value(&value)?;
        Ok(Self { property, value })
    }

    pub(crate) const fn new_unchecked(property: Property, value: Value) -> Self {
        Self { property, value }
    }
}
```

Do not keep `Declaration::new(Property, Value)` as the main public constructor.

- [ ] **Step 4: Change `Declarations` to typed insertion by default**

Add:

```rust
impl Declarations {
    pub fn from_typed(declarations: impl IntoIterator<Item = TypedDeclaration>) -> Result<Self> {
        let mut output = Self::new();
        for declaration in declarations {
            output.insert_typed(declaration)?;
        }
        Ok(output)
    }

    pub fn insert_typed(&mut self, declaration: TypedDeclaration) -> Result<&mut Self> {
        for declaration in canonical_declarations_unchecked(declaration.property, declaration.value) {
            self.insert_canonical_unchecked(declaration.property, declaration.value);
        }
        Ok(self)
    }

    pub fn try_insert_raw(&mut self, property: Property, value: Value) -> Result<&mut Self> {
        property.validate_value(&value)?;
        for declaration in canonical_declarations_unchecked(property, value) {
            self.insert_canonical_unchecked(declaration.property, declaration.value);
        }
        Ok(self)
    }
}
```

Remove or rename `insert(Property, Value)` and `set(Property, Value)` so `property_value_mismatch.rs` does not compile.

- [ ] **Step 5: Update shorthand expansion**

Keep shorthand expansion crate-private and unchecked only after typed builders or `try_insert_raw` validate the input:

```rust
fn canonical_declarations_unchecked(property: Property, value: Value) -> Vec<Declaration>
```

Use `Declaration::new_unchecked` inside it.

- [ ] **Step 6: Reexport typed front doors**

Add `TypedDeclaration` and any typed declaration helper aliases to `src/lib.rs`.

- [ ] **Step 7: Run declaration tests**

```sh
cargo test -p surgeist-style declaration
cargo test -p surgeist-style resolver
cargo test -p surgeist-style --test type_safety
```

Expected: `property_value_mismatch.rs` fails to compile because generic insertion is not the ordinary public API.

- [ ] **Step 8: Commit**

```sh
git add src/declaration.rs src/property.rs src/resolver.rs src/lib.rs tests/compile_fail tests/compile_pass
git commit -m "style: add typed declaration front doors"
```

## Task 4: Replace Raw Numeric And Unit Domains

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/adapters/layout.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add semantic numeric and unit types**

Add private-field newtypes with fallible constructors and accessors:

```rust
pub struct CssPx(f32);
pub struct Percent(f32);
pub struct Fr(f32);
pub struct Opacity(f32);
pub struct AspectRatio(f32);
pub struct DurationSeconds(f32);
pub struct NonNegativeNumber(f32);
pub struct ZIndex(i32);
pub struct AngleRadians(f32);
pub struct ScaleFactor(f32);
```

Use `validate_finite`, `validate_non_negative`, unit-interval checks, nonzero checks, or integer constructors according to the domain inventory.

- [ ] **Step 2: Remove raw primitive public constructors, enum payloads, and setters**

Replace or remove these unchecked public paths:

```rust
Color::rgba(f32, f32, f32, f32)
Length::Px(f32)
Length::Percent(f32)
Length::px(f32)
Length::percent(f32)
TrackSizing::px(f32)
TrackSizing::percent(f32)
TrackSizing::fr(f32)
MaxTrackSizing::Flex(f32)
MaxTrackSizing::fr(f32)
GridFlowTolerance::Percent(f32)
Shadow::soft(alpha: f32)
Dash::new(f32)
Dash::density(f32)
Dash::phase(f32)
TransformOp::Scale { x: f32, y: f32 }
TransformOp::Rotate { radians: f32 }
Viewport::new(width: f32, height: f32)
Container::new(width: f32, height: f32)
```

Replacement examples:

```rust
Color::new(r: UnitInterval, g: UnitInterval, b: UnitInterval, a: UnitInterval)
Length::px(CssPx)
Length::percent(Percent)
TrackSizing::fr(Fr)
TransformOp::scale(ScaleFactor, ScaleFactor)
TransformOp::rotate(AngleRadians)
```

`src/lib.rs::color(u32)` may remain infallible because the byte input is normalized into valid channel values internally. It must not require callers to use unchecked `Color::rgba`.

This step is complete only after the primitive inventory from Task 2 has no unclassified public primitive payload, constructor, or setter.

- [ ] **Step 3: Replace `Value::Number`**

Remove `Value::Number` unless the inventory identifies a real unconstrained finite-number style property. Use variants such as:

```rust
Value::Opacity(Opacity)
Value::AspectRatio(Option<AspectRatio>)
Value::FlexFactor(NonNegativeNumber)
Value::ScrollbarWidth(NonNegativeNumber)
Value::Duration(DurationSeconds)
Value::ZIndex(ZIndex)
```

- [ ] **Step 4: Update metadata, resolver, declaration, and layout lowering**

All public accessors for these properties must return semantic types, not `f32`.

- [ ] **Step 5: Run checks**

```sh
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
cargo fmt --check
```

Expected: pass.

- [ ] **Step 6: Commit**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/adapters/layout.rs src/lib.rs tests
git commit -m "style: model numeric domains semantically"
```

## Task 5: Model Names, Lists, Grid, And Subgrid Invariants

**Files:**
- Modify: `src/value.rs`
- Modify: `src/declaration.rs`
- Modify: `src/adapters/layout.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_fail/invalid_names_and_lists.rs`

- [ ] **Step 1: Add validated name and list types**

Add private-field wrappers:

```rust
pub struct StyleString(String);
pub struct GridLineName(String);
pub struct GridAreaName(String);
pub struct FontFamilyName(String);
pub struct AnimationName(String);
pub struct NonEmpty<T>(Vec<T>);
pub struct FontFamilyList(NonEmpty<FontFamilyName>);
pub struct AnimationNameList(Vec<AnimationName>);
pub struct TransitionPropertyList(Vec<Property>);
pub struct ShadowList(Vec<Shadow>);
```

Use existing validators such as `validate_style_string`, `validate_grid_line_name`, and `validate_grid_area_name`. Empty lists are allowed only when the property semantics explicitly use empty as a meaningful value; document those cases in the domain inventory.

- [ ] **Step 2: Replace raw public string/list payloads**

Replace these loose public surfaces:

```rust
Value::StringList(Vec<String>)
Value::PropertyList(Vec<Property>)
Value::ShadowList(Vec<Shadow>)
GridTrackComponent::LineNames(Vec<String>)
SubgridLineNameComponent::LineNames(Vec<String>)
SubgridLineNameComponent::Repeat { line_name_sets: Vec<Vec<String>>, .. }
GridTemplateAreaRow { cells: Vec<Option<String>> }
TextValue { font_family: Vec<String>, .. }
```

with semantic wrappers from Step 1 and task-specific constructors.

- [ ] **Step 3: Add grid and subgrid invariant newtypes**

Add:

```rust
pub struct GridLineIndex(i16);
pub struct GridSpanCount(std::num::NonZeroU16);
pub struct TrackRepeatFixedCount(std::num::NonZeroU16);
pub struct SubgridLineNameRepeatFixedCount(std::num::NonZeroUsize);
pub struct GridLineNameSet(Vec<GridLineName>);
pub struct SubgridLineNameSetList(NonEmpty<GridLineNameSet>);
```

Replace raw count payloads:

```rust
TrackRepeatCount::Count(TrackRepeatFixedCount)
SubgridLineNameRepeatCount::Count(SubgridLineNameRepeatFixedCount)
```

- [ ] **Step 4: Replace direct enum construction with fallible constructors**

Examples:

```rust
GridLine::line(i16) -> Result<GridLine>
GridLine::span(u16) -> Result<GridLine>
GridLine::bare_ident(impl Into<String>) -> Result<GridLine>
GridLine::named_line(impl Into<String>, i16) -> Result<GridLine>
GridTrackComponent::line_names(GridLineNameSet) -> GridTrackComponent
SubgridLineNameComponent::repeat(SubgridLineNameRepeatCount, SubgridLineNameSetList) -> Result<Self>
TrackRepeat::count(u16, NonEmpty<GridTrackComponent>) -> Result<Self>
```

Enum variants with invariant-bearing payloads may remain public only if their payload types already make invalid states unrepresentable.

- [ ] **Step 5: Update layout lowering and shorthand expansion**

Use accessors such as `.get()`, `.as_str()`, and `.iter()` at lowering boundaries. Do not use raw strings or vectors as intermediate public contracts.

- [ ] **Step 6: Run focused tests**

```sh
cargo test -p surgeist-style grid
cargo test -p surgeist-style adapters::layout
cargo test -p surgeist-style --test type_safety
```

Expected: invalid names/lists/subgrid repeats no longer compile or fail through typed constructors.

- [ ] **Step 7: Commit**

```sh
git add src/value.rs src/declaration.rs src/adapters/layout.rs src/lib.rs tests/compile_fail tests/compile_pass
git commit -m "style: type grid names and lists"
```

## Task 6: Model Calc And Contextual Length Domains

**Files:**
- Modify: `src/calc.rs`
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/adapters/layout.rs`
- Modify: `tests/compile_fail/invalid_calc_construction.rs`

- [ ] **Step 1: Make calc construction typed and non-empty**

Replace public `CalcLength::Sum(Vec<CalcLengthTerm>)` and public `CalcLengthTerm` fields with private-field or invariant-bearing types:

```rust
pub enum CalcLength {
    Px(CssPx),
    Percent(Percent),
    Sum(CalcSum),
}

pub struct CalcSum(NonEmpty<CalcLengthTerm>);

pub struct CalcLengthTerm {
    operator: CalcOperator,
    value: Box<CalcLength>,
}
```

Constructors:

```rust
impl CalcLength {
    pub fn sum(terms: impl IntoIterator<Item = CalcLengthTerm>) -> Result<Self> {
        Ok(Self::Sum(CalcSum::new(terms)?))
    }
}

impl CalcLengthTerm {
    pub fn add(value: CalcLength) -> Self
    pub fn sub(value: CalcLength) -> Self
    pub const fn operator(&self) -> CalcOperator
    pub const fn value(&self) -> &CalcLength
}
```

- [ ] **Step 2: Add contextual length wrappers**

Add wrappers from the inventory:

```rust
pub struct DimensionLength(Length);
pub struct EdgeLength(Length);
pub struct NonNegativeLength(Length);
pub struct GapLength(Length);
pub struct FontSizeLength(Length);
```

Each wrapper must enforce the exact legal subset for its property group.

- [ ] **Step 3: Preserve symbolic calc through wrappers**

Wrapper validation may reject definitely invalid symbolic values, such as a calc whose px and percent coefficients are both definitely negative for non-negative domains. It must not resolve basis-dependent calc early.

- [ ] **Step 4: Add calc preservation tests**

Add tests covering legal calc in:

```rust
width / height
row_gap / column_gap where legal
grid track sizing
grid flow tolerance where legal
transform translate
```

Each test must verify the value survives into `lower_with_store` and produces a `LayoutCalcStore` entry rather than resolving to a raw number in style.

- [ ] **Step 5: Update layout lowering**

Lower `CssPx`, `Percent`, `Fr`, and contextual wrappers through named helper functions.

- [ ] **Step 6: Run checks**

```sh
cargo test -p surgeist-style calc
cargo test -p surgeist-style adapters::layout
cargo test -p surgeist-style property
cargo test -p surgeist-style --test type_safety
```

Expected: pass.

- [ ] **Step 7: Commit**

```sh
git add src/calc.rs src/value.rs src/property.rs src/declaration.rs src/adapters/layout.rs tests
git commit -m "style: type calc and length domains"
```

## Task 7: Separate Or Justify Authored, Normalized, And Resolved Phases

**Files:**
- Modify: `src/value.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md`

- [ ] **Step 1: Decide the resolved storage contract**

Choose one:

```text
Option A: introduce `ResolvedValue` / `ResolvedStyleValue` and keep authored `Value` out of `Resolved`.
Option B: keep one internal canonical value type and make `Resolved::get(Property)` crate-private or explicitly documented as inspection-only.
```

The plan execution must not leave `Resolved::get(Property) -> &Value` as the primary public way to interpret resolved style.

- [ ] **Step 2: Update public resolved front doors**

Every public resolved accessor should return typed semantic values:

```rust
Resolved::opacity() -> Opacity
Resolved::width() -> DimensionLength
Resolved::transition_duration() -> DurationSeconds
Resolved::font_family() -> FontFamilyList
```

- [ ] **Step 3: Record rationale**

Update the domain inventory's phase-boundary section with the chosen option and a rationale for any type shared across phases.

- [ ] **Step 4: Run resolver tests**

```sh
cargo test -p surgeist-style resolver
cargo test -p surgeist-style adapters::layout
```

Expected: pass.

- [ ] **Step 5: Commit**

```sh
git add src/value.rs src/declaration.rs src/resolver.rs plans/2026-06-27-surgeist-style-type-safety-domain-inventory.md
git commit -m "style: clarify resolved style phase"
```

## Task 8: Hide Composite And Metadata Mutation Surfaces

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Make public composite fields private**

Apply private fields plus accessors to:

```rust
Color
Edges
Corners
GridTrackList
SubgridTrack
GridTemplateAreas
GridTemplateAreaRow
GridTemplate
GridDefinition
GridPlacement
GridAreaPlacement
Size
Shadow
Stroke
SideSet
Dash
TextValue
Transform
Metadata
Impact
```

- [ ] **Step 2: Make composite builder/setter methods preserve invariants**

Audit every public `impl` method for the types in Step 1. Any method that accepts an invariant-bearing primitive or external type must become fallible or accept a style-owned validated wrapper.

Required examples:

```rust
TextValue::style(...)
TextValue::underline(...)
TextValue::strikethrough(...)
Shadow::soft(...)
Dash::new(...)
Dash::density(...)
Dash::phase(...)
TransformOp::scale(...)
TransformOp::rotate(...)
```

This includes values from sibling crates when style currently validates their internals, such as text decorations with offsets, sizes, finite angles, or brush/channel constraints.

- [ ] **Step 3: Make `Metadata` read-only**

Replace public field mutation and arbitrary public construction with read-only accessors:

```rust
impl Metadata {
    pub(crate) fn new(default: Value) -> Self
    pub const fn default_value(&self) -> &Value
    pub const fn inherited(&self) -> bool
    pub const fn impact(&self) -> Impact
    pub const fn animatable(&self) -> bool
    pub const fn interpolation(&self) -> Interpolation
}
```

If public metadata construction is still needed, add a typed `MetadataDescriptor` with validated defaults rather than `Metadata::new(Value)`.

- [ ] **Step 4: Make `Impact` read-only or bitflag-like**

Expose builder methods and query methods, not public fields:

```rust
pub const fn affects_layout(self) -> bool
pub const fn affects_paint(self) -> bool
pub const fn affects_text(self) -> bool
pub const fn affects_effect(self) -> bool
pub const fn affects_animation(self) -> bool
```

- [ ] **Step 5: Run public API artifact**

```sh
cargo run --manifest-path api/generator/Cargo.toml
git diff -- api/public-api.txt
```

Expected: public fields disappear from the API artifact.

- [ ] **Step 6: Commit**

```sh
git add src/value.rs src/property.rs src/lib.rs api/public-api.txt
git commit -m "style: hide mutable modeling surfaces"
```

## Task 9: Add Structured Modeling Errors

**Files:**
- Modify: `src/error.rs`
- Modify: invariant constructors in `src/value.rs`, `src/calc.rs`, `src/declaration.rs`, and `src/property.rs`

- [ ] **Step 1: Add structured error detail**

Extend `Error` so tests can inspect more than a prose message:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorPhase {
    AuthoredValue,
    Declaration,
    Resolution,
    LayoutLowering,
    Interop,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorDetail {
    pub phase: ErrorPhase,
    pub subject: &'static str,
    pub invariant: &'static str,
}
```

Add `Error::detail(&self) -> Option<&ErrorDetail>` and constructors that accept `ErrorDetail`.

- [ ] **Step 2: Use structured errors at invariant boundaries**

Examples:

```rust
Opacity::new(2.0)
GridSpanCount::new(0)
CalcLength::sum([])
Declaration::try_from_raw(Property::Width, Value::Color(_))
adapters::layout::lower_with_store(...)
```

- [ ] **Step 3: Update tests to assert structure**

Tests should assert:

```rust
assert_eq!(error.code(), ErrorCode::InvalidValue);
assert_eq!(error.detail().unwrap().phase, ErrorPhase::AuthoredValue);
assert_eq!(error.detail().unwrap().subject, "Opacity");
assert_eq!(error.detail().unwrap().invariant, "unit interval");
```

Do not rely primarily on substring matching.

- [ ] **Step 4: Commit**

```sh
git add src/error.rs src/value.rs src/calc.rs src/declaration.rs src/property.rs tests
git commit -m "style: structure modeling errors"
```

## Task 10: Final API Refresh And Cross-Crate Report

**Files:**
- Modify: `api/public-api.txt`
- Create: `plans/2026-06-27-surgeist-style-type-safety-api-report.md`

- [ ] **Step 1: Run full verification**

```sh
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
cargo fmt --check
cargo test -p surgeist-style --test type_safety
cargo run --manifest-path api/generator/Cargo.toml
git diff --exit-code -- api/public-api.txt
```

Expected: all pass after committing or staging the refreshed API artifact.

- [ ] **Step 2: Write the API report**

Create `plans/2026-06-27-surgeist-style-type-safety-api-report.md`:

```markdown
# surgeist-style Type Safety API Readiness Report

## Summary

The style crate now uses semantic public types and typed declaration front doors. Generic property/value construction is an explicit fallible interop boundary, not the ordinary authoring API.

## Intentional Public API Breaks

- `Declaration::new(Property, Value)` is removed or no longer the primary public constructor.
- `Declarations::insert(Property, Value)` and `set(Property, Value)` are removed or renamed to explicit raw/interop methods.
- Raw numeric, string, list, grid, subgrid, calc, metadata, and composite field construction holes are removed.
- Resolved style access is typed; raw `Value` inspection is private or explicitly secondary.

## Cross-Crate Follow-Up

- `surgeist-css` must lower parsed syntax through typed style constructors.
- Root `surgeist` must update facade reexports and examples.
- Existing downstream code using raw public fields must migrate to accessors and constructors.

## Verification

- `cargo test -p surgeist-style`
- `cargo clippy -p surgeist-style --all-targets -- -D warnings`
- `cargo fmt --check`
- `cargo test -p surgeist-style --test type_safety`
- `cargo run --manifest-path api/generator/Cargo.toml`
- `git diff --exit-code -- api/public-api.txt`
```

- [ ] **Step 3: Commit**

```sh
git add api/public-api.txt plans/2026-06-27-surgeist-style-type-safety-api-report.md
git commit -m "style: report type safety api readiness"
```

## Reviewer Checklist

Reviewers must review against the full type-safety requirement and `guidance/surgeist-rust-modeling-guide.md`, not just against the plan's named tasks.

- Does the plan remove ordinary public construction of invalid style values?
- Does it make property-specific typed front doors primary?
- Does it demote generic `Property`/`Value` construction to an explicit fallible interop/parser boundary?
- Does every `Property` have an inventory row naming authored, normalized, and resolved domains?
- Does every surviving `Value` variant have one semantic domain?
- Does every `lib.rs` reexport owned by this crate have an inventory decision?
- Are all public primitive payloads, constructors, and setters classified and tightened?
- Are raw string/list/vector payloads removed where style owns validation?
- Are grid and subgrid invariant holes closed?
- Is calc non-empty, typed, symbolic, and tested through layout calc-store lowering?
- Are resolved style front doors typed rather than raw `Value`-centric?
- Are metadata and impact read-only public surfaces?
- Are semantic errors inspectable without relying primarily on message substrings?
- Are cross-crate follow-ups reported instead of edited from this crate?

## Completion Criteria

- At least two separate reviewers review the final plan.
- One reviewer focuses on public type-safety construction holes.
- One reviewer focuses on modeling-guide scope, phase boundaries, and omitted style-owned domains.
- All reviewer findings are incorporated or explicitly resolved in this plan.
- The plan is committed only after reviewer cycles are clean.
