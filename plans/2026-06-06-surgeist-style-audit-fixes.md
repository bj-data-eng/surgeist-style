# Surgeist Style Audit Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Ohm's `surgeist::style` audit fixes with regression coverage and logical commits.

**Architecture:** Keep fixes inside `surgeist::style` unless retained traversal data is strictly required. Preserve the public `surgeist::style::*` front door while tightening correctness contracts.

**Tech Stack:** Rust, Cargo integration tests in `crates/surgeist/tests/style.rs`, strict `cargo clippy -p surgeist --all-targets -- -D warnings`.

---

### Task 1: P1 Resolver And Declaration Correctness

**Files:**
- Modify: `crates/surgeist/tests/style.rs`
- Modify: `crates/surgeist/src/style/resolver.rs`
- Modify: `crates/surgeist/src/style/property.rs`
- Modify: `crates/surgeist/src/style/declaration.rs`

- [ ] **Step 1: Write failing tests**

Add tests proving:
- resolving the same node with different parent inherited colors does not reuse stale cache entries;
- `try_insert(Property::Padding, Value::Color(...))` is rejected;
- resolver rejects property/value mismatches built through infallible `insert`.

- [ ] **Step 2: Verify red**

Run: `cargo test -p surgeist --test style`

Expected: failures for stale inherited cache and missing property/value compatibility validation.

- [ ] **Step 3: Implement minimal fix**

Add a resolved-style fingerprint for parent-aware cache keys and a `Property::validate_value(&Value)` compatibility check used by `Declaration::try_new`, `Declarations::try_insert`, and `Resolved::apply`.

- [ ] **Step 4: Verify green**

Run: `cargo test -p surgeist --test style`

Expected: all style integration tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/surgeist/tests/style.rs crates/surgeist/src/style/resolver.rs crates/surgeist/src/style/property.rs crates/surgeist/src/style/declaration.rs
git commit -m "Fix Surgeist style resolution contracts"
```

### Task 2: Keyword And Selector Validation

**Files:**
- Modify: `crates/surgeist/tests/style.rs`
- Modify: `crates/surgeist/src/style/resolver.rs`
- Modify: `crates/surgeist/src/style/property.rs`
- Modify: `crates/surgeist/src/style/selector.rs`

- [ ] **Step 1: Write failing tests**

Add tests proving:
- `Keyword::Inherit`, `Keyword::Initial`, and `Keyword::Unset` resolve to parent/default behavior;
- `Selector::try_complex([])` and a first `Part::related(...)` are rejected.

- [ ] **Step 2: Verify red**

Run: `cargo test -p surgeist --test style`

Expected: failures for keyword resolution and missing fallible complex selector validation.

- [ ] **Step 3: Implement minimal fix**

Resolve keywords during cascade application and add `Selector::try_complex` validation. Keep `Selector::complex` available only if it can produce a valid selector or route through validation.

- [ ] **Step 4: Verify green**

Run: `cargo test -p surgeist --test style`

Expected: all style integration tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/surgeist/tests/style.rs crates/surgeist/src/style/resolver.rs crates/surgeist/src/style/property.rs crates/surgeist/src/style/selector.rs
git commit -m "Resolve Surgeist style keywords"
```

### Task 3: Invalidation Scope And API Polish

**Files:**
- Modify: `crates/surgeist/tests/style.rs`
- Modify: `crates/surgeist/src/style/invalidation.rs`
- Modify: `crates/surgeist/src/style/sheet.rs`
- Modify: `crates/surgeist/src/style/declaration.rs`
- Modify: `crates/surgeist/src/style/resolver.rs`
- Modify: `crates/surgeist/src/style/mod.rs`

- [ ] **Step 1: Write failing tests**

Add tests proving:
- structure/projection changes report sibling/subtree-style invalidation scope;
- inherited property changes report descendant impact;
- `Rule::conditions()` works and `conditions_slice()` is no longer needed by public callers;
- `s::color(0xRRGGBBAA)` helper and additional typed builders/accessors work.

- [ ] **Step 2: Verify red**

Run: `cargo test -p surgeist --test style`

Expected: failures for missing scope/API polish.

- [ ] **Step 3: Implement minimal fix**

Add explicit invalidation scope facts, public API aliases/helpers, and concise typed builder/accessor coverage for the first-pass style properties that are already represented.

- [ ] **Step 4: Verify green**

Run: `cargo test -p surgeist --test style`

Expected: all style integration tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/surgeist/tests/style.rs crates/surgeist/src/style/invalidation.rs crates/surgeist/src/style/sheet.rs crates/surgeist/src/style/declaration.rs crates/surgeist/src/style/resolver.rs crates/surgeist/src/style/mod.rs
git commit -m "Polish Surgeist style invalidation API"
```

### Task 4: Full Verification

**Files:**
- No new files expected.

- [ ] **Step 1: Run full checks**

```sh
cargo fmt --check -p surgeist
git diff --check
cargo test -p surgeist
cargo clippy -p surgeist --all-targets -- -D warnings
```

- [ ] **Step 2: Commit any final cleanup**

If verification requires small cleanup, commit it separately with a short concrete message.
