# Authored Layer Keywords Implementation Plan

> **For agentic workers:** Execute this plan through the local `AGENTS.md`
> coordinator workflow. Workers follow the checkbox steps, do not commit, do
> not create branches, and do not let external workflow guidance override the
> crate's worker/reviewer gate.

**Goal:** Add style-owned authored declaration inputs, layer/source-order rule
precedence, and the CSS-wide keywords Surgeist supports without adopting full
browser cascade mechanics.

**Architecture:** Style receives root-lowered authored declarations and a
simple precedence key: higher `LayerOrder` wins, and higher `SourceOrder` wins
only within the same layer. Root owns CSS parsing, `@layer` name/block
flattening, `!important` rejection, `revert` rejection, and lowering parsed CSS
into these style-owned types.

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
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `src/lib.rs`
- `src/declaration.rs`
- `src/sheet.rs`
- `src/resolver.rs`
- `src/property.rs`
- `src/value.rs`
- `tests/type_safety.rs`
- `tests/compile_pass/typed_public_construction.rs`

## Scope

This plan implements the Surgeist-native subset of the ledger rows whose later
plan is `Authored declarations, cascade metadata, and CSS-wide keywords`:

- `CssRule::Style` as a root-owned lowering boundary into style-owned authored
  rules.
- `CssRule::LayerStatement` and `CssRule::LayerBlock` as root-flattened
  `LayerOrder` inputs.
- `CssValue::GlobalKeyword` for supported keywords: `inherit`, `initial`,
  `unset`, and `revert-layer`.
- `CssProperty::All` as an authored-only property that expands over canonical
  style properties except `direction` and future custom properties.
- Existing and new declaration surfaces needed to carry ordinary typed values
  and supported CSS-wide keywords through layer/source precedence resolution.

This plan deliberately does not implement:

- CSS `!important`; root must reject it for this pass;
- CSS cascade origins or browser origin precedence;
- selector specificity as a precedence dimension;
- CSS `revert`, because it requires origins; root must reject it for this pass;
- custom properties or `var(...)` substitution;
- selector-list, pseudo-element, `:has`, or advanced selector matching;
- media/container/scope condition semantics beyond preserving existing
  `Condition` behavior;
- new layout/text/paint property families;
- root lowering from `surgeist-css` types;
- a `surgeist-css` dependency.

Style does not receive CSS layer names or parse layer statements. Root computes
the concrete `LayerOrder` for every lowered style rule, including unlayered
rules. Style compares only the numeric layer/source key it is given.

## Expected File Structure

- Create: `src/precedence.rs`
  - Owns `LayerOrder`, `SourceOrder`, and `RulePrecedence`.
- Create: `src/authored.rs`
  - Owns style-authored receiving types: `AuthoredProperty`,
    `CssWideKeyword`, `AuthoredValue`, `AuthoredDeclaration`, and
    `AuthoredDeclarations`.
- Modify: `src/lib.rs`
  - Exports the new public front-door types.
- Modify: `src/declaration.rs`
  - Shares canonical property expansion with authored declarations.
- Modify: `src/sheet.rs`
  - Stores rule precedence and adds authored rule insertion APIs.
- Modify: `src/resolver.rs`
  - Resolves sheet rules by layer/source precedence and resolves supported
    CSS-wide keywords.
- Modify: `tests/compile_pass/typed_public_construction.rs`
  - Proves public construction of the new typed APIs.
- Create: `tests/compile_fail/invalid_authored_struct_literal.rs`
  - Proves authored structs have private fields.
- Create: `tests/compile_fail/invalid_precedence_struct_literal.rs`
  - Proves precedence newtypes have private fields.
- Create: `tests/compile_fail/invalid_revert_layer_value.rs`
  - Proves `revert-layer` is not constructible as a legacy `Value::Keyword`.
- Create expected `trybuild` stderr files only by running the documented
  `TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety` command
  after the compile-fail tests are written.

## Public API Shape

Workers may refine names only if the reviewer agrees the refinement makes the
model more type-safe without widening scope. The expected public API is:

```rust
pub struct LayerOrder(u32);
pub struct SourceOrder(u32);

pub struct RulePrecedence { /* private fields */ }

pub enum AuthoredProperty {
    Property(Property),
    All,
}

pub enum CssWideKeyword {
    Inherit,
    Initial,
    Unset,
    RevertLayer,
}

pub enum AuthoredValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
}

pub struct AuthoredDeclaration { /* private fields */ }
pub struct AuthoredDeclarations { /* private fields */ }
```

The new public API must make invalid construction hard:

- `LayerOrder::new` and `SourceOrder::new` are infallible semantic constructors
  around `u32`.
- `RulePrecedence::new(layer_order, source_order)` requires both dimensions.
- `RulePrecedence::default()` is `LayerOrder::default()` and
  `SourceOrder::default()`.
- `RulePrecedence` ordering is exactly `(layer_order, source_order)`, with
  higher values winning.
- `AuthoredDeclaration::try_new` validates ordinary `Value` against a concrete
  `Property`.
- `AuthoredDeclaration::css_wide` accepts `AuthoredProperty` and
  `CssWideKeyword`; `AuthoredProperty::All` is valid only for CSS-wide keywords.
- `AuthoredDeclaration::try_new(property, AuthoredValue::CssWideKeyword(_))`
  returns `ErrorCode::InvalidProperty`; CSS-wide keywords enter through
  `AuthoredDeclaration::css_wide` so `all` and authored-only keywords cannot
  bypass the explicit authored keyword path.
- `AuthoredDeclarations` represents a homogeneous layer/source bucket. The
  precedence is attached when the declarations are inserted into a sheet, not
  to each declaration.
- Authored declarations expose iterators and accessors, not mutable public
  fields.
- `CssWideKeyword::RevertLayer` is authored-sheet-only and must not be added to
  the public `Keyword` or `Value` model. It remains a symbolic authored rule
  value until resolver candidate construction.
- `Rule` must carry private authored rule declarations separately from the
  public legacy `Declarations` model. `revert-layer` is valid only when the
  declaration came from `Sheet::push_authored_rule`.

There is no backwards-compatibility requirement for this plan. Breaking changes
are acceptable when they make the model more type-safe or prevent authored-only
keywords from leaking into legacy APIs. Do not keep compatibility aliases,
unchecked constructors, or legacy accessors that weaken the authored boundary.

Root must split lowered CSS into multiple `push_authored_rule` calls whenever a
single parsed CSS block cannot be represented as one homogeneous style rule. In
this pass, that mostly means root rejects `!important` and `revert`; later
passes may split for custom-property dependency handling.

## Precedence Rules

Style applies matching rule declarations in increasing `RulePrecedence` order.
The later application wins per canonical property.

Concrete order:

1. lower `LayerOrder` applies first;
2. higher `LayerOrder` applies later and wins over every lower layer regardless
   of global source order;
3. within the same `LayerOrder`, lower `SourceOrder` applies first;
4. within the same `LayerOrder`, higher `SourceOrder` applies later and wins.

This is the only cascade-like ordering implemented by this plan. There is no
origin, no `!important`, and no specificity ordering.

## Supported CSS-Wide Keyword Semantics

Supported keywords:

- `initial`: property metadata default;
- `inherit`: parent value if present, otherwise property metadata default;
- `unset`: `inherit` for inherited properties, otherwise `initial`;
- `revert-layer`: ignore declarations for the current property from the winning
  candidate's layer and all higher layers, then resolve the highest-priority
  candidate from a lower layer. If no lower-layer candidate exists, resolve as
  `unset`.

Unsupported keyword:

- `revert`: root must reject it for this pass because Surgeist does not model
  cascade origins.

## Task 1: Add Layer/Source Precedence Types

**Files:**

- Create: `src/precedence.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write precedence ordering tests**

Create `src/precedence.rs` with tests first:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_layer_outranks_later_source_order() {
        let lower_layer_late_source =
            RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(100));
        let higher_layer_early_source =
            RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(0));

        assert!(higher_layer_early_source > lower_layer_late_source);
    }

    #[test]
    fn source_order_breaks_ties_inside_same_layer() {
        let early = RulePrecedence::new(LayerOrder::new(7), SourceOrder::new(1));
        let late = RulePrecedence::new(LayerOrder::new(7), SourceOrder::new(2));

        assert!(late > early);
    }

    #[test]
    fn default_precedence_is_zero_layer_zero_source() {
        let precedence = RulePrecedence::default();

        assert_eq!(precedence.layer_order(), LayerOrder::new(0));
        assert_eq!(precedence.source_order(), SourceOrder::new(0));
    }
}
```

- [ ] **Step 2: Register the module and run the focused failing test**

Before running the test, register the module privately in `src/lib.rs`:

```rust
mod precedence;
```

Run:

```sh
cargo test -p surgeist-style precedence::tests -- --nocapture
```

Expected before implementation: compile failure because the precedence types do
not exist yet or have no implementation.

- [ ] **Step 3: Implement precedence types**

Create `src/precedence.rs` with private-field newtypes:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LayerOrder(u32);

impl LayerOrder {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceOrder(u32);

impl SourceOrder {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}
```

Implement `RulePrecedence`:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RulePrecedence {
    layer_order: LayerOrder,
    source_order: SourceOrder,
}

impl RulePrecedence {
    #[must_use]
    pub const fn new(layer_order: LayerOrder, source_order: SourceOrder) -> Self {
        Self {
            layer_order,
            source_order,
        }
    }

    #[must_use]
    pub const fn layer_order(self) -> LayerOrder {
        self.layer_order
    }

    #[must_use]
    pub const fn source_order(self) -> SourceOrder {
        self.source_order
    }

    #[must_use]
    pub const fn with_source_order(self, source_order: SourceOrder) -> Self {
        Self { source_order, ..self }
    }
}
```

- [ ] **Step 4: Export precedence types**

Modify `src/lib.rs`:

```rust
pub use precedence::{LayerOrder, RulePrecedence, SourceOrder};
```

- [ ] **Step 5: Verify Task 1**

Run:

```sh
cargo test -p surgeist-style precedence::tests -- --nocapture
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: precedence tests pass, formatting passes, and only Task 1 files are
modified.

## Task 2: Add Authored Declaration Receiving Types

**Files:**

- Create: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `src/declaration.rs`

- [ ] **Step 1: Write authored declaration tests**

Create `src/authored.rs` with tests first:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Display, ErrorCode, Keyword, Length, Property, Value};

    #[test]
    fn ordinary_authored_declaration_validates_against_property() {
        let declaration = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Display),
            AuthoredValue::Value(Value::Display(Display::Block)),
        )
        .unwrap();

        assert_eq!(declaration.property(), AuthoredProperty::Property(Property::Display));
    }

    #[test]
    fn ordinary_authored_declaration_rejects_property_value_mismatch() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Width),
            AuthoredValue::Value(Value::Color(Color::BLACK)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn all_rejects_ordinary_values() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::All,
            AuthoredValue::Value(Value::Color(Color::BLACK)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn ordinary_value_path_rejects_existing_keyword_values() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Color),
            AuthoredValue::Value(Value::Keyword(Keyword::Initial)),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn css_wide_keywords_must_use_explicit_constructor() {
        let error = AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Color),
            AuthoredValue::CssWideKeyword(CssWideKeyword::Initial),
        )
        .unwrap_err();

        assert_eq!(error.code(), ErrorCode::InvalidProperty);
    }

    #[test]
    fn all_accepts_css_wide_keywords() {
        let declaration =
            AuthoredDeclaration::css_wide(AuthoredProperty::All, CssWideKeyword::Initial);

        assert_eq!(declaration.property(), AuthoredProperty::All);
        assert_eq!(
            declaration.value(),
            AuthoredValue::CssWideKeyword(CssWideKeyword::Initial)
        );
    }

    #[test]
    fn all_expands_css_wide_keyword_to_canonical_properties_except_direction() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::All,
            CssWideKeyword::Unset,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.len(),
            Property::ALL
                .iter()
                .filter(|property| property.is_canonical() && **property != Property::Direction)
                .count()
        );
        assert_eq!(
            canonical.get(Property::Color),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
        );
        assert_eq!(canonical.get(Property::Direction), None);
    }

    #[test]
    fn shorthand_css_wide_keyword_expands_to_longhands() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(Property::Overflow),
            CssWideKeyword::Inherit,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.get(Property::OverflowX),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Inherit))
        );
        assert_eq!(
            canonical.get(Property::OverflowY),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Inherit))
        );
    }

    #[test]
    fn revert_layer_expands_without_entering_legacy_value_model() {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(Property::Color),
            CssWideKeyword::RevertLayer,
        ));

        let canonical = declarations.to_rule_declarations().unwrap();

        assert_eq!(
            canonical.get(Property::Color),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::RevertLayer))
        );
    }

    #[test]
    fn ordinary_values_still_expand_existing_shorthands() {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Width),
                    AuthoredValue::Value(Value::Length(Length::Px(12.0))),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            declarations.to_rule_declarations().unwrap().get(Property::Width),
            Some(&AuthoredCascadeValue::Value(Value::Length(Length::Px(12.0))))
        );
    }
}
```

- [ ] **Step 2: Register the module and run the focused failing test**

Before running the test, register the module privately in `src/lib.rs`:

```rust
mod authored;
```

Run:

```sh
cargo test -p surgeist-style authored::tests -- --nocapture
```

Expected before implementation: compile failure because the authored types do
not exist yet or have no implementation.

- [ ] **Step 3: Keep `revert-layer` out of the legacy value model**

Do not modify `src/value.rs` to add `Keyword::RevertLayer`. The existing
`Keyword` enum remains:

```rust
pub enum Keyword {
    Inherit,
    Initial,
    Unset,
}
```

`revert-layer` requires layer context, so this plan represents it only as
`CssWideKeyword::RevertLayer` inside authored rule declarations and resolver
sheet candidates. Do not add `Revert` in this pass. Root must reject CSS
`revert`. Do not add string parsing. Root owns CSS parsing and lowers into these
variants.

- [ ] **Step 4: Share canonical property expansion**

Modify `src/declaration.rs` so authored declarations can reuse canonical
shorthand expansion without duplicating the expansion matrix. The least
invasive route is to factor canonical property expansion and keep both helpers
private to the crate:

```rust
pub(crate) fn canonical_properties(property: Property) -> Vec<Property> {
    match property {
        // Move the current shorthand-to-longhand match arms here unchanged.
    }
}

pub(crate) fn canonical_declarations(property: Property, value: Value) -> Vec<Declaration> {
    canonical_properties(property)
        .into_iter()
        .map(|property| Declaration::new(property, value.clone()))
        .collect()
}
```

Keep public `Declarations` behavior unchanged.

- [ ] **Step 5: Implement authored types**

Implement `src/authored.rs` using private fields. Required behavior:

- `AuthoredProperty::Property(property)` wraps existing style properties.
- `AuthoredProperty::All` is valid only with `AuthoredValue::CssWideKeyword`.
- Existing `Value::Keyword(Keyword::Initial)`,
  `Value::Keyword(Keyword::Inherit)`, and
  `Value::Keyword(Keyword::Unset)` remain available for direct Rust-authored
  `Declarations`. Authored CSS-wide keywords remain symbolic in
  `AuthoredCascadeValue` through sheet candidate resolution.
- `AuthoredDeclaration::try_new` validates ordinary values with
  `Property::validate_value`.
- `AuthoredDeclaration::try_new` rejects
  `AuthoredValue::Value(Value::Keyword(_))`; supported CSS-wide keywords must
  enter authored declarations only through `AuthoredDeclaration::css_wide`.
- `AuthoredDeclaration::try_new` rejects
  `AuthoredValue::CssWideKeyword(_)`; this keeps `all` and authored-only keyword
  handling on one explicit constructor.
- Existing `Declarations` APIs may continue to accept
  `Value::Keyword(Keyword::Initial)`, `Value::Keyword(Keyword::Inherit)`, and
  `Value::Keyword(Keyword::Unset)` for direct Rust-authored declarations.
- No public `Value::Keyword(Keyword::RevertLayer)` exists.
- `AuthoredDeclaration::css_wide` is infallible for `Property` and `All`.
- `AuthoredDeclarations::to_rule_declarations` is crate-private and returns a
  crate-private authored canonical declaration container. The expected internal
  value enum is equivalent to:

  ```rust
  pub(crate) enum AuthoredCascadeValue {
      Value(Value),
      CssWideKeyword(CssWideKeyword),
  }
  ```

- `AuthoredDeclarations::to_rule_declarations` expands:
  - ordinary property/value declarations through `canonical_declarations` and
    stores `AuthoredCascadeValue::Value`;
  - property/css-wide keyword declarations through `canonical_properties` and
    stores `AuthoredCascadeValue::CssWideKeyword`;
  - `all`/css-wide keyword declarations to every canonical `Property::ALL`
    except `Property::Direction`.
- Do not expose a public helper that converts authored declarations containing
  `CssWideKeyword::RevertLayer` into legacy `Declarations`.
- Future custom properties are also excluded from `all` unless a later custom
  property plan explicitly models CSS custom-property behavior.
- In this plan, custom properties and variable-dependent declarations are not
  represented; they are explicitly next-plan work.

- [ ] **Step 6: Export authored types**

Modify `src/lib.rs`:

```rust
pub use authored::{
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, CssWideKeyword,
};
```

- [ ] **Step 7: Verify Task 2**

Run:

```sh
cargo test -p surgeist-style authored::tests -- --nocapture
cargo test -p surgeist-style declaration::tests -- --nocapture
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: authored and declaration tests pass. Existing public `Declarations`
APIs still compile and behave as before.

## Task 3: Add Rule Precedence And Authored Rule Insertion

**Files:**

- Modify: `src/sheet.rs`
- Modify: `src/authored.rs`

- [ ] **Step 1: Write sheet precedence tests**

Add tests to `src/sheet.rs`:

```rust
#[cfg(test)]
mod precedence_tests {
    use super::*;
    use crate::{
        AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, Color,
        LayerOrder, Property, RulePrecedence, Selector, SourceOrder, Value,
    };

    fn authored_color(color: Color) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Color),
                    AuthoredValue::Value(Value::Color(color)),
                )
                .unwrap(),
            )
            .unwrap();
        declarations
    }

    #[test]
    fn existing_rule_api_uses_default_layer_and_insertion_source_order() {
        let mut sheet = Sheet::new();
        sheet.push_rule(
            Selector::tag("button").unwrap(),
            Declarations::new().try_text_color(Color::BLACK).unwrap(),
        );

        let rule = &sheet.rules()[0];
        assert_eq!(
            rule.precedence(),
            RulePrecedence::new(LayerOrder::default(), SourceOrder::new(0))
        );
        assert_eq!(rule.order(), 0);
    }

    #[test]
    fn sheet_rule_builder_preserves_existing_source_order_behavior() {
        let sheet = Sheet::new()
            .rule(
                Selector::tag("button").unwrap(),
                Declarations::new().try_text_color(Color::BLACK).unwrap(),
            )
            .rule(
                Selector::class("primary").unwrap(),
                Declarations::new().try_text_color(Color::TRANSPARENT).unwrap(),
            );

        assert_eq!(sheet.rules()[0].order(), 0);
        assert_eq!(sheet.rules()[1].order(), 1);
        assert_eq!(
            sheet.rules()[1].precedence(),
            RulePrecedence::new(LayerOrder::default(), SourceOrder::new(1))
        );
    }

    #[test]
    fn authored_rule_preserves_supplied_precedence() {
        let precedence = RulePrecedence::new(LayerOrder::new(7), SourceOrder::new(3));
        let mut sheet = Sheet::new();
        sheet
            .push_authored_rule(
                Selector::tag("button").unwrap(),
                authored_color(Color::BLACK),
                precedence,
            )
            .unwrap();

        assert_eq!(sheet.rules()[0].precedence(), precedence);
    }

    #[test]
    fn authored_rules_do_not_expose_legacy_declarations() {
        let mut authored = AuthoredDeclarations::new();
        authored.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(Property::Color),
            CssWideKeyword::RevertLayer,
        ));

        let mut sheet = Sheet::new();
        sheet
            .push_authored_rule(
                Selector::tag("button").unwrap(),
                authored,
                RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(0)),
            )
            .unwrap();

        assert_eq!(sheet.rules()[0].legacy_declarations(), None);
    }

    #[test]
    fn extend_rebases_legacy_rules_and_preserves_authored_precedence() {
        let authored_precedence = RulePrecedence::new(LayerOrder::new(9), SourceOrder::new(20));
        let legacy_rule = Rule::new(
            Selector::tag("button").unwrap(),
            Declarations::new().try_text_color(Color::BLACK).unwrap(),
        );
        let mut authored_sheet = Sheet::new();
        authored_sheet
            .push_authored_rule(
                Selector::class("primary").unwrap(),
                authored_color(Color::TRANSPARENT),
                authored_precedence,
            )
            .unwrap();
        let authored_rule = authored_sheet.rules()[0].clone();

        let mut sheet = Sheet::new();
        sheet.push_rule(
            Selector::key("root").unwrap(),
            Declarations::new().try_text_color(Color::BLACK).unwrap(),
        );
        sheet.extend([legacy_rule, authored_rule]);

        assert_eq!(sheet.rules()[1].order(), 1);
        assert_eq!(sheet.rules()[1].precedence().layer_order(), LayerOrder::default());
        assert_eq!(sheet.rules()[2].precedence(), authored_precedence);
    }
}
```

- [ ] **Step 2: Run the focused failing test**

Run:

```sh
cargo test -p surgeist-style sheet::precedence_tests -- --nocapture
```

Expected before implementation: compile failure because `precedence` and
`push_authored_rule` do not exist.

- [ ] **Step 3: Extend `Rule`**

Modify `Rule` to carry `precedence: RulePrecedence`.

Requirements:

- `Rule::new` and `Sheet::push_rule` continue to work.
- Existing rule APIs use `RulePrecedence::default()` with source order replaced
  by insertion order.
- `Rule::precedence(&self) -> RulePrecedence` is public.
- Add a private source-order policy to `Rule`, such as
  `RuleSourceOrderPolicy::RebaseOnExtend` and
  `RuleSourceOrderPolicy::PreserveExplicit`.
- Add private declaration-origin metadata to `Rule`, such as
  `RuleDeclarationOrigin::Legacy` and `RuleDeclarationOrigin::Authored`. This
  origin is separate from source-order rebasing and is used by the resolver to
  choose between legacy `Declarations` and private authored declarations.
- Add private rule declaration storage equivalent to:

  ```rust
  enum RuleDeclarations {
      Legacy(Declarations),
      Authored(AuthoredCanonicalDeclarations),
  }
  ```

  `AuthoredCanonicalDeclarations` and `AuthoredCascadeValue` are crate-private
  types from `src/authored.rs`.
- `Rule::new`, `Rule::with_order`, `Sheet::push_rule`, and
  `Sheet::push_conditional_rule` create default-layer rules with
  `RebaseOnExtend` and `RuleDeclarationOrigin::Legacy`.
- `push_authored_rule` creates rules with `PreserveExplicit` and
  `RuleDeclarationOrigin::Authored`.
- `Sheet::extend` rebases only `RebaseOnExtend` rules by replacing
  `SourceOrder` with appended insertion order; it preserves explicit authored
  precedence exactly.
- Keep `Rule::order()` for now as an accessor over
  `self.precedence().source_order().get()` so task-local tests can prove default
  layer/source behavior.
- Replace `Rule::declarations()` with
  `Rule::legacy_declarations(&self) -> Option<&Declarations>`. It must return
  `Some` only for `RuleDeclarations::Legacy` and `None` for authored rules. Do
  not expose authored rules as public `Declarations`.
- Resolver, invalidation, and indexing code must inspect private
  `RuleDeclarations` so authored `revert-layer` remains visible to sheet
  resolution without becoming cloneable through public APIs.

- [ ] **Step 4: Add authored rule insertion**

Add:

```rust
pub fn push_authored_rule(
    &mut self,
    selector: Selector,
    declarations: AuthoredDeclarations,
    precedence: RulePrecedence,
) -> Result<&mut Self>
```

This method converts `AuthoredDeclarations` to crate-private
`AuthoredCanonicalDeclarations`, marks the rule origin as authored, and stores
the supplied precedence exactly. Root is expected to supply both layer order and
source order when lowering CSS. Existing `push_rule` remains the convenience
path that assigns source order from insertion order in the default layer and
marks declarations as legacy.

Do not implement this by exposing a public unchecked `Declarations` constructor.
The only public way to insert authored `revert-layer` into a sheet is
`Sheet::push_authored_rule` receiving `AuthoredDeclaration::css_wide(...,
CssWideKeyword::RevertLayer)`.

- [ ] **Step 5: Verify Task 3**

Run:

```sh
cargo test -p surgeist-style sheet::precedence_tests -- --nocapture
cargo test -p surgeist-style selector::tests -- --nocapture
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: sheet tests pass and existing selector/sheet candidate behavior is
unchanged.

## Task 4: Resolve Rules By Layer/Source Precedence And Supported Keywords

**Files:**

- Modify: `src/resolver.rs`
- Modify: `src/sheet.rs`

- [ ] **Step 1: Write resolver precedence tests**

Add tests to `src/resolver.rs`. Use the existing test tree helpers if present;
otherwise add a small private test tree in the resolver test module using
`crate::Node` and `crate::Tree`.

Add private resolver test helpers equivalent to:

```rust
fn precedence(layer: u32, source: u32) -> RulePrecedence {
    RulePrecedence::new(LayerOrder::new(layer), SourceOrder::new(source))
}

fn authored_color(value: Color) -> AuthoredDeclarations {
    let mut declarations = AuthoredDeclarations::new();
    declarations
        .try_push(
            AuthoredDeclaration::try_new(
                AuthoredProperty::Property(Property::Color),
                AuthoredValue::Value(Value::Color(value)),
            )
            .unwrap(),
        )
        .unwrap();
    declarations
}

fn authored_width(value: Length) -> AuthoredDeclarations {
    let mut declarations = AuthoredDeclarations::new();
    declarations
        .try_push(
            AuthoredDeclaration::try_new(
                AuthoredProperty::Property(Property::Width),
                AuthoredValue::Value(Value::Length(value)),
            )
            .unwrap(),
        )
        .unwrap();
    declarations
}

fn authored_keyword(property: Property, keyword: CssWideKeyword) -> AuthoredDeclarations {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(property),
        keyword,
    ));
    declarations
}
```

Required concrete resolver cases:

- `higher_layer_wins_over_later_source_order`: lower layer color red with
  source order 100 and higher layer color black with source order 0 both match;
  resolved color is black.
- `source_order_wins_within_same_layer`: same layer color red at source 0 and
  color black at source 1 both match; resolved color is black.
- `inherit_uses_parent_value`: parent color is black, child authored color is
  `CssWideKeyword::Inherit`, resolved child `text_color()` is black.
- `initial_uses_property_default`: authored color is `CssWideKeyword::Initial`,
  parent color is non-default, resolved child `text_color()` is `Color::BLACK`.
- `unset_inherits_inherited_properties_and_initializes_non_inherited_properties`:
  authored color and width are both `CssWideKeyword::Unset`; with parent color
  red and parent width `Length::Px(88.0)`, child color inherits red and child
  width resolves to the default `Length::Auto`.
- `revert_layer_uses_lower_layer_candidate`: layer 1 color black and layer 2
  color `CssWideKeyword::RevertLayer` both match; resolved color is black.
- `revert_layer_ignores_same_layer_earlier_source_order`: layer 2 color red at
  source 0, layer 2 color `CssWideKeyword::RevertLayer` at source 1, and layer
  1 color black all match; resolved color is black.
- `revert_layer_resolves_as_unset_without_lower_layer`: layer 2 color
  `CssWideKeyword::RevertLayer` with parent red resolves as inherited red
  because color is inherited; layer 2 width `CssWideKeyword::RevertLayer`
  resolves as `Length::Auto`.
- `local_declarations_still_override_sheet_rules`: local width or color
  overrides matching sheet rules after layer/source precedence is applied.

Use concrete assertions such as `resolved.text_color()` and `resolved.width()`.
Do not assert only that resolution succeeds.

- [ ] **Step 2: Run focused failing tests**

Run:

```sh
cargo test -p surgeist-style resolver::tests -- --nocapture
```

Expected before implementation: at least layer ordering and `revert-layer`
tests fail.

- [ ] **Step 3: Collect per-property sheet candidates by precedence**

Modify `Resolver::resolve` so matching rule declarations are collected with
their `RulePrecedence` and private rule declaration origin into
per-canonical-property candidate lists. Sort each candidate list from lower to
higher precedence, then resolve each property from the highest-precedence
candidate downward. Preserve existing behavior for:

- condition filtering;
- selector matching;
- parent inheritance before rule application;
- local declarations after sheet rules;
- animated declarations after local declarations;
- cache key inputs.

The existing flat source-order behavior must remain the degenerate case when
all rules are in the default layer with increasing source order.

Implementation requirement: do not resolve `revert-layer` by mutating a
`Resolved` map in a single lower-to-higher pass. Each sheet candidate must carry
the canonical `Property`, `RulePrecedence`, private `RuleDeclarationOrigin`, and
a private candidate value equivalent to:

```rust
enum RuleCandidateValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
}
```

Legacy rules produce `RuleCandidateValue::Value`. Authored rules produce
`RuleCandidateValue::Value` for ordinary typed values and
`RuleCandidateValue::CssWideKeyword` for authored CSS-wide keywords.
`revert-layer` must inspect the candidate list for the same property and choose
from lower layers only. Use a small visited guard keyed by candidate index or
precedence/value position to prevent accidental recursion if future symbolic
values are added.

- [ ] **Step 4: Resolve supported CSS-wide keywords**

Replace `resolve_keyword` with a helper that can see lower-layer cascaded values
for the same property and can resolve both legacy `Value::Keyword` values and
authored `RuleCandidateValue::CssWideKeyword` values.

Required semantics:

- `initial`: property metadata default.
- `inherit`: parent value if present, otherwise property metadata default.
- `unset`: `inherit` for inherited properties, otherwise `initial`.
- `revert-layer`: ignore declarations from the winning candidate's
  `LayerOrder` and all higher `LayerOrder` values for that property. Resolve
  from the highest-precedence candidate with a lower `LayerOrder`. If no lower
  layer candidate exists, resolve as `unset`.

Document the helper with a short comment naming this as style-owned CSS-wide
keyword resolution over root-supplied layer/source precedence.

- [ ] **Step 5: Preserve local and animated declaration semantics**

Keep `Context::local` and `Context::animated` behavior after sheet rules. For
this plan, local and animated declarations may continue to use existing
`Declarations`; they are not root-lowered CSS layer inputs. Add a regression
test showing `Context::local` still overrides matching sheet rules.

There is no local or animated `revert-layer` runtime rejection path because
`Keyword::RevertLayer` does not exist. Task 5 adds compile-fail coverage proving
callers cannot construct `Value::Keyword(Keyword::RevertLayer)` for local,
animated, or legacy sheet APIs.

- [ ] **Step 6: Verify Task 4**

Run:

```sh
cargo test -p surgeist-style resolver::tests -- --nocapture
cargo test -p surgeist-style sheet::precedence_tests -- --nocapture
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: resolver tests pass, and existing flat source-order behavior remains
covered.

## Task 5: Type Safety And Public API Tests

**Files:**

- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_authored_struct_literal.rs`
- Create: `tests/compile_fail/invalid_precedence_struct_literal.rs`
- Create: `tests/compile_fail/invalid_revert_layer_value.rs`
- Create or update generated stderr files under `tests/compile_fail/`

- [ ] **Step 1: Add public construction pass coverage**

Extend `tests/compile_pass/typed_public_construction.rs` with construction of:

```rust
use surgeist_style::{
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, CssWideKeyword,
    Color, LayerOrder, Property, RulePrecedence, SourceOrder, Value,
};

let precedence = RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(8));
assert_eq!(precedence.layer_order(), LayerOrder::new(2));

let mut authored = AuthoredDeclarations::new();
authored.push(AuthoredDeclaration::css_wide(
    AuthoredProperty::All,
    CssWideKeyword::Initial,
));
authored.try_push(AuthoredDeclaration::try_new(
    AuthoredProperty::Property(Property::Color),
    AuthoredValue::Value(Value::Color(Color::BLACK)),
)?)?;
assert!(authored.len() >= 2);
```

- [ ] **Step 2: Add authored compile-fail coverage**

Create `tests/compile_fail/invalid_authored_struct_literal.rs`:

```rust
use surgeist_style::{
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, Color, Property,
    Value,
};

fn main() {
    let _declaration = AuthoredDeclaration {
        property: AuthoredProperty::Property(Property::Color),
        value: AuthoredValue::Value(Value::Color(Color::BLACK)),
    };
    let _declarations = AuthoredDeclarations { values: Vec::new() };
}
```

- [ ] **Step 3: Add precedence compile-fail coverage**

Create `tests/compile_fail/invalid_precedence_struct_literal.rs`:

```rust
use surgeist_style::{LayerOrder, RulePrecedence, SourceOrder};

fn main() {
    let _layer = LayerOrder(2);
    let _source = SourceOrder(3);
    let _precedence = RulePrecedence {
        layer_order: LayerOrder::new(1),
        source_order: SourceOrder::new(2),
    };
}
```

- [ ] **Step 4: Add revert-layer legacy value compile-fail coverage**

Create `tests/compile_fail/invalid_revert_layer_value.rs`:

```rust
use surgeist_style::{Keyword, Value};

fn main() {
    let _value = Value::Keyword(Keyword::RevertLayer);
}
```

This compile-fail test is intentional. `revert-layer` is represented only as
`CssWideKeyword::RevertLayer` in authored declarations and must not enter
legacy `Declarations`, `Context::local`, `Context::animated`, `Rule::new`, or
`Sheet::push_rule` as a public `Value`.

- [ ] **Step 5: Generate trybuild stderr intentionally**

Run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

Expected: compile-fail stderr files are created or updated only for the three new
compile-fail tests and any line-number drift caused by public API export
changes. Review the generated stderr before keeping it.

- [ ] **Step 6: Verify type safety**

Run:

```sh
cargo test -p surgeist-style --test type_safety
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: trybuild passes and generated stderr is committed only when it
matches the intended private-field errors.

## Task 6: Final Integration Checks

**Files:**

- Verify the full crate.

- [ ] **Step 1: Confirm no `surgeist-css` dependency was introduced**

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
```

Expected: no output. Planning files may mention `surgeist-css`; code,
dependency, and tests must not.

- [ ] **Step 2: Run required checks**

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

- [ ] **Step 3: Final holistic review**

The coordinator assigns a final clean-context reviewer over the complete result.
The reviewer must check:

- no `surgeist-css` dependency;
- no broad untyped escape hatch;
- authored phase types have private fields and typed constructors;
- direct Rust-authored declaration and sheet APIs either still work or are
  intentionally changed without compatibility aliases or unchecked escape
  hatches;
- rule precedence is exactly layer order before source order;
- CSS `!important`, origins, specificity precedence, and `revert` are not
  implemented in style by this plan;
- `all` excludes `direction` and future custom properties;
- `revert-layer` is tested against lower layer candidates and no-lower-layer
  fallback;
- required checks passed.

## Coordinator Commit Guidance

Commit after each task-scoped worker/reviewer cycle is clean. Suggested commit
messages:

```sh
git commit -m "style: add layer source precedence"
git commit -m "style: add authored declaration inputs"
git commit -m "style: add authored sheet rules"
git commit -m "style: resolve supported css-wide keywords"
git commit -m "style: cover authored layer type safety"
```

Workers do not commit. The coordinator owns commits and final reporting.

## Next Sequence Context

The next implementation plan should cover custom properties and
variable-dependent values. It should consume the authored declaration and
supported CSS-wide keyword types from this plan rather than adding another
receiving path.

The next plan should specifically pick up:

- custom property names and authored token payloads as style-owned types;
- variable references and recursive fallback structure;
- custom property inheritance and environment construction;
- invalid-at-computed-value behavior;
- cycle detection;
- dependency tracking and invalidation for declarations that reference
  variables;
- root rejection behavior for `revert` can remain unchanged unless Surgeist
  later adds origin-like semantics.

Do not begin selector expansion, pseudo-element buckets, or broad property
family expansion until the authored layer/source and custom property plans are
both implemented and reviewed.
