# Surgeist Style Layout Integration Readiness Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the style crate's public integration contract current for layout browser parity support after the type-safety modeling work.

**Architecture:** `surgeist-style` keeps the typed declaration and semantic value constructors introduced by the modeling pass. Do not reopen unchecked declaration insertion, do not add compatibility aliases for old struct/enum payload shapes, and do not add another style-to-layout lowering layer. This plan only verifies and refreshes style-owned public API artifacts so layout can migrate to the intentional public API.

**Tech Stack:** Rust 2024, `surgeist-style`, `surgeist-layout`, trybuild, source-derived public API artifact generator in `api/generator`.

---

## Source Ledger

This plan handles the style-owned portion of:

- `/Users/codex/Development/surgeist-layout/plans/2026-06-27-surgeist-layout-generic-scalar-cross-crate-ledger.md`

Covered integration facts:

- Layout browser parity support must use `Declarations::try_insert`, not private `Declarations::insert`.
- Layout browser parity support must use style semantic constructors such as `GridTrackComponent::line_names`, `SubgridTrack::from_components`, `TrackRepeat::{count, auto_fill, auto_fit}`, `SubgridLineNameComponent::{line_names, repeat}`, `SubgridLineNameRepeatCount::count`, and `GridLine::{line, span, bare_ident, named_line, named_span}`.
- Style's source-derived API artifact must reflect the current source, including the `CalcLength::sum(first, rest)` API and `Declarations::try_insert`.

## Non-Goals

- Do not edit `surgeist-layout` from this repo.
- Do not make `Declarations::insert` public.
- Do not add unchecked constructors or compatibility aliases for the old public payload shapes.
- Do not move `src/adapters/layout.rs` in this pass.
- Do not update root `surgeist` submodule pointers from this repo.

## Task 1: Verify The Public Integration Contract

**Files:**

- Inspect: `src/declaration.rs`
- Inspect: `src/value.rs`
- Inspect: `src/calc.rs`
- Inspect: `src/lib.rs`

- [ ] **Step 1: Confirm declaration insertion is intentionally fallible**

Run:

```sh
rg -n "pub fn try_insert|fn insert\\(" src/declaration.rs
```

Expected output includes:

```text
pub fn try_insert(&mut self, property: Property, value: Value) -> Result<&mut Self>
fn insert(&mut self, property: Property, value: Value) -> &mut Self
```

Decision: keep `try_insert` public and `insert` private. `try_insert` is the integration API layout should call.

- [ ] **Step 2: Confirm semantic grid constructors exist**

Run:

```sh
rg -n "pub fn line_names|pub fn from_components|pub fn count\\(|pub fn auto_fill|pub fn auto_fit|pub fn line\\(|pub fn span\\(|pub fn bare_ident|pub fn named_line|pub fn named_span|pub fn repeat\\(" src/value.rs
```

Expected: each constructor is present. These are the only style value construction paths layout fixture support should use for these domains.

- [ ] **Step 3: Confirm calc sum source signature**

Run:

```sh
rg -n "pub fn sum\\(" src/calc.rs
```

Expected:

```text
pub fn sum(first: CalcLengthTerm, rest: impl IntoIterator<Item = CalcLengthTerm>) -> Self
```

Decision: keep this signature. It prevents ordinary empty sum construction while keeping concise public construction.

## Task 2: Refresh The Source-Derived Public API Artifact

**Files:**

- Modify: `api/public-api.txt`

- [ ] **Step 1: Regenerate the public API artifact**

Run:

```sh
cargo run --manifest-path api/generator/Cargo.toml
```

Expected:

```text
wrote /Users/codex/Development/surgeist-style/api/public-api.txt
```

- [ ] **Step 2: Verify the artifact reflects current source**

Run:

```sh
rg -n "CalcLength::sum|Declarations::try_insert|GridLine::named_span|GridTrackComponent::line_names" api/public-api.txt
```

Expected output includes:

```text
pub fn surgeist_style::CalcLength::sum(first: surgeist_style::CalcLengthTerm, rest: impl core::iter::traits::collect::IntoIterator<Item = surgeist_style::CalcLengthTerm>) -> Self
pub fn surgeist_style::Declarations::try_insert(&mut self, property: surgeist_style::Property, value: surgeist_style::Value) -> surgeist_style::Result<&mut Self>
pub fn surgeist_style::GridLine::named_span(name: impl core::convert::Into<alloc::string::String>, index: u16) -> surgeist_style::Result<Self>
pub fn surgeist_style::GridTrackComponent::line_names(names: impl core::iter::traits::collect::IntoIterator<Item = impl core::convert::Into<alloc::string::String>>) -> surgeist_style::Result<Self>
```

- [ ] **Step 3: Review the artifact diff**

Run:

```sh
git diff -- api/public-api.txt
```

Expected: the diff only reflects source-derived API changes from the committed style modeling work. It must not show handwritten edits, removed public constructors needed by layout fixture support, or a reintroduced public unchecked `Declarations::insert`.

## Task 3: Run Style Verification

**Files:**

- Verify: `src/declaration.rs`
- Verify: `src/value.rs`
- Verify: `src/calc.rs`
- Verify: `api/public-api.txt`

- [ ] **Step 1: Run style tests**

Run:

```sh
cargo test -p surgeist-style
```

Expected: PASS.

- [ ] **Step 2: Run style clippy**

Run:

```sh
cargo clippy -p surgeist-style --all-targets -- -D warnings
```

Expected: PASS.

- [ ] **Step 3: Run formatting check**

Run:

```sh
cargo fmt --check
```

Expected: PASS.

- [ ] **Step 4: Review final diff**

Run:

```sh
git diff --stat
git diff -- api/public-api.txt
```

Expected: only `api/public-api.txt` changes for this plan unless a verification command exposed a real style-owned source bug.

## Review Gate

Ask a clean reviewer to check:

- `Declarations::try_insert` remains the public declaration construction API for parser/fixture integration.
- `Declarations::insert` remains private.
- No compatibility aliases or unchecked public constructors were added.
- `api/public-api.txt` is source-derived and current.
- The layout plan can migrate browser parity support without needing any additional style API.

Completion for this plan is reviewer-clean style readiness plus passing style verification.
