# CSS Surface Style Operations Sequence

Date: 2026-07-05

## Goal

Create the ordered style-crate work sequence needed to support the new
`surgeist-css` authored surface without turning style into a parser, root
adapter, layout engine, text shaper, or render backend.

This is not an implementation plan. It is the sequencing document from which
the crate-local implementation plans should be developed.

## Source Snapshot

- Style directive:
  `plans/2026-07-05-css-surface-style-support-directive.md`
- Style crate snapshot reviewed:
  `fcc42de2c32a318e073233dd51508dd4cc28041a`
- CSS crate snapshot reviewed read-only:
  `/Users/codex/Development/surgeist-css` at
  `1c95d4218439f1696151e0ee9602671fab418314`
- Modeling guide:
  `guidance/surgeist-rust-modeling-guide.md`

## Boundary Conclusions

Style should own typed receiving models, cascade inputs, selector/runtime fact
contracts, variable substitution semantics, inheritance, invalid-at-computed-
value behavior, invalidation, and computed style outputs.

Style should not depend on `surgeist-css` types. Root remains responsible for
lowering CSS-authored syntax into style front-door APIs. CSS remains the strict
parser and source-location owner.

No broad untyped escape hatch should be added to style. If a CSS property or
value cannot yet be represented as a typed style receiving model, the future
plan must choose one of:

- add an explicit style-owned model,
- preserve it as typed symbolic style data for a later owner, or
- require root to reject it with an unsupported-integration diagnostic.

## CSS API Surface Observed

The current CSS crate delivers a strict authored syntax tree:

- `CssSheet` with `CssRule` variants for `@import`, `@layer` statements and
  blocks, `@font-face`, `@keyframes`, style rules, media rules, container
  rules, and scope rules.
- `CssStyleRule` containing a `CssSelector` and `CssDeclaration` list.
- `CssDeclaration` containing `CssProperty`, `CssValue`, and
  `CssSourceLocation`.
- CSS-wide values through `CssValue::GlobalKeyword(CssGlobalKeyword)` with
  `inherit`, `initial`, `unset`, `revert`, and `revert-layer`.
- Custom property declarations through `CssProperty::Custom` and
  `CssValue::CustomProperty`.
- Variable-dependent ordinary declarations through
  `CssValue::VariableDependent`, preserving authored CSS and variable
  references.
- Selectors for tag, key, class, compound, complex, relative selector lists,
  attribute matchers, structural pseudo-classes, runtime pseudo-classes,
  selector-list pseudo-classes, and supported pseudo-element sequences.
- Supported pseudo-elements: `::before`, `::after`, `::marker`, `::selection`,
  `::backdrop`, plus `::before::marker` and `::after::marker`.
- Property families for layout, box model, grid, typography, generated content,
  colors, paint/effects, interaction, masks, transitions, animations,
  keyframes, font-face, imports, layers, scopes, media queries, and container
  queries.
- Symbolic authored color values for `currentColor`, system colors, modern
  color spaces, `color-mix(...)`, relative colors, and variable-dependent color
  expressions.

The current style crate owns a smaller canonical model:

- `Property`, `Value`, `Declaration`, `Declarations`, `Sheet`, `Rule`,
  `Selector`, `Tree`, `Resolver`, `Resolved`, and `Invalidation`.
- Style-owned tree facts for tags, keys, classes, attributes, roles, and state.
- Style-owned text enums and existing layout/text/paint/effect value models.
- Canonical shorthand expansion for selected style properties.
- Basic condition handling for viewport and container size checks.
- No style-owned cascade origin/layer/specificity model yet.
- No custom-property environment or `var(...)` computed-value substitution yet.
- No pseudo-element style bucket model yet.
- No direct `surgeist-css` dependency.

## Operation Sequence

### Operation 0: Freeze The Surface Contract

Produce a reviewed crate-local surface contract that lists every CSS property,
rule kind, selector feature, global keyword, symbolic value family, and
condition family from the CSS snapshot.

Output:

- A style-local inventory plan artifact mapping each CSS surface item to one
  of: typed style model, typed symbolic style data, root rejection, or out of
  style boundary.
- A stable source snapshot SHA for both `surgeist-style` and `surgeist-css`.
- A rule that future implementation plans must not import `surgeist-css` into
  style.

Why first:

- Later implementation plans need a shared ledger to avoid selective support
  that looks broad but silently drops parts of the CSS surface.

### Operation 1: Add Authored Style Receiving Phases

Introduce explicit style-owned authored input types before expanding computed
outputs. These types should represent declarations as root-lowered style data,
not as CSS parser syntax.

Required concepts:

- authored style sheet or rule bundle
- authored rule metadata: source order, specificity, cascade origin if needed,
  layer identity/order, scope order, and applicable condition handles
- authored declaration entries that can carry ordinary typed property values,
  CSS-wide keywords, custom properties, and variable-dependent values
- source diagnostic handles that root can map back to CSS locations without
  style depending on CSS source-location types

Future plan boundary:

- Do not replace existing `Declaration`/`Declarations` until the new authored
  phase can lower into the existing canonical declaration store.
- Do not add raw strings except inside typed symbolic payloads that preserve
  an authored expression intentionally.

### Operation 2: Model Cascade Precedence Inputs

Extend style sheet/rule ordering so resolver behavior can account for the CSS
cascade dimensions supplied by root.

Required concepts:

- specificity
- source order
- layer order and unlayered ordering
- scoped rule order
- conditional rule applicability inputs
- `revert` and `revert-layer` bookkeeping hooks

Future plan boundary:

- This operation should update ordering and metadata, not property families.
- Existing simple rule order must remain explainable as the degenerate case.

### Operation 3: Model CSS-Wide Keyword Resolution

Replace the current broad `Keyword` treatment with property-aware CSS-wide
semantics.

Required behavior:

- `inherit`
- `initial`
- `unset`
- `revert`
- `revert-layer`
- inherited versus non-inherited defaults from `Property::metadata`
- property-aware shorthand behavior where CSS-wide keywords apply to all
  expanded longhands

Future plan boundary:

- Do this before custom properties because `var(...)` fallbacks can produce
  CSS-wide keywords at computed-value time.

### Operation 4: Add Custom Property And Variable Substitution Model

Add a style-owned model for custom property storage, inheritance,
substitution, fallback handling, dependency tracking, and invalid-at-computed-
value behavior.

Required concepts:

- custom property names and authored token payloads as style-owned types
- variable references with recursive fallback structure
- inherited custom property environment
- cycle detection
- invalid-at-computed-value result path
- invalidation when a custom property changes
- a policy for declarations whose ordinary value is variable-dependent

Future plan boundary:

- Root may lower CSS variable syntax into these style-owned types, but style
  owns the semantics after receipt.
- Do not parse CSS syntax in style.

### Operation 5: Expand Selector And Tree Matching Contracts

Extend style selector matching to cover the selector features the CSS crate can
produce.

Required concepts:

- selector lists
- specificity calculation inputs or precomputed specificity receipt
- compound and complex selectors with descendant, child, adjacent, and sibling
  combinators
- attribute matchers: exists, equals, includes, dash-match, prefix, suffix,
  substring, and case sensitivity
- structural selectors: first/last/only child, empty, nth-child, nth-last-child,
  first/last/only/nth-of-type, and selector-list filtered nth-child
- selector-list pseudo-classes: `:not`, `:is`, `:where`
- relative selectors and `:has`
- runtime pseudo-classes backed by style-owned tree/runtime facts
- `:root` and `:scope`

Future plan boundary:

- This operation should make unsupported selector facts impossible to fake.
- Runtime pseudo-classes should be explicit style-owned state facts, not
  retained or DOM concepts.

### Operation 6: Add Pseudo-Element Style Buckets

Represent pseudo-element style as buckets associated with an originating tree
node, not as additional DSL-addressable tree nodes.

Required buckets:

- element style
- `::before`
- `::after`
- `::marker`
- `::selection`
- `::backdrop`
- marker buckets nested under before/after where CSS supports the sequence

Future plan boundary:

- Resolver and cache keys must include the requested style bucket.
- Pseudo-element generated content policy belongs in style, while actual tree
  or render materialization remains outside style.

### Operation 7: Build The Property Coverage Ledger

Create a style-owned property ledger that maps each CSS `CssProperty` to
style behavior.

Required classification for every CSS property:

- existing style property and value model
- new style property needed
- shorthand that lowers into existing or new longhands
- symbolic property retained for another owner
- root rejection required
- out of style boundary

The first ledger should cover these families:

- layout and box model
- flex, grid, alignment, writing mode, visibility, aspect ratio, z-index,
  scrollbar width, and content visibility
- text and font-facing properties
- generated content, list style, counters, and marker styling
- color, background, borders, outlines, shadows, opacity, transforms, filters,
  clip path, masks, cursor, pointer events, and user select
- transitions, animations, easing, durations, delays, iteration, direction,
  fill mode, play state, and keyframe references
- font-face and import rules as root-owned loading boundaries with style-owned
  descriptors only where style needs them for computed output

Future plan boundary:

- This is a planning operation, not a large code implementation. It should
  split the remaining work into property-family implementation plans.

### Operation 8: Implement Layout-Facing Computed Property Families

Add typed style receiving and computed output models for layout-facing
properties.

Priority families:

- display, box sizing, position, overflow
- width, height, min/max sizing, aspect ratio
- margin, padding, inset, border widths, gap
- flex direction/wrap/grow/shrink/basis/order
- grid tracks, areas, placement, auto-flow, flow tolerance
- alignment and writing mode
- visibility, z-index, scrollbar width, content visibility

Future plan boundary:

- Preserve symbolic lengths and calc values until the owner with the right
  basis resolves them.
- Do not add a style-to-layout adapter in this crate.

### Operation 9: Implement Text-Facing Computed Property Families

Add typed style receiving and computed output models for text-facing
properties.

Priority families:

- font family and font shorthand
- font size, line height, weight, slant/style, stretch/width
- font variant and feature settings
- letter spacing
- text alignment and text-align-last
- text indent and vertical alignment
- text wrap, white space, word break, overflow wrap, text overflow
- decoration line/color/style/thickness
- text transform

Future plan boundary:

- Style owns authored/resolved text style values; final shaping, font loading,
  and text layout remain outside style.

### Operation 10: Implement Paint, Color, And Effect Families

Add typed style receiving and computed output models for paint/effect
properties.

Priority families:

- `color`, `currentColor`, and symbolic color propagation
- background color/image/position/size/repeat/origin/clip/attachment
- border color/style/radii and outline color/style/width
- box shadow and opacity
- transform, transform origin, translate, rotate, scale
- filter and backdrop filter
- clip path
- mask image/size/position/repeat
- cursor, pointer events, and user select

Future plan boundary:

- Modern color spaces, system colors, `color-mix(...)`, relative colors, and
  variable-dependent colors should stay symbolic unless style has the context
  to resolve them correctly.

### Operation 11: Implement Generated Content And Counter Families

Add typed style data for generated content without making pseudo-elements into
tree nodes.

Required concepts:

- `content`
- list style type/position/image/shorthand
- counter reset/increment/set
- counter and counters functions
- marker styling policy
- quote and attr generated-content payloads

Future plan boundary:

- Style can own computed generated-content data and policy. Actual retained
  projection or render materialization remains outside style.

### Operation 12: Implement Timing, Animation, And Keyframe Style Data

Add typed style receiving and computed output models for timing and animation
properties.

Required concepts:

- transition property/duration/delay/timing function/shorthand
- animation name/duration/delay/timing function/iteration/direction/fill
  mode/play state/shorthand
- keyframe references and keyframe declaration blocks
- symbolic easing function payloads where style should not evaluate the curve

Future plan boundary:

- Style owns style data and invalidation impact. Runtime animation scheduling
  belongs outside style.

### Operation 13: Expand Conditions For Media, Container, Scope, And Layers

Replace the current basic `Condition::Viewport`/`Condition::Container` shape
with style-owned condition inputs that can represent the CSS query surface root
lowers.

Required concepts:

- media feature facts needed by currently supported CSS queries
- container feature facts
- style query facts if root decides to support them through style
- scope anchors and scope boundaries
- layer statements and nested layer blocks

Future plan boundary:

- Network and filesystem handling for `@import` remains out of style.
- Host/environment facts are inputs to style; style should not query the host.

### Operation 14: Update Resolver, Cache Keys, And Invalidation

Make resolver and invalidation account for the new authored phase, cascade
dimensions, variables, pseudo-element buckets, selectors, and condition facts.

Required concepts:

- cache keys include sheet version, tree version, node identity, style bucket,
  traversal, condition inputs, custom property environment dependencies, and
  local/animated declarations
- invalidation can distinguish property changes, custom property dependency
  changes, selector fact changes, condition changes, and pseudo-element bucket
  changes
- resolver can report invalid-at-computed-value failures in a way root can
  associate with source diagnostics

Future plan boundary:

- This operation should follow the property and selector model operations so
  cache keys are not guessed ahead of the model.

### Operation 15: Add API Artifacts, Compile-Fail Tests, And Root Handoff Notes

For each implementation plan produced from this sequence, keep the public API
intentional and reviewable.

Required checks:

- update or regenerate any crate-owned API artifact if present in the plan
- add compile-fail tests for public constructor invariants
- add crate-local unit tests for cascade, variables, selector matching,
  pseudo-element buckets, property validation, and resolver output
- maintain the search invariant that `surgeist-style` does not depend on
  `surgeist-css`
- report root handoff notes for any new style front-door APIs root must lower
  into

Final checks for future implementation plans:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
git status --short --branch
```

## Recommended Implementation Plan Slices

Develop the sequence into implementation plans in this order:

1. CSS surface inventory and style behavior ledger.
2. Authored declarations, cascade metadata, and CSS-wide keywords.
3. Custom properties and variable substitution.
4. Selector, runtime fact, and pseudo-element bucket expansion.
5. Layout-facing property family expansion.
6. Text-facing property family expansion.
7. Paint, color, and effect property family expansion.
8. Generated content, counters, lists, and marker policy.
9. Timing, animation, and keyframe style data.
10. Conditions, layers, scope, resolver cache, and invalidation integration.

Each slice should have its own worker/reviewer execution plan before code work
begins. Prefer small logical commits per slice, with root handoff notes whenever
root must lower new CSS-authored syntax into style-owned APIs.

## Coordination Questions For Root

Before implementation plans begin, root should confirm:

1. Whether style should model cascade origin now, or whether root will only
   provide author-origin rules for the first CSS integration pass.
2. Which CSS properties root intends to reject initially despite being parsed
   by `surgeist-css`.
3. Whether root wants style to expose unsupported-integration diagnostics
   directly, or whether root will convert style validation failures into
   integration diagnostics.
4. Which media/container/environment facts root can provide to style in the
   first pass.
5. Whether font-face descriptors should become style-owned symbolic data now,
   or remain entirely root/text-owned until font loading exists.

## Self-Review

- The sequence keeps CSS parsing and root-owned lowering out of style.
- The sequence does not add compatibility aliases, broad raw value bags, or an
  extra downstream adapter layer.
- Variable substitution appears before property-family expansion because it can
  delay validation for any supported property.
- Selector and pseudo-element work appears before full resolver/cache updates
  so cache keys can be based on real style-owned models.
- Property-family expansion is deliberately split to keep future worker and
  reviewer cycles scoped.
