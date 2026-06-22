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
pub struct LayoutCalcStore;

impl LayoutCalcStore {
    pub fn push_length_calc(&mut self, calc: impl Into<surgeist_layout::LayoutCalc>) -> CalcId;
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
- Modify: `src/adapters/layout.rs`
  - Lower authored calc values into layout calc handles after the layout contract exists.
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
```

Import the calc types in the test module:

```rust
use crate::{BoxSizing, CalcLength, CalcLengthTerm, GridFlowTolerance};
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p surgeist-style declaration::tests::value_hash_distinguishes_calc_lengths declaration::tests::calc_lengths_validate_through_length_properties
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

Remove `Copy` from structs and enums that contain `Length` if the compiler requires it: `Edges`, `Corners`, `Size`, `TrackSizing`, `MinTrackSizing`, `MaxTrackSizing`, `GridFlowTolerance`, `Stroke`, and `TransformOp`. Prefer cloning at call sites over changing calc ownership back to handles in style.

- [ ] **Step 4: Add calc hashing**

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

- [ ] **Step 5: Update domain validation for non-negative length properties**

In `src/property.rs`, update `validate_non_negative_length` so calc is accepted only when every term is non-negative:

```rust
fn validate_non_negative_length(length: Length, property: Property) -> Result<()> {
    match length {
        Length::Px(value) | Length::Percent(value) if value < 0.0 => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        Length::Calc(calc) if calc_has_negative_term(&calc) => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        _ => Ok(()),
    }
}

fn calc_has_negative_term(calc: &CalcLength) -> bool {
    match calc {
        CalcLength::Px(value) | CalcLength::Percent(value) => *value < 0.0,
        CalcLength::Sum(terms) => terms.iter().any(|term| calc_has_negative_term(&term.value)),
    }
}
```

- [ ] **Step 6: Run tests to verify pass**

Run:

```sh
cargo test -p surgeist-style declaration::tests::value_hash_distinguishes_calc_lengths declaration::tests::calc_lengths_validate_through_length_properties
cargo test -p surgeist-style
```

Expected: all style tests pass.

- [ ] **Step 7: Commit**

```sh
git add -- src/value.rs src/declaration.rs src/property.rs
git commit -m "style: integrate calc lengths with values"
```

### Task 3: Layout Lowering Contract

**Files:**
- Modify: `src/adapters/layout.rs`

- [ ] **Step 1: Confirm layout APIs exist**

Run:

```sh
rg "pub struct CalcId|pub struct LayoutCalcStore|pub trait CalcResolver|fn calc" ../surgeist-layout/src ../surgeist-layout/api/public-api.txt
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

    let lowered = lower_with_calc_store(&resolved).unwrap();

    assert!(matches!(lowered.node.size.width, surgeist_layout::Dimension::Calc(_)));
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
pub struct LoweredLayout {
    pub node: layout::NodeInput,
    pub calc_store: layout::LayoutCalcStore,
}

pub fn lower_with_calc_store(resolved: &Resolved) -> Result<LoweredLayout> {
    let mut calc_store = layout::LayoutCalcStore::default();
    let node = lower_into_store(resolved, &mut calc_store)?;
    Ok(LoweredLayout { node, calc_store })
}
```

Refactor current `lower` to preserve compatibility:

```rust
pub fn lower(resolved: &Resolved) -> Result<layout::NodeInput> {
    Ok(lower_with_calc_store(resolved)?.node)
}
```

Move the existing `lower` body into:

```rust
fn lower_into_store(
    resolved: &Resolved,
    calc_store: &mut layout::LayoutCalcStore,
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
    lower_dimension_into_store(resolved.width(), calc_store)?,
    lower_dimension_into_store(resolved.height(), calc_store)?,
),
min_size: layout::Size::new(
    lower_dimension_into_store(length(resolved, Property::MinWidth), calc_store)?,
    lower_dimension_into_store(length(resolved, Property::MinHeight), calc_store)?,
),
max_size: layout::Size::new(
    lower_dimension_into_store(length(resolved, Property::MaxWidth), calc_store)?,
    lower_dimension_into_store(length(resolved, Property::MaxHeight), calc_store)?,
),
margin: lower_edges_auto_into_store(resolved.margin_edges(), calc_store)?,
padding: lower_edges_into_store(resolved.padding_edges(), calc_store)?,
border: lower_edges_into_store(resolved.border_width_edges(), calc_store)?,
gap: layout::Size::new(
    lower_gap_length_into_store(length(resolved, Property::ColumnGap), calc_store)?,
    lower_gap_length_into_store(length(resolved, Property::RowGap), calc_store)?,
),
flex_basis: lower_dimension_into_store(length(resolved, Property::FlexBasis), calc_store)?,
```

Add helpers:

```rust
fn lower_dimension_into_store(
    length: Length,
    calc_store: &mut layout::LayoutCalcStore,
) -> Result<layout::Dimension> {
    Ok(match length {
        Length::Calc(calc) => {
            let id = calc_store.push_length_calc(lower_calc_length(calc)?);
            layout::Dimension::calc(id)
        }
        length => lower_dimension(length)?,
    })
}
```

Repeat the same pattern for `layout::Length` and `layout::LengthAuto`; keep the existing non-calc helper behavior intact.

- [ ] **Step 5: Normalize percentages only in the adapter**

In the calc lowering helper, convert style percentages to layout factors:

```rust
fn lower_calc_length(calc: CalcLength) -> Result<layout::LayoutCalc> {
    match calc {
        CalcLength::Px(value) => Ok(layout::LayoutCalc::px(value)),
        CalcLength::Percent(value) => Ok(layout::LayoutCalc::percent(percent(value))),
        CalcLength::Sum(terms) => {
            let lowered = terms
                .into_iter()
                .map(lower_calc_term)
                .collect::<Result<Vec<_>>>()?;
            Ok(layout::LayoutCalc::sum(lowered))
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
cargo run --manifest-path api/generator/Cargo.toml > api/public-api.txt
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
