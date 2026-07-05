# CSS Property Coverage Ledger Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a focused, source-audited property coverage ledger that maps every `surgeist-css` `CssProperty` variant to the current or planned `surgeist-style` behavior.

**Architecture:** This is a planning and audit artifact pass, not a Rust feature implementation. The ledger lives in `plans/`, is generated from read-only `surgeist-css` source plus current style models, and becomes the single property roadmap input for the next layout/text/paint/timing implementation plans.

**Tech Stack:** Rust source inspection with `rg`, small temporary Python extraction scripts, Markdown tables, crate-local `plans/`, and `git diff --check`.

---

## Source Context

Read before editing:

- `AGENTS.md`
- `README.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-support-directive.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-pseudo-element-style-buckets-implementation.md`
- `src/property.rs`
- `src/value.rs`
- `src/declaration.rs`
- `src/authored.rs`
- `src/custom.rs`
- `src/bucket.rs`
- Read-only sibling repo `/Users/codex/Development/surgeist-css`
  - `src/syntax.rs`
  - `src/parser/mod.rs`
  - `src/tests.rs`

Current source snapshots used when this plan was written:

- `surgeist-style`: `055077b` (`style: resolve requested style buckets`)
- `surgeist-css`: `1c95d4218439f1696151e0ee9602671fab418314`

## Scope

Create:

- `plans/2026-07-05-css-property-coverage-ledger.md`

Do not modify:

- `Cargo.toml`
- `src/**`
- `tests/**`
- sibling repositories

This operation classifies property support. It does not add style properties,
CSS lowering, adapters, parser dependencies, generated content models, layout
algorithms, text shaping, rendering, or cache/invalidation generalization.

## Ledger Outcomes

Every `CssProperty` row must use exactly one of these outcome labels:

- `Existing style property`: style already has a semantically owned `Property`
  and `Value` model for the CSS longhand.
- `Existing style shorthand`: style already has a semantic shorthand or
  aggregate `Property`/`Value` model that can receive this CSS surface without
  adding a new style value family.
- `New style property needed`: CSS accepts a longhand that style does not yet
  model as computed style data.
- `New shorthand lowering needed`: CSS accepts a shorthand that should lower
  into existing or planned style longhands instead of becoming an unrelated bag.
- `Symbolic style data needed`: style should preserve authored symbolic data
  because another owner or later context is needed to resolve it.
- `Existing authored cascade model`: current authored declaration, CSS-wide
  keyword, custom property, or cascade-path code already owns the surface.
- `Root rejection required`: root should reject this property before style
  receives a normal declaration.
- `Out of style`: the surface is intentionally not represented in style output.

The ledger may include a short note when an outcome is partial. For example,
`Color` can be an existing style property for concrete RGBA, while the row must
still note that symbolic colors remain `Symbolic style data needed`.

## Ledger Table Schema

Use this table header exactly for the property table:

```markdown
| CSS property | CSS value kind | Family | Outcome | Style target | Lowering or gap | Next plan |
| --- | --- | --- | --- | --- | --- | --- |
```

Rules:

- The first cell must be formatted as `` `CssProperty::Variant` ``.
- `Custom(CssCustomPropertyName)` must be formatted as
  `` `CssProperty::Custom(CssCustomPropertyName)` ``.
- Each `CssProperty` variant must appear exactly once.
- `CSS value kind` should name the parsed `CssValue` variant from
  `surgeist-css` when known, such as `` `CssValue::Length` `` or
  `` `CssValue::GlobalKeyword` ``.
- `Family` must use one of the family names in the required family list below.
- `Style target` must name the existing style type when one exists, such as
  `` `Property::Width` + `Value::Length` ``.
- `Lowering or gap` must be concrete enough for the next implementation plan to
  know whether it should add a type, lower a shorthand, preserve symbolic data,
  or leave the property outside style.
- `Next plan` must point to one of the operation labels in the sequence:
  - `Operation 8 layout-facing properties`
  - `Operation 9 text-facing properties`
  - `Operation 10 paint/color/effects`
  - `Operation 11 generated content/counters/lists`
  - `Operation 12 timing/animation/keyframes`
  - `Operation 14 cache/invalidation`
  - `No property implementation`

## Required Family List

Use these family names in the ledger:

- `Authored cascade`
- `Display and box`
- `Overflow and visibility`
- `Sizing and spacing`
- `Position and stacking`
- `Flex`
- `Grid`
- `Alignment`
- `Writing mode`
- `Text and font`
- `Generated content and lists`
- `Color`
- `Background`
- `Border and outline`
- `Paint and effects`
- `Transforms`
- `Interaction`
- `Timing and animation`
- `Custom properties`

## Required `CssProperty` Coverage

The ledger must cover these 180 `CssProperty` variants from
`surgeist-css/src/syntax.rs` at commit
`1c95d4218439f1696151e0ee9602671fab418314`:

```text
All
Display
BoxSizing
Position
Direction
Overflow
OverflowX
OverflowY
FlexDirection
FlexWrap
Float
Clear
AlignContent
JustifyContent
AlignItems
AlignSelf
JustifyItems
JustifySelf
PlaceContent
PlaceItems
PlaceSelf
Visibility
Content
ContentVisibility
ListStyleType
ListStylePosition
ListStyleImage
ListStyle
CounterReset
CounterIncrement
CounterSet
Width
Height
MinWidth
MinHeight
MaxWidth
MaxHeight
FlexBasis
Gap
RowGap
ColumnGap
GridFlowTolerance
GridTemplateRows
GridTemplateColumns
GridTemplateAreas
GridTemplate
GridAutoRows
GridAutoColumns
GridAutoFlow
GridRowStart
GridRowEnd
GridColumnStart
GridColumnEnd
GridRow
GridColumn
GridArea
Grid
FontSize
LineHeight
WritingMode
TextAlign
TextAlignLast
TextIndent
VerticalAlign
FontFamily
Font
FontWeight
FontStyle
FontStretch
FontVariant
FontFeatureSettings
LetterSpacing
TextWrap
WhiteSpace
WordBreak
OverflowWrap
TextOverflow
TextDecoration
TextDecorationLine
TextDecorationColor
TextDecorationStyle
TextDecorationThickness
TextTransform
Inset
Top
Right
Bottom
Left
ZIndex
BoxDecorationBreak
Margin
MarginTop
MarginRight
MarginBottom
MarginLeft
Padding
PaddingTop
PaddingRight
PaddingBottom
PaddingLeft
Border
BorderTop
BorderRight
BorderBottom
BorderLeft
BorderWidth
BorderTopWidth
BorderRightWidth
BorderBottomWidth
BorderLeftWidth
Color
Background
BackgroundColor
BorderColor
BorderTopColor
BorderRightColor
BorderBottomColor
BorderLeftColor
BackgroundImage
BackgroundPosition
BackgroundSize
BackgroundRepeat
BackgroundOrigin
BackgroundClip
BackgroundAttachment
BorderStyle
BorderTopStyle
BorderRightStyle
BorderBottomStyle
BorderLeftStyle
BorderRadius
BorderTopLeftRadius
BorderTopRightRadius
BorderBottomRightRadius
BorderBottomLeftRadius
BoxShadow
Opacity
FlexGrow
FlexShrink
Order
Flex
JustifyTracks
AlignTracks
AspectRatio
ScrollbarWidth
Cursor
PointerEvents
UserSelect
Outline
OutlineColor
OutlineStyle
OutlineWidth
Transform
TransformOrigin
Translate
Rotate
Scale
Filter
BackdropFilter
ClipPath
Mask
MaskImage
MaskSize
MaskPosition
MaskRepeat
TransitionProperty
TransitionDuration
TransitionDelay
TransitionTimingFunction
Transition
AnimationName
AnimationDuration
AnimationDelay
AnimationTimingFunction
AnimationIterationCount
AnimationDirection
AnimationFillMode
AnimationPlayState
Animation
Custom(CssCustomPropertyName)
```

## Task 1: Create The Focused Ledger Skeleton

**Files:**

- Create: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: the repo is on `main` with a clean working tree. The ahead count may
vary because this plan may have been committed before ledger implementation
begins.

- [ ] **Step 2: Create the ledger header and source snapshot**

Create `plans/2026-07-05-css-property-coverage-ledger.md` with this header:

```markdown
# CSS Property Coverage Ledger

Date: 2026-07-05

## Source Snapshot

- `surgeist-style`: `055077b` (`style: resolve requested style buckets`)
- `surgeist-css`: `1c95d4218439f1696151e0ee9602671fab418314`
- Source CSS enum: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Source CSS parser dispatch: `/Users/codex/Development/surgeist-css/src/parser/mod.rs`
- Current style property model: `src/property.rs`
- Current style value model: `src/value.rs`
- Current authored/custom model: `src/authored.rs`, `src/custom.rs`

## Purpose

This ledger maps every parsed `surgeist-css` `CssProperty` to the current or
planned `surgeist-style` behavior. It is the handoff artifact for the next
property-family implementation plans.

This file is descriptive. It does not add Rust APIs, CSS lowering, parser
dependencies, adapters, or generated code.

## Outcome Labels

| Outcome | Meaning |
| --- | --- |
| `Existing style property` | Style already has a semantically owned longhand `Property` and `Value` model for this CSS surface. |
| `Existing style shorthand` | Style already has a semantic shorthand or aggregate `Property`/`Value` model for this CSS surface. |
| `New style property needed` | CSS accepts a longhand that style does not yet model as computed style data. |
| `New shorthand lowering needed` | CSS accepts a shorthand that should lower into existing or planned style longhands. |
| `Symbolic style data needed` | Style must preserve authored symbolic data because another owner or later context is needed. |
| `Existing authored cascade model` | Authored declarations, CSS-wide keywords, custom properties, or cascade-path code already owns the surface. |
| `Root rejection required` | Root should reject this property before normal style declaration input. |
| `Out of style` | The surface is intentionally not represented in style output. |

## Property Coverage

| CSS property | CSS value kind | Family | Outcome | Style target | Lowering or gap | Next plan |
| --- | --- | --- | --- | --- | --- | --- |
```

- [ ] **Step 3: Commit the skeleton only if Task 2 will be assigned later**

When executing this plan continuously, do not commit after the empty skeleton.
Commit after Task 2 populates the table.

## Task 2: Populate Every `CssProperty` Row

**Files:**

- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Extract the current CSS property list**

Run this temporary extraction command from the style repo:

```sh
python3 - <<'PY'
from pathlib import Path
text = Path('/Users/codex/Development/surgeist-css/src/syntax.rs').read_text()
start = text.index('pub enum CssProperty {')
end = text.index('\n}\n\n#[derive(Clone, Debug, Eq, Hash, PartialEq)]\npub struct CssCustomPropertyName', start)
variants = []
for line in text[start:end].splitlines()[1:]:
    line = line.strip()
    if line and not line.startswith('//'):
        variants.append(line.rstrip(','))
print(len(variants))
print('\n'.join(variants))
PY
```

Expected first line:

```text
180
```

- [ ] **Step 2: Extract current style property list**

Run:

```sh
python3 - <<'PY'
from pathlib import Path
text = Path('src/property.rs').read_text()
start = text.index('pub enum Property {')
end = text.index('\n}\n\nimpl Property', start)
variants = []
for line in text[start:end].splitlines()[1:]:
    line = line.strip()
    if line and not line.startswith('//'):
        variants.append(line.rstrip(','))
print(len(variants))
print('\n'.join(variants))
PY
```

Expected first line at the source snapshot in this plan:

```text
91
```

- [ ] **Step 3: Populate the property table**

Add one table row for every `CssProperty` variant in the required coverage
list. Use current source inspection plus the earlier broad ledger as input, but
make this file authoritative for property-family planning.

Row examples that define the expected style:

```markdown
| `CssProperty::Display` | `CssValue::Display` | Display and box | `Existing style property` | `Property::Display` + `Value::Display` | Concrete display values have a style model; CSS-only display combinations remain root-lowering concerns. | Operation 8 layout-facing properties |
| `CssProperty::PlaceContent` | `CssValue::PlaceAlignment` | Alignment | `New shorthand lowering needed` | Planned lowering to `Property::AlignContent` and `Property::JustifyContent` | CSS shorthand needs explicit style lowering; do not add a broad placement bag. | Operation 8 layout-facing properties |
| `CssProperty::Content` | `CssValue::Content` | Generated content and lists | `New style property needed` | Planned generated content model scoped to `StyleBucket` | Style should own generated content policy/data; retained/tree materialization remains outside style. | Operation 11 generated content/counters/lists |
| `CssProperty::Color` | `CssValue::Color` | Color | `Existing style property` | `Property::Color` + `Value::Color` | Concrete RGBA is supported; symbolic colors and variable-dependent components need symbolic style data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundImage` | `CssValue::BackgroundImage` | Background | `Symbolic style data needed` | Planned background image layer model | URLs/images are authored symbolic resources; loading and final render resources stay outside style. | Operation 10 paint/color/effects |
| `CssProperty::Animation` | `CssValue::Animation` | Timing and animation | `New shorthand lowering needed` | Planned animation list model plus longhand lowering | Style has `AnimationName` only; timing, direction, fill, play-state, and iteration counts are missing. | Operation 12 timing/animation/keyframes |
| `CssProperty::Custom(CssCustomPropertyName)` | `CssValue::CustomProperty` | Custom properties | `Existing authored cascade model` | `CustomPropertyName`, `CustomPropertyValue`, `VariableDependentValue` | Custom property storage and variable substitution exist; later plans may expand typed value coverage. | No property implementation |
```

Classification guidance:

- `All` belongs to `Authored cascade` and should use
  `Existing authored cascade model`.
- Longhands with direct current `Property`/`Value` pairs should use
  `Existing style property`, even when the row notes partial CSS syntax gaps.
- CSS shorthands that should expand into multiple style declarations should
  use `New shorthand lowering needed` unless style already has an equivalent
  semantic aggregate.
- CSS properties accepted by the parser for resource-like data, colors that
  cannot yet be resolved, images, filters, masks, animation references, and
  font-loading data should use `Symbolic style data needed` when style must
  preserve authored data.
- `Content`, list-style, counter, and marker-facing properties should point to
  Operation 11.
- Timing and animation properties should point to Operation 12.
- Font-face and import rule descriptors are not `CssProperty` variants. Mention
  them in the later context section, not in the property table.

- [ ] **Step 4: Add family rollups**

After the property table, add this section:

```markdown
## Family Rollup

| Family | Existing style support | Missing style support | Next implementation plan |
| --- | --- | --- | --- |
```

Populate one row for every family in the required family list. Each row must
summarize:

- which current style `Property`/`Value` surfaces exist;
- which CSS properties in that family still need style models or lowering;
- which operation should implement the family.

- [ ] **Step 5: Add source audit**

Append this section:

```markdown
## Coverage Audit

| Audit | Expected | Observed | Result |
| --- | --- | --- | --- |
| `CssProperty` variants in `surgeist-css` | `180` | `180` | Pass |
| Property ledger rows | `180` | `180` | Pass |
| Duplicate property rows | `0` | `0` | Pass |
| Missing property rows | `0` | `0` | Pass |

## Dependency And Boundary Check

`surgeist-style` source and tests do not depend on `surgeist-css`; this ledger
uses read-only source inspection only.

This ledger does not introduce Rust source changes, parser dependencies,
adapters, generated content materialization, layout algorithms, text shaping, or
render resources.
```

The observed values must match the verification commands in Task 3. If they do
not match, update the table or report the blocker.

## Task 3: Verify Coverage And Boundaries

**Files:**

- Verify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Count source enum variants and ledger rows**

Run:

```sh
python3 - <<'PY'
from pathlib import Path
import re
css = Path('/Users/codex/Development/surgeist-css/src/syntax.rs').read_text()
start = css.index('pub enum CssProperty {')
end = css.index('\n}\n\n#[derive(Clone, Debug, Eq, Hash, PartialEq)]\npub struct CssCustomPropertyName', start)
source = []
for line in css[start:end].splitlines()[1:]:
    line = line.strip()
    if line and not line.startswith('//'):
        source.append(line.rstrip(','))
ledger = Path('plans/2026-07-05-css-property-coverage-ledger.md').read_text()
rows = re.findall(r'^\| `CssProperty::([^`]+)` \|', ledger, flags=re.MULTILINE)
missing = sorted(set(source) - set(rows))
extra = sorted(set(rows) - set(source))
duplicates = sorted({row for row in rows if rows.count(row) > 1})
print(f'source={len(source)} rows={len(rows)}')
print(f'missing={missing}')
print(f'extra={extra}')
print(f'duplicates={duplicates}')
raise SystemExit(0 if len(source) == len(rows) == 180 and not missing and not extra and not duplicates else 1)
PY
```

Expected:

```text
source=180 rows=180
missing=[]
extra=[]
duplicates=[]
```

- [ ] **Step 2: Check forbidden unfinished markers**

Run:

```sh
rg -n "T[B]D|T[O]DO|place[ ]holder|fill[ ]in|later[?]|un[ ]known" plans/2026-07-05-css-property-coverage-ledger.md
```

Expected: no matches.

- [ ] **Step 3: Check dependency boundary**

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
```

Expected: no matches.

- [ ] **Step 4: Check plan/document diff hygiene**

Run:

```sh
git diff --check
git status --short --branch
git diff --stat
```

Expected:

- `git diff --check` exits successfully.
- `git status --short --branch` shows only
  `?? plans/2026-07-05-css-property-coverage-ledger.md` before staging.
- `git diff --stat` may be empty for the untracked file; use
  `git diff --stat --cached` after staging for commit review.

Rust checks are not required when this plan is executed exactly, because it
creates only a Markdown planning ledger. If any Rust source, tests, or
`Cargo.toml` change, stop and run the full crate checks before committing.

## Task 4: Add Next Sequence Context And Commit

**Files:**

- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Add next sequence context**

Append:

```markdown
## Next Sequence Context

The next implementation plan should cover Operation 8: layout-facing computed
property families.

Use this ledger instead of re-inventorying the full CSS property surface. The
layout plan should start with the `Display and box`, `Overflow and visibility`,
`Sizing and spacing`, `Position and stacking`, `Flex`, `Grid`, `Alignment`,
and `Writing mode` ledger rows that point to Operation 8.

The layout plan should implement style-owned models and lowering front doors
only where the ledger marks `New style property needed` or
`New shorthand lowering needed`. It should not add a style-to-layout adapter,
generated content, text shaping, paint resources, timing/keyframe models, or
Operation 14 cache/invalidation generalization.

`Interaction` rows such as cursor, pointer events, and user select should stay
with Operation 10 paint/color/effects unless the ledger identifies a concrete
layout dependency.

After Operation 8 implementation lands, rebase this ledger before writing
Operation 9 so the remaining property classifications stay honest.
```

- [ ] **Step 2: Final review before commit**

Run:

```sh
git diff -- plans/2026-07-05-css-property-coverage-ledger.md
git add plans/2026-07-05-css-property-coverage-ledger.md
git diff --cached --stat
git diff --cached -- plans/2026-07-05-css-property-coverage-ledger.md
```

Check that:

- the file has exactly one `Property Coverage` table;
- every `CssProperty` row appears exactly once;
- no row uses a label outside the outcome list;
- next sequence context points to Operation 8.

- [ ] **Step 3: Commit**

Run:

```sh
git commit -m "style: add css property coverage ledger"
```

## Final Review Prompt

After implementation and before reporting complete, assign a clean-context
reviewer with this prompt:

```text
Review /Users/codex/Development/surgeist-style/plans/2026-07-05-css-property-coverage-ledger.md
against:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-support-directive.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger-implementation.md

Check that the ledger:
- covers every `CssProperty` variant from `/Users/codex/Development/surgeist-css/src/syntax.rs` exactly once;
- uses only the approved outcome labels;
- classifies style-owned, root-rejected, symbolic, shorthand, and out-of-style boundaries consistently with the Rust modeling guide;
- reflects current surgeist-style capabilities after custom properties, authored cascade keywords, selector expansion, and pseudo-element buckets;
- does not sneak in Rust implementation scope or sibling-repo edits;
- leaves a concrete next context for Operation 8 layout-facing property families.

Report findings with file/line references. If clean, say clean.
```

## Completion Report

Report:

- ledger path;
- commit SHA;
- source `CssProperty` count;
- ledger row count;
- verification commands and results;
- reviewer result;
- final `git status --short --branch`.
