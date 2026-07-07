# Style CSS API Artifact

Date: 2026-07-07

## Purpose

Describe the crate-owned CSS-facing style API surface available after the CSS
surface implementation sequence through Operation 14.

This artifact is descriptive, not generated, and is not an exhaustive rustdoc
replacement.

## Source Snapshot

| Source | Value |
| --- | --- |
| pre-artifact/base style snapshot | `cf42d793fe14bf0ad28509d0c55a42980e2d093c` |
| CSS surface ledger | `plans/2026-07-05-css-surface-style-ledger.md` |
| property coverage ledger | `plans/2026-07-05-css-property-coverage-ledger.md` |
| sequence | `plans/2026-07-05-css-surface-style-operations-sequence.md` |

The final Operation 15 handoff commit is reported by the coordinator after
verification.

## Boundary

`surgeist-style` receives root-lowered typed data. It does not import
`surgeist-css`, CSS syntax types, or CSS source-location types.

Root owns lowering from parsed CSS into the public style APIs listed here.
Style owns the typed receiving models, cascade data it currently models,
selector/tree facts, resolver behavior, diagnostics, cache axes, invalidation,
and computed style outputs.

## Public Front Doors

### Identity And Tree Facts

- `StyleTag`: typed element tag identifier.
- `StyleClass`: typed class identifier.
- `StyleKey`: typed unique key identifier.
- `StyleAttributeName`: typed attribute name.
- `StyleAttributeValue`: typed attribute value.
- `StyleAttribute`: typed name/value attribute fact.
- `StyleRole`: typed accessibility or semantic role fact.
- `StyleState`: typed runtime state fact set.
- `Node`: root-supplied tree node wrapper.
- `Tree`: root-supplied trait for selector and resolver facts.
- `Traversal`: tree traversal mode for matching and cache axes.

### Selectors, Scopes, And Buckets

- `Selector`: style-owned selector entry point.
- `SelectorList`: non-empty selector list.
- `AttributeSelector`: typed attribute selector condition.
- `AttributeMatcher`: typed attribute match operator.
- `AttributeCaseSensitivity`: typed attribute case policy.
- `Combinator`: typed complex-selector relationship.
- `Compound`: typed compound selector.
- `ComplexSelectorPart`: typed selector part plus combinator.
- `RelativeSelector`: typed relative selector.
- `RelativeSelectorList`: non-empty relative selector list.
- `PseudoClassSelector`: typed pseudo-class selector.
- `RuntimePseudoClass`: pseudo-class backed by runtime facts.
- `StructuralSelector`: pseudo-class backed by tree structure facts.
- `NthSelector`: typed nth selector with optional filter.
- `NthPattern`: typed nth arithmetic pattern.
- `SelectorListPseudoClass`: typed selector-list pseudo-class.
- `PseudoElement`: supported pseudo-element token.
- `SelectorSpecificity`: typed specificity tuple.
- `ScopeSelectorList`: non-empty scope selector list.
- `RuleScope`: optional scope roots and limits.
- `StyleBucket`: style target bucket for element and pseudo-element styles.
- `StyleBucketPolicy`: typed policy attached to a style bucket.

### Authored Declarations And Variables

- `AuthoredDeclaration`: one root-lowered authored declaration.
- `AuthoredDeclarations`: ordered authored declaration store.
- `AuthoredProperty`: property or custom-property authored target.
- `AuthoredValue`: typed authored value, CSS-wide keyword, or variable path.
- `CssWideKeyword`: style-owned CSS-wide keyword enum.
- `CustomPropertyName`: typed custom property name.
- `CustomPropertyValue`: typed custom property payload.
- `CustomPropertyTypedValue`: typed custom property value branch.
- `AuthoredTokens`: preserved authored-token payload.
- `VariableDependentValue`: ordinary declaration dependent on variables.
- `VariableExpression`: variable expression tree.
- `VariableReference`: typed variable reference.
- `VariableFallback`: typed variable fallback expression.

### Properties And Values

- `Property`: non-exhaustive style property enum.
- `Value`: typed value enum accepted by style declarations.
- `Declaration`: validated `Property` plus `Value`.
- `Declarations`: canonical declaration collection and shorthand expander.
- `TypedDeclaration`: typed public constructor wrapper.

Operation 8 through Operation 12 rebased these typed value families:

- Layout: display, box, overflow, sizing, spacing, positioning, flex, grid,
  alignment, writing mode, visibility, z-index, scrollbar, and content
  visibility values.
- Text/font: font family, font shorthand, font weight/slant/stretch/variant,
  font features, text alignment, indentation, vertical alignment, wrapping,
  overflow, decoration, spacing, and transform values.
- Paint/color/effects: style colors, symbolic colors, backgrounds, borders,
  outlines, shadows, opacity, transforms, filters, clip paths, masks, cursor,
  pointer events, and user select values.
- Generated content: content items, quote/attr/counter payloads, list marker
  types, list images, list shorthands, and counter mutation values.
- Timing/animation: transition lists, animation lists, duration lists, easing
  lists, iteration counts, direction/fill/play-state lists, keyframe names,
  `KeyframesRule`, `KeyframeBlock`, `KeyframeSelectorList`, and
  `KeyframeOffset`.

### Sheets, Rules, Layers, And Conditions

- `Sheet`: ordered style rule and keyframes store.
- `Rule`: style rule with target, declarations, conditions, scope, and
  precedence.
- `RuleTarget`: selector plus `StyleBucket`.
- `RulePrecedence`: layer, specificity, and source-order precedence.
- `SourceOrder`: typed source-order index.
- `LayerOrder`: typed layer-order index.
- `StyleLayerName`: typed cascade layer name.
- `StyleLayerNameList`: non-empty layer name list.
- `LayerStatement`: typed `@layer` statement payload.
- `LayerBlock`: typed named or anonymous layer block.
- `LayerRegistry`: typed layer registration state.
- `Condition`: style-owned conditional rule.
- `ConditionFacts`: root-provided condition facts.
- `MediaEnvironment`: root-provided media facts.
- `MediaQueryList`: media query list.
- `MediaQuery`: typed media query.
- `MediaCondition`: typed media condition tree.
- `MediaFeatureQuery`: typed media feature query.
- `ContainerFacts`: root-provided container facts.
- `ContainerCondition`: typed container condition tree.
- `ContainerConditionList`: typed container condition list.
- `ContainerFeatureQuery`: typed container feature query.

### Resolver, Diagnostics, Cache Axes, And Invalidation

- `Context`: resolver input and cache-axis carrier.
- `Resolver`: rule application and style resolution entry point.
- `Resolved`: computed style output.
- `ResolvedWithDiagnostics`: computed style output plus diagnostics.
- `StyleSourceId`: opaque root-mapped source identifier.
- `StyleDiagnostic`: style diagnostic payload.
- `StyleDiagnosticKind`: typed diagnostic kind.
- `StyleDiagnosticSubject`: typed diagnostic subject.
- `InvalidAtComputedValueReason`: typed invalid-at-computed-value reason.
- `Invalidation`: property impact flags.
- `Change`: typed invalidation change summary.
- `SelectorFactChange`: selector fact invalidation axis.
- `ConditionFactChange`: condition fact invalidation axis.
- `CascadeChange`: cascade invalidation axis.

## Explicit Non-Goals

This crate does not own imports, parser behavior, CSS source spans, root
integration diagnostics, host fact discovery, layout, text shaping, retained
projection, rendering, render resources, image loading, font loading, animation
scheduling, or final color/resource realization.

`surgeist-style` also does not provide a root adapter, compatibility aliases, or
generated CSS syntax output in this pass.

## Verification Surface

- `tests/type_safety.rs`: trybuild harness for public compile-pass and
  compile-fail contracts.
- `tests/compile_pass/typed_public_construction.rs`: public construction paths
  for typed declarations, authored declarations, custom properties, selectors,
  buckets, layers, scopes, conditions, keyframes, diagnostics, resolver context,
  and invalidation changes.
- `tests/compile_fail/invalid_authored_struct_literal.rs`: authored declaration
  private-field invariants.
- `tests/compile_fail/invalid_custom_property_name_newtype_literal.rs` and
  `tests/compile_fail/invalid_custom_property_struct_literal.rs`: custom
  property construction invariants.
- `tests/compile_fail/invalid_variable_dependent_struct_literal.rs`: variable
  dependent value construction invariant.
- `tests/compile_fail/invalid_precedence_newtype_literal.rs` and
  `tests/compile_fail/invalid_precedence_struct_literal.rs`: layer/source-order
  and precedence invariants.
- `tests/compile_fail/invalid_layer_registry_struct_literal.rs`:
  `LayerRegistry` private construction invariant.
- `tests/compile_fail/invalid_rule_scope_literal.rs`,
  `tests/compile_fail/invalid_rule_target_struct_literal.rs`, and
  `tests/compile_fail/invalid_selector_specificity_struct_literal.rs`: scoped
  selector, target, and specificity invariants.
- `tests/compile_fail/invalid_style_source_id_literal.rs` and
  `tests/compile_fail/invalid_style_diagnostic_literal.rs`: opaque source and
  diagnostic privacy invariants.
- `tests/compile_fail/invalid_resolved_with_diagnostics_literal.rs`:
  `ResolvedWithDiagnostics` private construction invariant.
- `tests/compile_fail/invalid_keyframe_offset_literal.rs`: keyframe offset
  validation and privacy invariant.
