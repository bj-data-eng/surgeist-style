# Style Root Handoff Notes

Date: 2026-07-07

## Purpose

Record how root should lower parsed CSS-owned syntax into the style-owned API
surface after the CSS surface implementation sequence through Operation 14.

These notes describe lowering into public `surgeist-style` APIs only.

## Source Snapshot

| Source | Value |
| --- | --- |
| pre-artifact/base style snapshot | `cf42d793fe14bf0ad28509d0c55a42980e2d093c` |
| API artifact | `plans/2026-07-07-style-css-api-artifact.md` |

The final Operation 15 handoff commit is reported by the coordinator after
verification.

## Lowering Responsibilities

### Rule Ordering And Layers

Root lowers parsed `@layer` statements to `LayerStatement` and
`Sheet::declare_layers`. Root lowers named or anonymous layer blocks to
`LayerBlock`, registers orders through `LayerRegistry` or `Sheet` layer APIs,
and inserts rules with `Sheet::push_layer_rule` or equivalent public sheet rule
constructors.

Root supplies `SourceOrder` in CSS encounter order and combines it with
`LayerOrder` and selector specificity through `RulePrecedence`.

This style pass has no `!important` support and no cascade origin support.

### Selectors, Scopes, And Buckets

Root lowers parsed selectors into style-owned `Selector`, `SelectorList`,
`Compound`, `ComplexSelectorPart`, `RelativeSelector`,
`RelativeSelectorList`, `AttributeSelector`, `PseudoClassSelector`,
`StructuralSelector`, `NthSelector`, and `SelectorListPseudoClass` values.

Root lowers pseudo-element sequences into `PseudoElement` sequences and then
into `StyleBucket` with `StyleBucket::from_pseudo_elements`, or into
`RuleTarget` when constructing rules.

Root lowers `@scope` roots and limits into `ScopeSelectorList` and `RuleScope`.

### Declarations, CSS-Wide Keywords, And Variables

Root lowers ordinary supported property values into typed `Value` payloads and
then into `Declaration`, `Declarations`, `AuthoredDeclaration`, or
`AuthoredDeclarations` through public constructors.

Root lowers CSS-wide keywords into `CssWideKeyword` and
`AuthoredDeclaration::css_wide`. Root lowers custom properties into
`CustomPropertyName`, `CustomPropertyValue`, and authored custom declarations.
Root lowers variable-dependent ordinary declarations into
`VariableDependentValue`, `VariableExpression`, `VariableReference`, and
`VariableFallback`.

### Conditions And Environment Facts

Root lowers `@media` into `MediaQueryList`, `MediaQuery`, `MediaCondition`, and
`MediaFeatureQuery`.

Root lowers `@container` into `ContainerCondition`,
`ContainerConditionList`, and `ContainerFeatureQuery`.

Root supplies host and layout facts as `ConditionFacts`, `MediaEnvironment`, and
`ContainerFacts`. Style evaluates those facts but does not discover them.

### Generated Content, Timing, And Keyframes

Root lowers generated content, list marker, counter, quote, and attr data into
style-owned `Content`, `ContentItem`, `ListStyle`, `CounterChanges`, and
related typed value payloads.

Root lowers transition and animation properties into `TransitionList`,
`TransitionPropertyList`, `TimeList`, `EasingList`, `AnimationList`,
`AnimationNameList`, and related typed value payloads.

Root lowers `@keyframes` into `KeyframesName`, `KeyframesRule`,
`KeyframeBlock`, `KeyframeSelectorList`, and `KeyframeOffset`.

Runtime generated-content materialization, animation sampling, easing
evaluation, scheduling, interpolation realization, and render/compositor
decisions stay outside style.

### Resolver And Diagnostics

Root builds `Context` from the `Tree`, target node, `Traversal`,
`ConditionFacts`, optional selector root/scope anchors, selected `StyleBucket`,
parent resolved style, local declarations, and animated declarations where
relevant.

Root calls `Resolver::resolve` when diagnostics are not needed, or
`Resolver::resolve_with_diagnostics` when invalid-at-computed-value diagnostics
should be returned. Root maps `StyleSourceId` through its own source table.

### Invalidation

Root maps DOM, tree, runtime, environment, cascade, and style-bucket updates to
`Change`, `SelectorFactChange`, `ConditionFactChange`, `CascadeChange`, and
property invalidation inputs.

Root should treat these APIs as conservative invalidation hooks. Finer
dependency indexing for advanced selectors, scopes, layers, and custom property
dependencies can be planned after root integration pressure identifies concrete
needs.

## Unsupported Or Deferred Surfaces

- Imports and stylesheet fetching remain root owned.
- CSS parser behavior and CSS source spans remain CSS/root owned.
- Unsupported-integration diagnostics remain a root decision.
- Host media/container/environment fact discovery remains root or host owned.
- Layout, text shaping, retained projection, rendering, render resources, image
  loading, font loading, animation scheduling, and final color/resource
  realization remain outside style.
- Font-face descriptors remain root/text owned unless root decides otherwise.
- Cascade origin and `!important` ordering are not supported by this style
  pass.

## Root Questions

1. Does root want style to model cascade origin in the first authored-style
   pass, or only author-origin rules?
2. Which parsed CSS properties should root reject initially even though
   `surgeist-css` accepts them?
3. Should style expose unsupported-integration diagnostics directly, or should
   root convert style validation failures into integration diagnostics?
4. Which media, container, and environment facts can root provide to style in
   the first integration pass?
5. Should `@font-face` descriptors become style-owned symbolic data now, or
   remain root/text-owned until font loading exists?

## Next Work

The next step after this style plan is root integration and lowering planning
against the API artifact, these handoff notes, and the rebased ledgers.

Do not start another broad style surface expansion unless root discovers a
concrete modeling gap while lowering into these public APIs.
