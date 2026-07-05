# Pseudo-Element Style Buckets Implementation Plan

> This plan is for agentic execution in `/Users/codex/Development/surgeist-style`.
> Follow `AGENTS.md`: the coordinator splits this into scoped worker tasks,
> workers do not commit, separate reviewers inspect each task, and the
> coordinator commits clean logical points.

## Goal

Add style-owned pseudo-element style buckets for the CSS surface that root can
lower into style without creating synthetic tree nodes.

This operation owns the style-side target model:

- ordinary element style;
- `::before`;
- `::after`;
- `::marker`;
- `::selection`;
- `::backdrop`;
- `::before::marker`;
- `::after::marker`.

Rules must target an originating element selector plus one style bucket.
Resolution must request exactly one style bucket for one originating tree node,
and resolver cache identity must include that requested bucket.

## Source Context

Read these files before assigning implementation work:

- `AGENTS.md`
- `README.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-support-directive.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-selector-tree-matching-expansion-implementation.md`
- `src/lib.rs`
- `src/selector.rs`
- `src/sheet.rs`
- `src/resolver.rs`
- `src/invalidation.rs`
- `src/tree.rs`
- `tests/compile_pass/typed_public_construction.rs`

Read-only CSS source snapshot used for this plan:

- Repo: `/Users/codex/Development/surgeist-css`
- Commit: `1c95d4218439f1696151e0ee9602671fab418314`
- Relevant files:
  - `src/syntax/selector.rs`
  - tests covering pseudo-element parsing and sequence validation

The CSS crate currently exposes `CssPseudoElement::{Before, After, Marker,
Selection, Backdrop}` and accepts only these pseudo-element sequences:

- no pseudo-element sequence;
- `::before`;
- `::after`;
- `::marker`;
- `::selection`;
- `::backdrop`;
- `::before::marker`;
- `::after::marker`.

Style must not depend on `surgeist-css`; root owns CSS-to-style lowering.

## Non-Goals

This plan does not implement:

- CSS parsing;
- CSS selector syntax nodes inside style;
- pseudo-elements as retained, tree, layout, or DSL-addressable nodes;
- generated text/content values;
- counters, list-style values, or marker text generation;
- selection rendering or backdrop rendering;
- broad property family expansion;
- media/container/scope query expansion;
- font-face or keyframe modeling;
- Operation 14 full resolver/cache/invalidation generalization.

Do not add compatibility aliases for old or speculative names. Breaking API
changes are acceptable when they keep the style model clearer.

## Modeling Requirements

Use the Rust modeling guide as the review standard:

- Style owns the bucket vocabulary. Do not expose CSS crate types, retained
  types, layout types, or root adapter types from the new API.
- Model pseudo-element targets as buckets associated with an originating tree
  node, not as tree facts.
- Keep invalid pseudo-element sequences out of ordinary construction.
- Keep rule target invariants behind constructors instead of public struct
  fields.
- Keep generated-content policy as a style boundary classification only. Do not
  add generated-content properties in this operation.
- Keep resolver output as one resolved style for the requested bucket. Do not
  add an aggregate "resolved element plus all pseudos" type in this operation.

## Public API Shape

### New Bucket Module

Create `src/bucket.rs` and export its public types from `src/lib.rs`.

Required public types:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PseudoElement {
    Before,
    After,
    Marker,
    Selection,
    Backdrop,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StyleBucket {
    Element,
    Before,
    After,
    Marker,
    Selection,
    Backdrop,
    BeforeMarker,
    AfterMarker,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StyleBucketPolicy {
    Element,
    GeneratedContentBox,
    Marker,
    Highlight,
    Backdrop,
    GeneratedContentMarker,
}
```

`PseudoElement` represents a single supported pseudo-element token in style's
own vocabulary. `StyleBucket` represents a valid resolved style target bucket.
`StyleBucketPolicy` classifies ownership and materialization boundaries for
downstream users.

Required `StyleBucket` methods:

```rust
impl StyleBucket {
    pub const fn is_element(self) -> bool;
    pub const fn policy(self) -> StyleBucketPolicy;
    pub fn from_pseudo_elements(
        elements: impl IntoIterator<Item = PseudoElement>,
    ) -> Result<Self>;
}
```

`from_pseudo_elements` must map:

- empty sequence to `StyleBucket::Element`;
- `[Before]` to `StyleBucket::Before`;
- `[After]` to `StyleBucket::After`;
- `[Marker]` to `StyleBucket::Marker`;
- `[Selection]` to `StyleBucket::Selection`;
- `[Backdrop]` to `StyleBucket::Backdrop`;
- `[Before, Marker]` to `StyleBucket::BeforeMarker`;
- `[After, Marker]` to `StyleBucket::AfterMarker`.

Every other length or ordering must return an `Error` instead of falling back to
`Element`.

Use an existing error form if one fits cleanly. If none fits, add a specific
style error variant/code that names invalid style bucket construction. The
error must not mention CSS parsing as the owner.

Required `StyleBucket::policy` mapping:

- `Element` -> `StyleBucketPolicy::Element`
- `Before` and `After` -> `StyleBucketPolicy::GeneratedContentBox`
- `Marker` -> `StyleBucketPolicy::Marker`
- `Selection` -> `StyleBucketPolicy::Highlight`
- `Backdrop` -> `StyleBucketPolicy::Backdrop`
- `BeforeMarker` and `AfterMarker` ->
  `StyleBucketPolicy::GeneratedContentMarker`

## Task 1: Add Bucket Model

Files:

- Add `src/bucket.rs`
- Modify `src/lib.rs`
- Modify `src/error.rs` only if a new error code is needed
- Add or extend tests in `src/bucket.rs`
- Extend `tests/compile_pass/typed_public_construction.rs`

Implementation steps:

1. Add the `PseudoElement`, `StyleBucket`, and `StyleBucketPolicy` types.
2. Implement `StyleBucket::from_pseudo_elements` with the exact sequence table
   above.
3. Implement `StyleBucket::is_element` and `StyleBucket::policy`.
4. Export `PseudoElement`, `StyleBucket`, and `StyleBucketPolicy` from
   `src/lib.rs`.
5. Add unit tests for every accepted sequence.
6. Add unit tests for invalid sequences:
   - `[Marker, Before]`;
   - `[Before, Selection]`;
   - `[Before, Marker, Marker]`;
   - `[Selection, Marker]`.
7. Add compile-pass coverage that ordinary public callers can:
   - construct `StyleBucket::Before`;
   - convert `[PseudoElement::Before, PseudoElement::Marker]`;
   - inspect `StyleBucket::BeforeMarker.policy()`.

Focused commands:

```sh
cargo fmt --check
cargo test -p surgeist-style style_bucket
cargo test -p surgeist-style --test compile
```

Reviewer checks:

- Bucket types are style-owned and do not import `surgeist-css`.
- The sequence constructor rejects invalid pseudo-element sequences.
- The policy enum is classification only and does not model generated content
  values.

Commit after a clean worker/reviewer cycle:

```sh
git status --short --branch
git diff --stat
git diff -- src/bucket.rs src/lib.rs src/error.rs tests/compile_pass/typed_public_construction.rs
git add src/bucket.rs src/lib.rs src/error.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: add style bucket model"
```

Only stage `src/error.rs` if it changed.

## Rule Target Model

Rules currently own a selector directly. Replace that internal shape with a
typed rule target:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct RuleTarget {
    selector: Selector,
    bucket: StyleBucket,
}

impl RuleTarget {
    pub fn new(selector: Selector, bucket: StyleBucket) -> Self;
    pub fn element(selector: Selector) -> Self;
    pub fn selector(&self) -> &Selector;
    pub const fn bucket(&self) -> StyleBucket;
    pub fn specificity(&self) -> SelectorSpecificity;
    pub(crate) fn primary_key(&self) -> PrimaryKey;
}
```

`RuleTarget` belongs in `src/sheet.rs` unless the worker finds a clearer local
pattern for public model types. Its fields must stay private.

`RuleTarget::specificity` and `RuleTarget::primary_key` must delegate to the
originating element selector. The pseudo-element bucket must not change selector
matching specificity in style. Root and CSS own parsing-specific specificity
rules before lowering into style's `RulePrecedence`.

Existing public APIs that accept a selector should keep defaulting to
`StyleBucket::Element`:

- `Rule::new(selector, declarations)`
- `Sheet::rule(selector, declarations)`
- `Sheet::push_rule(selector, declarations)`
- `Sheet::push_authored_rule(selector, declarations, precedence)`

Add targeted equivalents:

- `Rule::targeted(target, declarations)`
- `Sheet::targeted_rule(target, declarations)`
- `Sheet::push_targeted_rule(target, declarations)`
- `Sheet::push_authored_targeted_rule(target, declarations, precedence)`

The exact names may be adjusted to match existing local style, but the API must
expose an intentional `RuleTarget` front door and must not require callers to
stuff pseudo-elements into `Selector`.

## Task 2: Add Rule Targets To Sheets

Files:

- Modify `src/sheet.rs`
- Modify `src/lib.rs`
- Extend sheet tests in `src/sheet.rs`
- Add a compile-fail test if compile-fail infrastructure already supports
  private-field checks
- Extend `tests/compile_pass/typed_public_construction.rs`

Implementation steps:

1. Import `StyleBucket` into `src/sheet.rs`.
2. Add `RuleTarget` with private fields and the methods above.
3. Change `Rule` from storing `selector: Selector` to storing
   `target: RuleTarget`.
4. Keep `Rule::selector()` as a semantic accessor over `target.selector()`.
   The method names the originating element selector used for matching; it is
   not an alias for pseudo-element syntax.
5. Add `Rule::target()` and `Rule::style_bucket()` accessors.
6. Update `Rule::with_order` to compute specificity from `target.specificity()`.
7. Add an internal `Rule::with_target_order` helper for targeted rules.
8. Update `Rule::with_authored` and add `Rule::with_authored_target` so authored
   rule lowering can preserve explicit precedence for pseudo buckets.
9. Update `RuleIndex` to index by `rule.target().primary_key()`.
10. Keep existing selector-only sheet APIs defaulting to `RuleTarget::element`.
11. Add targeted sheet APIs for legacy and authored declarations.
12. Export `RuleTarget` from `src/lib.rs`.

Tests to add:

- `rule_new_defaults_to_element_bucket`
- `targeted_rule_preserves_bucket`
- `targeted_rule_uses_origin_selector_primary_key`
- `push_authored_targeted_rule_preserves_explicit_precedence`
- compile-pass public construction for `RuleTarget::new`,
  `Sheet::targeted_rule`, and `Sheet::push_targeted_rule`
- compile-fail private-field construction if existing compile tests make that
  straightforward

Focused commands:

```sh
cargo fmt --check
cargo test -p surgeist-style sheet
cargo test -p surgeist-style --test compile
```

Reviewer checks:

- `RuleTarget` is a typed model, not a tuple or public bag.
- Existing selector-only rule APIs still target `StyleBucket::Element`.
- Targeted authored rules do not recalculate or erase root-provided
  precedence.
- The sheet index still keys off the originating selector, not a pseudo-tree
  identity.

Commit after a clean worker/reviewer cycle:

```sh
git status --short --branch
git diff --stat
git diff -- src/sheet.rs src/lib.rs tests/compile_pass tests/compile_fail
git add src/sheet.rs src/lib.rs tests/compile_pass tests/compile_fail
git commit -m "style: target rules at style buckets"
```

If no compile-fail file is added, do not stage `tests/compile_fail`.

## Resolver Bucket Selection

`Context` resolves one style bucket for one originating node. Add a
`style_bucket` field to `Context` with `StyleBucket::Element` as the default.
Keep it private like selector root/scope so callers use the builder:

```rust
impl<'a, T: Tree> Context<'a, T> {
    pub const fn style_bucket(mut self, bucket: StyleBucket) -> Self;
}
```

Resolver rule application must skip any rule whose `RuleTarget` bucket differs
from `context.style_bucket`.

Resolver cache identity must include `context.style_bucket`.

Inheritance remains caller-controlled through `Context::parent`. For this
operation, do not make the resolver automatically resolve an originating
element style before resolving a pseudo-element bucket. Root or the caller must
pass the correct parent style for the bucket being resolved.

`Context::local` and `Context::animated` apply to the requested bucket because
they are explicit overlays on the current resolution request. Do not add
separate local/animated pseudo maps in this operation.

## Task 3: Add Resolver Bucket Filtering And Cache Keys

Files:

- Modify `src/resolver.rs`
- Extend resolver tests in `src/resolver.rs`
- Extend `tests/compile_pass/typed_public_construction.rs`

Implementation steps:

1. Import `StyleBucket` into `src/resolver.rs`.
2. Add `style_bucket: StyleBucket` to `Context`.
3. Set `StyleBucket::Element` in `Context::new`.
4. Add `Context::style_bucket`.
5. Hash `context.style_bucket` in `Resolver::cache_key`.
6. Before condition checks or selector matching, skip rules whose
   `rule.style_bucket() != context.style_bucket`.
7. Keep selector matching against the originating element node using
   `SelectorMatchContext`.
8. Keep parent, local, animated, selector root, and selector scope behavior
   unchanged except for cache identity.

Tests to add:

- `resolver_defaults_to_element_bucket`
- `resolver_applies_only_requested_style_bucket`
- `resolver_cache_key_includes_style_bucket`
- `pseudo_bucket_inherits_from_supplied_parent_style`
- `local_and_animated_overlays_apply_to_requested_bucket`

Test shape:

- Build a test tree with one node carrying a class.
- Add an element rule and a `StyleBucket::Before` targeted rule for the same
  originating selector.
- Resolve the element bucket and assert only the element rule applies.
- Resolve the before bucket and assert only the before rule applies.
- Resolve both buckets with a versioned tree and assert repeated resolution of
  one bucket does not return the other bucket's cached result.
- Resolve a pseudo bucket with `Context::parent(&origin_resolved)` and assert
  inherited values come from the supplied parent when no pseudo rule overrides
  them.

Focused commands:

```sh
cargo fmt --check
cargo test -p surgeist-style resolver
cargo test -p surgeist-style --test compile
```

Reviewer checks:

- Cache keys include `StyleBucket`.
- Pseudo buckets use the same originating node selector facts as element rules.
- The resolver does not materialize or query synthetic pseudo nodes.
- Parent inheritance remains explicit and does not introduce hidden recursive
  resolution.

Commit after a clean worker/reviewer cycle:

```sh
git status --short --branch
git diff --stat
git diff -- src/resolver.rs tests/compile_pass/typed_public_construction.rs
git add src/resolver.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: resolve requested style buckets"
```

## Boundary And Regression Checks

After Tasks 1 through 3 are committed, run a final integration pass.

Searches:

```sh
rg -n "surgeist_css|surgeist-css|CssPseudo|CssSelector|Retained|TreeNode|NodeId" src tests Cargo.toml
rg -n "PseudoElement|StyleBucket|RuleTarget|style_bucket|targeted_rule|push_targeted" src tests
rg -n "Content|Counter|ListStyle|FontFace|Keyframes|Media|ContainerQuery|ScopeQuery" src tests
```

Expected search results:

- No `surgeist-css` dependency or imports.
- No retained/tree-node pseudo-element model.
- `PseudoElement`, `StyleBucket`, and `RuleTarget` appear only in the new model,
  rule targeting, resolver context, tests, and public exports.
- The third search may show existing unrelated value/property names, but this
  plan must not add generated content, counter, list-style, font-face, keyframe,
  media query, container query, or scope query modeling.

Full checks:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Final holistic reviewer prompt:

```text
Review the completed pseudo-element style bucket implementation in
/Users/codex/Development/surgeist-style against:
- plans/2026-07-05-pseudo-element-style-buckets-implementation.md
- guidance/surgeist-rust-modeling-guide.md
- AGENTS.md

Scope:
- new style-owned bucket vocabulary;
- rule target model;
- sheet APIs;
- resolver context, filtering, and cache identity;
- tests and public exports.

Check for:
- pseudo-elements modeled as tree nodes or retained/layout/root concepts;
- invalid pseudo-element sequences constructible by ordinary callers;
- broad enum or public-field modeling smells;
- generated content/list/counter/property expansion leaking into this slice;
- resolver cache or inheritance bugs;
- missing tests for bucket filtering and cache separation.

Report findings with file/line references, or say clean if there are no
blockers.
```

After a clean final reviewer cycle, coordinator final commit is only needed if
the final pass changed files. If no final edits are needed, push the task
commits only when root or the user asks for a fetchable commit.

## Completion Report

Report:

- commit SHAs for each logical task commit;
- final reviewer result;
- checks run and their outcomes;
- any intentional non-goals left for later operations;
- final `git status --short --branch`.

## Next Sequence Context

The next implementation plan should cover Operation 7: property coverage
ledger.

That plan should classify each CSS property from `surgeist-css` into one of
these style-owned outcomes:

- already represented by an existing style `Property` and `Value` model;
- new style property/value model needed;
- shorthand lowering into existing or new longhands;
- symbolic property retained for another owner;
- root rejection required;
- explicitly out of style.

Start with layout and box model, flex/grid/alignment, writing mode, visibility,
aspect ratio, z-index, scrollbar width, content visibility, and text/font-facing
families. Do not begin implementing broad property families until the ledger
plan and its review are complete.

Operation 11 will later add generated content, counters, list styles, and marker
styling. Operation 14 will later generalize cache and invalidation across
dimensions, variables, pseudo-element buckets, selectors, and condition facts.
