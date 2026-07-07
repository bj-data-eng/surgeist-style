# Resolver Cache, Invalidation, And Diagnostics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Operation 14 by making resolver cache behavior, invalidation summaries, and invalid-at-computed-value diagnostics explicit over the now-real authored/property/selector/condition/layer/scope models.

**Architecture:** `surgeist-style` owns typed cache/invalidation/diagnostic contracts, while root owns CSS source spans, host facts, import loading, and converting style diagnostics into root-facing reports. Keep `Resolver::resolve` as the simple existing API, and add explicit opt-in surfaces for diagnostics and cache/invalidation detail. Do not add CSS parser dependencies, source-span types, host querying, or sibling-crate adapters.

**Tech Stack:** Rust 2024, `surgeist-style`, crate-local unit tests, trybuild public type-safety tests, `cargo fmt`, `cargo test -p surgeist-style`, `cargo clippy -p surgeist-style --all-targets -- -D warnings`.

---

## Context

Operation 13 completed media/container condition models, cascade layer registration, scoped rule matching, and focused resolver cache hashing for the current condition facts. Operation 14 should tighten and expose the remaining integration contracts:

- cache keys and cache invalidation need tests and public invalidation summaries that account for authored declarations, style buckets, traversal, condition facts, selector root/scope, local/animated declarations, parent style, and tree/node state;
- invalidation should distinguish property output changes, custom property dependencies, selector fact changes, condition fact changes, layer/scope changes, and pseudo-element bucket changes;
- resolver diagnostics should report invalid-at-computed-value events without owning CSS source spans. Root can associate diagnostics with source records through an opaque style provenance token supplied while lowering.

## Boundaries

In scope:

- style-owned provenance token newtype for root-to-style diagnostic association;
- optional provenance on authored declarations after canonicalization into rule candidates;
- style-owned diagnostic models for invalid-at-computed-value resolution events;
- `Resolver::resolve_with_diagnostics` as the opt-in API that returns `Resolved` plus diagnostics;
- cache-key tests and any small private cache-key helper reshaping needed for correctness and readability;
- typed invalidation reason models and constructors;
- plan/ledger rebasing for Operation 14 completion and Operation 15 next context.

Out of scope:

- CSS source spans, filenames, byte offsets, parser diagnostics, or source maps;
- `!important` support;
- cascade origin support;
- network/filesystem import loading;
- host/media/container fact discovery;
- layout/text/render adapter work;
- broad selector dependency indexing beyond the typed invalidation contract documented here;
- changing the existing `Resolver::resolve` return type.

## Files

- Create: `src/diagnostic.rs`
- Modify: `src/authored.rs`
- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `src/invalidation.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_style_source_id_literal.rs`
- Create: `tests/compile_fail/invalid_style_source_id_literal.stderr`
- Create: `tests/compile_fail/invalid_style_diagnostic_literal.rs`
- Create: `tests/compile_fail/invalid_style_diagnostic_literal.stderr`
- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md` only if next-context text needs rebasing

---

### Task 1: Add Style Provenance And Diagnostic Models

**Files:**

- Create: `src/diagnostic.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_style_source_id_literal.rs`
- Create: `tests/compile_fail/invalid_style_source_id_literal.stderr`
- Create: `tests/compile_fail/invalid_style_diagnostic_literal.rs`
- Create: `tests/compile_fail/invalid_style_diagnostic_literal.stderr`

- [ ] **Step 1: Add failing diagnostic model tests**

In `src/diagnostic.rs`, add tests while creating the module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CustomPropertyName, Property};

    #[test]
    fn style_source_ids_reject_zero_and_preserve_opaque_value() {
        assert!(StyleSourceId::try_new(0).is_err());
        let source = StyleSourceId::try_new(42).unwrap();
        assert_eq!(source.get(), 42);
    }

    #[test]
    fn invalid_at_computed_value_diagnostics_preserve_subject_and_source() {
        let source = StyleSourceId::try_new(7).unwrap();
        let diagnostic = StyleDiagnostic::invalid_at_computed_value(
            StyleDiagnosticSubject::Property(Property::Color),
            Some(source),
            InvalidAtComputedValueReason::MissingCustomProperty(
                CustomPropertyName::try_new("--brand").unwrap(),
            ),
        );

        assert_eq!(diagnostic.kind(), StyleDiagnosticKind::InvalidAtComputedValue);
        assert_eq!(diagnostic.source(), Some(source));
        assert_eq!(
            diagnostic.subject(),
            &StyleDiagnosticSubject::Property(Property::Color)
        );
    }
}
```

Run:

```sh
cargo test -p surgeist-style diagnostic
```

Expected: fail until the module is implemented and exported.

- [ ] **Step 2: Implement the diagnostic module**

Create `src/diagnostic.rs`:

```rust
use crate::{CustomPropertyName, Error, ErrorCode, Property};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StyleSourceId {
    value: u64,
}

impl StyleSourceId {
    pub fn try_new(value: u64) -> crate::Result<Self> {
        if value == 0 {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "style source id must be non-zero",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleDiagnostic {
    kind: StyleDiagnosticKind,
    subject: StyleDiagnosticSubject,
    source: Option<StyleSourceId>,
    reason: InvalidAtComputedValueReason,
}

impl StyleDiagnostic {
    #[must_use]
    pub fn invalid_at_computed_value(
        subject: StyleDiagnosticSubject,
        source: Option<StyleSourceId>,
        reason: InvalidAtComputedValueReason,
    ) -> Self {
        Self {
            kind: StyleDiagnosticKind::InvalidAtComputedValue,
            subject,
            source,
            reason,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> StyleDiagnosticKind {
        self.kind
    }

    #[must_use]
    pub const fn subject(&self) -> &StyleDiagnosticSubject {
        &self.subject
    }

    #[must_use]
    pub const fn source(&self) -> Option<StyleSourceId> {
        self.source
    }

    #[must_use]
    pub const fn reason(&self) -> &InvalidAtComputedValueReason {
        &self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StyleDiagnosticKind {
    InvalidAtComputedValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StyleDiagnosticSubject {
    Property(Property),
    CustomProperty(CustomPropertyName),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvalidAtComputedValueReason {
    MissingCustomProperty(CustomPropertyName),
    InvalidCustomProperty(CustomPropertyName),
    MissingTypedCustomPropertyValue(CustomPropertyName, Property),
    CyclicCustomProperty(CustomPropertyName),
}
```

- [ ] **Step 3: Export the module and public types**

In `src/lib.rs`, add:

```rust
mod diagnostic;
```

and export:

```rust
pub use diagnostic::{
    InvalidAtComputedValueReason, StyleDiagnostic, StyleDiagnosticKind, StyleDiagnosticSubject,
    StyleSourceId,
};
```

- [ ] **Step 4: Add compile-fail tests for private fields**

Create `tests/compile_fail/invalid_style_source_id_literal.rs`:

```rust
use surgeist_style::StyleSourceId;

fn main() {
    let _source = StyleSourceId { value: 1 };
}
```

Create `tests/compile_fail/invalid_style_diagnostic_literal.rs`:

```rust
use surgeist_style::{
    InvalidAtComputedValueReason, Property, StyleDiagnostic, StyleDiagnosticKind,
    StyleDiagnosticSubject,
};

fn main() {
    let _diagnostic = StyleDiagnostic {
        kind: StyleDiagnosticKind::InvalidAtComputedValue,
        subject: StyleDiagnosticSubject::Property(Property::Color),
        source: None,
        reason: InvalidAtComputedValueReason::MissingTypedCustomPropertyValue(
            surgeist_style::CustomPropertyName::try_new("--brand").unwrap(),
            Property::Color,
        ),
    };
}
```

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
```

Expected: new stderr files prove private-field protection with `E0451`.

- [ ] **Step 5: Update public construction coverage**

In `tests/compile_pass/typed_public_construction.rs`, add construction for:

```rust
let source = StyleSourceId::try_new(99)?;
let diagnostic = StyleDiagnostic::invalid_at_computed_value(
    StyleDiagnosticSubject::Property(Property::Color),
    Some(source),
    InvalidAtComputedValueReason::MissingCustomProperty(
        CustomPropertyName::try_new("--brand")?,
    ),
);
assert_eq!(diagnostic.source(), Some(source));
```

- [ ] **Step 6: Run focused checks**

```sh
cargo test -p surgeist-style diagnostic
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/diagnostic.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_style_source_id_literal.rs tests/compile_fail/invalid_style_source_id_literal.stderr tests/compile_fail/invalid_style_diagnostic_literal.rs tests/compile_fail/invalid_style_diagnostic_literal.stderr
git commit -m "style: add diagnostic model"
```

---

### Task 2: Carry Optional Provenance Through Authored Rules

**Files:**

- Modify: `src/authored.rs`
- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add authored provenance propagation tests**

In `src/authored.rs`, add tests that do not depend on resolver diagnostics:

```rust
#[test]
fn authored_declarations_preserve_source_ids() {
    let source = StyleSourceId::try_new(5).unwrap();
    let declaration = AuthoredDeclaration::try_new(
        AuthoredProperty::Property(Property::Color),
        AuthoredValue::Value(Value::StyleColor(StyleColor::current_color())),
    )
    .unwrap()
    .with_source(source);

    assert_eq!(declaration.source(), Some(source));

    let mut declarations = AuthoredDeclarations::new();
    declarations.push(declaration);
    let declarations = declarations.with_source(source);

    assert!(declarations.iter().all(|item| item.source() == Some(source)));
}

#[test]
fn authored_canonical_declarations_preserve_replacement_source_ids() {
    let first = StyleSourceId::try_new(5).unwrap();
    let second = StyleSourceId::try_new(6).unwrap();
    let mut declarations = AuthoredDeclarations::new();
    declarations
        .try_push(
            AuthoredDeclaration::try_new(
                AuthoredProperty::Property(Property::Color),
                AuthoredValue::Value(Value::StyleColor(StyleColor::current_color())),
            )
            .unwrap()
            .with_source(first),
        )
        .unwrap();
    declarations
        .try_push(
            AuthoredDeclaration::css_wide(
                AuthoredProperty::Property(Property::Color),
                CssWideKeyword::Unset,
            )
            .with_source(second),
        )
        .unwrap();
    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.source(Property::Color), Some(second));
}
```

The second test requires adding a crate-private canonical source accessor in Step 3.

Run:

```sh
cargo test -p surgeist-style authored_declarations_preserve_source_ids
cargo test -p surgeist-style authored_canonical_declarations_preserve_replacement_source_ids
```

Expected: fail until provenance is stored on authored declarations and canonical declaration items.

- [ ] **Step 2: Add optional source to authored declarations**

In `src/authored.rs`, add a private field:

```rust
source: Option<StyleSourceId>,
```

to `AuthoredDeclaration`, initialize it to `None` in `try_new` and `css_wide`, and add:

```rust
#[must_use]
pub fn with_source(mut self, source: StyleSourceId) -> Self {
    self.source = Some(source);
    self
}

#[must_use]
pub const fn source(&self) -> Option<StyleSourceId> {
    self.source
}
```

Add matching `AuthoredDeclarations::with_source` for bulk lowering:

```rust
#[must_use]
pub fn with_source(mut self, source: StyleSourceId) -> Self {
    for declaration in &mut self.values {
        declaration.source = Some(source);
    }
    self
}
```

This gives root a simple way to map a lowered rule block or declaration group back to source records without style owning spans.

- [ ] **Step 3: Carry source through canonical declarations**

Change `AuthoredDeclarationItem` shapes to include source:

```rust
Property(Property, AuthoredCascadeValue, Option<StyleSourceId>),
Custom(CustomPropertyName, CustomPropertyCascadeValue, Option<StyleSourceId>),
```

Update insertion and accessors:

```rust
fn insert_property(
    &mut self,
    property: Property,
    value: AuthoredCascadeValue,
    source: Option<StyleSourceId>,
)

fn insert_custom(
    &mut self,
    name: CustomPropertyName,
    value: CustomPropertyCascadeValue,
    source: Option<StyleSourceId>,
)

pub(crate) fn source(&self, property: Property) -> Option<StyleSourceId>

pub(crate) fn custom_source(&self, name: &CustomPropertyName) -> Option<StyleSourceId>
```

When a later declaration replaces an earlier canonical property, replace both the value and source with the later declaration's source.

- [ ] **Step 4: Carry source into resolver candidates**

In `src/sheet.rs`, add `source: Option<StyleSourceId>` to `RuleDeclarationItem` and `RuleCustomDeclarationItem`, with getters:

```rust
#[must_use]
pub(crate) const fn source(&self) -> Option<StyleSourceId> {
    self.source
}
```

In `src/resolver.rs`, add the same field to `RuleCandidate` and `CustomPropertyCandidate`. Preserve it in `try_from_declaration` and `from_declaration`.

- [ ] **Step 5: Update public construction coverage**

In `tests/compile_pass/typed_public_construction.rs`, add:

```rust
let sourced_declaration = AuthoredDeclaration::try_new(
    AuthoredProperty::Property(Property::Color),
    AuthoredValue::Value(Value::StyleColor(StyleColor::current_color())),
)?
.with_source(StyleSourceId::try_new(100)?);
assert_eq!(sourced_declaration.source().unwrap().get(), 100);
```

- [ ] **Step 6: Run focused checks**

```sh
cargo test -p surgeist-style authored_declarations_preserve_source_ids
cargo test -p surgeist-style authored_canonical_declarations_preserve_replacement_source_ids
cargo test -p surgeist-style authored
cargo test -p surgeist-style resolver
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/authored.rs src/sheet.rs src/resolver.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: carry diagnostic provenance"
```

---

### Task 3: Add Diagnostic Resolver Output

**Files:**

- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add failing diagnostic behavior tests**

Add resolver tests:

```rust
#[test]
fn resolve_with_diagnostics_reports_missing_custom_property() {
    let source = StyleSourceId::try_new(11).unwrap();
    let missing = CustomPropertyName::try_new("--missing").unwrap();
    let declarations = variable_color_declarations(missing.clone(), None).with_source(source);
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let mut sheet = Sheet::new();
    sheet
        .push_authored_rule(
            Selector::tag("button").unwrap(),
            declarations,
            precedence(0, 0),
        )
        .unwrap();
    let mut resolver = Resolver::new(sheet);

    let output = resolver
        .resolve_with_diagnostics(Context::new(&tree, 0))
        .unwrap();

    assert_eq!(output.resolved().text_color(), &StyleColor::rgba(Color::BLACK));
    assert_eq!(
        output.diagnostics()[0].reason(),
        &InvalidAtComputedValueReason::MissingCustomProperty(missing)
    );
    assert_eq!(output.diagnostics()[0].source(), Some(source));
}

#[test]
fn existing_resolve_api_still_returns_only_resolved_style() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let mut resolver = Resolver::new(Sheet::new());

    let resolved = resolver.resolve(Context::new(&tree, 0)).unwrap();

    assert_eq!(resolved.text_color(), &StyleColor::rgba(Color::BLACK));
}
```

Run:

```sh
cargo test -p surgeist-style diagnostics
```

Expected: fail until `resolve_with_diagnostics` exists.

- [ ] **Step 2: Add resolved output wrapper**

In `src/resolver.rs`, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedWithDiagnostics {
    resolved: Resolved,
    diagnostics: Vec<StyleDiagnostic>,
}

impl ResolvedWithDiagnostics {
    #[must_use]
    pub(crate) fn new(resolved: Resolved, diagnostics: Vec<StyleDiagnostic>) -> Self {
        Self {
            resolved,
            diagnostics,
        }
    }

    #[must_use]
    pub const fn resolved(&self) -> &Resolved {
        &self.resolved
    }

    #[must_use]
    pub fn into_resolved(self) -> Resolved {
        self.resolved
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[StyleDiagnostic] {
        &self.diagnostics
    }
}
```

Export `ResolvedWithDiagnostics` from `src/lib.rs`.

- [ ] **Step 3: Refactor resolver internals without changing `resolve`**

Keep:

```rust
pub fn resolve<T: Tree>(&mut self, context: Context<'_, T>) -> Result<Resolved>
```

and implement:

```rust
pub fn resolve_with_diagnostics<T: Tree>(
    &mut self,
    context: Context<'_, T>,
) -> Result<ResolvedWithDiagnostics>
```

Use a private helper:

```rust
fn resolve_inner<T: Tree>(
    &mut self,
    context: Context<'_, T>,
    collect_diagnostics: bool,
) -> Result<ResolvedWithDiagnostics>
```

`resolve` should call `resolve_inner(..., false).map(ResolvedWithDiagnostics::into_resolved)`. Cache entries must continue to store `Resolved` only; diagnostics are recomputed when `resolve_with_diagnostics` is called so provenance does not become an implicit cache-key axis.

- [ ] **Step 4: Report invalid-at-computed-value diagnostics**

Extend variable evaluation so diagnostics are emitted when a variable-dependent ordinary property resolves through an invalid path and no fallback produces a value:

- missing referenced custom property;
- referenced custom property is invalid due to cycle;
- referenced custom property exists but has no typed value for the target property;
- fallback exists but also fails for one of these reasons.

Do not report diagnostics when a valid fallback produces a value. Do not report diagnostics for CSS parser errors or unsupported syntax; those remain root-owned.

Implementation shape:

```rust
struct DiagnosticCollector {
    enabled: bool,
    diagnostics: Vec<StyleDiagnostic>,
}

impl DiagnosticCollector {
    fn push_invalid_at_computed_value(
        &mut self,
        subject: StyleDiagnosticSubject,
        source: Option<StyleSourceId>,
        reason: InvalidAtComputedValueReason,
    ) {
        if self.enabled {
            self.diagnostics.push(StyleDiagnostic::invalid_at_computed_value(
                subject, source, reason,
            ));
        }
    }
}
```

Pass a mutable collector through `RuleEvaluator`. Include `RuleCandidate.source` in emitted diagnostics.

- [ ] **Step 5: Update public construction coverage**

In `tests/compile_pass/typed_public_construction.rs`, add a public type assertion for `ResolvedWithDiagnostics`:

```rust
fn _accepts_resolved_with_diagnostics(_value: Option<ResolvedWithDiagnostics>) {}
```

- [ ] **Step 6: Run focused checks**

```sh
cargo test -p surgeist-style diagnostics
cargo test -p surgeist-style resolver
cargo test -p surgeist-style custom_property
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: report resolver diagnostics"
```

---

### Task 4: Make Cache Key Coverage Explicit And Tested

**Files:**

- Modify: `src/resolver.rs`

- [ ] **Step 1: Add cache-key regression tests**

Add resolver tests:

```rust
#[test]
fn resolver_cache_key_distinguishes_condition_facts() {
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

    let wide = resolver
        .resolve(Context::new(&tree, 0).media_environment(
            MediaEnvironment::new().width(QueryLength::try_new(800.0, QueryLengthUnit::Px).unwrap()),
        ))
        .unwrap();
    let narrow = resolver
        .resolve(Context::new(&tree, 0).media_environment(
            MediaEnvironment::new().width(QueryLength::try_new(320.0, QueryLengthUnit::Px).unwrap()),
        ))
        .unwrap();

    assert_ne!(wide.text_color(), narrow.text_color());
}

#[test]
fn resolver_cache_key_distinguishes_local_and_animated_overlays() {
    let tree = TestTree::new(vec![TestNode::new(0, "button")]);
    let mut resolver = Resolver::new(Sheet::new());
    let local = Declarations::new()
        .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
        .unwrap();
    let animated = Declarations::new()
        .try_concrete_text_color(Color::raw_rgba(0.0, 1.0, 0.0, 1.0))
        .unwrap();

    let local_resolved = resolver
        .resolve(Context::new(&tree, 0).local(&local))
        .unwrap();
    let animated_resolved = resolver
        .resolve(Context::new(&tree, 0).animated(&animated))
        .unwrap();

    assert_ne!(local_resolved.text_color(), animated_resolved.text_color());
}
```

Run:

```sh
cargo test -p surgeist-style resolver_cache_key
```

Expected: pass if current cache keys are sufficient; fail if a missing axis remains.

- [ ] **Step 2: Add selector root/scope cache coverage**

Add this test unless an equivalent selector root/scope cache isolation test already exists in `src/resolver.rs`; if an equivalent exists, update that test to use the current selector APIs and assert the same isolation:

```rust
#[test]
fn resolver_cache_key_distinguishes_selector_scope_anchor() {
    let tree = TestTree::new(vec![
        TestNode::new(0, "root").children([1, 2]),
        TestNode::new(1, "section").class("scope").children([3]),
        TestNode::new(2, "section").class("other").children([4]),
        TestNode::new(3, "button"),
        TestNode::new(4, "button"),
    ]);
    let selector = Selector::complex([
        ComplexSelectorPart::root(Selector::compound().scope_anchor().class("scope").unwrap()),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().tag("button").unwrap(),
        ),
    ])
    .unwrap();
    let sheet = Sheet::new().rule(
        selector,
        Declarations::new()
            .try_concrete_text_color(Color::raw_rgba(1.0, 0.0, 0.0, 1.0))
            .unwrap(),
    );
    let mut resolver = Resolver::new(sheet);

    let scoped = resolver
        .resolve(Context::new(&tree, 3).selector_root(0).selector_scope(1))
        .unwrap();
    let wrong_scope = resolver
        .resolve(Context::new(&tree, 3).selector_root(0).selector_scope(2))
        .unwrap();

    assert_ne!(scoped.text_color(), wrong_scope.text_color());
}
```

- [ ] **Step 3: Refactor private cache hashing only when tests expose drift**

If all cache-axis tests pass, leave cache implementation unchanged and report that result in the worker summary. If a cache-axis test fails, update `Resolver::cache_key` or helper functions to include the missing axis. Keep cache internals private. Do not introduce a public cache key type in this task.

Rules:

- hash float-bearing style facts through `to_bits()`;
- do not hash source/provenance because diagnostics are recomputed outside cached `Resolved` entries;
- keep `tree.version_hint() == None` as the opt-out path for caching;
- do not add broad dependency indexing in this task.

- [ ] **Step 4: Run focused checks**

```sh
cargo test -p surgeist-style resolver_cache_key
cargo test -p surgeist-style resolver
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 5: Commit after worker/reviewer clean**

```sh
git add src/resolver.rs
git commit -m "style: cover resolver cache axes"
```

---

### Task 5: Add Typed Invalidation Reasons

**Files:**

- Modify: `src/invalidation.rs`
- Modify: `src/sheet.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add invalidation reason tests**

In `src/invalidation.rs`, add:

```rust
#[test]
fn condition_fact_changes_rematch_condition_dependent_rules() {
    let media = Change::from_condition_fact_change(ConditionFactChange::Media);
    let container = Change::from_condition_fact_change(ConditionFactChange::Container);

    assert!(media.rematch);
    assert!(media.scope.whole_tree);
    assert_eq!(media.invalidation, Invalidation::empty());
    assert!(container.rematch);
    assert!(container.scope.whole_tree);
}

#[test]
fn cascade_structure_changes_rematch_without_claiming_property_output() {
    for change in [
        Change::from_cascade_change(CascadeChange::LayerOrder),
        Change::from_cascade_change(CascadeChange::RuleScope),
        Change::from_style_bucket_change(StyleBucket::Before),
    ] {
        assert!(change.rematch);
        assert!(change.scope.whole_tree);
        assert_eq!(change.invalidation, Invalidation::empty());
    }
}
```

Run:

```sh
cargo test -p surgeist-style invalidation
```

Expected: fail until the new reason types exist.

- [ ] **Step 2: Add typed change reason enums**

In `src/invalidation.rs`, add:

```rust
use super::StyleBucket;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionFactChange {
    Media,
    Container,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CascadeChange {
    LayerOrder,
    RuleScope,
    SourceOrder,
}
```

Add `Change` constructors:

```rust
#[must_use]
pub fn from_condition_fact_change(_fact: ConditionFactChange) -> Self {
    let mut change = Self::empty();
    change.rematch = true;
    change.scope.include_whole_tree();
    change
}

#[must_use]
pub fn from_cascade_change(_change: CascadeChange) -> Self {
    let mut change = Self::empty();
    change.rematch = true;
    change.scope.include_whole_tree();
    change
}

#[must_use]
pub fn from_style_bucket_change(_bucket: StyleBucket) -> Self {
    let mut change = Self::empty();
    change.rematch = true;
    change.scope.include_whole_tree();
    change
}
```

These constructors are intentionally conservative. Operation 14 adds the typed reason contract; it does not implement a dependency-indexed invalidation engine.

- [ ] **Step 3: Wire sheet condition helpers through typed reasons**

Keep existing `Sheet::media_condition_change` and `Sheet::container_condition_change`, but ensure their behavior aligns with the new typed constructors:

- they should still include affected property impacts for rules using the matching condition type;
- keep the helpers sheet-local: if no matching conditional rules exist, return `Change::empty()`.

Add tests in `src/sheet.rs`:

```rust
#[test]
fn condition_change_helpers_remain_sheet_local() {
    let sheet = Sheet::new();

    assert_eq!(sheet.media_condition_change(), Change::empty());
    assert_eq!(sheet.container_condition_change(), Change::empty());
}
```

- [ ] **Step 4: Export new invalidation reason types**

In `src/lib.rs`, update the invalidation export:

```rust
pub use invalidation::{
    CascadeChange, Change, ConditionFactChange, Invalidation, Scope, SelectorFactChange,
};
```

- [ ] **Step 5: Update public construction coverage**

In `tests/compile_pass/typed_public_construction.rs`, add:

```rust
let condition_change = Change::from_condition_fact_change(ConditionFactChange::Media);
assert!(condition_change.rematch);
let cascade_change = Change::from_cascade_change(CascadeChange::LayerOrder);
assert!(cascade_change.scope.whole_tree);
```

- [ ] **Step 6: Run focused checks**

```sh
cargo test -p surgeist-style invalidation
cargo test -p surgeist-style sheet
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 7: Commit after worker/reviewer clean**

```sh
git add src/invalidation.rs src/sheet.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: model invalidation reasons"
```

---

### Task 6: Rebase Ledgers And Handoff Notes

**Files:**

- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md` only if next-context text needs rebasing

- [ ] **Step 1: Update Operation 14 ledger rows**

In `plans/2026-07-05-css-surface-style-ledger.md`, update only rows and summary text that refer to Operation 14 cache/invalidation/diagnostic gaps:

- `Resolver` summary should mention `resolve_with_diagnostics` and invalid-at-computed-value diagnostics;
- `Context` summary should mention current cache axes remain explicit inputs;
- `Invalidation` and `Change` summary should mention `ConditionFactChange`, `CascadeChange`, `SelectorFactChange`, and style-bucket changes;
- `Condition`, `MediaEnvironment`, and `ContainerFacts` gaps should say generalized invalidation hooks now exist, while host fact discovery remains outside style;
- root coordination questions should preserve any unresolved root decisions about source diagnostics and font-face ownership.

Do not mark unsupported selectors, `!important`, cascade origins, source spans, imports, or font loading complete.

- [ ] **Step 2: Update next context**

Add or update trailing context:

```markdown
After Operation 14 lands, the next implementation plan should cover Operation 15: API artifacts, compile-fail coverage, and root handoff notes over the completed CSS-facing style surface sequence.
```

- [ ] **Step 3: Run ledger search**

```sh
rg -n "Operation 14|Operation 15|resolve_with_diagnostics|StyleDiagnostic|StyleSourceId|ConditionFactChange|CascadeChange|SelectorFactChange|cache key|invalidation" plans
```

Expected: Operation 14 rows point to implemented models and Operation 15 is the next context.

- [ ] **Step 4: Run focused checks**

```sh
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 5: Commit after worker/reviewer clean**

```sh
git add plans/2026-07-05-css-surface-style-ledger.md plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: rebase cache invalidation diagnostic ledger"
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
! rg -n "StyleSourceId \\{|StyleDiagnostic \\{|ResolvedWithDiagnostics \\{" tests/compile_pass
git diff --check
git status --short --branch
```

Expected:

- full tests pass;
- clippy passes with `-D warnings`;
- no direct `surgeist-css` or `surgeist-text` coupling;
- public diagnostic/provenance structs cannot be constructed through literals outside the crate;
- status is clean except expected unpushed commits.

Assign a final holistic clean-context reviewer against:

- this plan;
- `AGENTS.md`;
- `guidance/surgeist-rust-modeling-guide.md`;
- the full implementation diff.

Reviewer must explicitly check:

- diagnostics use style-owned models and opaque source ids, not CSS spans;
- cache-key tests cover real implemented axes and do not invent speculative Operation 15 surfaces;
- invalidation reason types are semantic and do not collapse unrelated phases into string messages;
- root-owned source mapping, host fact discovery, imports, font loading, `!important`, and cascade origins remain outside this pass;
- `Resolver::resolve` remains source-compatible while diagnostics are opt-in.

## This Comes Next

After this Operation 14 plan is implemented and reviewed clean, write the Operation 15 implementation plan for API artifacts, compile-fail coverage, and root handoff notes. Operation 15 should audit the completed CSS-facing style sequence, update any crate-owned API artifacts, and produce explicit root-lowering notes without adding new style semantics.
