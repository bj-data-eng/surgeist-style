# Surgeist Style Typed Calc Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add style-owned authored calc values and the style-to-layout lowering contract needed by the layout typed-value work.

**Architecture:** `surgeist-style` owns authored calc ASTs, validation, hashing, property compatibility, and lowering from authored percentages to layout-normalized values. `surgeist-layout` owns opaque calc handles, calc stores, and basis-dependent layout behavior; style may depend on those public layout APIs only after they exist. `surgeist-css` parses CSS syntax into the style-owned `CalcLength` API and does not own semantic lowering.

**Tech Stack:** Rust 2024 in `surgeist-style`, `surgeist-layout` as the layout contract dependency, focused tests with `cargo test -p surgeist-style`, formatting with `cargo fmt --check`, linting with `cargo clippy -p surgeist-style --all-targets -- -D warnings`.

---

## Non-Negotiable Constraints

- Work only in the `surgeist-style` repo. Do not edit `../surgeist-layout`, `../surgeist-css`, or the top-level `../surgeist` repo from this plan.
- Keep CSS percentage spelling as authored style data: `Length::percent(10.0)` means `10%`. Normalize to layout factors such as `0.10` exactly once in `src/adapters/layout.rs`.
- Do not parse CSS syntax in this crate. CSS parsing belongs to `surgeist-css`.
- Do not store calc arenas inside individual style values or layout `NodeInput` values.
- Do not add lint suppressions. Fix warnings by improving code shape.
- Commit after each task with the message listed in that task.

## Required Layout Contract

Task 3 depends on the layout crate exposing these public APIs from its typed-value calc implementation:

```rust
pub struct CalcId;
pub trait CalcResolver;
pub struct CalcExpression;
pub enum CalcTerm;
pub struct LayoutCalcStore;

impl LayoutCalcStore {
    pub fn push(&mut self, expression: CalcExpression) -> CalcId;
    pub fn get(&self, id: CalcId) -> Option<&CalcExpression>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl surgeist_layout::Length {
    pub fn calc(id: CalcId) -> Self;
}

impl surgeist_layout::LengthAuto {
    pub fn calc(id: CalcId) -> Self;
}

impl surgeist_layout::Dimension {
    pub fn calc(id: CalcId) -> Self;
}
```

If those APIs differ, stop Task 3 and file an upstream issue in `surgeist-layout` with the exact missing symbols and the style lowering call sites that need them.

## File Map

- Create: `src/calc.rs`
  - Authored calc AST, validation, percentage detection, and stable CSS-like serialization.
- Modify: `src/lib.rs`
  - Export `CalcLength`, `CalcLengthTerm`, and related front-door types.
- Modify: `src/value.rs`
  - Add `Length::Calc(CalcLength)`, validate calc values, and keep length constructors focused.
- Modify: `src/declaration.rs`
  - Include calc values in declaration fingerprints and shorthand expansion.
- Modify: `src/property.rs`
  - Ensure calc lengths are accepted for length-bearing properties and rejected by existing domain rules when negative or invalid.
- Modify: `src/resolver.rs`
  - Update resolved-value accessors for non-`Copy` length-bearing values.
- Modify: `src/adapters/layout.rs`
  - Update non-`Copy` style accessors and lower authored calc values into layout calc handles after the layout contract exists.
- Modify: `api/public-api.txt`
  - Refresh only after implementation and API review.

### Task 1: Authored Calc Length AST

**Files:**
- Create: `src/calc.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing calc AST tests**

Create `src/calc.rs` with this test module first:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorCode;

    #[test]
    fn calc_length_reports_percentage_use() {
        let calc = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::px(20.0)),
            CalcLengthTerm::add(CalcLength::percent(10.0)),
        ]);

        assert!(calc.uses_percentage());
        assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
    }

    #[test]
    fn calc_length_rejects_non_finite_terms() {
        let error = CalcLength::try_px(f32::NAN).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn nested_calc_serializes_stably() {
        let inner = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::px(12.0)),
            CalcLengthTerm::add(CalcLength::percent(3.0)),
        ]);
        let outer = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::percent(100.0)),
            CalcLengthTerm::sub(inner),
        ]);

        assert_eq!(outer.to_css_string(), "calc(100% - calc(12px + 3%))");
    }
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p surgeist-style calc::tests::calc_length_reports_percentage_use calc::tests::calc_length_rejects_non_finite_terms calc::tests::nested_calc_serializes_stably
```

Expected: compile failure because `calc` is not declared from `src/lib.rs` and `CalcLength` does not exist.

- [ ] **Step 3: Implement the AST**

Add `mod calc;` to `src/lib.rs`, and export the front-door types:

```rust
pub use calc::{CalcLength, CalcLengthTerm, CalcOperator};
```

Replace the temporary `src/calc.rs` body with:

```rust
use crate::{Error, ErrorCode, Result, error::validate_finite};

#[derive(Clone, Debug, PartialEq)]
pub enum CalcLength {
    Px(f32),
    Percent(f32),
    Sum(Vec<CalcLengthTerm>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalcLengthTerm {
    pub operator: CalcOperator,
    pub value: Box<CalcLength>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CalcOperator {
    Add,
    Sub,
}

impl CalcLength {
    #[must_use]
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    pub fn try_px(value: f32) -> Result<Self> {
        validate_finite(value, "calc px term")?;
        Ok(Self::Px(value))
    }

    #[must_use]
    pub const fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    pub fn try_percent(value: f32) -> Result<Self> {
        validate_finite(value, "calc percent term")?;
        Ok(Self::Percent(value))
    }

    #[must_use]
    pub fn sum(terms: impl IntoIterator<Item = CalcLengthTerm>) -> Self {
        Self::Sum(terms.into_iter().collect())
    }

    #[must_use]
    pub fn uses_percentage(&self) -> bool {
        match self {
            Self::Px(_) => false,
            Self::Percent(_) => true,
            Self::Sum(terms) => terms.iter().any(|term| term.value.uses_percentage()),
        }
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Px(value) => validate_finite(*value, "calc px term"),
            Self::Percent(value) => validate_finite(*value, "calc percent term"),
            Self::Sum(terms) if terms.is_empty() => Err(Error::new(
                ErrorCode::InvalidValue,
                "calc sum must contain at least one term",
            )),
            Self::Sum(terms) => terms.iter().try_for_each(|term| term.value.validate()),
        }
    }

    #[must_use]
    pub fn to_css_string(&self) -> String {
        match self {
            Self::Px(value) => format_number(*value, "px"),
            Self::Percent(value) => format_number(*value, "%"),
            Self::Sum(terms) => {
                let mut output = String::from("calc(");
                for (index, term) in terms.iter().enumerate() {
                    if index == 0 {
                        if matches!(term.operator, CalcOperator::Sub) {
                            output.push('-');
                        }
                    } else {
                        output.push(' ');
                        output.push(match term.operator {
                            CalcOperator::Add => '+',
                            CalcOperator::Sub => '-',
                        });
                        output.push(' ');
                    }
                    output.push_str(&term.value.to_css_string());
                }
                output.push(')');
                output
            }
        }
    }
}

impl CalcLengthTerm {
    #[must_use]
    pub fn add(value: CalcLength) -> Self {
        Self {
            operator: CalcOperator::Add,
            value: Box::new(value),
        }
    }

    #[must_use]
    pub fn sub(value: CalcLength) -> Self {
        Self {
            operator: CalcOperator::Sub,
            value: Box::new(value),
        }
    }
}

fn format_number(value: f32, suffix: &str) -> String {
    let mut number = value.to_string();
    if let Some(stripped) = number.strip_suffix(".0") {
        number = stripped.to_owned();
    }
    format!("{number}{suffix}")
}
```

- [ ] **Step 4: Run tests to verify pass**

Run:

```sh
cargo test -p surgeist-style calc::tests::calc_length_reports_percentage_use calc::tests::calc_length_rejects_non_finite_terms calc::tests::nested_calc_serializes_stably
```

Expected: all three tests pass.

- [ ] **Step 5: Commit**

```sh
git add -- src/calc.rs src/lib.rs
git commit -m "style: add authored calc length values"
```

### Task 2: Length Value Integration

**Files:**
- Modify: `src/value.rs`
- Modify: `src/declaration.rs`
- Modify: `src/property.rs`
- Modify: `src/resolver.rs`
- Modify: `src/adapters/layout.rs`

- [ ] **Step 1: Write failing value and fingerprint tests**

Add these tests near the existing tests in `src/declaration.rs`:

```rust
#[test]
fn value_hash_distinguishes_calc_lengths() {
    let calc_a = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::px(20.0)),
        CalcLengthTerm::add(CalcLength::percent(10.0)),
    ]);
    let calc_b = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::px(21.0)),
        CalcLengthTerm::add(CalcLength::percent(10.0)),
    ]);

    assert_ne!(
        value_hash(&Value::Length(Length::Calc(calc_a))),
        value_hash(&Value::Length(Length::Calc(calc_b)))
    );
}

#[test]
fn calc_lengths_validate_through_length_properties() {
    let calc = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::px(20.0)),
        CalcLengthTerm::add(CalcLength::percent(10.0)),
    ]);

    Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap();
}

#[test]
fn calc_px_only_negative_results_are_rejected_for_non_negative_properties() {
    let calc = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::px(0.0)),
        CalcLengthTerm::sub(CalcLength::px(1.0)),
    ]);

    let error = Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc)))
        .unwrap_err();
    assert_eq!(error.code(), ErrorCode::InvalidValue);
}

#[test]
fn calc_percent_only_negative_results_are_rejected_for_non_negative_properties() {
    let calc = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::percent(0.0)),
        CalcLengthTerm::sub(CalcLength::percent(1.0)),
    ]);

    let error = Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc)))
        .unwrap_err();
    assert_eq!(error.code(), ErrorCode::InvalidValue);
}

#[test]
fn grid_flow_tolerance_calc_reaches_property_domain_validation() {
    let calc = CalcLength::sum([
        CalcLengthTerm::add(CalcLength::px(8.0)),
        CalcLengthTerm::add(CalcLength::percent(2.0)),
    ]);

    let error = Declaration::try_new(
        Property::GridFlowTolerance,
        Value::GridFlowTolerance(GridFlowTolerance::Length(Length::Calc(calc))),
    )
    .unwrap_err();
    assert!(error.to_string().contains("grid flow tolerance length"));
}
```

Import the calc types in the test module:

```rust
use crate::{BoxSizing, CalcLength, CalcLengthTerm, ErrorCode, GridFlowTolerance};
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p surgeist-style declaration::tests::value_hash_distinguishes_calc_lengths declaration::tests::calc_lengths_validate_through_length_properties declaration::tests::calc_px_only_negative_results_are_rejected_for_non_negative_properties declaration::tests::calc_percent_only_negative_results_are_rejected_for_non_negative_properties declaration::tests::grid_flow_tolerance_calc_reaches_property_domain_validation
```

Expected: compile failure because `Length::Calc` and calc hashing do not exist.

- [ ] **Step 3: Add `Length::Calc` and validation**

In `src/value.rs`, import `CalcLength` and update `Length`:

```rust
use super::{
    CalcLength, Error, ErrorCode, Interpolation, Property, Result,
    error::{validate_finite, validate_non_negative},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    Normal,
    Px(f32),
    Percent(f32),
    Calc(CalcLength),
    Fill,
    Fit,
    MinContent,
    MaxContent,
    Auto,
}
```

Update `Length::validate`:

```rust
Self::Calc(value) => value.validate(),
```

Remove `Copy` from structs and enums that contain `Length`: `Edges`, `Corners`, `Size`, `TrackSizing`, `MinTrackSizing`, `MaxTrackSizing`, `GridFlowTolerance`, `Stroke`, and `TransformOp`. Do not change calc ownership back to handles in style.

- [ ] **Step 4: Update non-`Copy` accessors and call sites**

In `src/resolver.rs`, change resolved-value accessors that currently dereference `Length`, `Edges`, `Size`, or other length-bearing values to clone instead:

```rust
pub fn width(&self) -> Length {
    match self.get(Property::Width) {
        Value::Length(length) => length.clone(),
        _ => Length::Auto,
    }
}

pub fn padding_edges(&self) -> Edges {
    match self.get(Property::Padding) {
        Value::Edges(edges) => edges.clone(),
        _ => Edges::default(),
    }
}
```

Apply the same pattern to `height`, `margin_edges`, `border_width_edges`, `font_size`, `line_height`, `transform_origin`, and any other accessor that returns a length-bearing value by copying from `Value`.

In `src/adapters/layout.rs`, update helper extraction functions to clone:

```rust
fn length(resolved: &Resolved, property: Property) -> Length {
    match resolved.get(property) {
        Value::Length(length) => length.clone(),
        _ => Length::Auto,
    }
}

fn edges(resolved: &Resolved, property: Property) -> Edges {
    match resolved.get(property) {
        Value::Edges(edges) => edges.clone(),
        _ => Edges::default(),
    }
}
```

In `src/declaration.rs` and `src/property.rs`, replace dereference patterns such as `*length`, `*edges`, and `*value` for non-`Copy` length-bearing values with `.clone()` or borrowed validation helpers. Keep numeric `f32` dereferences unchanged.

In `src/value.rs`, update length-bearing constructors, validators, and convenience methods so they do not rely on `Length: Copy`. Keep constructors such as `Edges::all`, `Edges::new`, `Size::new`, `TrackSizing::px`, and `TrackSizing::percent` taking owned `Length` or scalar inputs, and clone repeated owned values inside constructors where a value is reused:

```rust
impl Edges {
    #[must_use]
    pub fn all(value: Length) -> Self {
        Self {
            top: value.clone(),
            right: value.clone(),
            bottom: value.clone(),
            left: value,
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.top.validate()?;
        self.right.validate()?;
        self.bottom.validate()?;
        self.left.validate()
    }
}
```

Apply the same borrow-based validation pattern to `Corners`, `Size`, `GridTrackList`, `TrackSizing`, `MinTrackSizing`, `MaxTrackSizing`, `GridFlowTolerance`, `Stroke`, `Transform`, and any helper that currently takes a length-bearing type by value only because it was `Copy`.

- [ ] **Step 5: Add calc hashing**

In `src/declaration.rs`, import calc types:

```rust
use super::{
    CalcLength, CalcLengthTerm, Color, Corners, Cursor, Display, Edges, GridAreaPlacement,
    GridAutoFlow, GridDefinition, GridFlowTolerance, GridLine, GridPlacement, GridTemplate,
    GridTemplateAreas, GridTrackComponent, GridTrackList, Length, MaxTrackSizing, MinTrackSizing,
    PointerEvents, Property, Result, Shadow, Size, SubgridLineNameComponent, TrackRepeatCount,
    TrackSizing, Transform, Value, Visibility,
};
```

Update `hash_length`:

```rust
super::Length::Calc(value) => {
    8u8.hash(state);
    hash_calc_length(value, state);
}
```

Add:

```rust
fn hash_calc_length(value: &CalcLength, state: &mut DefaultHasher) {
    match value {
        CalcLength::Px(value) => {
            0u8.hash(state);
            hash_f32(*value, state);
        }
        CalcLength::Percent(value) => {
            1u8.hash(state);
            hash_f32(*value, state);
        }
        CalcLength::Sum(terms) => {
            2u8.hash(state);
            terms.len().hash(state);
            for term in terms {
                hash_calc_term(term, state);
            }
        }
    }
}

fn hash_calc_term(term: &CalcLengthTerm, state: &mut DefaultHasher) {
    term.operator.hash(state);
    hash_calc_length(&term.value, state);
}
```

- [ ] **Step 6: Update domain validation for non-negative length properties**

In `src/property.rs`, import `CalcLength` and `CalcOperator`, then update `validate_non_negative_length` so calc expressions that are definitely negative for every non-negative basis are rejected while mixed indefinite expressions remain valid for layout-time resolution:

```rust
fn validate_non_negative_length(length: Length, property: Property) -> Result<()> {
    match length {
        Length::Px(value) | Length::Percent(value) if value < 0.0 => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        Length::Calc(calc) if calc_is_definitely_negative(&calc) => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        _ => Ok(()),
    }
}

fn calc_is_definitely_negative(calc: &CalcLength) -> bool {
    calc_coefficients(calc, 1.0).is_some_and(|coefficients| {
        coefficients.px < 0.0 && coefficients.percent <= 0.0
            || coefficients.px <= 0.0 && coefficients.percent < 0.0
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct CalcCoefficients {
    px: f32,
    percent: f32,
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
                let term_sign = match term.operator {
                    CalcOperator::Add => sign,
                    CalcOperator::Sub => -sign,
                };
                let term = calc_coefficients(&term.value, term_sign)?;
                total.px += term.px;
                total.percent += term.percent;
            }
            Some(total)
        }
    }
}
```

- [ ] **Step 7: Move grid-flow-tolerance compatibility into property validation**

In `src/value.rs`, keep `GridFlowTolerance::validate` to structural value validation only:

```rust
impl GridFlowTolerance {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Length(length) => length.validate(),
            Self::Percent(value) => validate_finite(*value, "grid flow tolerance percent"),
            Self::Normal | Self::Infinite => Ok(()),
        }
    }
}
```

In `src/property.rs`, keep the compatibility rule in `validate_grid_flow_tolerance`:

```rust
fn validate_grid_flow_tolerance(value: &GridFlowTolerance, property: Property) -> Result<()> {
    value.validate()?;
    match value {
        GridFlowTolerance::Length(Length::Px(value)) => {
            validate_non_negative_number(*value, property)
        }
        GridFlowTolerance::Length(_) => Err(Error::new(
            ErrorCode::InvalidValue,
            "grid flow tolerance length must be a concrete px length",
        )),
        GridFlowTolerance::Percent(value) => validate_non_negative_number(*value, property),
        GridFlowTolerance::Normal | GridFlowTolerance::Infinite => Ok(()),
    }
}
```

Update the caller to pass a borrowed value:

```rust
(Self::GridFlowTolerance, Value::GridFlowTolerance(value)) => {
    validate_grid_flow_tolerance(value, self)
}
```

This lets `surgeist-css` construct `GridFlowTolerance::Length(Length::Calc(_))` and leaves the rejection with the owning style property domain.

- [ ] **Step 8: Run tests to verify pass**

Run:

```sh
cargo test -p surgeist-style declaration::tests::value_hash_distinguishes_calc_lengths declaration::tests::calc_lengths_validate_through_length_properties declaration::tests::calc_px_only_negative_results_are_rejected_for_non_negative_properties declaration::tests::calc_percent_only_negative_results_are_rejected_for_non_negative_properties declaration::tests::grid_flow_tolerance_calc_reaches_property_domain_validation
cargo test -p surgeist-style
```

Expected: all style tests pass.

- [ ] **Step 9: Commit**

```sh
git add -- src/value.rs src/declaration.rs src/property.rs src/resolver.rs src/adapters/layout.rs
git commit -m "style: integrate calc lengths with values"
```

### Task 3: Layout Lowering Contract

**Files:**
- Modify: `src/adapters/layout.rs`

- [ ] **Step 1: Confirm layout APIs exist**

Run:

```sh
rg "pub struct CalcId|pub struct CalcExpression|pub enum CalcTerm|pub struct LayoutCalcStore|pub trait CalcResolver|fn calc|fn push\\(" ../surgeist-layout/src ../surgeist-layout/api/public-api.txt
```

Expected: the required layout contract symbols are visible. If they are not visible, stop this task and open an upstream `surgeist-layout` issue instead of editing style lowering.

- [ ] **Step 2: Write failing lowering tests**

Add these tests to the existing test module in `src/adapters/layout.rs`, creating `#[cfg(test)] mod tests` at the end of the file if needed:

```rust
use surgeist_retained::{Element, Model, Patch, Tag};

#[test]
fn lowers_calc_dimension_into_layout_calc_store() {
    let calc = crate::CalcLength::sum([
        crate::CalcLengthTerm::add(crate::CalcLength::px(20.0)),
        crate::CalcLengthTerm::add(crate::CalcLength::percent(10.0)),
    ]);
    let declarations = crate::Declarations::new()
        .try_set(crate::Property::Width, crate::Value::Length(crate::Length::Calc(calc)))
        .unwrap();
    let mut model = Model::empty();
    let root = model.root();
    let panel = model
        .apply(Patch::Insert {
            parent: root,
            index: 0,
            element: Element::tagged(Tag::new("panel").unwrap()),
        })
        .unwrap()
        .changes()
        .inserted()[0];
    let tree = model.snapshot();
    let resolved = crate::Resolver::new(crate::Sheet::new())
        .resolve(crate::Context::new(&tree, panel).local(&declarations))
        .unwrap();

    let lowered = lower_with_store(&resolved).unwrap();

    let surgeist_layout::Dimension::Calc(id) = lowered.node.size.width else {
        panic!("expected calc width, got {:?}", lowered.node.size.width);
    };
    assert!(lowered.calc_store.get(id).is_some());
    assert_eq!(lowered.calc_store.len(), 1);
}
```

- [ ] **Step 3: Run test to verify failure**

Run:

```sh
cargo test -p surgeist-style adapters::layout::tests::lowers_calc_dimension_into_layout_calc_store
```

Expected: compile failure until style exposes a lowering result with a calc store and layout exposes calc-bearing value variants.

- [ ] **Step 4: Add lowering result and calc conversion helpers**

Add the result type:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutLoweringOutput {
    pub node: layout::NodeInput,
    pub calc_store: layout::LayoutCalcStore,
}

#[derive(Clone, Debug, Default)]
pub struct LayoutLoweringSession {
    calc_store: layout::LayoutCalcStore,
}
```

Refactor current `lower` so calc-free callers keep the old API without receiving unresolvable calc handles:

```rust
pub fn lower(resolved: &Resolved) -> Result<layout::NodeInput> {
    if resolved_uses_calc(resolved) {
        return Err(unsupported("calc values require lower_with_store"));
    }
    let mut session = LayoutLoweringSession::new();
    session.lower_node(resolved)
}

pub fn lower_with_store(resolved: &Resolved) -> Result<LayoutLoweringOutput> {
    let mut session = LayoutLoweringSession::new();
    let node = session.lower_node(resolved)?;
    Ok(LayoutLoweringOutput {
        node,
        calc_store: session.finish(),
    })
}

impl LayoutLoweringSession {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_node(&mut self, resolved: &Resolved) -> Result<layout::NodeInput> {
        lower_node_with_session(resolved, self)
    }

    #[must_use]
    pub fn finish(self) -> layout::LayoutCalcStore {
        self.calc_store
    }
}
```

Add a calc-detection helper that checks every length-bearing resolved field lowered into layout:

```rust
fn resolved_uses_calc(resolved: &Resolved) -> bool {
    length_uses_calc(resolved.width())
        || length_uses_calc(resolved.height())
        || edges_use_calc(edges(resolved, Property::Inset))
        || length_uses_calc(length(resolved, Property::MinWidth))
        || length_uses_calc(length(resolved, Property::MinHeight))
        || length_uses_calc(length(resolved, Property::MaxWidth))
        || length_uses_calc(length(resolved, Property::MaxHeight))
        || edges_use_calc(resolved.margin_edges())
        || edges_use_calc(resolved.padding_edges())
        || edges_use_calc(resolved.border_width_edges())
        || length_uses_calc(length(resolved, Property::ColumnGap))
        || length_uses_calc(length(resolved, Property::RowGap))
        || length_uses_calc(length(resolved, Property::FlexBasis))
        || track_list_uses_calc(track_list(resolved, Property::GridTemplateColumns))
        || track_list_uses_calc(track_list(resolved, Property::GridTemplateRows))
        || track_list_uses_calc(track_list(resolved, Property::GridAutoColumns))
        || track_list_uses_calc(track_list(resolved, Property::GridAutoRows))
}

fn length_uses_calc(length: Length) -> bool {
    matches!(length, Length::Calc(_))
}

fn edges_use_calc(edges: Edges) -> bool {
    length_uses_calc(edges.top)
        || length_uses_calc(edges.right)
        || length_uses_calc(edges.bottom)
        || length_uses_calc(edges.left)
}

fn track_list_uses_calc(list: &GridTrackList) -> bool {
    list.components.iter().any(track_component_uses_calc)
}

fn track_component_uses_calc(component: &GridTrackComponent) -> bool {
    match component {
        GridTrackComponent::Track(track) => track_sizing_uses_calc(track),
        GridTrackComponent::Repeat(repeat) => {
            repeat.components.iter().any(track_component_uses_calc)
        }
        GridTrackComponent::LineNames(_) | GridTrackComponent::Subgrid(_) => false,
    }
}

fn track_sizing_uses_calc(track: &TrackSizing) -> bool {
    min_track_sizing_uses_calc(&track.min) || max_track_sizing_uses_calc(&track.max)
}

fn min_track_sizing_uses_calc(track: &MinTrackSizing) -> bool {
    matches!(track, MinTrackSizing::Length(Length::Calc(_)))
}

fn max_track_sizing_uses_calc(track: &MaxTrackSizing) -> bool {
    matches!(
        track,
        MaxTrackSizing::Length(Length::Calc(_)) | MaxTrackSizing::FitContent(Length::Calc(_))
    )
}
```

Move the existing `lower` body into:

```rust
fn lower_node_with_session(
    resolved: &Resolved,
    session: &mut LayoutLoweringSession,
) -> Result<layout::NodeInput> {
    Ok(layout::NodeInput {
        // keep the current field list from `lower`, changing only length-bearing calls
        ..layout::NodeInput::DEFAULT
    })
}
```

During that move, preserve every existing field assignment from `lower` and make these exact substitutions:

```rust
size: layout::Size::new(
    lower_dimension_with_session(resolved.width(), session)?,
    lower_dimension_with_session(resolved.height(), session)?,
),
min_size: layout::Size::new(
    lower_dimension_with_session(length(resolved, Property::MinWidth), session)?,
    lower_dimension_with_session(length(resolved, Property::MinHeight), session)?,
),
max_size: layout::Size::new(
    lower_dimension_with_session(length(resolved, Property::MaxWidth), session)?,
    lower_dimension_with_session(length(resolved, Property::MaxHeight), session)?,
),
inset: lower_edges_auto_with_session(edges(resolved, Property::Inset), session)?,
margin: lower_edges_auto_with_session(resolved.margin_edges(), session)?,
padding: lower_edges_with_session(resolved.padding_edges(), session)?,
border: lower_edges_with_session(resolved.border_width_edges(), session)?,
gap: layout::Size::new(
    lower_gap_length_with_session(length(resolved, Property::ColumnGap), session)?,
    lower_gap_length_with_session(length(resolved, Property::RowGap), session)?,
),
flex_basis: lower_dimension_with_session(length(resolved, Property::FlexBasis), session)?,
grid_template_columns: lower_track_list_with_session(
    track_list(resolved, Property::GridTemplateColumns),
    session,
)?,
grid_template_rows: lower_track_list_with_session(
    track_list(resolved, Property::GridTemplateRows),
    session,
)?,
grid_auto_columns: lower_track_list_with_session(
    track_list(resolved, Property::GridAutoColumns),
    session,
)?,
grid_auto_rows: lower_track_list_with_session(
    track_list(resolved, Property::GridAutoRows),
    session,
)?,
```

Add helpers:

```rust
fn lower_dimension_with_session(
    length: Length,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Dimension> {
    Ok(match length {
        Length::Calc(calc) => {
            let id = session.push_calc_length(&calc);
            layout::Dimension::calc(id)
        }
        length => lower_dimension(length)?,
    })
}
```

Repeat the same pattern for `layout::Length` and `layout::LengthAuto`; keep the existing non-calc helper behavior intact.

Update grid track lowering to borrow style track structures and route length-bearing track values through the same session:

```rust
fn lower_track_list_with_session(
    list: &GridTrackList,
    session: &mut LayoutLoweringSession,
) -> Result<Vec<layout::TrackComponent>> {
    let mut lowered = Vec::new();
    for component in &list.components {
        match component {
            GridTrackComponent::Track(track) => {
                lowered.push(layout::TrackComponent::Track(
                    lower_track_sizing_with_session(track, session)?,
                ));
            }
            GridTrackComponent::Repeat(repeat) => {
                lowered.push(layout::TrackComponent::Repeat(
                    lower_track_repeat_with_session(repeat, session)?,
                ));
            }
            GridTrackComponent::LineNames(names) => {
                lowered.push(layout::TrackComponent::LineNames(names.clone()));
            }
            GridTrackComponent::Subgrid(subgrid) => {
                lowered.push(layout::TrackComponent::Subgrid(layout::SubgridTrack {
                    name_components: lower_subgrid_line_name_components(&subgrid.name_components),
                }));
            }
        }
    }
    Ok(lowered)
}

fn lower_track_sizing_with_session(
    track: &TrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::TrackSizing> {
    Ok(layout::TrackSizing::new(
        lower_min_track_sizing_with_session(&track.min, session)?,
        lower_max_track_sizing_with_session(&track.max, session)?,
    ))
}

fn lower_min_track_sizing_with_session(
    track: &MinTrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::MinTrackSizing> {
    Ok(match track {
        MinTrackSizing::Length(length) => {
            layout::MinTrackSizing::Length(lower_length_with_session(length.clone(), session)?)
        }
        MinTrackSizing::Auto => layout::MinTrackSizing::AUTO,
        MinTrackSizing::MinContent => layout::MinTrackSizing::MIN_CONTENT,
        MinTrackSizing::MaxContent => layout::MinTrackSizing::MAX_CONTENT,
    })
}

fn lower_max_track_sizing_with_session(
    track: &MaxTrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::MaxTrackSizing> {
    Ok(match track {
        MaxTrackSizing::Length(length) => {
            layout::MaxTrackSizing::Length(lower_length_with_session(length.clone(), session)?)
        }
        MaxTrackSizing::Flex(value) => layout::MaxTrackSizing::fr(*value),
        MaxTrackSizing::Auto => layout::MaxTrackSizing::AUTO,
        MaxTrackSizing::MinContent => layout::MaxTrackSizing::MIN_CONTENT,
        MaxTrackSizing::MaxContent => layout::MaxTrackSizing::MAX_CONTENT,
        MaxTrackSizing::FitContent(limit) => {
            layout::MaxTrackSizing::fit_content(lower_length_with_session(limit.clone(), session)?)
        }
    })
}
```

Update repeat lowering the same way: iterate `&repeat.components`, call `lower_track_sizing_with_session(track, session)` for track components, and clone line-name vectors instead of copying track values.

- [ ] **Step 5: Normalize percentages only in the adapter**

In the calc lowering helper, convert style percentages to layout factors:

```rust
impl LayoutLoweringSession {
    fn push_calc_length(&mut self, calc: &CalcLength) -> layout::CalcId {
        let expression = lower_calc_expression(calc);
        self.calc_store.push(expression)
    }
}

fn lower_calc_expression(calc: &CalcLength) -> layout::CalcExpression {
    match calc {
        CalcLength::Px(value) => layout::CalcExpression::sum([layout::CalcTerm::px(*value)]),
        CalcLength::Percent(value) => {
            layout::CalcExpression::sum([layout::CalcTerm::percent(percent(*value))])
        }
        CalcLength::Sum(terms) => {
            let mut lowered = Vec::new();
            for term in terms {
                let sign = match term.operator {
                    CalcOperator::Add => 1.0,
                    CalcOperator::Sub => -1.0,
                };
                collect_calc_terms(&term.value, sign, &mut lowered);
            }
            layout::CalcExpression::sum(lowered)
        }
    }
}

fn collect_calc_terms(calc: &CalcLength, sign: f32, output: &mut Vec<layout::CalcTerm>) {
    match calc {
        CalcLength::Px(value) => output.push(layout::CalcTerm::px(sign * *value)),
        CalcLength::Percent(value) => {
            output.push(layout::CalcTerm::percent(sign * percent(*value)));
        }
        CalcLength::Sum(terms) => {
            for term in terms {
                let term_sign = match term.operator {
                    CalcOperator::Add => sign,
                    CalcOperator::Sub => -sign,
                };
                collect_calc_terms(&term.value, term_sign, output);
            }
        }
    }
}
```

- [ ] **Step 6: Run focused and full style checks**

Run:

```sh
cargo test -p surgeist-style adapters::layout::tests::lowers_calc_dimension_into_layout_calc_store
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
cargo fmt --check
```

Expected: all commands pass.

- [ ] **Step 7: Commit**

```sh
git add -- src/adapters/layout.rs
git commit -m "style: lower calc lengths into layout store"
```

### Task 4: Public API Artifact And Final Verification

**Files:**
- Modify: `api/public-api.txt`

- [ ] **Step 1: Refresh public API artifact**

Run the crate's existing API generator command from its README or current crate convention. If no command is documented, run:

```sh
cargo run --manifest-path api/generator/Cargo.toml
```

Expected: `api/public-api.txt` includes the new calc front-door types and any intentional layout-lowering result type.

- [ ] **Step 2: Final verification**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
```

Expected: all commands pass.

- [ ] **Step 3: Commit**

```sh
git add -- api/public-api.txt
git commit -m "style: refresh public api for calc values"
```
