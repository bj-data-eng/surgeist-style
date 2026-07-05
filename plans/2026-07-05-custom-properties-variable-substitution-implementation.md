# Custom Properties And Variable Substitution Implementation Plan

> **For agentic workers:** Execute this plan through the local `AGENTS.md`
> coordinator workflow. Workers follow the checkbox steps, do not commit, do
> not create branches, and do not let external workflow guidance override the
> crate's worker/reviewer gate.

**Goal:** Add style-owned custom property names, authored token payloads,
variable reference/fallback models, custom property inheritance, variable
substitution evaluation, cycle handling, and invalid-at-computed-value behavior
without importing `surgeist-css` or parsing CSS syntax in style.

**Architecture:** Root lowers parsed CSS custom property and `var(...)`
syntax into style-owned typed data. Style owns the resulting cascade,
inheritance, custom property environment, variable dependency graph,
substitution traversal, fallback handling, cycle detection, and
invalid-at-computed-value resolution. Style stores authored custom property
tokens for equality, diagnostics, and future style queries, but ordinary
computed properties resolve through typed style expressions, not raw CSS text.

**Tech Stack:** Rust 2024, crate-local modules in `src/`, existing unit tests
and `trybuild`, `cargo test -p surgeist-style`, `cargo clippy -p
surgeist-style --all-targets -- -D warnings`, `cargo fmt --check`.

---

## Coordinator Workflow Note

The coordinator executes this plan through `AGENTS.md`. Assign one scoped task
or tightly coupled task group to each worker, assign a separate reviewer after
each worker, commit each clean task as a logical point, and assign a final
holistic reviewer after all tasks are complete.

Workers must not commit. Reviewers must not edit files. Do not fork full
conversation context into workers or reviewers; provide only the scoped prompt,
relevant files, commands, and constraints.

## Source Context

Read these files before executing Task 1:

- `AGENTS.md`
- `Cargo.toml`
- `README.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-support-directive.md`
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `plans/2026-07-05-authored-cascade-keywords-implementation.md`
- `src/lib.rs`
- `src/authored.rs`
- `src/declaration.rs`
- `src/sheet.rs`
- `src/resolver.rs`
- `src/invalidation.rs`
- `src/property.rs`
- `src/value.rs`
- `tests/type_safety.rs`
- `tests/compile_pass/typed_public_construction.rs`

Read-only CSS source checked while writing this plan:

- `/Users/codex/Development/surgeist-css` at
  `1c95d4218439f1696151e0ee9602671fab418314`
- Relevant CSS API types in `src/syntax.rs`:
  `CssCustomPropertyName`, `CssAuthoredDeclarationValue`,
  `CssVariableReference`, `CssVariableFallback`,
  `CssCustomPropertyValue`, and `CssVariableDependentValue`.
- Relevant CSS parser behavior in `src/parser/variables.rs` and
  `src/parser/mod.rs`.

Style source snapshot used for this plan:

- `493dd04dc4ddf756bae24485a84f3fdb10b46146`

## Scope

This plan implements Operation 4 from
`plans/2026-07-05-css-surface-style-operations-sequence.md`:

- style-owned custom property names;
- style-owned authored token payloads for custom properties and variable
  fallback diagnostics;
- style-owned variable references with recursive fallback expressions;
- custom property declarations in authored sheets;
- custom property inheritance by default;
- CSS-wide keyword semantics for custom properties;
- variable substitution over root-lowered typed expressions;
- cycle detection across custom property references;
- invalid-at-computed-value fallback for ordinary properties;
- dependency tracking sufficient for cache fingerprints, resolved-style
  inspection, and custom-property invalidation summaries.

This plan deliberately does not implement:

- CSS parsing or reparsing substituted CSS token streams in style;
- a `surgeist-css` dependency;
- root lowering from `surgeist-css` into these new style APIs;
- CSS `!important`, origin precedence, specificity precedence, or CSS
  `revert`;
- selector expansion, pseudo-element buckets, media/container/scope query
  expansion, or broad property family expansion;
- `@property` registration or typed custom property registration;
- arbitrary CSS grammar interpretation from raw custom property token text;
- compatibility aliases for older names or unchecked constructors.

Root must reject CSS variable-dependent ordinary declarations it cannot lower
into the typed style expressions from this plan. This is intentional: style
owns variable semantics after receipt, but root remains the CSS parser and
syntax-to-style lowering boundary.

## Public API Shape

Workers may refine names only if the reviewer agrees the refinement makes the
model more type-safe without widening scope. The expected public API is:

```rust
pub struct CustomPropertyName { /* private fields */ }
pub struct AuthoredTokens { /* private fields */ }

pub enum VariableExpression {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
    Reference(VariableReference),
}

pub struct VariableReference { /* private fields */ }
pub struct VariableFallback { /* private fields */ }
pub struct CustomPropertyValue { /* private fields */ }
pub struct CustomPropertyTypedValue { /* private fields */ }
pub struct VariableDependentValue { /* private fields */ }
pub struct CustomPropertyDependencies { /* private fields */ }
pub struct CustomPropertyResolution { /* private fields */ }
```

Expected constructor and accessor behavior:

- `CustomPropertyName::try_new(value) -> Result<Self>` accepts names with a
  `--` prefix, a non-empty suffix, and suffix characters that are
  alphanumeric, `-`, or `_`. It preserves case. Invalid names return
  `ErrorCode::InvalidString`.
- `CustomPropertyName::as_str(&self) -> &str` exposes the preserved name.
- `AuthoredTokens::new(value) -> Self` preserves root-supplied authored text,
  including an empty string for empty `var()` fallbacks and empty custom
  property values. It is symbolic data only, not a parsing API.
- `AuthoredTokens::as_css(&self) -> &str` exposes the preserved text for root
  diagnostics and future style query comparisons.
- `VariableReference::new(name, fallback)` stores a custom property name and an
  optional recursive `VariableFallback`.
- `VariableFallback::new(authored, expression)` stores the authored fallback
  text and the root-lowered style expression to use if the referenced custom
  property is missing, invalid, cyclic, or not typed for the target property.
  Its private storage should use indirection such as `Box<VariableExpression>`
  where needed so recursive fallback expressions are representable in Rust
  without infinitely sized types.
- `VariableExpression::references()` or an equivalent accessor returns the
  custom property dependency set implied by the expression and nested
  fallbacks.
- `VariableExpression::Value(value)` branches are validated against the target
  `Property` when placed in `VariableDependentValue` or
  `CustomPropertyTypedValue`.
- `CustomPropertyValue::new(authored, references)` stores the authored custom
  property value and symbolic reference list.
- `CustomPropertyValue::with_typed_value(property, expression)` or an
  equivalent builder attaches a target-property-specific typed interpretation
  for ordinary property substitution. If no typed value exists for a target
  property, referencing that custom property for that target is invalid at
  computed-value time.
- `CustomPropertyTypedValue::try_new(property, expression) -> Result<Self>`
  validates literal `Value` branches against `property`. It may contain zero
  variable references because custom properties such as `--brand: red` can have
  target-specific literal interpretations.
- `VariableDependentValue::try_new(property, authored, expression) ->
  Result<Self>` validates literal branches against `property`, requires at
  least one variable reference, and records the target property.
- `VariableDependentValue::property(&self) -> Property`,
  `authored(&self) -> &AuthoredTokens`, and dependency accessors expose
  immutable facts only.
- `CustomPropertyResolution` exposes immutable inspection such as `value()` and
  `is_invalid()` while keeping constructors crate-private. Invalid resolution
  is a computed environment state for missing, invalid, and cyclic values, not
  a public constructor shortcut for authored declarations.

If the worker finds a simpler shape that preserves these invariants, prefer
the simpler shape. Do not replace typed expressions with a raw string that the
resolver later interprets.

## CSS-To-Style Lowering Contract

Root lowers CSS types into style types after this plan lands:

- `CssCustomPropertyName` -> `CustomPropertyName`
- `CssAuthoredDeclarationValue` -> `AuthoredTokens`
- `CssVariableReference` -> `VariableReference`
- `CssVariableFallback` -> `VariableFallback`
- `CssCustomPropertyValue` -> `CustomPropertyValue`
- `CssVariableDependentValue` -> `VariableDependentValue`

The CSS crate currently preserves authored variable-dependent values
symbolically and skips post-substitution validation. Style therefore must
perform computed-time validation from typed lowered expressions. If root cannot
lower a CSS authored value or fallback into a target-property-specific
`VariableExpression`, root rejects that declaration for now with an
unsupported-integration diagnostic. Style must not parse the `AuthoredTokens`
string to compensate.

Examples:

- `gap: var(--space, 8px)` can lower to a `VariableDependentValue` targeting
  `Property::Gap`, with a reference to `--space` and a typed fallback
  expression containing `Value::Length(...)`.
- `color: var(--brand, red)` can lower when root can represent `red` as a
  style `Color`.
- `width: calc(var(--w) + 1px)` must be rejected by root until root and style
  have an explicit typed expression for that grammar. It must not be passed to
  style as an ordinary raw CSS string for style to parse.

## Custom Property Semantics

Custom properties inherit by default:

- a child `Resolved` starts with the parent's valid custom property
  environment;
- a child custom property declaration with a valid value overrides the
  inherited value;
- a missing custom property remains missing;
- a cyclic custom property is invalid for substitution;
- a custom property whose target-property typed interpretation is absent is
  invalid only for that target property, not necessarily absent from authored
  custom property inspection.

CSS-wide keywords on custom properties use the authored keyword path:

- `inherit`: copy the parent custom property resolution for that name;
- `unset`: same as `inherit`, because custom properties inherit by default;
- `initial`: clear the valid custom property value for that name so `var()`
  uses fallback or becomes invalid at computed-value time;
- `revert-layer`: resolve the same custom property from lower layer candidates;
  if no lower layer candidate exists, resolve as `unset`.

`AuthoredProperty::All` must not affect custom properties.

## Invalid-At-Computed-Value Semantics

When a winning ordinary property declaration is variable-dependent, style
evaluates the `VariableDependentValue` against the computed custom property
environment:

- if the expression resolves to `Value(value)`, validate `value` against the
  target property and use it;
- if the expression resolves to `CssWideKeyword(keyword)`, resolve the keyword
  through the existing supported CSS-wide keyword path for that property;
- if the expression resolves to `CssWideKeyword::RevertLayer`, resolve it with
  the winning ordinary candidate's layer/candidate context. It must not be
  treated as a contextless keyword fallback; if no candidate context is
  available, resolve it as `unset`;
- if a reference is missing, invalid, cyclic, or lacks a typed value for the
  target property, evaluate its fallback expression if present;
- if no fallback produces a valid result, the declaration is invalid at
  computed-value time and the property resolves as `unset`;
- cycles are detected with a per-resolution stack of `CustomPropertyName`
  values and must not recurse indefinitely;
- cycle invalidation is per custom property graph, not a process-wide poison
  state.

The invalid-at-computed-value result must be observable enough for tests and
future root diagnostics. A public diagnostic type is optional in this plan, but
the worker must at least keep the resolver behavior deterministic and tested.

## Expected File Structure

- Create: `src/custom.rs`
  - Owns custom property names, authored tokens, variable references,
    fallback/expression models, typed custom property interpretations,
    dependency collection, and unit tests.
- Modify: `src/lib.rs`
  - Exports the new public front-door types.
- Modify: `src/authored.rs`
  - Adds `AuthoredProperty::Custom`, `AuthoredValue::CustomProperty`, and
    `AuthoredValue::VariableDependent` or equivalent typed variants.
- Modify: `src/sheet.rs`
  - Allows authored rules to carry both canonical property declarations and
    custom property declarations.
- Modify: `src/resolver.rs`
  - Builds inherited custom property environments, resolves custom property
    candidates by layer/source precedence, resolves variable-dependent ordinary
    property candidates, handles cycles, and includes custom properties in
    fingerprints.
- Modify: `src/invalidation.rs`
  - Adds the minimal custom-property dependency facts needed to describe
    changes caused by custom property updates.
- Modify: `tests/compile_pass/typed_public_construction.rs`
  - Proves public construction of valid custom property and variable APIs.
- Create: `tests/compile_fail/invalid_custom_property_struct_literal.rs`
  - Proves custom property structs have private fields.
- Create: `tests/compile_fail/invalid_custom_property_name_newtype_literal.rs`
  - Proves custom property names cannot be tuple-literal constructed.
- Create: `tests/compile_fail/invalid_variable_dependent_struct_literal.rs`
  - Proves variable-dependent declarations cannot bypass constructors.
- Create expected `trybuild` stderr files only by running the documented
  `TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety` command
  after the compile-fail tests are written.

## Task 1: Add Custom Property Core Types

**Files:**

- Create: `src/custom.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add name and token tests first**

Create unit tests in `src/custom.rs`:

```rust
#[test]
fn custom_property_name_preserves_case_and_accepts_css_custom_shape() {
    let name = CustomPropertyName::try_new("--BrandColor").unwrap();
    assert_eq!(name.as_str(), "--BrandColor");
    assert_eq!(
        CustomPropertyName::try_new("--brand_color-1")
            .unwrap()
            .as_str(),
        "--brand_color-1",
    );
}

#[test]
fn custom_property_name_rejects_non_custom_names() {
    for invalid in ["color", "-gap", "--", "-- bad", "--;", "--gap;", "--gap\n"] {
        let error = CustomPropertyName::try_new(invalid).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidString);
    }
}

#[test]
fn authored_tokens_preserve_empty_and_non_empty_css() {
    assert_eq!(AuthoredTokens::new("").as_css(), "");
    assert_eq!(
        AuthoredTokens::new("calc(var(--space) * 2)").as_css(),
        "calc(var(--space) * 2)"
    );
}
```

- [ ] **Step 2: Implement `CustomPropertyName`**

Create `CustomPropertyName` with private fields. Validation must match the CSS
surface observed for custom property names:

- name starts with `--`;
- suffix is non-empty;
- suffix chars are alphanumeric, `-`, or `_`;
- case is preserved.

Use `ErrorCode::InvalidString` for invalid names.

- [ ] **Step 3: Implement `AuthoredTokens`**

Create `AuthoredTokens` with private fields and immutable accessors. Do not
interpret its string as CSS inside style.

- [ ] **Step 4: Export the new public types**

Modify `src/lib.rs` to `mod custom;` and export `CustomPropertyName` and
`AuthoredTokens`.

- [ ] **Step 5: Update compile-pass public construction**

Add examples to `tests/compile_pass/typed_public_construction.rs` showing valid
public construction:

```rust
let custom_name = CustomPropertyName::try_new("--brand")?;
let authored = AuthoredTokens::new("var(--brand, #000)");
assert_eq!(custom_name.as_str(), "--brand");
assert_eq!(authored.as_css(), "var(--brand, #000)");
```

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style custom_property
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 2: Add Variable Reference And Typed Expression Models

**Files:**

- Modify: `src/custom.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add tests for recursive references and dependencies**

Add tests proving:

- a `VariableReference` exposes its `CustomPropertyName`;
- a fallback preserves authored CSS;
- nested fallback references are included in dependencies;
- a `VariableDependentValue` requires at least one reference;
- literal `Value` branches validate against the target property.

Use direct style values such as `Property::Color` with `Value::Color` and
`Property::Width` with `Value::Length`.

- [ ] **Step 2: Implement `VariableExpression`**

Implement:

```rust
pub enum VariableExpression {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
    Reference(VariableReference),
}
```

Add methods needed by resolver and tests:

- dependency collection over nested references and fallbacks;
- target-property validation for all literal `Value` branches;
- `contains_reference()` or equivalent.

- [ ] **Step 3: Implement `VariableReference` and `VariableFallback`**

`VariableReference` stores:

- `CustomPropertyName`;
- optional `VariableFallback`.

`VariableFallback` stores:

- `AuthoredTokens`;
- recursive `VariableExpression`.

Use private indirection such as `Box<VariableExpression>` for recursive
fallback storage so the type is representable in Rust.

All fields remain private.

- [ ] **Step 4: Implement `VariableDependentValue`**

`VariableDependentValue::try_new(property, authored, expression)` must:

- reject expressions with no variable reference using `ErrorCode::InvalidValue`;
- validate literal values against `property`;
- store the target property;
- expose immutable accessors for property, authored tokens, expression, and
  dependencies.

- [ ] **Step 5: Export the new types and update compile-pass examples**

Export `VariableExpression`, `VariableReference`, `VariableFallback`, and
`VariableDependentValue` from `src/lib.rs`.

Add compile-pass construction of:

- `var(--space, 8px)` for `Property::Gap` or `Property::Width`;
- a nested fallback such as `var(--space, var(--fallback, 4px))`.

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style variable
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 3: Add Custom Property Values And Authored Declaration Inputs

**Files:**

- Modify: `src/custom.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add custom property authored declaration tests**

Add tests proving:

- `AuthoredProperty::Custom(name)` accepts `AuthoredValue::CustomProperty`;
- `AuthoredProperty::Custom(name)` accepts CSS-wide keywords through
  `AuthoredDeclaration::css_wide`;
- `AuthoredProperty::All` does not expand to custom properties;
- ordinary `Value` cannot be attached to `AuthoredProperty::Custom`;
- `VariableDependentValue` can be attached only to its own target property;
- `AuthoredDeclaration::try_new` rejects mismatched variable-dependent target
  and property.

- [ ] **Step 2: Implement `CustomPropertyValue`**

`CustomPropertyValue` stores:

- authored custom property tokens;
- symbolic references for diagnostics/dependency inspection;
- zero or more `CustomPropertyTypedValue` entries keyed by `Property`.

Provide immutable accessors:

- `authored()`;
- `references()`;
- `typed_value(property) -> Option<&CustomPropertyTypedValue>`;
- dependency accessors.

Provide a checked builder path for typed interpretations, such as
`try_with_typed_value(property, expression) -> Result<Self>` or
`try_push_typed_value(CustomPropertyTypedValue) -> Result<&mut Self>`. The
builder must validate through `CustomPropertyTypedValue::try_new` and must not
expose a mutable public `Vec` of typed entries.

The authored value may be empty. Do not parse the authored token text.

- [ ] **Step 3: Implement `CustomPropertyTypedValue`**

`CustomPropertyTypedValue::try_new(property, expression)` must:

- validate literal `Value` branches against `property`;
- allow literal expressions with no references;
- allow expressions that reference other custom properties;
- expose `property()` and `expression()`.

If a custom property value has no typed entry for a target property, it remains
valid authored custom property data but is invalid for substituting into that
target ordinary property.

- [ ] **Step 4: Extend authored declarations**

Extend authored declarations with:

```rust
pub enum AuthoredProperty {
    Property(Property),
    Custom(CustomPropertyName),
    All,
}

pub enum AuthoredValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
    CustomProperty(CustomPropertyValue),
    VariableDependent(VariableDependentValue),
}
```

Validation rules:

- `Property(property)` accepts ordinary `Value` if `property.validate_value`
  passes;
- `Property(property)` accepts `VariableDependentValue` only when
  `variable_value.property() == property`;
- `Property(property)` accepts CSS-wide keywords through the explicit authored
  keyword path;
- `Custom(name)` accepts `CustomPropertyValue`;
- `Custom(name)` accepts CSS-wide keywords through the explicit authored
  keyword path;
- `Custom(name)` rejects ordinary `Value` and ordinary
  `VariableDependentValue`;
- `All` accepts only CSS-wide keywords and expands only over canonical ordinary
  properties, excluding `direction` and custom properties.

Keep authored structs private-fielded. Do not add compatibility aliases.

- [ ] **Step 5: Update internal canonical declaration storage**

Replace `AuthoredCanonicalDeclarations` internals with a representation that
can store ordinary property items and custom property items without forcing
custom properties into `Property`.

One acceptable shape:

```rust
pub(crate) enum AuthoredDeclarationItem {
    Property(Property, AuthoredCascadeValue),
    Custom(CustomPropertyName, CustomPropertyCascadeValue),
}
```

The exact private type names can differ if the review confirms the same
invariants.

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style authored
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 4: Resolve Custom Property Environment And Variable-Dependent Properties

**Files:**

- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `src/custom.rs`

- [ ] **Step 1: Add resolver tests first**

Add tests in `src/resolver.rs` or a focused test module proving:

- child styles inherit custom properties from parent `Resolved`;
- a matching authored custom property declaration overrides parent custom
  property value;
- `initial` clears a custom property so `var()` uses fallback;
- `unset` inherits a custom property from the parent;
- `revert-layer` on a custom property resolves to the lower layer custom
  property candidate;
- `revert-layer` with no lower layer resolves as `unset`;
- `color: var(--brand, fallback)` resolves through the custom property when a
  typed `Property::Color` interpretation exists;
- the same declaration uses fallback when the custom property is missing;
- missing custom property with no fallback resolves the target property as
  `unset`;
- invalid custom property typed value for the target property uses fallback or
  resolves as `unset`;
- a custom property cycle uses fallback or resolves the target property as
  `unset` and does not recurse indefinitely;
- local and animated legacy `Declarations` still override sheet rules after
  variable-dependent sheet resolution, preserving current override order.

- [ ] **Step 2: Extend rule declaration item plumbing**

Modify `src/sheet.rs` so `Rule::declaration_items()` returns a private enum
that can carry:

- legacy ordinary property declarations;
- authored ordinary property declarations;
- authored custom property declarations.

`Sheet::condition_change` must account for custom property declarations by
including descendant scope because custom properties inherit. It must also
include the target property impact for variable-dependent ordinary property
declarations.

- [ ] **Step 3: Build custom property candidate maps**

Modify `Resolver::resolve` to collect two candidate maps from matching rules:

- `BTreeMap<Property, Vec<RuleCandidate>>` for ordinary properties;
- `BTreeMap<CustomPropertyName, Vec<CustomPropertyCandidate>>` for custom
  properties.

Both maps sort by `RulePrecedence`. Custom property candidates need origin and
precedence for `revert-layer` behavior, mirroring ordinary property candidates.

- [ ] **Step 4: Add custom property environment to `Resolved`**

Extend `Resolved` with private custom property state:

```rust
custom_properties: BTreeMap<CustomPropertyName, CustomPropertyResolution>
```

Add public accessors:

- `custom_property(&self, name: &CustomPropertyName) -> Option<&CustomPropertyValue>`
  for valid computed custom property values;
- `custom_property_resolution(&self, name: &CustomPropertyName) ->
  Option<&CustomPropertyResolution>` if the worker needs to expose invalid
  state for tests or future diagnostics;
- `custom_property_dependencies(&self) -> &CustomPropertyDependencies` or an
  equivalent immutable dependency accessor.

Include custom property state and dependency state in `Resolved::fingerprint`
so resolver cache keys change when parent custom properties change.

- [ ] **Step 5: Resolve custom property candidates**

Before ordinary variable-dependent candidates are evaluated:

- inherit the parent's custom property environment;
- resolve matching custom property candidates by layer/source precedence;
- apply CSS-wide keyword semantics from this plan;
- detect custom property cycles with a per-name stack;
- store invalid resolutions for cyclic names where needed so repeated
  references are deterministic.

- [ ] **Step 6: Resolve variable-dependent ordinary property candidates**

Extend `RuleCandidateValue` with a `VariableDependent(VariableDependentValue)`
case.

When a variable-dependent candidate wins:

- evaluate its expression against the custom property environment and target
  property;
- use typed custom property interpretations for that target property;
- use fallback expressions for missing, invalid, cyclic, or untyped
  references;
- resolve CSS-wide keyword results through the existing keyword resolver, using
  the winning candidate context for `revert-layer`;
- resolve invalid-at-computed-value as `unset` for the target property.

Do not evaluate raw `AuthoredTokens` as CSS.

- [ ] **Step 7: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style resolver
cargo test -p surgeist-style sheet
```

Expected: all pass before review.

## Task 5: Add Custom Property Invalidation And Type Safety Tests

**Files:**

- Modify: `src/invalidation.rs`
- Modify: `src/custom.rs`
- Modify: `tests/type_safety.rs`
- Create: `tests/compile_fail/invalid_custom_property_struct_literal.rs`
- Create: `tests/compile_fail/invalid_custom_property_name_newtype_literal.rs`
- Create: `tests/compile_fail/invalid_variable_dependent_struct_literal.rs`
- Update generated `.stderr` files through `trybuild` only.

- [ ] **Step 1: Add dependency and invalidation tests**

Add tests proving:

- `CustomPropertyDependencies` records which ordinary properties depend on
  which custom property names;
- changing a custom property dependency can produce the ordinary property
  impact flags for dependent properties;
- inherited custom property changes include descendant scope;
- unrelated custom property changes do not claim unrelated ordinary property
  impacts.

Keep this as the minimal useful invalidation surface for Operation 4. Full
selector, condition, pseudo-element, and environment invalidation is Operation
14 and must not be guessed here.

- [ ] **Step 2: Implement `CustomPropertyDependencies`**

One acceptable shape:

```rust
pub struct CustomPropertyDependencies {
    by_property: BTreeMap<Property, BTreeSet<CustomPropertyName>>,
}
```

Expose immutable query methods:

- `for_property(property)`;
- `properties_for_custom_property(name)`;
- `is_empty()`.

Keep fields private.

- [ ] **Step 3: Extend invalidation helpers**

Add a helper such as:

```rust
impl Change {
    pub fn from_custom_properties(
        changed: impl IntoIterator<Item = CustomPropertyName>,
        dependencies: &CustomPropertyDependencies,
    ) -> Self
}
```

Required behavior:

- include node scope for the changed element;
- include descendant scope because custom properties inherit;
- include ordinary property invalidation flags for properties that depend on
  changed custom property names;
- do not mark rematch unless existing condition/query code requires it.

If the worker finds a better API that avoids cloning names unnecessarily, use
it with reviewer approval.

- [ ] **Step 4: Add compile-fail tests**

Add compile-fail tests that prove:

- `CustomPropertyName("--x".into())` is impossible;
- `CustomPropertyValue { ... }` struct literals are impossible;
- `VariableDependentValue { ... }` struct literals are impossible.

Update `.stderr` files only by running:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

- [ ] **Step 5: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style invalidation
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 6: Root Handoff Notes And Final Integration Checks

**Files:**

- Modify this plan if implementation discovers an API correction that reviewers
  accepted.
- Verify the full crate.

- [ ] **Step 1: Confirm no `surgeist-css` dependency was introduced**

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
```

Expected: no output. Planning files may mention `surgeist-css`; code,
dependency, and tests must not.

- [ ] **Step 2: Confirm variable work stayed in scope**

Run:

```sh
git diff -- src tests | rg -n "specificity|Pseudo|PseudoElement|Media|scope|Scope|FontFace|Keyframes|important|surgeist_css|surgeist-css"
```

Expected:

- no new `surgeist-css` references;
- no new selector specificity, pseudo-element bucket, media/scope, font-face,
  keyframe, or `!important` implementation added by this plan.

Existing strings in planning files and pre-existing `src`/`tests` code do not
matter for this check; this command is scoped to the implementation diff.

- [ ] **Step 3: Run required checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Expected: all commands pass. If `cargo clippy` reports warnings in code touched
by this plan, fix them before review. If it reports unrelated pre-existing
warnings, capture the exact warning and stop for coordinator direction.

- [ ] **Step 4: Final holistic review**

The coordinator assigns a final clean-context reviewer over the complete
result. The reviewer must check:

- no `surgeist-css` dependency;
- no CSS parsing in style;
- no broad untyped escape hatch;
- custom property and variable structs have private fields and typed
  constructors;
- custom property names preserve case and match the supported `--*`
  validation contract;
- authored token strings are preserved but not interpreted by resolver code;
- ordinary variable-dependent declarations resolve through typed expressions;
- missing, invalid, untyped, and cyclic references use fallback or
  invalid-at-computed-value `unset` behavior;
- custom properties inherit by default;
- CSS-wide keywords on custom properties follow this plan's semantics;
- `all` does not affect custom properties;
- cache fingerprints include parent custom property state;
- invalidation exposes custom property dependency impact without guessing
  Operation 14 features;
- required checks passed.

## Coordinator Commit Guidance

Commit after each task-scoped worker/reviewer cycle is clean. Suggested commit
messages:

```sh
git commit -m "style: add custom property core types"
git commit -m "style: add variable expression inputs"
git commit -m "style: add custom authored declarations"
git commit -m "style: resolve custom properties"
git commit -m "style: cover custom property invalidation"
```

Workers do not commit. The coordinator owns commits and final reporting.

## Root Handoff Notes

After implementation, root must add a lowering task that maps
`surgeist-css` custom property and variable-dependent syntax into the new
style-owned APIs.

Root must explicitly reject unsupported variable-dependent ordinary values
until it can lower every possible branch into a typed `VariableExpression` for
the target style property. Rejection is preferable to sending raw CSS token
strings to style and making computed output look supported when it is not.

Root should treat style errors from custom property names and typed branch
validation as unsupported-integration or lowering bugs, not as CSS parse
errors. CSS syntax validation remains in `surgeist-css`.

## Next Sequence Context

The next implementation plan should cover Operation 5: selector and tree
matching expansion. It should consume the authored layer/source model and the
custom property environment from this plan without adding another declaration
receiving path.

The next plan should specifically pick up:

- selector lists and complex selector combinators;
- selector specificity or precomputed specificity receipt, if root confirms it
  is needed at that point;
- attribute matcher variants and case sensitivity;
- structural selectors and nth-child variants;
- selector-list pseudo-classes such as `:not`, `:is`, and `:where`;
- relative selectors and `:has`;
- runtime pseudo-class matching over style-owned tree/state facts;
- `:root` and `:scope`;
- keeping selector facts style-owned and independent from retained or DOM
  concepts.

Do not begin pseudo-element buckets, broad property family expansion, media
and container query expansion, or Operation 14's full cache/invalidation
generalization until the selector/tree matching plan is written and reviewed.
