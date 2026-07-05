# CSS Surface Style Ledger

Date: 2026-07-05

## Purpose

This ledger maps the current `surgeist-css` authored API surface to explicit
`surgeist-style` ownership decisions. It is the source contract for later
style implementation plans and root lowering work.

## Source Snapshot

| Repo | Path | Commit | Status |
| --- | --- | --- | --- |
| style | `/Users/codex/Development/surgeist-style` | `5ac070a3d8270a7ec368023769b6b056853f8f57` | `ahead 1 with untracked inventory ledger implementation plan` |
| css | `/Users/codex/Development/surgeist-css` | `1c95d4218439f1696151e0ee9602671fab418314` | `clean` |

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
