# Selector And Tree Matching Expansion Implementation Plan

> **For agentic workers:** Execute this plan through the local `AGENTS.md`
> coordinator workflow. Workers follow checkbox steps, do not commit, do not
> create branches, and do not let external workflow guidance override the
> crate's worker/reviewer gate.

**Goal:** Expand `surgeist-style` selector and tree matching so root can lower
the current `surgeist-css` selector surface into style-owned, type-safe matcher
models without importing CSS parser types.

**Architecture:** Style owns selector matching semantics over root-provided
tree/runtime facts. Root lowers parsed CSS selectors into style-owned selector,
specificity, scope-anchor, and pseudo-class models; style evaluates those
models against `Tree` and uses specificity/layer/source precedence during rule
resolution. Pseudo-element buckets remain out of scope for the next operation.

**Tech Stack:** Rust 2024, crate-local modules in `src/`, existing unit tests
and `trybuild`, `cargo test -p surgeist-style`, `cargo clippy -p
surgeist-style --all-targets -- -D warnings`, `cargo fmt --check`.

---

## Coordinator Workflow Note

The coordinator executes this plan through `AGENTS.md`. Assign one scoped task
or tightly coupled task group to each worker, assign a separate reviewer after
each worker, commit each clean task as a logical point, and assign a final
holistic reviewer after all tasks are complete.

Workers must not commit. Reviewers must not edit files. Do not fork full
conversation context into workers or reviewers; provide only the scoped prompt,
relevant files, commands, and constraints.

## Source Context

Read these files before executing Task 1:

- `AGENTS.md`
- `Cargo.toml`
- `README.md`
- `guidance/surgeist-rust-modeling-guide.md`
- `plans/2026-07-05-css-surface-style-support-directive.md`
- `plans/2026-07-05-css-surface-style-ledger.md`
- `plans/2026-07-05-css-surface-style-operations-sequence.md`
- `plans/2026-07-05-authored-cascade-keywords-implementation.md`
- `plans/2026-07-05-custom-properties-variable-substitution-implementation.md`
- `src/lib.rs`
- `src/identity.rs`
- `src/state.rs`
- `src/tree.rs`
- `src/selector.rs`
- `src/sheet.rs`
- `src/resolver.rs`
- `src/invalidation.rs`
- `tests/type_safety.rs`
- `tests/compile_pass/typed_public_construction.rs`

Read-only CSS source checked while writing this plan:

- `/Users/codex/Development/surgeist-css` at
  `1c95d4218439f1696151e0ee9602671fab418314`
- Relevant CSS API types in `src/syntax.rs`:
  `CssSelector`, `CssSelectorList`, `CssComplexSelector`,
  `CssComplexSelectorPart`, `CssSelectorCombinator`,
  `CssPseudoSelectorList`, `CssRelativeSelector`,
  `CssRelativeSelectorList`, `CssPseudoClass`, `CssNthChildPattern`,
  `CssNthPattern`, `CssNthAnPlusB`, `CssCompoundSelector`,
  `CssAttributeSelector`, `CssAttributeMatcher`, and
  `CssAttributeCaseSensitivity`.
- Relevant CSS parser behavior in `src/parser/selectors.rs`.

Style source snapshot used for this plan:

- `ba6614c` (`style: cover custom property invalidation`)

## Scope

This plan implements Operation 5 from
`plans/2026-07-05-css-surface-style-operations-sequence.md`:

- style-owned selector lists;
- selector specificity as a style-owned cascade input;
- compound and complex selectors with descendant, child, adjacent, and sibling
  combinators;
- attribute selector matcher variants and ASCII case-sensitivity controls;
- structural pseudo-classes, including reverse and type-filtered variants;
- `An+B` nth-pattern matching, including `of <selector-list>` filters for
  child-index selectors;
- selector-list pseudo-classes `:not`, `:is`, and `:where`;
- relative selectors and `:has`;
- runtime pseudo-class matching over style-owned tree/state facts;
- `:root`, `:scope`, and scoped selector anchor support;
- selector dependency invalidation summaries sufficient for this operation.

This plan deliberately does not implement:

- CSS parsing or a `surgeist-css` dependency;
- pseudo-element buckets, pseudo-element selector targets, or generated content;
- media query, container query, `@scope` rule cascade proximity, or broad
  condition integration beyond the selector matching context needed for
  `:scope` and `&`;
- CSS `!important`, cascade origins, or compatibility aliases;
- shadow DOM, slots, namespace selectors, `||` column combinators, or language
  selectors not present in the accepted CSS selector surface;
- broad property-family expansion.

Root must reject or defer CSS selectors containing pseudo-element sequences
until Operation 6 creates pseudo-element style buckets. Style must not model
pseudo-elements as synthetic tree nodes.

## Public API Shape

Workers may refine names only if the reviewer agrees the refinement improves
type safety without widening scope. The expected public API is:

```rust
pub struct SelectorList { /* private fields */ }
pub struct SelectorSpecificity { /* private fields */ }
pub struct RelativeSelectorList { /* private fields */ }
pub struct RelativeSelector { /* private fields */ }
pub struct NthPattern { /* private fields */ }
pub struct NthSelector { /* private fields */ }

pub enum Selector {
    Any,
    Tag(StyleTag),
    Class(StyleClass),
    Key(StyleKey),
    State(StateFlag),
    Attribute(AttributeSelector),
    Position(PositionSelector),
    Pseudo(PseudoClassSelector),
    Compound(Compound),
    Complex(ComplexSelector),
    List(SelectorList),
}

pub struct ComplexSelector { /* private fields */ }
pub struct ComplexSelectorPart { /* private fields */ }

pub enum Combinator {
    Descendant,
    Child,
    Adjacent,
    Sibling,
}

pub enum AttributeSelector {
    Exists { name: StyleAttributeName },
    Matcher {
        name: StyleAttributeName,
        matcher: AttributeMatcher,
        case_sensitivity: AttributeCaseSensitivity,
    },
}

pub enum AttributeMatcher {
    Equals(StyleAttributeValue),
    Includes(StyleAttributeValue),
    DashMatch(StyleAttributeValue),
    Prefix(StyleAttributeValue),
    Suffix(StyleAttributeValue),
    Substring(StyleAttributeValue),
}

pub enum AttributeCaseSensitivity {
    DocumentDefault,
    AsciiCaseInsensitive,
    ExplicitSensitive,
}

pub enum PseudoClassSelector {
    Root,
    Scope,
    Runtime(RuntimePseudoClass),
    Structural(StructuralSelector),
    SelectorList(SelectorListPseudoClass),
    Has(RelativeSelectorList),
}

pub enum RuntimePseudoClass {
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Disabled,
    Enabled,
    Checked,
    Required,
    Optional,
    Valid,
    Invalid,
    PlaceholderShown,
    Modal,
    Fullscreen,
    PopoverOpen,
    Default,
    Indeterminate,
    ReadOnly,
    ReadWrite,
    InRange,
    OutOfRange,
}

pub enum StructuralSelector {
    FirstChild,
    LastChild,
    OnlyChild,
    Empty,
    NthChild(NthSelector),
    NthLastChild(NthSelector),
    FirstOfType,
    LastOfType,
    OnlyOfType,
    NthOfType(NthPattern),
    NthLastOfType(NthPattern),
}

pub enum SelectorListPseudoClass {
    Not(SelectorList),
    Is(SelectorList),
    Where(SelectorList),
}

pub struct SelectorMatchContext<Id> { /* private fields */ }
```

Expected constructor and accessor behavior:

- List, complex, relative-list, and nth selector fields stay private.
- Empty selector lists and empty relative selector lists are rejected with
  `ErrorCode::InvalidSelector`.
- Add explicit selector constructors:
  `Selector::list(SelectorList)`, `Selector::pseudo(PseudoClassSelector)`,
  `Selector::complex_selector(ComplexSelector)`, and
  `Selector::matches_with_context(tree, context)`.
- Add explicit pseudo-class constructors:
  `PseudoClassSelector::runtime(RuntimePseudoClass)`,
  `PseudoClassSelector::structural(StructuralSelector)`,
  `PseudoClassSelector::selector_list(SelectorListPseudoClass)`, and
  `PseudoClassSelector::has(RelativeSelectorList)`.
- `SelectorSpecificity::new(ids, classes, elements)` accepts bounded unsigned
  component values and orders lexicographically. Use `u16` or another explicit
  bounded component type; do not use raw tuple aliases in the public API.
- Specificity contributions must match the style-owned lowering contract:
  `StyleKey` contributes `(1, 0, 0)`; classes, attributes, runtime
  pseudo-classes, structural pseudo-classes, and scope/root pseudo-classes
  contribute `(0, 1, 0)`; tags contribute `(0, 0, 1)`; `Any` contributes zero;
  compound and complex selectors sum their components; selector lists use the
  maximum selector specificity; `:where(...)` contributes zero; `:is(...)`,
  `:not(...)`, and `:has(...)` contribute the maximum specificity of their
  argument list; `:nth-child(... of <selector-list>)` and
  `:nth-last-child(... of <selector-list>)` contribute `(0, 1, 0)` plus the
  maximum specificity of the filter list.
- `RulePrecedence` should include specificity between layer order and source
  order. `RulePrecedence::default()` remains zero-specificity; after sheet
  integration in Task 5, legacy `Rule::new` and `Sheet::push_rule` derive
  specificity from their selector, while authored rules preserve the
  specificity supplied by root.
- `SelectorList::matches(...)` returns true if any selector matches.
- `SelectorList::max_specificity()` returns the maximum specificity in the
  list. `:where()` contributes zero specificity for its argument list, while
  `:is()`, `:not()`, and `:has()` contribute their most specific argument.
- `RelativeSelector` stores a leading combinator plus a selector. For `:has()`,
  descendant is the default leading combinator when root lowers a relative
  selector without an explicit `>`, `+`, or `~`.
- When a relative selector stores a complex selector, the leading combinator
  anchors the first compound against nodes related to the `:has` subject; the
  rest of the complex selector is then evaluated forward from that anchor. This
  is required for selectors such as `:has(> .card .target)` and
  `:has(+ .panel .target)`.
- `SelectorMatchContext` carries the subject node id, traversal mode, optional
  scope root id, and optional document root id. It must not borrow a CSS type.
- Existing convenience constructors such as `Selector::tag`, `Selector::class`,
  `Selector::key`, `Selector::attribute_exists`,
  `Selector::attribute_equals`, and `Selector::complex` keep working unless a
  task explicitly replaces them with a stricter fallible constructor and updates
  type-safety tests.

## CSS-To-Style Lowering Contract

Root lowers CSS selector syntax into these style types after this plan lands:

- `CssSelectorList` -> `SelectorList`
- `CssSelector` -> `Selector`
- `CssComplexSelector` -> `ComplexSelector`
- `CssComplexSelectorPart` -> `ComplexSelectorPart`
- `CssSelectorCombinator::{Descendant, Child, NextSibling, SubsequentSibling}`
  -> `Combinator::{Descendant, Child, Adjacent, Sibling}`
- `CssAttributeSelector` -> `AttributeSelector`
- `CssAttributeMatcher` -> `AttributeMatcher`
- `CssAttributeCaseSensitivity` -> `AttributeCaseSensitivity`
- `CssPseudoClass` -> `PseudoClassSelector`
- `CssNthPattern` -> `NthPattern`
- `CssNthChildPattern` -> `NthSelector`
- `CssPseudoSelectorList` -> `SelectorList`
- `CssRelativeSelectorList` -> `RelativeSelectorList`

Root must not pass CSS pseudo-elements to this plan's selector model. When the
CSS selector subject has a pseudo-element sequence, root should defer lowering
until Operation 6 adds style buckets.

## Selector Matching Semantics

Selector matching remains right-to-left for complex selectors:

- the final compound selector matches the subject node;
- descendant combinator walks ancestors;
- child combinator checks the direct parent;
- adjacent combinator checks the previous sibling;
- sibling combinator walks previous siblings;
- relative selectors for `:has()` walk away from the subject according to the
  leading combinator, then evaluate the stored selector at candidate nodes.
  If the stored selector is complex, the candidate node is the anchor for the
  complex selector's first compound rather than the final subject.

Attribute matching:

- `Exists` matches by name only;
- `Equals` matches exact value;
- `Includes` matches whitespace-separated tokens;
- `DashMatch` matches exact value or `value-...`;
- `Prefix`, `Suffix`, and `Substring` use string prefix/suffix/contains;
- `AsciiCaseInsensitive` compares ASCII-insensitively;
- `ExplicitSensitive` compares exactly;
- `DocumentDefault` compares exactly until root/style introduce document
  language-specific attribute defaults in a later operation.

Structural matching:

- `FirstChild`, `LastChild`, and `OnlyChild` use the current traversal's parent
  and children list;
- `Empty` is true when the node has no child element nodes in the current
  traversal and does not carry text content;
- type-filtered selectors compare sibling `StyleTag` values, and nodes without
  a tag never match type-filtered selectors;
- `NthPattern` supports CSS `odd`, `even`, integer, and signed `An+B`
  patterns, including negative `a`;
- `NthChild` and `NthLastChild` with an `of` selector list filter siblings to
  those matching the filter list before calculating position.

Runtime pseudo-classes:

- Existing state facts remain style-owned.
- Add only the runtime facts required by the current CSS pseudo-class list:
  focus-visible, enabled participation, required/optional, valid/invalid,
  placeholder-shown, modal, fullscreen, popover-open, default, indeterminate,
  read-only/read-write, in-range/out-of-range.
- Avoid deriving inverse states implicitly where participation matters. For
  example, `:enabled` is not simply `!disabled`; root must be able to indicate
  whether a node participates in enabled/disabled semantics.

Root and scope:

- `:root` matches the document root id from `SelectorMatchContext`; if no root
  is provided, it falls back to a node with no parent in the selected traversal.
- `:scope` matches the scope root id from `SelectorMatchContext`; if no scope
  root is provided, it matches the subject node.
- CSS nesting's `&` lowers into a scope-anchor fact on `Compound`, and that
  fact matches the same node as `:scope` in this operation.

## Task 1: Add Specificity And Selector List Receiving Types

**Files:**

- Modify: `src/selector.rs`
- Modify: `src/sheet.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_selector_list_empty.rs`
- Create: `tests/compile_fail/invalid_complex_selector_part_struct_literal.rs`
- Create: `tests/compile_fail/invalid_complex_selector_struct_literal.rs`
- Create: `tests/compile_fail/invalid_selector_specificity_struct_literal.rs`
- Update generated `.stderr` files through `trybuild` only.

- [ ] **Step 1: Add failing tests for selector lists and specificity**

Add unit tests in `src/selector.rs`:

```rust
#[test]
fn selector_list_matches_any_selector_and_rejects_empty_lists() {
    let tree = TestTree::new(vec![TestNode::new(0).tag("button").class("primary")]);
    let list = SelectorList::try_new([
        Selector::tag("label").unwrap(),
        Selector::class("primary").unwrap(),
    ])
    .unwrap();

    assert!(list.matches(&tree, SelectorMatchContext::for_subject(0)).unwrap());
    assert_eq!(
        SelectorList::try_new([]).unwrap_err().code(),
        ErrorCode::InvalidSelector
    );
}

#[test]
fn selector_specificity_orders_between_layer_and_source_order() {
    let low_specificity_late =
        RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(9))
            .with_specificity(SelectorSpecificity::new(0, 0, 1));
    let high_specificity_early =
        RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(1))
            .with_specificity(SelectorSpecificity::new(0, 1, 0));
    let higher_layer =
        RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(0))
            .with_specificity(SelectorSpecificity::zero());

    assert!(high_specificity_early > low_specificity_late);
    assert!(higher_layer > high_specificity_early);
}
```

Run:

```sh
cargo test -p surgeist-style selector_list
cargo test -p surgeist-style specificity
```

Expected: fail because `SelectorList`, `SelectorSpecificity`, and precedence
integration do not exist yet.

- [ ] **Step 2: Implement `SelectorSpecificity`**

In `src/selector.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SelectorSpecificity {
    ids: u16,
    classes: u16,
    elements: u16,
}

impl SelectorSpecificity {
    #[must_use]
    pub const fn new(ids: u16, classes: u16, elements: u16) -> Self {
        Self {
            ids,
            classes,
            elements,
        }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    #[must_use]
    pub const fn ids(self) -> u16 {
        self.ids
    }

    #[must_use]
    pub const fn classes(self) -> u16 {
        self.classes
    }

    #[must_use]
    pub const fn elements(self) -> u16 {
        self.elements
    }

    #[must_use]
    pub const fn saturating_add(self, other: Self) -> Self {
        Self::new(
            self.ids.saturating_add(other.ids),
            self.classes.saturating_add(other.classes),
            self.elements.saturating_add(other.elements),
        )
    }
}
```

Do not expose public fields. Do not use a public tuple alias.

- [ ] **Step 3: Add explicit specificity calculation tests**

Add unit tests in `src/selector.rs` that pin the contribution table:

```rust
#[test]
fn selector_specificity_uses_css_lowering_contract() {
    let key = Selector::key("submit").unwrap();
    let class = Selector::class("primary").unwrap();
    let attr = Selector::attribute_exists("data-mode").unwrap();
    let tag = Selector::tag("button").unwrap();

    assert_eq!(key.specificity(), SelectorSpecificity::new(1, 0, 0));
    assert_eq!(class.specificity(), SelectorSpecificity::new(0, 1, 0));
    assert_eq!(attr.specificity(), SelectorSpecificity::new(0, 1, 0));
    assert_eq!(tag.specificity(), SelectorSpecificity::new(0, 0, 1));
}

#[test]
fn selector_specificity_sums_compound_and_complex_and_uses_list_max() {
    let compound = Selector::compound()
        .tag("button").unwrap()
        .key("submit").unwrap()
        .class("primary").unwrap()
        .attribute_exists("data-mode").unwrap()
        .selector();
    let complex = Selector::complex([
        ComplexSelectorPart::root(Selector::compound().tag("form").unwrap()),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().class("primary").unwrap(),
        ),
    ]).unwrap();
    let list = Selector::list(SelectorList::try_new([
        Selector::tag("button").unwrap(),
        Selector::key("submit").unwrap(),
    ]).unwrap());

    assert_eq!(compound.specificity(), SelectorSpecificity::new(1, 2, 1));
    assert_eq!(complex.specificity(), SelectorSpecificity::new(0, 1, 1));
    assert_eq!(list.specificity(), SelectorSpecificity::new(1, 0, 0));
}
```

If workers choose a cleaner construction API for test setup, the reviewer must
confirm the replacement still proves compound, complex, and list specificity.

- [ ] **Step 4: Add selector lists**

In `src/selector.rs`, add:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorList {
    selectors: Vec<Selector>,
}

impl SelectorList {
    pub fn try_new(selectors: impl IntoIterator<Item = Selector>) -> Result<Self> {
        let selectors: Vec<_> = selectors.into_iter().collect();
        if selectors.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidSelector,
                "selector list must not be empty",
            ));
        }
        Ok(Self { selectors })
    }

    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }

    pub fn matches<T: Tree>(
        &self,
        tree: &T,
        context: SelectorMatchContext<T::Id>,
    ) -> Result<bool> {
        for selector in &self.selectors {
            if selector.matches_with_context(tree, context)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[must_use]
    pub fn max_specificity(&self) -> SelectorSpecificity {
        self.selectors
            .iter()
            .map(Selector::specificity)
            .max()
            .unwrap_or_else(SelectorSpecificity::zero)
    }
}
```

If `SelectorMatchContext` does not exist yet, add the minimal context in this
task:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectorMatchContext<Id> {
    subject: Id,
    traversal: Traversal,
    root: Option<Id>,
    scope: Option<Id>,
}

impl<Id: Copy> SelectorMatchContext<Id> {
    #[must_use]
    pub const fn new(subject: Id, traversal: Traversal) -> Self {
        Self {
            subject,
            traversal,
            root: None,
            scope: None,
        }
    }

    #[must_use]
    pub const fn for_subject(subject: Id) -> Self {
        Self::new(subject, Traversal::Canonical)
    }

    #[must_use]
    pub const fn with_root(mut self, root: Id) -> Self {
        self.root = Some(root);
        self
    }

    #[must_use]
    pub const fn with_scope(mut self, scope: Id) -> Self {
        self.scope = Some(scope);
        self
    }

    #[must_use]
    pub const fn subject(self) -> Id {
        self.subject
    }

    #[must_use]
    pub const fn traversal(self) -> Traversal {
        self.traversal
    }
}
```

Keep `Selector::matches(tree, id, traversal)` as a compatibility convenience
only if it delegates to `matches_with_context`; do not add a second matching
implementation.

- [ ] **Step 5: Replace public complex selector parts with private-field models**

Replace the current public `Selector::Complex(Vec<Part>)` / public-field
`Part` representation with private-field models:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexSelector {
    parts: Vec<ComplexSelectorPart>,
}

impl ComplexSelector {
    pub fn try_new(parts: impl IntoIterator<Item = ComplexSelectorPart>) -> Result<Self> {
        let parts: Vec<_> = parts.into_iter().collect();
        validate_complex_parts(&parts)?;
        Ok(Self { parts })
    }

    pub fn parts(&self) -> &[ComplexSelectorPart] {
        &self.parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplexSelectorPart {
    combinator: Option<Combinator>,
    selector: Compound,
}

impl ComplexSelectorPart {
    #[must_use]
    pub const fn root(selector: Compound) -> Self {
        Self {
            combinator: None,
            selector,
        }
    }

    #[must_use]
    pub const fn related(combinator: Combinator, selector: Compound) -> Self {
        Self {
            combinator: Some(combinator),
            selector,
        }
    }

    #[must_use]
    pub const fn combinator(&self) -> Option<Combinator> {
        self.combinator
    }

    #[must_use]
    pub const fn selector(&self) -> &Compound {
        &self.selector
    }
}
```

`validate_complex_parts` must reject empty lists, first related parts, and
unrelated non-first parts. `Selector::complex(parts)` may remain as a fallible
convenience wrapper around `ComplexSelector::try_new(parts)`, but `Part` should
not remain as a public compatibility alias.

Add compile-fail tests:

```rust
use surgeist_style::{ComplexSelectorPart, Selector};

fn main() {
    let _part = ComplexSelectorPart {
        combinator: None,
        selector: Selector::compound(),
    };
}
```

```rust
use surgeist_style::ComplexSelector;

fn main() {
    let _selector = ComplexSelector { parts: Vec::new() };
}
```

Update stderr with:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

- [ ] **Step 6: Integrate specificity into `RulePrecedence`**

Modify `src/sheet.rs` or the local precedence module that defines
`RulePrecedence` so it stores and orders:

1. `LayerOrder`
2. `SelectorSpecificity`
3. `SourceOrder`

Add methods:

```rust
pub const fn specificity(self) -> SelectorSpecificity;
pub const fn with_specificity(self, specificity: SelectorSpecificity) -> Self;
```

`RulePrecedence::default()` remains zero-specificity at this point.
`push_authored_rule` keeps the specificity supplied by root/coordinator tests.
Task 5 derives legacy rule specificity from selectors when integrating the
sheet rule constructors.

- [ ] **Step 7: Export and type-safety-test the new types**

Update `src/lib.rs` exports to include:

```rust
ComplexSelector, ComplexSelectorPart, SelectorList, SelectorMatchContext,
SelectorSpecificity
```

Add compile-pass usage:

```rust
let specificity = SelectorSpecificity::new(0, 1, 0);
assert!(specificity > SelectorSpecificity::new(0, 0, 1));
let selector_list = SelectorList::try_new([
    Selector::tag("button")?,
    Selector::class("primary")?,
])?;
assert_eq!(selector_list.selectors().len(), 2);
```

Add compile-fail tests:

```rust
use surgeist_style::SelectorList;

fn main() {
    let _list = SelectorList { selectors: Vec::new() };
}
```

```rust
use surgeist_style::SelectorSpecificity;

fn main() {
    let _specificity = SelectorSpecificity {
        ids: 0,
        classes: 1,
        elements: 0,
    };
}
```

Update stderr with:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

- [ ] **Step 8: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style selector_list
cargo test -p surgeist-style specificity
cargo test -p surgeist-style complex
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 2: Expand Attribute Matchers And Runtime State Facts

**Files:**

- Modify: `src/identity.rs`
- Modify: `src/state.rs`
- Modify: `src/selector.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add failing tests for attribute matcher variants**

Add tests in `src/selector.rs`:

```rust
#[test]
fn attribute_selector_supports_css_matcher_variants() {
    let tree = TestTree::new(vec![
        TestNode::new(0)
            .attribute("data-tags", "primary featured")
            .attribute("lang", "en-US")
            .attribute("data-id", "Card-Primary"),
    ]);

    assert!(AttributeSelector::includes("data-tags", "featured").unwrap().matches(&tree, 0).unwrap());
    assert!(AttributeSelector::dash_match("lang", "en").unwrap().matches(&tree, 0).unwrap());
    assert!(AttributeSelector::prefix("data-id", "Card").unwrap().matches(&tree, 0).unwrap());
    assert!(AttributeSelector::suffix("data-id", "Primary").unwrap().matches(&tree, 0).unwrap());
    assert!(AttributeSelector::substring("data-id", "rd-P").unwrap().matches(&tree, 0).unwrap());
    assert!(AttributeSelector::equals_with_case(
        "data-id",
        "card-primary",
        AttributeCaseSensitivity::AsciiCaseInsensitive,
    )
    .unwrap()
    .matches(&tree, 0)
    .unwrap());
    assert!(!AttributeSelector::equals_with_case(
        "data-id",
        "card-primary",
        AttributeCaseSensitivity::ExplicitSensitive,
    )
    .unwrap()
    .matches(&tree, 0)
    .unwrap());
}
```

Run:

```sh
cargo test -p surgeist-style attribute_selector_supports_css_matcher_variants
```

Expected: fail because only exists/equals matching exists.

- [ ] **Step 2: Implement attribute matcher models**

Replace the two-variant `AttributeSelector` with private-field or structured
variants equivalent to:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeSelector {
    Exists { name: StyleAttributeName },
    Matcher {
        name: StyleAttributeName,
        matcher: AttributeMatcher,
        case_sensitivity: AttributeCaseSensitivity,
    },
}
```

Add constructors:

```rust
pub fn includes(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self>;
pub fn dash_match(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self>;
pub fn prefix(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self>;
pub fn suffix(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self>;
pub fn substring(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self>;
pub fn equals_with_case(
    name: impl AsRef<str>,
    value: impl AsRef<str>,
    case_sensitivity: AttributeCaseSensitivity,
) -> Result<Self>;
```

Keep `exists` and `equals` as constructors. `equals` should use
`AttributeCaseSensitivity::DocumentDefault`.

Implement matcher helpers:

```rust
fn compare_attribute_value(
    actual: &StyleAttributeValue,
    expected: &StyleAttributeValue,
    case_sensitivity: AttributeCaseSensitivity,
) -> bool;
```

`DocumentDefault` and `ExplicitSensitive` compare exactly in this operation.
`AsciiCaseInsensitive` uses ASCII-insensitive comparison and matching.

- [ ] **Step 3: Add runtime pseudo-class facts**

Extend `StateFlag` and `StyleState` with the runtime facts from CSS:

```rust
FocusVisible,
Enabled,
Required,
Optional,
Valid,
Invalid,
PlaceholderShown,
Modal,
Fullscreen,
PopoverOpen,
Default,
Indeterminate,
ReadOnly,
ReadWrite,
InRange,
OutOfRange,
```

Model tri-state or participation-sensitive facts explicitly in `StyleState`
where needed:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RangeState {
    InRange,
    OutOfRange,
}

enabled: Option<bool>,
required: Option<bool>,
valid: Option<bool>,
read_write: Option<bool>,
range_state: Option<RangeState>,
```

Use `Option<bool>` and `Option<RangeState>` when neither side of a pseudo-class
should match because the node does not participate in that semantic domain. Do
not model `:enabled` as `!disabled`, `:optional` as unconditionally
`!required`, or `:out-of-range` as unconditionally `!in-range`.

- [ ] **Step 4: Add runtime pseudo-class selector model**

Add `RuntimePseudoClass` and `PseudoClassSelector::Runtime`, then make
`Selector::state(StateFlag)` delegate to the same matching path.

Extend `Compound` with private pseudo-class storage so root can lower ordinary
CSS compounds such as `button:hover`, `.item:not(.disabled)`, and
`li:nth-child(2)` without splitting pseudo-classes into separate complex parts:

```rust
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Compound {
    tag: Option<StyleTag>,
    key: Option<StyleKey>,
    classes: Vec<StyleClass>,
    states: Vec<StateFlag>,
    attributes: Vec<AttributeSelector>,
    pseudo_classes: Vec<PseudoClassSelector>,
    position: Option<PositionSelector>,
    scope_anchor: bool,
}

impl Compound {
    #[must_use]
    pub fn pseudo(mut self, pseudo_class: PseudoClassSelector) -> Self {
        self.pseudo_classes.push(pseudo_class);
        self
    }

    pub fn runtime_pseudo(mut self, pseudo_class: RuntimePseudoClass) -> Self {
        self.pseudo_classes
            .push(PseudoClassSelector::runtime(pseudo_class));
        self
    }

    pub fn pseudo_classes(&self) -> &[PseudoClassSelector] {
        &self.pseudo_classes
    }
}
```

`Compound::matches` must require all pseudo-classes to match the same subject
under the current `SelectorMatchContext`. `Compound::specificity()` must add
pseudo-class specificity to tag/key/class/attribute specificity. Existing
`position: Option<PositionSelector>` can remain during the migration, but
structural selectors added by this plan should use `PseudoClassSelector` so CSS
pseudo-classes have one model.

Add tests:

```rust
#[test]
fn runtime_pseudo_classes_use_explicit_style_state_facts() {
    let tree = TestTree::new(vec![
        TestNode::new(0).state(
            StyleState::default()
                .with_enabled(Some(true))
                .with_focus_visible(true)
                .with_valid(Some(false))
                .with_read_write(Some(true)),
        ),
    ]);

    assert!(Selector::pseudo(PseudoClassSelector::runtime(RuntimePseudoClass::Enabled))
        .matches(&tree, 0, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::runtime(RuntimePseudoClass::Invalid))
        .matches(&tree, 0, Traversal::Canonical)
        .unwrap());
    assert!(!Selector::pseudo(PseudoClassSelector::runtime(RuntimePseudoClass::Disabled))
        .matches(&tree, 0, Traversal::Canonical)
        .unwrap());
}

#[test]
fn compound_selectors_can_combine_tag_class_attribute_and_runtime_pseudo_classes() {
    let tree = TestTree::new(vec![
        TestNode::new(0)
            .tag("button")
            .class("primary")
            .attribute("data-mode", "submit")
            .state(StyleState::default().with_hovered(true)),
    ]);
    let selector = Selector::compound()
        .tag("button").unwrap()
        .class("primary").unwrap()
        .attribute_exists("data-mode").unwrap()
        .runtime_pseudo(RuntimePseudoClass::Hover)
        .selector();

    assert!(selector.matches(&tree, 0, Traversal::Canonical).unwrap());
    assert_eq!(selector.specificity(), SelectorSpecificity::new(0, 3, 1));
}
```

- [ ] **Step 5: Export and compile-pass-test new APIs**

Export:

```rust
AttributeCaseSensitivity, AttributeMatcher, PseudoClassSelector,
RangeState, RuntimePseudoClass
```

Add compile-pass usage that constructs an ASCII-insensitive attribute selector
and a runtime pseudo-class selector, and sets a `StyleState` range fact:

```rust
let state = StyleState::default().with_range_state(Some(RangeState::InRange));
assert!(state.has_flag(StateFlag::InRange));
```

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style attribute_selector
cargo test -p surgeist-style runtime_pseudo
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 3: Model Structural Selectors And Nth Patterns

**Files:**

- Modify: `src/selector.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_nth_selector_struct_literal.rs`
- Update generated `.stderr` files through `trybuild` only.

- [ ] **Step 1: Add failing structural selector tests**

Add tests in `src/selector.rs`:

Extend the local `TestNode` helper in `src/selector.rs` with:

```rust
fn text(mut self) -> Self {
    self.text = true;
    self
}
```

and ensure its `Tree::node` implementation copies the helper's `text` value
into `Node { text, .. }`.

```rust
#[test]
fn structural_selectors_match_child_and_type_positions() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1, 2, 3, 4]),
        TestNode::new(1).tag("button"),
        TestNode::new(2).tag("label"),
        TestNode::new(3).tag("button"),
        TestNode::new(4).tag("button"),
    ]);

    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::FirstChild))
        .matches(&tree, 1, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::LastChild))
        .matches(&tree, 4, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::NthOfType(
        NthPattern::integer(2),
    )))
    .matches(&tree, 3, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(
        StructuralSelector::NthLastOfType(NthPattern::integer(1)),
    ))
    .matches(&tree, 4, Traversal::Canonical)
    .unwrap());
}

#[test]
fn compound_selectors_can_combine_tag_and_structural_pseudo_classes() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("ul").children([1, 2]),
        TestNode::new(1).tag("li"),
        TestNode::new(2).tag("li"),
    ]);
    let selector = Selector::compound()
        .tag("li").unwrap()
        .pseudo(PseudoClassSelector::structural(StructuralSelector::NthChild(
            NthSelector::new(NthPattern::integer(2), None),
        )))
        .selector();

    assert!(selector.matches(&tree, 2, Traversal::Canonical).unwrap());
    assert_eq!(selector.specificity(), SelectorSpecificity::new(0, 1, 1));
}

#[test]
fn structural_selectors_cover_only_empty_reverse_filtered_and_type_edge_cases() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1, 2, 3, 4, 5]),
        TestNode::new(1).tag("button").class("candidate"),
        TestNode::new(2).tag("button").class("candidate"),
        TestNode::new(3).tag("label"),
        TestNode::new(4).tag("button").class("candidate"),
        TestNode::new(5),
        TestNode::new(6).tag("empty"),
        TestNode::new(7).tag("text").text(),
        TestNode::new(8).tag("single-parent").children([9]),
        TestNode::new(9).tag("only"),
    ]);

    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::OnlyChild))
        .matches(&tree, 9, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::Empty))
        .matches(&tree, 6, Traversal::Canonical)
        .unwrap());
    assert!(!Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::Empty))
        .matches(&tree, 7, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::FirstOfType))
        .matches(&tree, 1, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::LastOfType))
        .matches(&tree, 4, Traversal::Canonical)
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::OnlyOfType))
        .matches(&tree, 3, Traversal::Canonical)
        .unwrap());
    assert!(!Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::FirstOfType))
        .matches(&tree, 5, Traversal::Canonical)
        .unwrap());

    let filter = SelectorList::try_new([Selector::class("candidate").unwrap()]).unwrap();
    let nth_last = Selector::pseudo(PseudoClassSelector::structural(
        StructuralSelector::NthLastChild(NthSelector::new(NthPattern::integer(2), Some(filter))),
    ));
    assert!(nth_last.matches(&tree, 2, Traversal::Canonical).unwrap());
    assert!(!nth_last.matches(&tree, 1, Traversal::Canonical).unwrap());
}
```

Add an `of <selector-list>` test:

```rust
#[test]
fn nth_child_can_filter_siblings_by_selector_list() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1, 2, 3, 4]),
        TestNode::new(1).tag("button").class("candidate"),
        TestNode::new(2).tag("button"),
        TestNode::new(3).tag("button").class("candidate"),
        TestNode::new(4).tag("button").class("candidate"),
    ]);
    let filter = SelectorList::try_new([Selector::class("candidate").unwrap()]).unwrap();
    let selector = Selector::pseudo(PseudoClassSelector::structural(
        StructuralSelector::NthChild(NthSelector::new(NthPattern::integer(2), Some(filter))),
    ));

    assert!(selector.matches(&tree, 3, Traversal::Canonical).unwrap());
    assert!(!selector.matches(&tree, 4, Traversal::Canonical).unwrap());
}
```

Run:

```sh
cargo test -p surgeist-style structural
cargo test -p surgeist-style nth_child
```

Expected: fail until structural selector APIs exist.

- [ ] **Step 2: Replace `Nth` with signed CSS nth pattern support**

Keep the existing `Nth` API only if it can delegate to the new model without a
second implementation. Add:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NthPattern {
    a: i32,
    b: i32,
}

impl NthPattern {
    #[must_use]
    pub const fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    #[must_use]
    pub const fn odd() -> Self {
        Self::new(2, 1)
    }

    #[must_use]
    pub const fn even() -> Self {
        Self::new(2, 0)
    }

    #[must_use]
    pub const fn integer(position: i32) -> Self {
        Self::new(0, position)
    }

    #[must_use]
    pub fn matches(self, one_based_position: usize) -> bool {
        let position = i32::try_from(one_based_position).unwrap_or(i32::MAX);
        if position <= 0 {
            return false;
        }
        if self.a == 0 {
            return position == self.b;
        }
        let delta = position - self.b;
        if self.a > 0 {
            delta >= 0 && delta % self.a == 0
        } else {
            delta <= 0 && delta % self.a == 0
        }
    }
}
```

Add tests for `odd`, `even`, `3`, `2n+1`, `-n+3`, and unmatched zero/negative
positions.

- [ ] **Step 3: Add `NthSelector` and structural selector matching**

Add:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NthSelector {
    pattern: NthPattern,
    filter: Option<SelectorList>,
}

impl NthSelector {
    #[must_use]
    pub const fn new(pattern: NthPattern, filter: Option<SelectorList>) -> Self {
        Self { pattern, filter }
    }

    #[must_use]
    pub const fn pattern(&self) -> NthPattern {
        self.pattern
    }

    #[must_use]
    pub const fn filter(&self) -> Option<&SelectorList> {
        self.filter.as_ref()
    }
}
```

If `const fn new` is not possible because `SelectorList` is not const-friendly,
use a non-const constructor with the same signature and private fields.

Implement child position, reverse child position, type-filtered position, and
reverse type-filtered position helpers in `src/selector.rs`. The helpers should
collect sibling ids from `Tree::children(parent, traversal)`, optionally filter
with `SelectorList`, then calculate one-based position.

- [ ] **Step 4: Implement `:empty`**

Use `Tree::children(id, traversal)` and `Node::text`:

```rust
fn node_is_empty<T: Tree>(tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
    let node = tree.node(id)?;
    if node.text {
        return Ok(false);
    }
    Ok(tree.children(id, traversal)?.next().is_none())
}
```

This is the current style fact contract. Do not infer text content from
retained or DOM nodes.

- [ ] **Step 5: Add type-safety tests**

Export:

```rust
NthPattern, NthSelector, StructuralSelector
```

Add compile-fail:

```rust
use surgeist_style::{NthPattern, NthSelector};

fn main() {
    let _selector = NthSelector {
        pattern: NthPattern::odd(),
        filter: None,
    };
}
```

Update stderr with:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

- [ ] **Step 6: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style structural
cargo test -p surgeist-style nth
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 4: Add Selector-List Pseudo-Classes And Relative `:has`

**Files:**

- Modify: `src/selector.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_relative_selector_list_empty.rs`
- Update generated `.stderr` files through `trybuild` only.

- [ ] **Step 1: Add failing tests for `:not`, `:is`, and `:where`**

Add tests in `src/selector.rs`:

```rust
#[test]
fn selector_list_pseudo_classes_match_over_nested_selector_lists() {
    let tree = TestTree::new(vec![TestNode::new(0).tag("button").class("primary")]);
    let primary = SelectorList::try_new([Selector::class("primary").unwrap()]).unwrap();
    let danger = SelectorList::try_new([Selector::class("danger").unwrap()]).unwrap();

    assert!(Selector::pseudo(PseudoClassSelector::selector_list(
        SelectorListPseudoClass::Is(primary.clone()),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::selector_list(
        SelectorListPseudoClass::Where(primary),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::selector_list(
        SelectorListPseudoClass::Not(danger),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
}

#[test]
fn compound_selectors_can_combine_class_and_selector_list_pseudo_classes() {
    let tree = TestTree::new(vec![TestNode::new(0).tag("button").class("item")]);
    let disabled = SelectorList::try_new([Selector::class("disabled").unwrap()]).unwrap();
    let selector = Selector::compound()
        .class("item").unwrap()
        .pseudo(PseudoClassSelector::selector_list(SelectorListPseudoClass::Not(
            disabled,
        )))
        .selector();

    assert!(selector.matches(&tree, 0, Traversal::Canonical).unwrap());
    assert_eq!(selector.specificity(), SelectorSpecificity::new(0, 2, 0));
}
```

Run:

```sh
cargo test -p surgeist-style selector_list_pseudo
```

Expected: fail until selector-list pseudo-classes exist.

- [ ] **Step 2: Add failing relative selector and `:has` tests**

Add tests:

```rust
#[test]
fn has_matches_relative_descendant_child_and_sibling_selectors() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("section").children([1, 2, 3, 4]),
        TestNode::new(1).tag("button").class("primary"),
        TestNode::new(2).tag("label"),
        TestNode::new(3).tag("button").class("later"),
        TestNode::new(4).tag("input").class("adjacent-target"),
    ]);

    let descendant = RelativeSelector::new(
        Combinator::Descendant,
        Selector::class("primary").unwrap(),
    );
    let child = RelativeSelector::new(Combinator::Child, Selector::tag("label").unwrap());
    let sibling = RelativeSelector::new(Combinator::Sibling, Selector::class("later").unwrap());
    let adjacent = RelativeSelector::new(
        Combinator::Adjacent,
        Selector::class("adjacent-target").unwrap(),
    );
    let wrong_adjacent = RelativeSelector::new(
        Combinator::Adjacent,
        Selector::class("later").unwrap(),
    );

    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([descendant]).unwrap(),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([child]).unwrap(),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([sibling]).unwrap(),
    ))
    .matches(&tree, 1, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([adjacent]).unwrap(),
    ))
    .matches(&tree, 3, Traversal::Canonical)
    .unwrap());
    assert!(!Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([wrong_adjacent]).unwrap(),
    ))
    .matches(&tree, 3, Traversal::Canonical)
    .unwrap());
}

#[test]
fn has_matches_complex_relative_selectors_from_child_and_adjacent_anchors() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("section").children([1, 4, 3]),
        TestNode::new(1).tag("div").class("card").children([2]),
        TestNode::new(2).tag("span").class("target"),
        TestNode::new(3).tag("aside").class("panel").children([5]),
        TestNode::new(4).tag("div").class("before-panel"),
        TestNode::new(5).tag("span").class("target"),
    ]);
    let child_complex = Selector::complex([
        ComplexSelectorPart::root(Selector::compound().class("card").unwrap()),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().class("target").unwrap(),
        ),
    ]).unwrap();
    let adjacent_complex = Selector::complex([
        ComplexSelectorPart::root(Selector::compound().class("panel").unwrap()),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().class("target").unwrap(),
        ),
    ]).unwrap();

    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([RelativeSelector::new(
            Combinator::Child,
            child_complex,
        )]).unwrap(),
    ))
    .matches(&tree, 0, Traversal::Canonical)
    .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([RelativeSelector::new(
            Combinator::Adjacent,
            adjacent_complex,
        )]).unwrap(),
    ))
    .matches(&tree, 4, Traversal::Canonical)
    .unwrap());
}
```

- [ ] **Step 3: Add failing pseudo-class specificity tests**

Add tests:

```rust
#[test]
fn selector_list_pseudo_class_specificity_uses_argument_rules() {
    let key_list = SelectorList::try_new([Selector::key("primary").unwrap()]).unwrap();
    let class_list = SelectorList::try_new([Selector::class("primary").unwrap()]).unwrap();
    let relative_key = RelativeSelectorList::try_new([RelativeSelector::new(
        Combinator::Descendant,
        Selector::key("target").unwrap(),
    )])
    .unwrap();

    assert_eq!(
        Selector::pseudo(PseudoClassSelector::selector_list(SelectorListPseudoClass::Where(
            key_list.clone(),
        )))
        .specificity(),
        SelectorSpecificity::zero()
    );
    assert_eq!(
        Selector::pseudo(PseudoClassSelector::selector_list(SelectorListPseudoClass::Is(
            key_list.clone(),
        )))
        .specificity(),
        SelectorSpecificity::new(1, 0, 0)
    );
    assert_eq!(
        Selector::pseudo(PseudoClassSelector::selector_list(SelectorListPseudoClass::Not(
            class_list,
        )))
        .specificity(),
        SelectorSpecificity::new(0, 1, 0)
    );
    assert_eq!(
        Selector::pseudo(PseudoClassSelector::has(relative_key)).specificity(),
        SelectorSpecificity::new(1, 0, 0)
    );
}
```

Add an nth-child filter specificity test in the structural selector test block:

```rust
#[test]
fn nth_child_filter_specificity_adds_filter_maximum() {
    let filter = SelectorList::try_new([Selector::key("candidate").unwrap()]).unwrap();
    let nth = Selector::pseudo(PseudoClassSelector::structural(StructuralSelector::NthChild(
        NthSelector::new(NthPattern::odd(), Some(filter)),
    )));

    assert_eq!(nth.specificity(), SelectorSpecificity::new(1, 1, 0));
}
```

- [ ] **Step 4: Implement selector-list pseudo-classes**

Add:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectorListPseudoClass {
    Not(SelectorList),
    Is(SelectorList),
    Where(SelectorList),
}
```

Matching:

- `Not(list)` is true when `list` does not match the subject;
- `Is(list)` is true when `list` matches the subject;
- `Where(list)` matches the same as `Is(list)`;
- specificity for `Where` is zero;
- specificity for `Not` and `Is` is `list.max_specificity()`.

- [ ] **Step 5: Implement relative selectors and `:has`**

Add:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelativeSelector {
    combinator: Combinator,
    selector: Box<Selector>,
}

impl RelativeSelector {
    #[must_use]
    pub fn new(combinator: Combinator, selector: Selector) -> Self {
        Self {
            combinator,
            selector: Box::new(selector),
        }
    }

    #[must_use]
    pub const fn combinator(&self) -> Combinator {
        self.combinator
    }

    #[must_use]
    pub fn selector(&self) -> &Selector {
        &self.selector
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelativeSelectorList {
    selectors: Vec<RelativeSelector>,
}
```

`RelativeSelectorList::try_new` rejects empty input. Matching is true if any
relative selector matches.

Candidate direction for `:has()`:

- `Child`: direct children of the subject;
- `Descendant`: all descendants in traversal order;
- `Adjacent`: next sibling of the subject;
- `Sibling`: following siblings of the subject.

For a one-compound stored selector, candidates are tested directly with that
selector. For a complex stored selector, each candidate is treated as the
anchor that must match the complex selector's first compound, and the complex
selector's remaining parts are evaluated forward from that anchor. Do not
evaluate only the final compound at the candidate; that would break
`:has(> .card .target)` and `:has(+ .panel .target)`.

Add a private helper requiring no new `Tree` trait method:

```rust
fn next_sibling<T: Tree>(tree: &T, id: T::Id, traversal: Traversal) -> Result<Option<T::Id>>;
```

Implement it by asking the parent for children and finding the item after the
subject.

- [ ] **Step 6: Type-safety and compile-pass coverage**

Export:

```rust
RelativeSelector, RelativeSelectorList, SelectorListPseudoClass
```

Add compile-pass usage constructing `:has(> .child)` and `:not(.disabled)`.

Add compile-fail:

```rust
use surgeist_style::RelativeSelectorList;

fn main() {
    let _list = RelativeSelectorList { selectors: Vec::new() };
}
```

Update stderr with:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style --test type_safety
```

- [ ] **Step 7: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style selector_list_pseudo
cargo test -p surgeist-style has_matches
cargo test -p surgeist-style specificity
cargo test -p surgeist-style --test type_safety
```

Expected: all pass before review.

## Task 5: Add Root, Scope, Compound Anchor, And Sheet Integration

**Files:**

- Modify: `src/selector.rs`
- Modify: `src/sheet.rs`
- Modify: `src/resolver.rs`
- Modify: `src/invalidation.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add failing tests for `:root`, `:scope`, and `&`**

Add tests in `src/selector.rs`:

```rust
#[test]
fn root_and_scope_pseudo_classes_use_match_context() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1]),
        TestNode::new(1).tag("section").children([2]),
        TestNode::new(2).tag("button"),
    ]);

    let context = SelectorMatchContext::new(2, Traversal::Canonical)
        .with_root(0)
        .with_scope(1);

    assert!(Selector::pseudo(PseudoClassSelector::Root)
        .matches_with_context(&tree, context.with_subject(0))
        .unwrap());
    assert!(Selector::pseudo(PseudoClassSelector::Scope)
        .matches_with_context(&tree, context.with_subject(1))
        .unwrap());
    assert!(!Selector::pseudo(PseudoClassSelector::Scope)
        .matches_with_context(&tree, context.with_subject(2))
        .unwrap());
}
```

Add a compound anchor test:

```rust
#[test]
fn compound_scope_anchor_matches_scope_node() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1]),
        TestNode::new(1).tag("section").class("scope"),
    ]);
    let selector = Selector::compound()
        .scope_anchor()
        .class("scope")
        .unwrap()
        .selector();

    assert!(selector
        .matches_with_context(
            &tree,
            SelectorMatchContext::new(1, Traversal::Canonical).with_scope(1),
        )
        .unwrap());
}
```

- [ ] **Step 2: Implement context subject replacement and root/scope matching**

Add:

```rust
impl<Id: Copy> SelectorMatchContext<Id> {
    #[must_use]
    pub const fn with_subject(mut self, subject: Id) -> Self {
        self.subject = subject;
        self
    }
}
```

Implement `:root` fallback by walking parents until no parent exists if no
explicit root exists in the context.

Implement `:scope` fallback as subject matching if no explicit scope exists.

- [ ] **Step 3: Add compound scope anchor**

Add a private `scope_anchor: bool` field to `Compound` with a builder:

```rust
#[must_use]
pub const fn scope_anchor(mut self) -> Self {
    self.scope_anchor = true;
    self
}
```

In `Compound::matches`, require the current node to match context scope when
`scope_anchor` is true.

- [ ] **Step 4: Pass selector match context through resolver rule matching**

Extend `src/resolver.rs` `Context` with optional selector root and scope ids:

```rust
pub struct Context<'a, T: Tree> {
    // existing fields
    selector_root: Option<T::Id>,
    selector_scope: Option<T::Id>,
}

impl<'a, T: Tree> Context<'a, T> {
    #[must_use]
    pub const fn selector_root(mut self, root: T::Id) -> Self {
        self.selector_root = Some(root);
        self
    }

    #[must_use]
    pub const fn selector_scope(mut self, scope: T::Id) -> Self {
        self.selector_scope = Some(scope);
        self
    }
}
```

When resolving sheet rules, build a selector context and call
`matches_with_context`:

```rust
let mut selector_context = SelectorMatchContext::new(context.node, context.traversal);
if let Some(root) = context.selector_root {
    selector_context = selector_context.with_root(root);
}
if let Some(scope) = context.selector_scope {
    selector_context = selector_context.with_scope(scope);
}

if rule.selector().matches_with_context(context.tree, selector_context)? {
    // existing candidate collection
}
```

Do not keep a separate resolver-only scope matching path. The resolver should
use the same `SelectorMatchContext` semantics as direct selector matching.

Add a resolver test:

```rust
#[test]
fn resolver_applies_scope_anchor_rules_only_with_matching_selector_scope() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("root").children([1, 2]),
        TestNode::new(1).tag("section").class("scope").children([3]),
        TestNode::new(2).tag("section").class("other").children([4]),
        TestNode::new(3).tag("button"),
        TestNode::new(4).tag("button"),
    ]);
    let selector = Selector::complex([
        ComplexSelectorPart::root(
            Selector::compound().scope_anchor().class("scope").unwrap(),
        ),
        ComplexSelectorPart::related(
            Combinator::Descendant,
            Selector::compound().tag("button").unwrap(),
        ),
    ]).unwrap();
    let sheet = Sheet::new().rule(
        selector,
        Declarations::new().try_set(Property::Color, Value::Color(Color::BLACK)).unwrap(),
    );
    let mut resolver = Resolver::new(sheet);

    let scoped = resolver
        .resolve(Context::new(&tree, 3).selector_root(0).selector_scope(1))
        .unwrap();
    let unscoped = resolver
        .resolve(Context::new(&tree, 4).selector_root(0).selector_scope(1))
        .unwrap();

    assert_eq!(scoped.text_color(), Color::BLACK);
    assert_ne!(unscoped.text_color(), Color::BLACK);
}
```

If the exact property helper differs, use an existing color/text-color
declaration helper from resolver tests. The test must prove resolver rule
matching receives `selector_scope`, not only direct selector calls.

- [ ] **Step 5: Integrate selector lists and specificity with sheet rules**

Update `Rule::new`, `Sheet::push_rule`, and `Sheet::push_authored_rule`
behavior so rule precedence specificity is derived from the rule selector when
the caller has not supplied explicit specificity.

One acceptable approach:

```rust
impl Rule {
    fn with_order(selector: Selector, declarations: Declarations, order: u32) -> Self {
        let specificity = selector.specificity();
        Self {
            precedence: RulePrecedence::default()
                .with_specificity(specificity)
                .with_source_order(SourceOrder::new(order)),
            // existing fields
        }
    }
}
```

For authored rules, preserve the supplied `RulePrecedence`; root may already
have supplied specificity. Add a test proving authored specificity affects
resolution inside the same layer before source order.

- [ ] **Step 6: Keep selector indexing sound**

Update `PrimaryKey` extraction:

- `Selector::List` should index by universal unless every selector in the list
  shares an obviously identical primary key. Prefer universal for correctness.
- `:has`, selector-list pseudo-classes, root/scope, and structural selectors
  should index universal unless they sit inside a compound that also has a key,
  class, or tag.
- Complex selectors should keep indexing by their subject/final compound.

Add a sheet test:

```rust
#[test]
fn selector_lists_and_has_rules_are_not_dropped_by_rule_index() {
    let tree = TestTree::new(vec![
        TestNode::new(0).tag("section").children([1]),
        TestNode::new(1).tag("button").class("primary"),
    ]);
    let list = Selector::list(SelectorList::try_new([
        Selector::tag("label").unwrap(),
        Selector::class("primary").unwrap(),
    ]).unwrap());
    let has = Selector::pseudo(PseudoClassSelector::has(
        RelativeSelectorList::try_new([RelativeSelector::new(
            Combinator::Child,
            Selector::class("primary").unwrap(),
        )]).unwrap(),
    ));
    let list_sheet = Sheet::new().rule(list, Declarations::new());
    let has_sheet = Sheet::new().rule(has, Declarations::new());

    assert_eq!(list_sheet.candidate_rule_count(&tree, 1).unwrap(), 1);
    assert_eq!(has_sheet.candidate_rule_count(&tree, 0).unwrap(), 1);
}
```

This test intentionally uses separate one-rule sheets because
`candidate_rule_count` counts raw index candidates, not rules that have already
matched the node.

- [ ] **Step 7: Add conservative selector invalidation summaries**

Extend `Scope` and `Change` so style can summarize selector rematch scope
honestly for ancestor-sensitive `:has`, previous-sibling-sensitive `:has(+/~)`,
and filtered structural selectors. Operation 14 may later replace this broad
scope with dependency-indexed invalidation, but this operation must not
under-report selector impacts.

Add a whole-tree scope bit:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scope {
    pub node: bool,
    pub siblings: bool,
    pub descendants: bool,
    pub whole_tree: bool,
}

impl Scope {
    pub const fn include_whole_tree(&mut self) {
        self.whole_tree = true;
    }
}
```

Existing constructors must initialize `whole_tree` to false, and existing
property/condition invalidation behavior should not set it unless a selector
fact change requires broad rematch.

Add a selector-focused helper:

```rust
impl Change {
    pub fn from_selector_fact_change(fact: SelectorFactChange) -> Self
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectorFactChange {
    Tag,
    Key,
    Class,
    Attribute,
    RuntimeState,
    Structure,
    Scope,
}
```

Required behavior:

- tag, key, class, attribute, runtime state, scope, and structure changes set
  `rematch = true` and include `whole_tree` scope;
- this conservative whole-tree rematch is required because a descendant class,
  attribute, or state change can affect ancestor `:has(...)`, a following
  sibling change can affect previous-sibling `:has(+ ...)` and `:has(~ ...)`,
  and `:nth-child(... of <selector-list>)` can affect sibling matches;
- this helper should not set property invalidation flags, because rematch will
  identify affected rules before property invalidation.

Add tests:

```rust
#[test]
fn selector_fact_changes_use_whole_tree_rematch_for_has_and_filtered_structural_safety() {
    for fact in [
        SelectorFactChange::Tag,
        SelectorFactChange::Key,
        SelectorFactChange::Class,
        SelectorFactChange::Attribute,
        SelectorFactChange::RuntimeState,
        SelectorFactChange::Structure,
        SelectorFactChange::Scope,
    ] {
        let change = Change::from_selector_fact_change(fact);
        assert!(change.rematch);
        assert!(change.scope.whole_tree);
        assert_eq!(change.invalidation, Invalidation::empty());
    }
}
```

Also add comments in the test explaining the three motivating cases:
ancestor `:has(.changed)`, previous-sibling `:has(+ .changed)`, and
`:nth-child(of .candidate)` sibling reshuffling.

Export `SelectorFactChange` from `src/lib.rs`, and add compile-pass usage:

```rust
let selector_change = Change::from_selector_fact_change(SelectorFactChange::Class);
assert!(selector_change.rematch);
assert!(selector_change.scope.whole_tree);
```

- [ ] **Step 8: Run focused checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style root_and_scope
cargo test -p surgeist-style compound_scope
cargo test -p surgeist-style selector_lists_and_has
cargo test -p surgeist-style selector_fact_change
cargo test -p surgeist-style resolver_applies_scope_anchor
cargo test -p surgeist-style resolver
```

Expected: all pass before review.

## Task 6: Final Integration And Boundary Checks

**Files:**

- Modify this plan if implementation discovers an API correction that
  reviewers accepted.
- Verify the full crate.

- [ ] **Step 1: Confirm no `surgeist-css` dependency was introduced**

Run:

```sh
rg -n "surgeist_css|surgeist-css" Cargo.toml src tests
```

Expected: no output. Planning files may mention `surgeist-css`; code,
dependency, and tests must not.

- [ ] **Step 2: Confirm pseudo-elements and condition expansion stayed out**

Run:

```sh
git diff -- src tests | rg -n "PseudoElement|pseudo-element|FontFace|Keyframes|Media|Container|surgeist_css|surgeist-css|important"
```

Expected:

- no new `surgeist-css` references;
- no pseudo-element bucket implementation;
- no font-face, keyframe, media/container query, or `!important`
  implementation added by this plan.

Existing strings in planning files and pre-existing code do not matter for this
check; this command is scoped to the implementation diff.

- [ ] **Step 3: Run required checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Expected: all commands pass. If `cargo clippy` reports warnings in code touched
by this plan, fix them before review. If it reports unrelated pre-existing
warnings, capture the exact warning and stop for coordinator direction.

- [ ] **Step 4: Final holistic review**

The coordinator assigns a final clean-context reviewer over the complete
result. The reviewer must check:

- no `surgeist-css` dependency;
- no CSS parsing in style;
- no pseudo-element bucket or synthetic pseudo-element tree-node model;
- selector models have private fields where invariants matter;
- empty selector and relative selector lists are rejected;
- specificity ordering is layer, specificity, then source order;
- selector-list pseudo-class specificity is correct, including zero
  specificity for `:where`;
- attribute matcher semantics and case sensitivity are explicit;
- runtime pseudo-classes use style-owned facts and avoid invalid inverse
  assumptions;
- structural selectors and nth patterns handle reverse, type-filtered, and
  filtered child cases;
- `:has` relative selectors search the correct direction from the subject;
- `:root`, `:scope`, and compound scope anchors use `SelectorMatchContext`;
- sheet indexing remains sound and does not drop universal or complex rules;
- invalidation summaries are conservative and honest for Operation 5, and do
  not pretend to implement Operation 14 dependency-indexed precision.

## Coordinator Commit Guidance

Commit after each task-scoped worker/reviewer cycle is clean. Suggested commit
messages:

```sh
git commit -m "style: add selector list specificity"
git commit -m "style: expand selector state and attributes"
git commit -m "style: add structural selector matching"
git commit -m "style: add selector list pseudo matching"
git commit -m "style: integrate scoped selector matching"
```

Workers do not commit. The coordinator owns commits and final reporting.

## Root Handoff Notes

After implementation, root must add lowering from `surgeist-css` selector types
into these style-owned selector APIs.

Root must supply explicit runtime facts for participation-sensitive
pseudo-classes. For example, a node that does not participate in form
enabled/disabled semantics should not match either `:enabled` or `:disabled`
unless root supplies the corresponding style-owned fact.

Root must continue to reject or defer selectors with pseudo-element sequences
until Operation 6 lands. The style selector model from this plan is for element
subjects only.

Root should treat style selector validation failures as unsupported integration
or lowering bugs, not as CSS parse errors. CSS syntax validation remains in
`surgeist-css`.

## Next Sequence Context

The next implementation plan should cover Operation 6: pseudo-element style
buckets. It should consume the selector model from this plan without modeling
pseudo-elements as tree nodes.

The next plan should specifically pick up:

- element, `::before`, `::after`, `::marker`, `::selection`, and `::backdrop`
  style buckets;
- `::before::marker` and `::after::marker` nested marker buckets;
- resolver cache keys that include requested style bucket;
- sheet rule targets that combine an originating element selector with a
  pseudo-element bucket;
- generated content policy boundaries for `::before` and `::after`;
- marker bucket ownership without requiring retained/tree materialization.

Do not begin broad property family expansion, media/container query expansion,
font-face/keyframe modeling, or Operation 14's full cache/invalidation
generalization until pseudo-element bucket planning and review are complete.
