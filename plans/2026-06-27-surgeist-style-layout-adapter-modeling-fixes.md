# Surgeist Style Layout Adapter Modeling Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update `surgeist-style` layout lowering so it consumes the validated layout API for aspect ratio, grid placement, and track repetition, unblocking the layout cross-crate ledger without weakening style ownership.

**Architecture:** Keep `surgeist-style` as the owner of authored/resolved style values and treat `src/adapters/layout.rs` as the explicit style-to-layout conversion boundary. Do not redesign style's public grid/style value model in this pass. Because all three ledger blockers are current compile errors in the same adapter file, restore all three layout API compatibility points before expecting any crate test command to pass or making implementation commits.

**Tech Stack:** Rust 2024, `surgeist-style`, sibling `surgeist-layout`, crate-local tests in `src/adapters/layout.rs`, source-derived API artifact tooling in `api/generator`.

---

## Source Ledger

This plan implements the style-owned entries from:

- `/Users/codex/Development/surgeist-layout/plans/2026-06-27-surgeist-layout-modeling-fixes-cross-crate-ledger.md`

Covered entries:

- `LAYOUT-XCRATE-0001`: style adapter must lower validated aspect ratio.
- `LAYOUT-XCRATE-0002`: style adapter must lower validated grid placement.
- `LAYOUT-XCRATE-0003`: style adapter must handle fallible track repetition.

Non-goals:

- Do not edit `surgeist-layout` from this repo.
- Do not update root `surgeist` submodule pointers from this repo.
- Do not redesign style's public `GridPlacement`, `GridLine`, `TrackRepeat`, or `GridTrackList` models in this pass.
- Do not remove named-grid raw placement lowering. Numeric layout placement remains `AUTO` when style placement contains named grid syntax; raw placement continues carrying named data for layout's named-line resolver.

## File Structure

- Modify `src/adapters/layout.rs`
  - Add focused tests for the style-to-layout adapter boundary.
  - Change aspect-ratio lowering to return `Result<Option<layout::AspectRatio>>`.
  - Add adapter helpers for layout `GridLine` and `GridSpan`.
  - Update `lower_grid_placement` to use layout validated APIs.
  - Update `lower_track_repeat_with_session` to handle fallible layout repeat constructors.
- No intended public API changes.
- No intended changes to `api/public-api.txt`; run the generator to verify there is no drift.
- No intended changes to `Cargo.toml`, `README.md`, or sibling crates.

## Modeling Rationale

Layout now owns semantic invariants for layout-ready values:

- `layout::AspectRatio` is finite and greater than zero.
- `layout::GridLine` cannot be zero.
- `layout::GridSpan` cannot be zero.
- `layout::TrackRepetition` cannot have zero repeat count or empty repeated components.

Style still owns authored and resolved style data. The adapter should therefore be the narrow chokepoint that translates style values into layout-ready semantic values. Even when style validation currently rejects invalid grid spans or repeat data, the adapter must not bypass layout's constructors or assume layout invariants are always satisfied.

## Important Sequencing Constraint

The starting `surgeist-style` crate does not compile against the current sibling `surgeist-layout` API because all three ledger blockers are compile errors in `src/adapters/layout.rs`. Cargo compiles the whole crate even when filtering tests, so focused test commands cannot pass until aspect ratio, grid placement, and track repetition are all updated.

Implementation commits therefore happen after the compile-restoring adapter changes are applied together. Keep the code sections below separate for review clarity, but do not stop after only one section expecting tests to pass.

## Task 1: Add Adapter Boundary Tests

**Files:**

- Modify: `src/adapters/layout.rs`

- [ ] **Step 1: Add aspect-ratio tests**

Add these tests inside the existing `#[cfg(test)] mod tests` in `src/adapters/layout.rs`:

```rust
#[test]
fn lower_node_converts_positive_aspect_ratio_to_layout_type() {
    let declarations = crate::Declarations::new()
        .try_set(crate::Property::AspectRatio, crate::Value::Number(1.5))
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

    let lowered = lower(&resolved).unwrap();

    assert_eq!(lowered.aspect_ratio.map(layout::AspectRatio::get), Some(1.5));
}

#[test]
fn default_zero_aspect_ratio_lowers_to_none() {
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
        .resolve(crate::Context::new(&tree, panel))
        .unwrap();

    let lowered = lower(&resolved).unwrap();

    assert_eq!(lowered.aspect_ratio, None);
}

#[test]
fn adapter_rejects_non_finite_positive_aspect_ratio() {
    let error = layout_aspect_ratio(f32::INFINITY).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("aspect ratio"));
}

#[test]
fn adapter_rejects_negative_aspect_ratio_if_validation_is_bypassed() {
    let error = layout_aspect_ratio(-1.0).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("aspect ratio"));
}
```

The first two tests use `lower` because calc values are not involved. The invalid-value tests target a private scalar helper because `Declarations::try_set` and `Resolver` validate declarations before the adapter can see non-finite or negative aspect ratios.

- [ ] **Step 2: Add grid placement tests**

Add these tests inside the same test module:

```rust
#[test]
fn lower_grid_placement_uses_layout_validated_line() {
    let placement = lower_grid_placement(GridLine::Line(2), GridLine::Auto).unwrap();

    assert_eq!(placement.start().map(layout::GridLine::get), Some(2));
    assert_eq!(placement.end(), None);
    assert_eq!(placement.span(), None);
}

#[test]
fn lower_grid_placement_uses_layout_validated_line_span() {
    let placement = lower_grid_placement(GridLine::Line(2), GridLine::Span(3)).unwrap();

    assert_eq!(placement.start().map(layout::GridLine::get), Some(2));
    assert_eq!(placement.end(), None);
    assert_eq!(placement.span().map(layout::GridSpan::get), Some(3));
}

#[test]
fn lower_grid_placement_rejects_invalid_zero_line_if_validation_is_bypassed() {
    let error = lower_grid_placement(GridLine::Line(0), GridLine::Auto).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("grid line"));
}

#[test]
fn lower_grid_placement_rejects_invalid_zero_span_if_validation_is_bypassed() {
    let error = lower_grid_placement(GridLine::Span(0), GridLine::Auto).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("grid span"));
}

#[test]
fn lower_grid_placement_keeps_named_numeric_placement_auto() {
    let placement = lower_grid_placement(
        GridLine::NamedLine {
            name: "content".to_owned(),
            index: 1,
        },
        GridLine::Auto,
    )
    .unwrap();

    assert!(placement.is_auto());
}
```

- [ ] **Step 3: Add track repetition tests**

Add this helper and tests inside the same test module:

```rust
fn simple_track_component() -> GridTrackComponent {
    GridTrackComponent::Track(TrackSizing::AUTO)
}

#[test]
fn lower_track_repeat_count_uses_layout_fallible_constructor() {
    let repeat = TrackRepeat::count(2, vec![simple_track_component()]);
    let mut session = LayoutLoweringSession::new();

    let lowered = lower_track_repeat_with_session(&repeat, &mut session).unwrap();

    assert_eq!(
        lowered.repeat(),
        layout::TrackRepeat::Count(layout::TrackRepeatCount::new(2).unwrap())
    );
    assert_eq!(lowered.components().len(), 1);
}

#[test]
fn lower_track_repeat_rejects_zero_count_if_validation_is_bypassed() {
    let repeat = TrackRepeat::count(0, vec![simple_track_component()]);
    let mut session = LayoutLoweringSession::new();

    let error = lower_track_repeat_with_session(&repeat, &mut session).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("track repeat"));
}

#[test]
fn lower_track_repeat_rejects_empty_components_if_validation_is_bypassed() {
    let repeat = TrackRepeat::auto_fit(Vec::new());
    let mut session = LayoutLoweringSession::new();

    let error = lower_track_repeat_with_session(&repeat, &mut session).unwrap_err();

    assert_eq!(error.code(), ErrorCode::InvalidValue);
    assert!(error.message().contains("track repeat"));
}
```

- [ ] **Step 4: Confirm the expected pre-implementation failure**

Run:

```sh
cargo test -p surgeist-style aspect_ratio -- --nocapture
```

Expected: the crate still fails to compile. The relevant failures should point at one or more of these existing adapter mismatches:

- `NodeInput::aspect_ratio` expects `Option<layout::AspectRatio>`.
- `layout::GridPlacement` constructors expect layout `GridLine`/`GridSpan` types or fallible construction.
- `layout::TrackRepetition::{count_components, auto_fill_components, auto_fit_components}` return `Result`.

Do not commit after this step. The crate is expected to remain red until Task 2 is complete.

## Task 2: Restore Layout Adapter API Compatibility

**Files:**

- Modify: `src/adapters/layout.rs`

- [ ] **Step 1: Lower aspect ratio through `layout::AspectRatio`**

In `lower_node_with_session`, change:

```rust
aspect_ratio: aspect_ratio(resolved),
```

to:

```rust
aspect_ratio: aspect_ratio(resolved)?,
```

Replace the existing helper:

```rust
fn aspect_ratio(resolved: &Resolved) -> Option<f32> {
    match number(resolved, Property::AspectRatio) {
        value if value > 0.0 => Some(value),
        _ => None,
    }
}
```

with:

```rust
fn layout_aspect_ratio(value: f32) -> Result<Option<layout::AspectRatio>> {
    if value == 0.0 {
        return Ok(None);
    }

    layout::AspectRatio::new(value).map(Some).ok_or_else(|| {
        Error::new(
            ErrorCode::InvalidValue,
            "aspect ratio must be finite and greater than zero",
        )
    })
}

fn aspect_ratio(resolved: &Resolved) -> Result<Option<layout::AspectRatio>> {
    layout_aspect_ratio(number(resolved, Property::AspectRatio))
}
```

This preserves the current style adapter meaning that resolved `0.0` means “no aspect ratio,” while negative, non-finite, and otherwise invalid values fail at the adapter boundary.

- [ ] **Step 2: Add layout grid invariant helpers**

Add these helpers near `lower_grid_placement`:

```rust
fn layout_grid_line(line: i16) -> Result<layout::GridLine> {
    let line = isize::from(line);
    layout::GridLine::new(line)
        .ok_or_else(|| Error::new(ErrorCode::InvalidValue, "grid line index cannot be zero"))
}

fn layout_grid_span(span: u16) -> Result<layout::GridSpan> {
    let span = usize::from(span);
    layout::GridSpan::new(span)
        .ok_or_else(|| Error::new(ErrorCode::InvalidValue, "grid span count cannot be zero"))
}
```

Do not use `unwrap` or `expect` here. These helpers are the adapter's explicit guard against bypassed style validation.

- [ ] **Step 3: Replace `lower_grid_placement` with validated constructors**

Replace the existing `lower_grid_placement` body with:

```rust
fn lower_grid_placement(start: GridLine, end: GridLine) -> Result<layout::GridPlacement> {
    Ok(match (start, end) {
        (GridLine::Auto, GridLine::Auto) => layout::GridPlacement::AUTO,
        (GridLine::Line(line), GridLine::Auto) => {
            layout::GridPlacement::line(layout_grid_line(line)?)
        }
        (GridLine::Auto, GridLine::Line(line)) => {
            layout::GridPlacement::end_line(layout_grid_line(line)?)
        }
        (GridLine::Line(start), GridLine::Line(end)) => {
            layout::GridPlacement::lines(layout_grid_line(start)?, layout_grid_line(end)?)
        }
        (GridLine::Line(line), GridLine::Span(span)) => {
            layout::GridPlacement::line_span(layout_grid_line(line)?, layout_grid_span(span)?)
        }
        (GridLine::Span(span), GridLine::Line(line)) => {
            layout::GridPlacement::span_line(layout_grid_span(span)?, layout_grid_line(line)?)
        }
        (GridLine::Span(span), GridLine::Auto) | (GridLine::Auto, GridLine::Span(span)) => {
            layout::GridPlacement::try_span(usize::from(span))
                .ok_or_else(|| Error::new(ErrorCode::InvalidValue, "grid span count cannot be zero"))?
        }
        (GridLine::Span(_), GridLine::Span(_)) => {
            return Err(unsupported("span-to-span grid placement"));
        }
        (GridLine::BareIdent(_) | GridLine::NamedLine { .. } | GridLine::NamedSpan { .. }, _)
        | (_, GridLine::BareIdent(_) | GridLine::NamedLine { .. } | GridLine::NamedSpan { .. }) => {
            layout::GridPlacement::AUTO
        }
    })
}
```

This uses explicit layout `GridLine` and `GridSpan` construction where both pieces are needed, and `try_span` for the span-only case where layout does not expose a direct `span(GridSpan)` constructor.

- [ ] **Step 4: Propagate fallible layout track repetition**

Add this helper near `lower_track_repeat_with_session`:

```rust
fn map_track_repetition_error(error: layout::TrackRepetitionError) -> Error {
    Error::new(
        ErrorCode::InvalidValue,
        format!("layout track repeat cannot be lowered: {error}"),
    )
}
```

Replace the final `match repeat.count` in `lower_track_repeat_with_session` with:

```rust
match repeat.count {
    TrackRepeatCount::Count(count) => {
        layout::TrackRepetition::count_components(usize::from(count), components)
            .map_err(map_track_repetition_error)
    }
    TrackRepeatCount::AutoFill => layout::TrackRepetition::auto_fill_components(components)
        .map_err(map_track_repetition_error),
    TrackRepeatCount::AutoFit => layout::TrackRepetition::auto_fit_components(components)
        .map_err(map_track_repetition_error),
}
```

Do not wrap these calls in `Ok(...)`; they already return `Result<layout::TrackRepetition, layout::TrackRepetitionError>`.

- [ ] **Step 5: Run focused style tests now that compile compatibility is restored**

Run:

```sh
cargo test -p surgeist-style aspect_ratio -- --nocapture
cargo test -p surgeist-style lower_grid_placement -- --nocapture
cargo test -p surgeist-style lower_track_repeat -- --nocapture
```

Expected: all focused adapter tests compile and pass.

- [ ] **Step 6: Run full style tests**

Run:

```sh
cargo test -p surgeist-style
```

Expected: all style tests pass.

- [ ] **Step 7: Commit the compile-restoring adapter changes**

Run:

```sh
git status --short --branch
git diff -- src/adapters/layout.rs
git add src/adapters/layout.rs
git commit -m "style: align layout adapter with validated layout types"
```

Expected: one implementation commit touching `src/adapters/layout.rs`.

## Task 3: Verify Layout Ledger And Handoff

**Files:**

- No intended edits unless API generation reveals unexpected drift.

- [ ] **Step 1: Run style Clippy**

Run:

```sh
cargo clippy -p surgeist-style --all-targets -- -D warnings
```

Expected: no warnings.

- [ ] **Step 2: Run formatting check**

Run:

```sh
cargo fmt --check
```

Expected: no formatting diff.

- [ ] **Step 3: Verify public API artifact has no source-derived drift**

Run:

```sh
cargo run --manifest-path api/generator/Cargo.toml
git diff --exit-code -- api/public-api.txt
```

Expected: no diff. If `api/public-api.txt` changes, inspect the diff. This plan does not intend public API changes; commit the artifact only if the generator reveals legitimate existing drift and explain that in the handoff.

- [ ] **Step 4: Run layout ledger pending verification**

Run from `/Users/codex/Development/surgeist-layout`:

```sh
cargo test -p surgeist-layout tests::aspect_ratio_rejects_non_positive_or_non_finite_values -- --nocapture
cargo test -p surgeist-layout --test layout layout::leaf -- --nocapture
cargo test -p surgeist-layout --test layout layout::block -- --nocapture
cargo test -p surgeist-layout --test layout layout::flex -- --nocapture
cargo test -p surgeist-layout --test layout layout::grid -- --nocapture
cargo test -p surgeist-layout grid::tests::public_grid_placement_rejects_zero_line_and_span -- --nocapture
cargo test -p surgeist-layout grid::tests::grid_placement_fields_are_constructed_through_validated_values -- --nocapture
cargo test -p surgeist-layout tests::track_repetition_rejects_zero_count_and_empty_components -- --nocapture
```

Expected: the ledger-listed layout checks no longer fail on `surgeist-style/src/adapters/layout.rs` type errors. If any command fails for a different layout-local reason, capture the exact failing command and error for layout follow-up.

- [ ] **Step 5: Review final git state**

Run:

```sh
git status --short --branch
git log --oneline -5
```

Expected: implementation files are committed and the working tree has no implementation leftovers.

- [ ] **Step 6: Report to top-level coordinator**

Include:

- commit SHAs
- style commands run and pass/fail results
- layout ledger commands run and pass/fail results
- whether `api/public-api.txt` changed
- whether layout ledger entries `LAYOUT-XCRATE-0001`, `LAYOUT-XCRATE-0002`, and `LAYOUT-XCRATE-0003` are ready for layout retest or closure

Do not update the root `surgeist` submodule pointer from this crate repo.

## Self-Review Checklist

- The plan implements only style-owned adapter changes from the layout ledger.
- Layout semantic invariants are enforced through layout constructors at the adapter boundary.
- Style public value-model refactors are out of scope.
- Named grid raw placement data remains preserved.
- Tests cover successful lowering plus validation-bypass cases for each ledger blocker.
- The plan no longer asks the worker to run passing test checkpoints before all current compile blockers are fixed.
- No sibling crate edits are required from this repo.
- No public API changes are intended.
