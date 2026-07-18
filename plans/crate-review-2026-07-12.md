# `surgeist-style` Completeness, Correctness, and Quality Review

VERDICT: NOT CLEAN

SCOPE: Repository-wide review of `surgeist-style` at
`cb24432aa9e63c6da10ded4e2e626529d621e77d` on 2026-07-12, augmented with static
performance findings on 2026-07-17. This is a review reference, not a
specification, implementation sequence, or cycle plan. Product code remained
read-only.

CSS COMPLETENESS BASELINE: [CSS Snapshot 2026](https://www.w3.org/TR/css-2026/)
Section 2.1, the official stable definition, plus Section 2.2, Reliable Candidate
Recommendations. Snapshot Sections 2.3 and 2.4 and other Working Draft/current-work
modules are excluded from completeness findings. Existing implemented behavior was
still reviewed for correctness when it represents a deployed CSS feature. CSS has
no monolithic Level 4; modules advance independently.

BOUNDARY: The review applies the repository-owned style boundary: typed property,
value, selector, condition, cascade, resolution, validation, diagnostics, and
invalidation contracts are in scope. Authored CSS tokenization/parsing, layout and
text algorithms, render lowering, and root-owned cross-crate adapters are out of
scope. A missing typed style contract remains in scope even when its downstream
layout or rendering algorithm belongs elsewhere.

EVIDENCE CHECKED: `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `README.md`, all tracked
Rust source, the 262 tracked `#[test]` functions, 63 compile-fail cases, the public
compile-pass case, and the complete public front door. The following cached/offline
checks passed:

- `cargo metadata --offline --locked --no-deps --format-version 1`
- `cargo check --offline --locked -p surgeist-style`
- `cargo test --offline --locked -p surgeist-style` (261 unit tests, all trybuild
  cases, and zero doctests)
- `cargo clippy --offline --locked -p surgeist-style --all-targets -- -F unsafe-code -D warnings`
- `cargo fmt --check`
- Repository-wide tracked-Rust unsafe scan using the canonical Surgeist pattern;
  no executable or textual unsafe construct matched.

The 2026-07-17 performance augmentation used static source inspection against the
same source revision. The checks above were not rerun for that augmentation.

FINDINGS:

## [Important] Resolver cache keys alias different trees and colliding node identities

Location: `src/resolver.rs:1150-1159`, `src/resolver.rs:1272-1300`,
`src/resolver.rs:2061-2069`, `src/resolver.rs:2191-2195`

Evidence: Cache lookup is keyed solely by a `u64` digest. The digest includes the
tree-provided revision and a second digest of the node ID, but it contains no tree
instance identity and retains no equality-preserving node identity. Two different
`Tree` instances can legitimately report the same local revision and use the same
node ID while exposing different tag/class/attribute facts; the second resolution
then returns the first tree's cached style before selector matching. Rust also
permits unequal IDs to have the same `Hash` output, producing the same failure in
one tree.

Impact: `Resolver::resolve` can return another tree or node's style, violating
selector, cascade, and cache correctness.

Required remediation: Make cache identity equality-preserving for the tree and
node, or scope/disable caching when that identity is unavailable. Add cross-tree
and colliding-ID regression cases.

## [Important] Resolver storage makes warm cache hits scale with the complete style

Location: `src/resolver.rs:34-55`, `src/resolver.rs:910-935`,
`src/resolver.rs:1082-1088`, `src/resolver.rs:1150-1169`,
`src/resolver.rs:1254-1300`

Evidence: `Resolved::new` allocates a `BTreeMap` and clones the default value for
every canonical property. Inheritance clones inherited values and the complete
custom-property map. A cache hit clones the complete `Resolved`; a cache miss
clones it again when inserting it. Building a child cache key recomputes the
parent fingerprint by hashing every resolved property, custom property, and
dependency. Consequently even a warm cache hit traverses and copies state
proportional to the complete resolved style rather than the changed or requested
properties.

Impact: Resolving or revisiting every node incurs repeated map allocation, value
cloning, and full-style hashing. The cost grows with both tree size and the
property/custom-property catalog, so completing the CSS vocabulary makes the core
restyle path progressively more expensive and erodes the benefit of the cache.

Required remediation: Give resolved styles an immutable, cheaply shared or
otherwise allocation-conscious representation with canonical-property lookup that
does not require a tree map per style. Make cache hits avoid full-style cloning and
make parent identity/fingerprinting reusable rather than recomputed by traversal.
Preserve equality-correct cache identity and add representative cold-resolution,
inheritance, and warm-hit scale evidence that detects work proportional to the
complete style where it is not semantically required.

## [Important] Custom-property cycle detection omits fallback references

Location: `src/resolver.rs:1644-1684`; contradictory regression at
`src/resolver.rs:3665-3727`

Evidence: `collect_required_reference_edges` traverses a `var()` fallback only
when the primary reference is missing or invalid. The existing test therefore
treats `--b: var(--a, var(--b))` as non-cyclic when `--a` is valid. The stable
[Custom Properties Level 1 cycle algorithm](https://www.w3.org/TR/css-variables-1/#cycles)
requires an edge for every `var()` reference, explicitly including references in
fallback arguments, and makes every property in a cycle invalid at computed-value
time.

Impact: Cyclic custom properties can be accepted as valid and ordinary properties
can receive a primary value when the cycle should force the outer fallback or
invalid-at-computed-value behavior.

Required remediation: Build the custom-property cycle graph from all references,
including nested fallbacks, while retaining lazy fallback traversal only for value
evaluation and invalidation. Replace the contradictory regression with the stable
spec result.

## [Important] Cascade precedence cannot represent stable CSS and misorders unlayered rules

Location: `src/precedence.rs:93-138`, `src/precedence.rs:183-230`,
`src/authored.rs:15-20`, `src/sheet.rs:547-617`, `src/resolver.rs:1234-1244`

Evidence: `RulePrecedence` contains only layer order, specificity, and source order.
Declarations contain neither CSS origin/context nor importance, and
`CssWideKeyword` omits stable `revert`. Normal unlayered rules receive layer zero,
named layers receive increasing positive orders, candidates sort ascending, and
the resolver selects the last candidate. This makes every named layer outrank
unlayered normal declarations. Stable [Cascade 4](https://www.w3.org/TR/css-cascade-4/)
requires origin and importance ordering plus `revert`; reliable
[Cascade 5](https://www.w3.org/TR/css-cascade-5/#layer-order) places unlayered
normal declarations after all named layers and reverses layer precedence for
important declarations.

Impact: User-agent/user/author and normal/important precedence cannot be expressed,
unlayered declarations lose incorrectly, and `revert`/`revert-layer` cannot produce
the required stable cascade result.

Required remediation: Model semantic origin/context and importance, represent the
implicit unlayered layer explicitly, implement both rollback keywords, and prove
the complete origin/importance/layer/specificity/source-order matrix.

## [Important] Top-level selector lists use specificity from nonmatching branches

Location: `src/selector.rs:164-177`, `src/sheet.rs:37-68`,
`src/sheet.rs:127-141`

Evidence: `Selector::List` always reports `SelectorList::max_specificity()`, and
`RuleTarget` stores that fixed maximum before matching. For `.match, #never`, a
node matching only `.match` is nevertheless assigned ID specificity and can defeat
a later `.match` rule. A top-level comma-separated selector list has the cascade
effect of separate selectors; only matching branches contribute candidates.

Impact: The wrong declaration wins whenever a selector list contains a
nonmatching branch with higher specificity than the branch that matched.

Required remediation: Preserve selector-list branch identity through matching and
candidate creation, then use the greatest specificity among branches that actually
matched. Add a cascade regression with a nonmatching high-specificity branch.

## [Important] Stable property metadata produces incorrect initial, inherited, and shorthand-reset values

Location: `src/value.rs:1094-1105`, `src/value.rs:4157-4201`,
`src/property.rs:601-613`, `src/property.rs:702-722`,
`src/property.rs:800-803`, `src/property.rs:890-917`, `src/resolver.rs:43-49`,
`src/resolver.rs:910-917`, `src/resolver.rs:2047-2058`

Evidence: Fresh resolution and `initial`/`unset` use `Property::metadata()` directly.
The metadata derives `display: flex`, `box-sizing: border-box`, and
`position: relative`; stable CSS initial values are `inline`, `content-box`, and
`static`. Maximum sizes use `auto` although stable CSS uses `none`. `line-height`
uses `16px` although its initial and computed keyword is `normal`, and canonical
`font-family` is an empty public list while the noncanonical `font` record uses
`serif`. `visibility` and `cursor` omit the inherited flag even though both are
inherited; parent propagation and `unset` depend exclusively on that flag. See
[CSS2 display/position](https://www.w3.org/TR/CSS2/visuren.html),
[CSS2 sizing and line height](https://www.w3.org/TR/CSS2/visudet.html), and
[CSS UI 3 box sizing/cursor](https://www.w3.org/TR/css-ui-3/).

Impact: A fresh style, CSS-wide defaulting, font shorthand reset, and child
inheritance return observably wrong CSS values across core layout, text, and UI
properties.

Required remediation: Derive a reviewed metadata table from the stable property
definitions, represent every initial value without approximation, and add focused
fresh-style, `initial`, `unset`, inherit, and shorthand-reset tests for every
property.

## [Important] Public resolved lookup panics for valid public `Property` variants

Location: `src/resolver.rs:43-61`, `src/property.rs:405-442`

Evidence: `Resolved::new` stores only canonical properties, while public
`Resolved::get(Property)` unconditionally calls `expect`. Valid public shorthand
variants such as `Property::Margin`, `Property::Font`, and `Property::Animation`
are intentionally noncanonical, so `Resolved::new().get(Property::Margin)` panics.

Impact: Safe downstream input can terminate the process through the core public
query API.

Required remediation: Make arbitrary-property lookup explicitly fallible, or
accept a distinct type that can represent only canonical properties. Add shorthand
lookup regression coverage.

## [Important] Public selector position values can overflow and panic

Location: `src/selector.rs:1167-1224`, `src/selector.rs:1227-1268`,
reexported by `src/lib.rs:67-71`

Evidence: `SelectorPosition` exposes public fields and an infallible constructor
without the invariant `index < sibling_count`. `is_last` and `matches` calculate
`index + 1`, which overflows for `usize::MAX`. `NthPattern::new` accepts arbitrary
`i32` coefficients and `matches` performs unchecked `position - b`, which can also
overflow.

Impact: Public safe methods can panic in checked builds and silently compute an
incorrect selector result under wrapping/saturating release behavior.

Required remediation: Make fields private, validate position invariants, use
overflow-safe `an+b` arithmetic, and cover extreme public inputs.

## [Important] Square media and container boxes are classified as landscape

Location: `src/condition.rs:1157-1168`, `src/condition.rs:1359-1370`

Evidence: Both inferred-orientation paths return landscape when `width >= height`.
[Media Queries Level 4](https://www.w3.org/TR/mediaqueries-4/#orientation) defines
portrait when height is greater than or equal to width; landscape requires width
to be strictly greater.

Impact: Portrait and landscape conditions toggle incorrectly for every square
viewport or container.

Required remediation: Use strict width-greater-than-height for landscape in both
paths and add square media and container regressions.

## [Important] Condition invalidation helpers underreport global rematch scope

Location: `src/sheet.rs:710-717`, `src/sheet.rs:739-762`,
`src/invalidation.rs:166-179`

Evidence: `Change::from_condition_fact_change` correctly requests whole-tree
rematching, but `Sheet::media_condition_change` and
`Sheet::container_condition_change` route through `condition_change`, which marks
only node plus descendants. These sheet helpers accept no anchor node, and a media
fact change can toggle matching rules in multiple branches.

Impact: Consumers can rematch too narrow a scope and leave cached or resolved
styles stale outside one subtree.

Required remediation: Base sheet condition helpers on the typed condition-fact
change's whole-tree scope and then add affected property impacts. Assert
`scope.whole_tree` for media and unanchored container changes.

## [Important] Selector-fact invalidation always requests a whole-tree rematch

Location: `src/invalidation.rs:54-104`, `src/invalidation.rs:165-170`,
`src/sheet.rs:798-804`

Evidence: `SelectorFactChange` distinguishes tag, key, class, attribute, runtime
state, structure, and scope changes, but `Change::from_selector_fact_change`
ignores that value and unconditionally sets `rematch` plus `whole_tree`. `Scope`
cannot express ancestor or directional-sibling impact, and the sheet retains no
reverse dependency information describing which selector facts and combinators
can be affected by a local change. A class, attribute, or runtime-state change is
therefore represented identically to a genuinely global selector change.

Impact: Ordinary interactive updates such as hover, focus, class, and attribute
changes force every node to be reconsidered. Relational selectors such as
`:has()` do require ancestor or sibling propagation, but the current contract can
express that correctness only by discarding all locality, defeating incremental
style resolution and downstream caches.

Required remediation: Model selector dependencies and rematch directions for
subject, ancestor, descendant, sibling, structure, and scope effects, and derive a
fact-sensitive rematch plan from the compiled sheet plus the changed-node anchor.
Use whole-tree rematching only when the selector or condition dependency is
actually global. Add focused invalidation cases for local class/state/attribute
changes and for descendant, sibling, and `:has()` relationships that prove both
correct propagation and unaffected-region exclusion.

## [Important] The stable value algebra is incomplete and conflates incompatible property domains

Location: `src/value.rs:623-660`, `src/value.rs:679-719`,
`src/value.rs:1094-1105`, `src/calc.rs:3-27`,
`src/value.rs:2905-2991`, `src/property.rs:1138-1161`

Evidence: Ordinary `Length` represents only px, percentage, a px/percentage
calculation, and a few keywords; it cannot preserve stable Values 3 font-relative,
viewport, or physical units. `CalcLength` supports only addition/subtraction of px
and percentages. Cubic Bezier, steps, filter, shape, and rotate arguments are
opaque strings checked only for empty/NUL input. One nonnegative
`DurationSeconds` type is used for both duration and delay properties, so the
public API rejects valid negative transition and animation delays even though
durations must remain nonnegative. See [Values and Units Level 3](https://www.w3.org/TR/css-values-3/),
[Easing Functions Level 1](https://www.w3.org/TR/css-easing-1/),
[Transitions Level 1](https://www.w3.org/TR/css-transitions-1/#transition-delay-property),
and [Animations Level 1](https://www.w3.org/TR/css-animations-1/#animation-delay).

Impact: Large classes of valid stable values cannot enter the typed boundary,
invalid branded function values can enter declarations, and valid deployed delay
semantics are rejected.

Required remediation: Introduce phase- and domain-specific typed units,
calculations, angles, times, and easing/function models. Preserve authored tokens
only in an explicit authored phase and resolve contextual units through an
explicit computed-value context.

## [Important] The property, value, and rule catalog omits entire stable and reliable modules

Location: `src/property.rs:27-213`, `src/value.rs:3073-3084`,
`src/value.rs:4157-4201`, `src/sheet.rs:411-440`

Evidence: The 185-property enum is substantial, but it omits stable `contain`, all
multi-column properties, compositing/blending properties, `border-image`, and many
CSS2, Fonts 3, Writing Modes 3, and UI 3 properties. `Display` cannot represent
stable inline/list/table families, and `Overflow` lacks `auto`. `ImageLayer` is
only `none` or URL, so stable Images 3 gradients are absent. `Sheet` has no stable
`@counter-style` rule/descriptors. Reliable Scroll Snap is absent; Scrollbars has
width but not color; Color Adjustment is absent. The public names also conflict
with CSS: `try_background_color` stores canonical `Property::Background`, while
CSS `background` is a shorthand and `background-color` is the longhand. The
[Snapshot 2026 stable/reliable lists](https://www.w3.org/TR/css-2026/#css-official)
include these modules; representative contracts are
[Containment 1](https://www.w3.org/TR/css-contain-1/),
[Multi-column 1](https://www.w3.org/TR/css-multicol-1/), and
[Images 3](https://www.w3.org/TR/css-images-3/).

Impact: The crate cannot represent the complete in-boundary Snapshot 2026 style
contract even if every downstream layout/render implementation were present, and
some public names cannot map unambiguously to CSS properties.

Required remediation: Establish a source-backed Snapshot Sections 2.1-2.2
capability matrix, then supply every in-scope property, shorthand, descriptor,
rule, value, initial/inheritance entry, validation rule, and invalidation fact
without renaming standard CSS concepts into incompatible aliases.

## [Important] Stable selectors, namespaces, media queries, and conditional rules are incomplete

Location: `src/condition.rs:99-121`, `src/condition.rs:870-888`,
`src/condition.rs:1524-1548`, `src/selector.rs:705-854`,
`src/bucket.rs:3-22`, `src/tree.rs:25-35`

Evidence: `Condition` supports media and container conditions but no already-lowered
`@supports` capability model, although Conditional 3 is stable and reliable
Conditional 4 adds selector support queries. Media Queries 4 is missing `speech`,
aspect ratio, color gamut/index, update, overflow block/inline, scan, grid, and
other defined features. Selector matching lacks stable link/visited/target/language
facts and first-line/first-letter pseudo-elements; `Tree::Node` carries no
namespace, language, link, visited, or target facts, so Namespaces 3 and the
missing Selectors 3 behavior cannot be lowered into the matching boundary. See
[Media Queries Level 4](https://www.w3.org/TR/mediaqueries-4/),
[Selectors Level 3](https://www.w3.org/TR/selectors-3/), and the
[Snapshot 2026 classification](https://www.w3.org/TR/css-2026/).

Impact: Stable conditional and selector semantics cannot be preserved through the
style boundary, while reliable Media Queries 4 and Conditional 4 support is only
partial.

Required remediation: Add typed feature-support conditions, complete the MQ4 fact
and matching model, and extend tree/selector facts for complete stable selector and
namespace-aware matching.

## [Important] Selector matching forces owned fact copies and materialized tree walks

Location: `src/tree.rs:11-35`, `src/selector.rs:138-160`,
`src/selector.rs:527-579`, `src/selector.rs:886-888`,
`src/selector.rs:1014-1029`, `src/selector.rs:1578-1644`,
`src/sheet.rs:720-729`, `src/sheet.rs:765-784`

Evidence: `Tree::node` must return an owned `Node` containing owned class and
attribute vectors. Simple selectors request that owned node independently, and a
compound selector obtains one node before requesting it again for each state and
attribute selector. Rule candidate lookup materializes a `BTreeSet` and then a
`Vec`; relative, sibling, descendant, and `:has()` matching similarly collects
temporary vectors before matching, including recursive descendant collection.
The public tree contract gives an adapter no borrowed or query-oriented way to
serve facts without constructing the owned snapshot.

Impact: Selector matching copies node facts and allocates relationship collections
through the core per-node/per-rule path. Deep trees, broad sibling sets, relational
selectors, and the review's required expansion of selector facts multiply that
work and prevent short-circuiting before complete candidate collections are built.

Required remediation: Replace the allocation-forcing tree boundary with borrowed,
revision-scoped, or query-oriented selector facts, and expose relationship
iteration that permits matching to short-circuit without first materializing the
complete candidate set. Preserve deterministic candidate deduplication and order
without requiring a fresh ordered set per node. Add representative selector-scale
evidence proving allocation-free simple fact queries and early exit for successful
relative and `:has()` matches.

## [Important] The leaf owns a render-lowering adapter and dependency outside its boundary

Location: `Cargo.toml:15`, `src/value.rs:69-73`

Evidence: `peniko` is used only by `impl From<Color> for peniko::Color`. Repository
policy explicitly excludes render lowering and assigns cross-boundary adapters to
root.

Impact: The style public API and dependency graph are coupled to a rendering type
for a single conversion, violating the leaf/root ownership boundary.

Required remediation: Remove the dependency and conversion from this leaf and
place the conversion in its root-owned/render adapter boundary.

## [Minor] The public API and supported CSS contract are effectively undocumented

Location: `README.md:1-3`, `src/lib.rs:1-5`, `src/lib.rs:27-112`, nearly all public
items

Evidence: The README contains only a title and one sentence. The source exposes
approximately 1,620 public item/method declarations but contains only nine Rustdoc
lines, four of which document one counter type. The test gate runs zero doctests.
The supported module subset, authored/normalized/resolved phases, contextual
inputs, defaults, error semantics, and primary construction/resolution workflow
are not documented.

Impact: Downstream callers cannot determine invariants or supported CSS behavior
from the published front door, and maintainers cannot objectively distinguish a
deliberate extension from an incomplete standard feature.

Required remediation: Document the stable capability boundary and primary public
workflow with runnable examples, document every public invariant/error/panic
contract, and enforce public documentation coverage.

## [Minor] Safety and lint policy are not enforced by the crate source

Location: `src/lib.rs:1-6`; broad allowances at `src/authored.rs:163`,
`src/authored.rs:240-272`, `src/authored.rs:403`, `src/sheet.rs:237`,
`src/sheet.rs:358-367`, `src/resolver.rs:1467`

Evidence: The crate has no `#![forbid(unsafe_code)]`; the repository-wide absolute
prohibition currently depends on invoking the configured Clippy flags. Several
whole items and implementations use broad `#[allow(dead_code)]` without reasons.

Impact: Ordinary builds do not enforce the repository's safety invariant, and
broad suppressions can conceal abandoned or incompletely wired paths.

Required remediation: Add source-level `forbid(unsafe_code)`, remove unused paths,
and replace only genuinely necessary suppressions with narrow, reasoned
expectations.
