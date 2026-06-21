# Surgeist Style Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `surgeist::style` from `surgeist/requirements/style.md` as a headless, typed, Rust-authored style layer.

**Architecture:** Add `crates/surgeist/src/style` as a sibling module with focused files for values, properties, declarations, selectors, conditions, tree adapters, sheets, resolver, invalidation, and tests. Keep style free of CSS parsing and host rendering while reusing retained facts through a read-only tree trait.

**Tech Stack:** Rust, `surgeist::retained`, `surgeist::text`, `peniko`, standard collections, `cargo test -p surgeist`.

---

### Task 1: Values, Properties, Declarations, And Metadata

**Files:**
- Create: `crates/surgeist/src/style/mod.rs`
- Create: `crates/surgeist/src/style/error.rs`
- Create: `crates/surgeist/src/style/value.rs`
- Create: `crates/surgeist/src/style/property.rs`
- Create: `crates/surgeist/src/style/declaration.rs`
- Modify: `crates/surgeist/src/lib.rs`
- Test: `crates/surgeist/src/style/tests.rs`

- [ ] **Step 1: Write failing tests**

Add tests named:

```rust
#[test]
fn declarations_override_earlier_values_and_expose_typed_accessors() {}

#[test]
fn declaration_fingerprint_changes_with_content() {}

#[test]
fn metadata_reports_defaults_inheritance_impact_and_animation() {}
```

Run: `cargo test -p surgeist style::tests::declarations --lib`

Expected: compile failure because `surgeist::style` does not exist.

- [ ] **Step 2: Implement minimal module front door and value/property types**

Create the module files and expose `Length`, `Edges`, `Corners`, `Color`, `Shadow`, `Stroke`, `Value`, `Property`, `Metadata`, `Impact`, `Declaration`, `Declarations`, `Fingerprint`, `Error`, and `Result`.

- [ ] **Step 3: Verify tests**

Run: `cargo test -p surgeist style::tests --lib`

Expected: style value/declaration/property tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/surgeist/src/lib.rs crates/surgeist/src/style
git commit -m "Implement Surgeist style values"
```

### Task 2: Selectors, Conditions, And Tree Fixtures

**Files:**
- Create: `crates/surgeist/src/style/selector.rs`
- Create: `crates/surgeist/src/style/condition.rs`
- Create: `crates/surgeist/src/style/tree.rs`
- Modify: `crates/surgeist/src/style/mod.rs`
- Test: `crates/surgeist/src/style/tests.rs`

- [ ] **Step 1: Write failing tests**

Add tests named:

```rust
#[test]
fn selectors_match_tags_classes_keys_states_attributes_and_positions() {}

#[test]
fn complex_selectors_match_descendant_child_adjacent_and_sibling_paths() {}

#[test]
fn conditions_require_all_viewport_and_container_queries_to_match() {}
```

Run: `cargo test -p surgeist style::tests::selectors --lib`

Expected: compile failure because selector/tree/condition APIs are missing.

- [ ] **Step 2: Implement selector, condition, and fixture tree APIs**

Implement `Selector`, `Compound`, `Part`, `Combinator`, `AttributeSelector`, `PositionSelector`, `Position`, `Nth`, `Condition`, `Viewport`, `Container`, `Tree`, `Node`, `Traversal`, and `StateFlag` re-export.

- [ ] **Step 3: Verify tests**

Run: `cargo test -p surgeist style::tests --lib`

Expected: selector and condition tests pass with existing declaration tests.

- [ ] **Step 4: Commit**

```bash
git add crates/surgeist/src/style
git commit -m "Implement Surgeist style selectors"
```

### Task 3: Sheets, Rule Indexing, And Resolver

**Files:**
- Create: `crates/surgeist/src/style/sheet.rs`
- Create: `crates/surgeist/src/style/resolver.rs`
- Modify: `crates/surgeist/src/style/mod.rs`
- Test: `crates/surgeist/src/style/tests.rs`

- [ ] **Step 1: Write failing tests**

Add tests named:

```rust
#[test]
fn sheet_indexes_reduce_candidate_rules() {}

#[test]
fn resolver_applies_defaults_inheritance_rules_local_and_animated_overlays() {}

#[test]
fn resolver_cache_reuses_entries_only_with_matching_identity() {}
```

Run: `cargo test -p surgeist style::tests::resolver --lib`

Expected: compile failure because sheet/resolver APIs are missing.

- [ ] **Step 2: Implement sheet and resolver APIs**

Implement `Sheet`, `Rule`, `Version`, rule indexes, `Resolved`, `Resolver`, `Context`, all-match condition filtering, projected traversal defaults, inheritance, overlay precedence, and cache identity using sheet version, tree version hint, node id, traversal, viewport, container, and declaration fingerprints.

- [ ] **Step 3: Verify tests**

Run: `cargo test -p surgeist style::tests --lib`

Expected: resolver tests pass with earlier tests.

- [ ] **Step 4: Commit**

```bash
git add crates/surgeist/src/style
git commit -m "Implement Surgeist style resolver"
```

### Task 4: Retained Adapter And Invalidation

**Files:**
- Create: `crates/surgeist/src/style/invalidation.rs`
- Modify: `crates/surgeist/src/style/tree.rs`
- Modify: `crates/surgeist/src/style/mod.rs`
- Test: `crates/surgeist/src/style/tests.rs`

- [ ] **Step 1: Write failing tests**

Add tests named:

```rust
#[test]
fn retained_snapshot_adapts_canonical_and_default_projected_traversal() {}

#[test]
fn invalidation_classifies_changed_properties_and_retained_changes() {}

#[test]
fn large_tree_resolution_uses_indexes_and_cache() {}
```

Run: `cargo test -p surgeist style::tests --lib`

Expected: failures for retained adapter, invalidation, and large-tree behavior.

- [ ] **Step 2: Implement retained adapter and invalidation APIs**

Implement `Tree` for a retained snapshot adapter, default-slot projected traversal, `Change`, `Invalidation`, property-impact accumulation, and retained `ChangeSet` classification.

- [ ] **Step 3: Verify focused tests**

Run: `cargo test -p surgeist style::tests --lib`

Expected: all style tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/surgeist/src/style
git commit -m "Implement Surgeist style retained integration"
```

### Task 5: Final Verification And Spec Audit

**Files:**
- Read: `surgeist/requirements/style.md`
- Read: `crates/surgeist/src/style/*.rs`

- [ ] **Step 1: Run focused verification**

Run:

```bash
cargo test -p surgeist style --lib
```

Expected: all style tests pass.

- [ ] **Step 2: Run crate verification**

Run:

```bash
cargo test -p surgeist --lib
cargo fmt --check
```

Expected: all tests pass and formatting is clean.

- [ ] **Step 3: Audit against the spec**

Check each section in `surgeist/requirements/style.md` against the implementation. Record any intentional deferrals in the final response and keep the goal active if required scope remains unimplemented.

- [ ] **Step 4: Final commit if needed**

```bash
git add crates/surgeist/src/style crates/surgeist/src/lib.rs
git commit -m "Finalize Surgeist style implementation"
```
