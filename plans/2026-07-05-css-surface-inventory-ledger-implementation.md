# CSS Surface Inventory Ledger Implementation Plan

> **For agentic workers:** Execute this plan through the local `AGENTS.md` coordinator workflow. Workers follow the checkbox steps, do not commit, do not create branches, and do not let any external workflow guidance override the crate's worker/reviewer gate.

**Goal:** Create a source-derived ledger that maps the current `surgeist-css` authored surface to explicit `surgeist-style` ownership decisions before any CSS surface code migration begins.

**Architecture:** This plan is documentation-only and establishes the contract for later implementation plans. The ledger must be derived from the current CSS and style source snapshots, keep `surgeist-style` independent from `surgeist-css`, and classify every CSS rule, property, selector feature, value family, symbolic value, and condition family as style-owned typed model, typed symbolic style data, root rejection, or out of style boundary.

**Tech Stack:** Markdown plans in `plans/`, Rust source inspection with `rg`/`sed`, `surgeist-style` at the current repo checkout, read-only `surgeist-css` at `/Users/codex/Development/surgeist-css`, and `guidance/surgeist-rust-modeling-guide.md`.

---

## Coordinator Workflow Note

The coordinator executes this plan through the `AGENTS.md` worker/reviewer
gate. Assigned workers should follow the task checklist, must not commit, and
must not spawn their own implementation workers unless the coordinator
explicitly asks for that extra delegation.

## Source Context

Read these files before executing any task:

- `AGENTS.md`
- `README.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-support-directive.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `/Users/codex/Development/surgeist-css/src/lib.rs`
- `/Users/codex/Development/surgeist-css/src/syntax.rs`
- `/Users/codex/Development/surgeist-css/src/validation.rs`
- `/Users/codex/Development/surgeist-css/src/parser/mod.rs`
- `/Users/codex/Development/surgeist-css/src/parser/selectors.rs`
- `/Users/codex/Development/surgeist-css/src/parser/queries.rs`
- `/Users/codex/Development/surgeist-css/src/parser/variables.rs`
- `/Users/codex/Development/surgeist-css/src/parser/values.rs`
- `src/lib.rs`
- `src/calc.rs`
- `src/declaration.rs`
- `src/property.rs`
- `src/value.rs`
- `src/selector.rs`
- `src/sheet.rs`
- `src/resolver.rs`
- `src/tree.rs`
- `src/identity.rs`
- `src/state.rs`
- `src/condition.rs`
- `src/invalidation.rs`

## Expected File Structure

- Create: `plans/2026-07-05-css-surface-style-ledger.md`
  - The source-derived surface ledger produced by this plan.
- Modify existing files: none.
  - This first implementation plan creates and then modifies only the new ledger file. It must not change Rust source or existing planning files. If source inspection reveals a style source bug, stop and report it as a blocker rather than fixing it in this plan.

## Execution Requirements

- The coordinator must execute each task through a task-scoped worker/reviewer
  cycle.
- For AGENTS.md commit cadence, this entire plan is one scoped planning
  artifact because it creates one ledger file. The numbered tasks are checklist
  sections within that scoped artifact, not separate commit points.
- Workers must not commit.
- Reviewers must be separate from workers.
- After Tasks 1 through 5 are complete, the coordinator must assign a final
  holistic reviewer over the complete ledger before committing.
- The coordinator owns the final commit.

## Non-Goals

- Do not edit `/Users/codex/Development/surgeist-css`.
- Do not add `surgeist-css` as a dependency.
- Do not implement style models, adapters, lowering, cascade behavior, selectors, variables, or resolver behavior.
- Do not create broad untyped style escape hatches.
- Do not update root `surgeist` submodule pointers.
- Do not answer root coordination questions by guessing. Record open questions in the ledger.

## Classification Vocabulary

Use exactly these classification labels in the ledger:

- `Existing style model`
- `New style model needed`
- `Typed symbolic style data`
- `Root rejection`
- `Out of style boundary`
- `Root-owned lowering boundary`

Use exactly these owner labels:

- `style`
- `root`
- `css`
- `layout`
- `text`
- `render`
- `retained`
- `window`
- `host`

## Task 1: Capture Source Snapshot And Ledger Skeleton

**Files:**

- Create: `plans/2026-07-05-css-surface-style-ledger.md`
- Inspect: `Cargo.toml`
- Inspect: `/Users/codex/Development/surgeist-css/Cargo.toml`
- Inspect: `plans/2026-07-05-css-surface-style-operations-sequence.md`

- [ ] **Step 1: Confirm crate status and snapshots**

Run from `/Users/codex/Development/surgeist-style`:

```sh
git status --short --branch
git rev-parse HEAD
sed -n '1,40p' Cargo.toml
git -C /Users/codex/Development/surgeist-css status --short --branch
git -C /Users/codex/Development/surgeist-css rev-parse HEAD
sed -n '1,40p' /Users/codex/Development/surgeist-css/Cargo.toml
```

Expected:

- Style status is clean except for coordinator-approved plan work.
- CSS status is clean, because this plan uses CSS read-only.
- Style crate name is `surgeist-style`.
- CSS crate name is `surgeist-css`.

- [ ] **Step 2: Create the ledger file with the required skeleton**

Create `plans/2026-07-05-css-surface-style-ledger.md` with this initial structure:

```markdown
# CSS Surface Style Ledger

Date: 2026-07-05

## Purpose

This ledger maps the current `surgeist-css` authored API surface to explicit
`surgeist-style` ownership decisions. It is the source contract for later
style implementation plans and root lowering work.

## Source Snapshot

| Repo | Path | Commit | Status |
| --- | --- | --- | --- |
| style | `/Users/codex/Development/surgeist-style` | `<STYLE_SHA>` | `<STYLE_STATUS_SUMMARY>` |
| css | `/Users/codex/Development/surgeist-css` | `<CSS_SHA>` | `<CSS_STATUS_SUMMARY>` |

## Boundary Rules

- `surgeist-css` owns strict parsing, CSS syntax recovery policy, CSS source
  locations, and authored CSS syntax types.
- Root owns lowering from `surgeist-css` types into style front-door APIs and
  integration diagnostics for unsupported CSS.
- `surgeist-style` owns typed receiving models, cascade semantics, selector
  matching contracts over root-provided facts, inheritance, invalidation, and
  computed style outputs.
- `surgeist-style` must not depend on `surgeist-css`.
- Every CSS surface item in this ledger is classified as `Existing style model`,
  `New style model needed`, `Typed symbolic style data`, `Root rejection`,
  `Out of style boundary`, or `Root-owned lowering boundary`.

## Classification Labels

| Label | Meaning |
| --- | --- |
| `Existing style model` | Style already has a typed model or behavior that can receive this surface intentionally. |
| `New style model needed` | Style owns the semantic domain, but a new typed model or behavior must be added. |
| `Typed symbolic style data` | Style should preserve typed symbolic data until a later owner or context can resolve it. |
| `Root rejection` | Root should reject this parsed CSS surface for now with an unsupported-integration diagnostic. |
| `Out of style boundary` | The surface is parsed by CSS but belongs to another crate or host concern, not style. |
| `Root-owned lowering boundary` | Root must translate this CSS syntax into style-owned inputs; style must not import CSS types. |

## CSS Rule Ledger

| CSS rule | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## CSS Declaration And Value Family Ledger

| CSS value family | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## CSS-Wide Keyword Ledger

| CSS-wide keyword | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## CSS Property Ledger

| CSS property | CSS value family | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- | --- |

## Selector And Tree Fact Ledger

| CSS selector surface | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## Condition, Layer, Scope, And Environment Ledger

| CSS surface | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## Symbolic Data Ledger

| Symbolic surface | Classification | Owner | Style implication | Later plan |
| --- | --- | --- | --- | --- |

## Current Style Surface Summary

| Style surface | Current role | Gap against CSS surface |
| --- | --- | --- |

## Coverage Audit

| Surface | Expected source count or minimum | Ledger row count | Audit result |
| --- | --- | --- | --- |

## Root Coordination Questions

1. Does root want style to model cascade origin in the first authored-style pass, or only author-origin rules?
2. Which parsed CSS properties should root reject initially even though `surgeist-css` accepts them?
3. Should style expose unsupported-integration diagnostics directly, or should root convert style validation failures into integration diagnostics?
4. Which media, container, and environment facts can root provide to style in the first integration pass?
5. Should `@font-face` descriptors become style-owned symbolic data now, or remain root/text-owned until font loading exists?

## Next Sequence Context

The next implementation plan should cover authored declarations, cascade
metadata, and CSS-wide keywords. It should consume this ledger rather than
re-inspecting the entire CSS surface from scratch, then rebase the plan on any
ledger corrections found during review.
```

Replace `<STYLE_SHA>` and `<CSS_SHA>` with the exact commit SHAs from Step 1.
Replace `<STYLE_STATUS_SUMMARY>` and `<CSS_STATUS_SUMMARY>` with single-line
status summaries such as `clean`, `ahead 1 with ledger plan work`, or
`dirty: <short reason>`. Do not paste multi-line status output into the table,
and do not leave angle-bracket placeholders in the committed ledger.

- [ ] **Step 3: Verify the skeleton has no placeholders**

Run:

```sh
rg -n "<STYLE_SHA>|<STYLE_STATUS_SUMMARY>|<CSS_SHA>|<CSS_STATUS_SUMMARY>|TBD|TODO|fill in|unknown commit|unknown status|\\?\\?\\?" plans/2026-07-05-css-surface-style-ledger.md
```

Expected: no output.

## Task 2: Inventory CSS Rules, Declarations, Values, And Properties

**Files:**

- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Inspect: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/parser/mod.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/validation.rs`
- Inspect: `src/property.rs`
- Inspect: `src/value.rs`

- [ ] **Step 1: Extract CSS rule variants**

Run:

```sh
sed -n '37,48p' /Users/codex/Development/surgeist-css/src/syntax.rs
```

Expected CSS rules to classify:

```text
Import
LayerStatement
LayerBlock
FontFace
Keyframes
Style
Media
Container
Scope
```

Add one row for each rule in `## CSS Rule Ledger`.

Required classifications:

- `Style`: `Root-owned lowering boundary`, owner `root`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `Media`: `New style model needed`, owner `style`, later plan `Conditions, layers, scope, resolver cache, and invalidation integration`.
- `Container`: `New style model needed`, owner `style`, later plan `Conditions, layers, scope, resolver cache, and invalidation integration`.
- `Scope`: `New style model needed`, owner `style`, later plan `Conditions, layers, scope, resolver cache, and invalidation integration`.
- `LayerStatement`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `LayerBlock`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `Keyframes`: `Typed symbolic style data`, owner `style`, later plan `Timing, animation, and keyframe style data`.
- `FontFace`: `Out of style boundary`, owner `text`, later plan `CSS surface inventory follow-up/root decision`.
- `Import`: `Out of style boundary`, owner `root`, later plan `CSS surface inventory follow-up/root decision`.

- [ ] **Step 2: Extract CSS value families**

Run:

```sh
sed -n '2385,2495p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '382,520p' src/value.rs
sed -n '423,760p' src/property.rs
```

Add one row in `## CSS Declaration And Value Family Ledger` for every
`CssValue` variant in that enum.

Use these required family-level classifications:

- `GlobalKeyword`: `New style model needed`, owner `style`.
- `CustomProperty`: `New style model needed`, owner `style`.
- `VariableDependent`: `New style model needed`, owner `style`.
- Direct layout/text/paint/timing typed variants: classify as `Existing style model` only when a style-owned `Value` variant and `Property::validate_value` path already exist; otherwise classify as `New style model needed`.
- Authored function argument containers for transform/filter/easing/basic shape: classify as `Typed symbolic style data`.
- Rule or host loading surfaces that style should not own: classify as `Out of style boundary`.

- [ ] **Step 3: Extract CSS-wide keyword variants**

Run:

```sh
sed -n '2376,2384p' /Users/codex/Development/surgeist-css/src/syntax.rs
rg -n "parse_global_keyword|revert-layer|revert\\)|unset\\)|initial\\)|inherit\\)" /Users/codex/Development/surgeist-css/src/validation.rs
```

Expected CSS-wide keywords to classify:

```text
Inherit
Initial
Unset
Revert
RevertLayer
```

Add one row in `## CSS-Wide Keyword Ledger` for each keyword.

Required classifications:

- `Inherit`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `Initial`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `Unset`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `Revert`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.
- `RevertLayer`: `New style model needed`, owner `style`, later plan `Authored declarations, cascade metadata, and CSS-wide keywords`.

Each row must explain that root lowers the parsed CSS keyword into a
style-owned CSS-wide keyword receiving model, while style owns property-aware
resolution against inherited values, initial values, cascade origin, and layer
state.

- [ ] **Step 4: Extract CSS property variants**

Run:

```sh
sed -n '2001,2183p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '382,520p' src/value.rs
sed -n '423,760p' src/property.rs
```

Add one row in `## CSS Property Ledger` for every `CssProperty` variant,
including `Custom(CssCustomPropertyName)`.

For each row, set `CSS value family` from the parser mapping in:

```sh
sed -n '707,737p' /Users/codex/Development/surgeist-css/src/parser/mod.rs
sed -n '734,1048p' /Users/codex/Development/surgeist-css/src/parser/mod.rs
sed -n '1,60p' /Users/codex/Development/surgeist-css/src/parser/variables.rs
```

Required classification rules:

- `All` must be `New style model needed`; it is CSS-wide keyword only and has no current style-wide cascade model.
- `Custom(CssCustomPropertyName)` must be `New style model needed` with CSS value family `CssValue::CustomProperty`.
- Ordinary supported properties whose authored values contain `var(...)` must note the `CssValue::VariableDependent` receiving path as `New style model needed`, even when the non-variable form maps to an existing style value.
- A shorthand parsed by CSS must be classified according to the style longhands it needs, not as supported merely because style has a similarly named canonical shorthand.
- Properties that currently map to `Value::Keyword(Keyword::Initial)` in style metadata but lack a typed value model must be `New style model needed`, not `Existing style model`.
- `FontFace` descriptors are not `CssProperty` variants; do not add them to this table.
- `@import` is not a `CssProperty`; keep it only in the rule ledger.

- [ ] **Step 5: Cross-check supported property names**

Run:

```sh
rg -n "supported_property!" /Users/codex/Development/surgeist-css/src/validation.rs
```

Verify that every `supported_property!` entry corresponds to a row in
`## CSS Property Ledger`. If a supported property name maps to a `CssProperty`
variant that was missed, add the missing row before continuing. The output
must include the final animation entries through `supported_property!("animation", Animation)`.

## Task 3: Inventory Selectors, Runtime Facts, Conditions, And Symbolic Data

**Files:**

- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Inspect: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/parser/selectors.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/parser/queries.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/parser/variables.rs`
- Inspect: `/Users/codex/Development/surgeist-css/src/parser/values.rs`
- Inspect: `src/selector.rs`
- Inspect: `src/tree.rs`
- Inspect: `src/identity.rs`
- Inspect: `src/state.rs`
- Inspect: `src/condition.rs`

- [ ] **Step 1: Extract selector API types**

Run:

```sh
sed -n '7285,8010p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '1,260p' src/selector.rs
sed -n '1,220p' src/tree.rs
sed -n '1,260p' src/identity.rs
sed -n '1,220p' src/state.rs
```

Add one row in `## Selector And Tree Fact Ledger` for each item named below,
not merely one row per bullet group:

- tag selectors
- key selectors
- class selectors
- compound selectors
- complex selectors
- descendant, child, adjacent, and subsequent sibling combinators
- selector lists
- attribute exists/equality/includes/dash-match/prefix/suffix/substring
- attribute case sensitivity
- `:root`
- `:scope`
- runtime pseudo-classes: hover, active, focus, focus-visible, focus-within, disabled, enabled, checked, required, optional, valid, invalid, placeholder-shown, modal, fullscreen, popover-open, default, indeterminate, read-only, read-write, in-range, out-of-range
- structural pseudo-classes: first-child, last-child, only-child, empty, nth-child, nth-last-child, first-of-type, last-of-type, only-of-type, nth-of-type, nth-last-of-type
- selector-list pseudo-classes: not, is, where
- relative selectors and `:has`
- pseudo-elements: before, after, marker, selection, backdrop, before-marker, after-marker

Expected selector ledger row minimum: 65 rows.

Required classification rules:

- Existing style `Selector::Tag`, `Selector::Class`, `Selector::Key`, simple `Compound`, simple `Complex`, `AttributeSelector::Exists`, `AttributeSelector::Equals`, and simple `PositionSelector` rows may be `Existing style model` when the row explicitly notes the missing CSS-specific semantics.
- Unsupported attribute matchers and case sensitivity must be `New style model needed`.
- Runtime pseudo-classes must be `New style model needed` unless `StyleState`/`StateFlag` already has the exact fact.
- Pseudo-elements must be `New style model needed`; do not model them as tree nodes.

- [ ] **Step 2: Extract media and container query types**

Run:

```sh
sed -n '1435,1901p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '1,817p' /Users/codex/Development/surgeist-css/src/parser/queries.rs
sed -n '1,260p' src/condition.rs
```

Add one row in `## Condition, Layer, Scope, And Environment Ledger` for each
item named below, not merely one row per bullet group:

- media query list
- typed media query
- condition-only media query
- media type
- media modifier `not`
- media modifier `only`
- typed media query `and` condition attachment
- media condition list
- media condition `not`
- media condition `and`
- media condition `or`
- media feature queries
- container names
- container condition lists
- container condition `not`
- container condition `and`
- container condition `or`
- container feature queries
- container style queries
- container style custom property presence query
- container style custom property value query
- container width
- container height
- container inline-size
- container block-size
- container aspect-ratio
- container orientation
- media width
- media height
- media resolution
- media color
- media monochrome
- media orientation
- prefers-color-scheme
- prefers-reduced-motion
- prefers-reduced-transparency
- prefers-contrast
- forced-colors
- hover
- any-hover
- pointer
- any-pointer
- display-mode
- range feature without comparison
- range comparison less-than
- range comparison less-than-or-equal
- range comparison equal
- range comparison greater-than-or-equal
- range comparison greater-than
- scoped rule order
- layer order
- unlayered order

Expected condition/layer/scope/environment ledger row minimum: 52 rows.

Classify current `Viewport` and `Container` width/height facts as `Existing style model` only for the current min/max width and height subset. Everything else in this list should be `New style model needed` or `Out of style boundary` according to the owner.

- [ ] **Step 3: Extract symbolic variable and color data**

Run:

```sh
sed -n '2185,2385p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '6571,7284p' /Users/codex/Development/surgeist-css/src/syntax.rs
sed -n '1,140p' /Users/codex/Development/surgeist-css/src/parser/variables.rs
sed -n '462,820p' /Users/codex/Development/surgeist-css/src/parser/values.rs
```

Add rows in `## Symbolic Data Ledger` for:

- custom property names
- custom property authored values
- variable references
- variable fallbacks
- variable-dependent ordinary declaration values
- currentColor
- system colors
- hsl, hwb, lab, lch, oklab, oklch
- color function with predefined color spaces
- color-mix
- relative colors
- color component expressions with variable references
- transform function arguments
- filter function arguments
- basic shape arguments
- easing function arguments

Required classification rules:

- Custom properties and `var(...)` structures are `New style model needed`.
- Modern color and function families that style cannot concretely resolve yet are `Typed symbolic style data`.
- Do not collapse symbolic colors to RGBA in the ledger.

## Task 4: Inventory Current Style Surface And Record Gaps

**Files:**

- Modify: `plans/2026-07-05-css-surface-style-ledger.md`
- Inspect: `src/lib.rs`
- Inspect: `src/calc.rs`
- Inspect: `src/property.rs`
- Inspect: `src/value.rs`
- Inspect: `src/declaration.rs`
- Inspect: `src/sheet.rs`
- Inspect: `src/selector.rs`
- Inspect: `src/tree.rs`
- Inspect: `src/condition.rs`
- Inspect: `src/resolver.rs`
- Inspect: `src/invalidation.rs`

- [ ] **Step 1: Extract current style public surface**

Run:

```sh
sed -n '1,140p' src/lib.rs
sed -n '1,220p' src/calc.rs
sed -n '1,220p' src/property.rs
sed -n '1,220p' src/value.rs
sed -n '382,520p' src/value.rs
sed -n '1900,2100p' src/value.rs
sed -n '1,180p' src/declaration.rs
sed -n '1,260p' src/sheet.rs
sed -n '1,260p' src/selector.rs
sed -n '1,220p' src/tree.rs
sed -n '1,260p' src/condition.rs
sed -n '1,230p' src/resolver.rs
sed -n '1,120p' src/invalidation.rs
```

Add rows in `## Current Style Surface Summary` for:

- `Property`
- `Value`
- `Declaration`
- `Declarations`
- `TypedDeclaration`
- `Sheet`
- `Rule`
- `Selector`
- `Tree`
- `Context`
- `Resolver`
- `Resolved`
- `Invalidation`
- `Change`
- `Condition`
- `Viewport`
- `Container`
- `Color`
- `Length`
- `CalcLength`
- `TextValue`
- `Transform`

For each row, state the current role and the specific gap against the CSS surface.

- [ ] **Step 2: Confirm style has no CSS dependency**

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests plans
```

Expected output may include references in planning files only. If `Cargo.toml`,
`src`, or `tests` references `surgeist-css`, stop and report a blocker.

- [ ] **Step 3: Add root coordination notes**

Update `## Root Coordination Questions` with any additional questions that arise while filling the ledger. Each question must name the affected CSS surface and the later style plan it blocks or informs.

## Task 5: Coverage Audit And Final Checks

**Files:**

- Verify: `plans/2026-07-05-css-surface-style-ledger.md`

- [ ] **Step 1: Check required sections exist**

Run:

```sh
rg -n "^## (CSS Rule Ledger|CSS Declaration And Value Family Ledger|CSS-Wide Keyword Ledger|CSS Property Ledger|Selector And Tree Fact Ledger|Condition, Layer, Scope, And Environment Ledger|Symbolic Data Ledger|Current Style Surface Summary|Coverage Audit|Root Coordination Questions|Next Sequence Context)$" plans/2026-07-05-css-surface-style-ledger.md
```

Expected: one match for each required section.

- [ ] **Step 2: Populate and verify the coverage audit**

Run:

```sh
printf "CssRule source count: "
perl -0ne 'if (/pub enum CssRule \{(.*?)\n\}/s) { my $body = $1; while ($body =~ /^\s+[A-Z]/mg) { $count++ } print "$count\n" }' /Users/codex/Development/surgeist-css/src/syntax.rs
printf "CssValue source count: "
perl -0ne 'if (/pub enum CssValue \{(.*?)\n\}/s) { my $body = $1; while ($body =~ /^\s+[A-Z]/mg) { $count++ } print "$count\n" }' /Users/codex/Development/surgeist-css/src/syntax.rs
printf "CssGlobalKeyword source count: "
perl -0ne 'if (/pub enum CssGlobalKeyword \{(.*?)\n\}/s) { my $body = $1; while ($body =~ /^\s+[A-Z]/mg) { $count++ } print "$count\n" }' /Users/codex/Development/surgeist-css/src/syntax.rs
printf "CssProperty source count: "
perl -0ne 'if (/pub enum CssProperty \{(.*?)\n\}/s) { my $body = $1; while ($body =~ /^\s+[A-Z]/mg) { $count++ } print "$count\n" }' /Users/codex/Development/surgeist-css/src/syntax.rs

ledger="plans/2026-07-05-css-surface-style-ledger.md"
count_ledger_rows() {
  section="$1"
  awk -v section="$section" '
    $0 == "## " section { in_section = 1; next }
    in_section && /^## / { in_section = 0 }
    in_section && /^\|/ && $0 !~ /\| ---/ { rows++ }
    END { print rows > 0 ? rows - 1 : 0 }
  ' "$ledger"
}
printf "CSS Rule Ledger row count: "
count_ledger_rows "CSS Rule Ledger"
printf "CSS Declaration And Value Family Ledger row count: "
count_ledger_rows "CSS Declaration And Value Family Ledger"
printf "CSS-Wide Keyword Ledger row count: "
count_ledger_rows "CSS-Wide Keyword Ledger"
printf "CSS Property Ledger row count: "
count_ledger_rows "CSS Property Ledger"
printf "Selector And Tree Fact Ledger row count: "
count_ledger_rows "Selector And Tree Fact Ledger"
printf "Condition, Layer, Scope, And Environment Ledger row count: "
count_ledger_rows "Condition, Layer, Scope, And Environment Ledger"
```

Update `## Coverage Audit` with rows for:

- `CssRule`
- `CssValue`
- `CssGlobalKeyword`
- `CssProperty`
- `Selector required rows`
- `Condition/layer/scope/environment required rows`

Expected audit values:

- `CssRule` ledger row count equals the source count from the command.
- `CssValue` ledger row count equals the source count from the command.
- `CssGlobalKeyword` ledger row count equals `5`.
- `CssProperty` ledger row count equals the source count from the command.
- `Selector required rows` ledger row count is at least `65`.
- `Condition/layer/scope/environment required rows` ledger row count is at least `52`.

If any row count is lower than expected, add the missing ledger rows before
continuing. If a source count changes because `surgeist-css` changed, update
the source snapshot and explain the change in `## Coverage Audit`.

- [ ] **Step 3: Check for forbidden placeholders**

Run:

```sh
rg -n "TBD|TODO|fill in|unknown commit|unknown status|\\?\\?\\?|<STYLE_SHA>|<CSS_SHA>|<STYLE_STATUS_SUMMARY>|<CSS_STATUS_SUMMARY>" plans/2026-07-05-css-surface-style-ledger.md
```

Expected: no output.

- [ ] **Step 4: Check that every classification label is from the approved vocabulary**

Run:

```sh
rg -n "\\| `?(Existing style model|New style model needed|Typed symbolic style data|Root rejection|Out of style boundary|Root-owned lowering boundary)`? \\|" plans/2026-07-05-css-surface-style-ledger.md
```

Expected: every data row in sections with a `Classification` column uses one
of the approved labels.

This command finds approved labels; it does not prove every classification
cell is populated. The worker must manually scan the ledger tables after the
command and report that every classification cell is present and uses one of
the approved labels.

- [ ] **Step 5: Check owner labels**

Run:

```sh
rg -n "\\| `?(style|root|css|layout|text|render|retained|window|host)`? \\|" plans/2026-07-05-css-surface-style-ledger.md
```

Expected: every data row in sections with an `Owner` column uses one of the
approved owner labels.

This command finds approved owner labels; it does not prove every owner cell is
populated. The worker must manually scan the ledger tables after the command
and report that every owner cell is present and uses one of the approved owner
labels.

- [ ] **Step 6: Run markdown and git hygiene checks**

Run:

```sh
git add -N plans/2026-07-05-css-surface-style-ledger.md
git diff --check
git diff --stat
git status --short --branch
```

Expected:

- `git diff --check` passes.
- `git diff --stat` only includes `plans/2026-07-05-css-surface-style-ledger.md`.
- `git status --short --branch` shows the ledger file as an intent-to-add or
  new file for this plan, unless coordinator-approved plan files are already
  present.

`git add -N` is used only to make the untracked ledger visible to `git diff`
for review. Workers must not make a commit.

- [ ] **Step 7: Document skipped Rust checks**

Do not run `cargo test`, `cargo clippy`, or `cargo fmt --check` for this plan unless Rust source changed unexpectedly.

Expected ledger note under `## Current Style Surface Summary` or a final audit paragraph:

```text
Rust checks were skipped because this plan creates a source-derived planning
ledger only and does not modify Rust source.
```

If any Rust source changed, stop and report that the plan scope was violated.

## Coordinator Commit

After each task-scoped worker/reviewer cycle is clean and a final holistic
reviewer approves the complete ledger, the coordinator should commit the ledger
as a logical planning point:

```sh
git add plans/2026-07-05-css-surface-style-ledger.md
git commit -m "plan: map css surface to style ledger"
```

Workers must not commit.

## Next Sequence Context

The next implementation plan should cover authored declarations, cascade
metadata, and CSS-wide keywords. It should consume
`plans/2026-07-05-css-surface-style-ledger.md` as its source map instead of
starting from the whole CSS parser surface again.

The next plan should specifically pick up:

- CSS-wide keyword rows: `inherit`, `initial`, `unset`, `revert`, and
  `revert-layer`.
- Rule metadata rows: specificity, source order, layer order, scoped rule order,
  and conditional applicability inputs.
- Declaration rows: ordinary typed values, custom properties, and
  variable-dependent values as authored-style receiving concerns.
- Any root coordination question that affects cascade origin, unsupported
  integration diagnostics, or source diagnostic handles.

Do not begin selector, pseudo-element, variable substitution, or property-family
implementation until the authored/cascade/CSS-wide keyword plan has been
implemented and reviewed. Rebase this sequence after each implemented plan so
later plans inherit the actual committed model rather than stale assumptions.
