# API Artifacts, Compile-Fail Coverage, And Root Handoff Implementation Plan

Date: 2026-07-07

## Goal

Complete Operation 15 from
`plans/2026-07-05-css-surface-style-operations-sequence.md` by making the
completed CSS-facing style surface reviewable and handoff-ready for root.

This plan should not add new style semantics. Its work is to document the
current crate-owned front-door APIs, lock public construction invariants with
targeted compile-fail coverage, and record root lowering notes for the CSS
surface completed through Operation 14.

Completion means:

- style has a crate-local API artifact describing the CSS-facing style surface,
- root has crate-local handoff notes for lowering `surgeist-css` output into
  style-owned APIs,
- compile-pass and compile-fail type-safety coverage is audited and extended
  where Operation 8 through Operation 14 introduced uncovered public model
  edges,
- the ledgers are rebased to show the Operation 15 artifact state,
- final checks pass, and
- a clean-context reviewer reports no blockers.

## Context

The current style commit before this plan was drafted is:

```text
1670bfda84c6117051143f694934d0ed06128618
```

Operation 14 landed resolver diagnostics, cache-axis coverage, and typed
invalidation hooks. The current ledgers point to Operation 15 as the next and
final style-local sequence step:

- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-property-coverage-ledger.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`

The root and CSS boundary remains unchanged:

- `surgeist-css` owns parsing and CSS syntax/source-location types.
- root owns lowering from CSS syntax into style-owned APIs.
- `surgeist-style` owns typed receiving models, selector/tree facts, cascade
  semantics, variables, property validation, resolver behavior, diagnostics,
  cache axes, invalidation, and computed style outputs.
- `surgeist-style` must not depend on `surgeist-css`.

## Boundaries

- Do not edit sibling repositories.
- Do not add a `surgeist-css` dependency or import CSS syntax types.
- Do not add a root adapter or downstream lowering layer in this crate.
- Do not add compatibility aliases.
- Do not introduce new cascade semantics, parser behavior, layout behavior,
  text shaping, retained projection, rendering, image loading, font loading, or
  host environment discovery.
- Do not broaden public constructors just to simplify tests or root lowering.
- If the artifact audit finds a real API or modeling gap, record it in the
  handoff notes or ledgers instead of smuggling a workaround into Operation 15.

## Files

Create:

- `plans/2026-07-07-style-css-api-artifact.md`
- `plans/2026-07-07-style-root-handoff-notes.md`

Modify:

- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-property-coverage-ledger.md`
- `tests/compile_pass/typed_public_construction.rs`
- `tests/compile_fail/*.rs`
- `tests/compile_fail/*.stderr`

Only add compile-fail files when the audit in Task 3 finds an uncovered public
construction invariant. Do not churn existing `.stderr` files unrelated to new
or edited compile-fail cases.

## Task 1: Create The API Artifact

Add `plans/2026-07-07-style-css-api-artifact.md`.

The artifact is descriptive. It is not generated in this pass, and it must not
claim to be an exhaustive rustdoc replacement. It should name the style-owned
front-door types root can lower into after Operations 8 through 14.

Use this structure:

```markdown
# Style CSS API Artifact

Date: 2026-07-07

## Purpose

Describe the crate-owned CSS-facing style API surface available after the CSS
surface implementation sequence through Operation 14.

## Source Snapshot

| Source | Value |
| --- | --- |
| style commit | `<current HEAD when artifact is written>` |
| CSS surface ledger | `plans/2026-07-05-css-surface-style-ledger.md` |
| property coverage ledger | `plans/2026-07-05-css-property-coverage-ledger.md` |
| sequence | `plans/2026-07-05-css-surface-style-operations-sequence.md` |

## Boundary

State that style receives root-lowered typed data and does not import
`surgeist-css`.

## Public Front Doors

### Identity And Tree Facts

List `StyleTag`, `StyleClass`, `StyleKey`, `StyleAttributeName`,
`StyleAttributeValue`, `StyleAttribute`, `StyleRole`, `StyleState`, `Node`,
`Tree`, and `Traversal`.

### Selectors, Scopes, And Buckets

List `Selector`, `SelectorList`, `AttributeSelector`, `AttributeMatcher`,
`AttributeCaseSensitivity`, `Combinator`, `Compound`, `ComplexSelectorPart`,
`RelativeSelector`, `RelativeSelectorList`, `PseudoClassSelector`,
`RuntimePseudoClass`, `StructuralSelector`, `NthSelector`, `NthPattern`,
`SelectorListPseudoClass`, `PseudoElement`, `SelectorSpecificity`,
`ScopeSelectorList`, `RuleScope`, `StyleBucket`, and `StyleBucketPolicy`.

### Authored Declarations And Variables

List `AuthoredDeclaration`, `AuthoredDeclarations`, `AuthoredProperty`,
`AuthoredValue`, `CssWideKeyword`, `CustomPropertyName`,
`CustomPropertyValue`, `CustomPropertyTypedValue`, `AuthoredTokens`,
`VariableDependentValue`, `VariableExpression`, `VariableReference`, and
`VariableFallback`.

### Properties And Values

Summarize `Property`, `Value`, `Declaration`, `Declarations`,
`TypedDeclaration`, and the typed value families added or rebased by Operations
8 through 12: layout, text/font, paint/color/effects, generated content,
counters/lists, timing/animation, and keyframes.

### Sheets, Rules, Layers, And Conditions

List `Sheet`, `Rule`, `RuleTarget`, `RulePrecedence`, `SourceOrder`,
`LayerOrder`, `StyleLayerName`, `StyleLayerNameList`, `LayerStatement`,
`LayerBlock`, `LayerRegistry`, `Condition`, `ConditionFacts`,
`MediaEnvironment`, `MediaQueryList`, `MediaQuery`, `MediaCondition`,
`MediaFeatureQuery`, `ContainerFacts`, `ContainerCondition`,
`ContainerConditionList`, and `ContainerFeatureQuery`.

### Resolver, Diagnostics, Cache Axes, And Invalidation

List `Context`, `Resolver`, `Resolved`, `ResolvedWithDiagnostics`,
`StyleSourceId`, `StyleDiagnostic`, `StyleDiagnosticKind`,
`StyleDiagnosticSubject`, `InvalidAtComputedValueReason`, `Invalidation`,
`Change`, `SelectorFactChange`, `ConditionFactChange`, and `CascadeChange`.

## Explicit Non-Goals

Record that imports, font loading, CSS source spans, root integration
diagnostics, host fact discovery, layout, text shaping, retained projection,
render resources, image loading, animation scheduling, and final color/resource
realization remain outside this crate.

## Verification Surface

Name the compile-pass and compile-fail tests that demonstrate public
construction paths and private construction invariants.
```

When listing APIs, keep the descriptions short and typed. Avoid marketing
language and avoid restating every enum variant unless the variant set is itself
the root handoff contract.

## Task 2: Create Root Handoff Notes

Add `plans/2026-07-07-style-root-handoff-notes.md`.

Use this structure:

```markdown
# Style Root Handoff Notes

Date: 2026-07-07

## Purpose

Record how root should lower parsed CSS-owned syntax into the style-owned API
surface after the CSS surface implementation sequence through Operation 14.

## Source Snapshot

| Source | Value |
| --- | --- |
| style commit | `<current HEAD when notes are written>` |
| API artifact | `plans/2026-07-07-style-css-api-artifact.md` |

## Lowering Responsibilities

### Rule Ordering And Layers

Describe lowering `@layer` statements and blocks through `LayerStatement`,
`LayerBlock`, `LayerRegistry`, `Sheet::declare_layers`,
`Sheet::push_layer_rule`, and rule source order. State that this style pass has
no `!important` or cascade origin support.

### Selectors, Scopes, And Buckets

Describe lowering parsed selectors into style-owned selectors, pseudo-element
sequences into `StyleBucket`, and `@scope` roots/limits into `RuleScope`.

### Declarations, CSS-Wide Keywords, And Variables

Describe lowering ordinary values into typed `Value` payloads, CSS-wide
keywords into `AuthoredDeclaration::css_wide`, custom properties into
`CustomPropertyValue`, and variable-dependent ordinary declarations into
`VariableDependentValue`.

### Conditions And Environment Facts

Describe lowering `@media` into `MediaQueryList`/`MediaCondition`, `@container`
into `ContainerCondition`, and host facts into `ConditionFacts`,
`MediaEnvironment`, and `ContainerFacts`.

### Generated Content, Timing, And Keyframes

Describe lowering generated content/list/counter data, transition and animation
properties, and `@keyframes` into style-owned symbolic data. State that runtime
materialization and scheduling stay outside style.

### Resolver And Diagnostics

Describe building `Context`, selecting `StyleBucket`, passing parent/local/
animated declarations where relevant, calling `Resolver::resolve` or
`Resolver::resolve_with_diagnostics`, and mapping `StyleSourceId` through root's
source table.

### Invalidation

Describe root mapping DOM/tree/runtime/environment/cascade/style-bucket changes
to `Change`, `SelectorFactChange`, `ConditionFactChange`, `CascadeChange`, and
property invalidation.

## Unsupported Or Deferred Surfaces

List the deliberate non-goals from the API artifact in root-facing terms.

## Root Questions

Carry forward unresolved root decisions from the surface ledger, including
cascade origin, unsupported parsed CSS properties, unsupported-integration
diagnostics, available media/container facts, and font-face ownership.

## Next Work

State that the next step after this style plan is root integration/lowering
planning against these artifacts, not another broad style surface expansion
unless root discovers a concrete modeling gap.
```

## Task 3: Audit And Extend Type-Safety Coverage

Inspect the current trybuild harness before editing:

```sh
sed -n '1,120p' tests/type_safety.rs
sed -n '1,940p' tests/compile_pass/typed_public_construction.rs
find tests/compile_fail -maxdepth 1 -type f | sort
rg -n "StyleSourceId|StyleDiagnostic|ResolvedWithDiagnostics|ConditionFactChange|CascadeChange|SelectorFactChange|StyleBucket|LayerRegistry|LayerOrder|LayerStatement|LayerBlock|RuleScope|RuleTarget|SelectorSpecificity|KeyframesRule|KeyframeBlock|KeyframeOffset" tests/compile_pass tests/compile_fail src
```

Confirm the existing coverage at least includes:

- public construction paths for the front doors listed in the API artifact,
- compile-fail coverage for private construction of representative newtypes and
  structs,
- source/diagnostic privacy through `invalid_style_source_id_literal.rs` and
  `invalid_style_diagnostic_literal.rs`,
- layer/source-order privacy through `invalid_precedence_newtype_literal.rs`
  and `invalid_precedence_struct_literal.rs`,
- scoped selector and target privacy through `invalid_rule_scope_literal.rs`
  and `invalid_rule_target_struct_literal.rs`,
- selector specificity privacy through
  `invalid_selector_specificity_struct_literal.rs`, and
- keyframe offset validation/privacy through `invalid_keyframe_offset_literal.rs`.

If any Operation 8 through Operation 14 public type has a public constructor or
private invariant that is not represented by either
`tests/compile_pass/typed_public_construction.rs` or a focused compile-fail
case, add the smallest focused test.

Expected likely additions:

- Add a compile-fail case for `ResolvedWithDiagnostics` struct literals if the
  audit confirms no existing case prevents direct construction of its private
  `resolved` and `diagnostics` fields.
- Add a compile-fail case for `LayerRegistry` struct literals if the audit
  confirms no existing case prevents direct construction of its private
  registry fields.
- Add compile-pass lines only when needed to show an existing public accessor or
  constructor path remains usable from outside the crate.

Do not add tests that reach into crate-private APIs. Do not make fields public
to satisfy tests.

When adding trybuild cases, run:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
cargo test -p surgeist-style --test type_safety
```

Review new `.stderr` files before keeping them. They should demonstrate privacy
or type mismatch errors for exactly the attempted invalid construction.

## Task 4: Rebase The Ledgers To Operation 15

Update the ending context in both ledgers:

- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-property-coverage-ledger.md`

Required updates:

- Record that Operation 15 has an API artifact and root handoff notes.
- Link to `plans/2026-07-07-style-css-api-artifact.md`.
- Link to `plans/2026-07-07-style-root-handoff-notes.md`.
- Keep the unresolved root coordination questions intact unless the artifact
  work uncovers a precise answer in this crate.
- Replace the old "After Operation 14 lands..." next context with a "Next Work"
  note saying root integration/lowering planning should proceed from the API
  artifact, handoff notes, and the rebased ledgers.

Do not re-inventory the full CSS property table unless the implementation audit
finds a concrete mismatch.

## Task 5: Boundary And Dependency Audit

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests plans
rg -n "pub [^\\n]*(surgeist_css|Css[A-Z][A-Za-z0-9_]*)" src tests
```

Expected result:

- `Cargo.toml`, `src`, and `tests` must not reference `surgeist-css` or
  `surgeist_css`.
- Planning files may mention `surgeist-css` descriptively.
- Style-owned names such as `CssWideKeyword` and `CssPx` are allowed; do not
  rename them in this pass.

If a real dependency or public CSS crate type appears in code, stop and report
the blocker.

## Task 6: Final Verification

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Review before committing:

```sh
git diff --stat
git diff -- plans/2026-07-07-api-artifacts-compile-fail-root-handoff-implementation.md
git diff -- plans/2026-07-07-style-css-api-artifact.md
git diff -- plans/2026-07-07-style-root-handoff-notes.md
git diff -- tests/compile_pass/typed_public_construction.rs tests/compile_fail
git diff -- plans/2026-07-05-css-surface-style-ledger.md plans/2026-07-05-css-property-coverage-ledger.md
```

Commit at a logical point after worker/reviewer cycles are clean.

Suggested commit message:

```text
style: add css api handoff artifacts
```

## Review Rubric

Use a blocker/non-blocker rubric.

Blockers:

- The plan or implementation adds a `surgeist-css` dependency or imports CSS
  crate syntax types into style.
- The API artifact claims behavior style does not own or support.
- Root handoff notes tell root to use crate-private APIs.
- Type-safety tests require weakening constructor privacy or public invariants.
- Ledgers are left pointing to Operation 14 as future work.
- Required verification fails.

Non-blockers:

- Wording improvements in artifact descriptions.
- Suggestions for additional examples that do not expose a missing invariant.
- Requests to split artifact sections differently without changing content.
- Future root integration concerns already listed as open questions or deferred
  surfaces.

Complete the plan when a clean-context reviewer reports no blockers. Non-
blocker comments should be recorded for future refinement but should not keep
this goal open.

## This Will Come Next

After Operation 15 implementation lands, the CSS-facing style surface sequence
is closed for this crate. The next work should be root-side integration and CSS
lowering planning against:

- the API artifact,
- the root handoff notes,
- the CSS surface ledger,
- the property coverage ledger, and
- the final style commit from Operation 15.

Further style plans should be driven by concrete root integration findings or
new CSS/parser surface changes, not by another broad style inventory pass.
