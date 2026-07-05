# CSS Surface Style Support Directive

Date: 2026-07-05

Style should plan to support the new `surgeist-css` authored surface where the
boundary is genuinely style-owned. CSS remains the strict parser. Root remains
the CSS-to-Surgeist lowering and integration boundary. Style should expose
typed receiving models, resolution rules, inheritance behavior, invalidation
behavior, and computed style outputs.

## Style-Owned CSS Surface

Style should support:

- property cascade, specificity, source order, layers, scoped rule order, and
  conditional rule applicability inputs
- custom properties, inheritance, `var(...)` substitution, fallbacks, and
  invalid-at-computed-value behavior
- CSS-wide keywords: `inherit`, `initial`, `unset`, `revert`, `revert-layer`
- selector matching contracts over root-provided tree/runtime facts, including
  compound, complex, relative, attribute, structural, nth, selector-list, and
  runtime pseudo-class selectors
- pseudo-element style buckets for `::before`, `::after`, `::marker`,
  `::selection`, and `::backdrop`, without requiring pseudo-elements to become
  DSL-addressable tree nodes
- layout-facing computed properties: display, box sizing, position, overflow,
  sizing, spacing, flex, grid, alignment, writing mode, visibility, aspect
  ratio, z-index, scrollbar width, and content visibility
- text-facing computed properties: font family, font shorthand, font size,
  line height, weight, style/slant, stretch/width, variant, feature settings,
  letter spacing, text alignment, indent, vertical alignment, wrapping,
  whitespace, word breaking, overflow wrapping, text overflow, decoration, and
  transform
- paint/effect computed properties: color, background, border color/style/radii,
  box shadow, opacity, outline, transform, filter, backdrop filter, clip path,
  mask, cursor, pointer events, and user select
- generated-content style data for `content`, list styles, counters, marker
  styling, and pseudo-element content generation policy
- timing style data for transitions, animations, easing, durations, delays,
  iteration, direction, fill mode, play state, and keyframe references
- symbolic color values until style has enough context to resolve them:
  `currentColor`, system colors, modern color spaces, `color-mix(...)`,
  relative colors, and variable-dependent colors

## Out Of Style Boundary

Style should not own:

- CSS parsing or CSS syntax recovery
- root-owned lowering from `surgeist-css` types into style front-door APIs
- DOM/template construction
- retained identity allocation
- final text shaping or font loading
- final layout algorithms
- final render backend draw data
- window host behavior
- network or filesystem loading for `@import` and `@font-face`

## Planning Bar

Future style plans should treat every `surgeist-css` rule, property, selector,
and value as either:

- supported by a typed style receiving model,
- intentionally passed through as symbolic style data for a later owner, or
- rejected by root before reaching style with a clear unsupported-integration
  diagnostic.

Do not add broad untyped escape hatches to make the surface appear supported.
