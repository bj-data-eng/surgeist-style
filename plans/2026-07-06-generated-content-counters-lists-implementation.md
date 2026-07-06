# Generated Content, Counters, And Lists Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add style-owned generated content, list marker, and counter mutation property models for Operation 11.

**Architecture:** `surgeist-style` receives typed, root-lowered generated-content data without importing `surgeist-css` and without materializing pseudo-elements into tree nodes. The implementation adds semantic value models first, wires them through `Property`/`Value`/declaration canonicalization/resolution, then rebases the CSS property ledger for Operation 11.

**Tech Stack:** Rust 2024, `surgeist-style` property/value/resolver model, trybuild compile-fail tests, Cargo baseline checks.

---

## Source Context

- Crate: `surgeist-style`
- Current branch: `main`
- Crate workflow: `AGENTS.md`
- Modeling guidance: `guidance/surgeist-rust-modeling-guide.md`
- Sequence source: `plans/2026-07-05-css-surface-style-operations-sequence.md`
- Coverage ledger: `plans/2026-07-05-css-property-coverage-ledger.md`
- CSS source inspected read-only: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- CSS generated-content parser inspected read-only: `/Users/codex/Development/surgeist-css/src/parser/generated_content.rs`

Operation 11 owns these ledger rows:

- `CssProperty::Content`
- `CssProperty::ListStyleType`
- `CssProperty::ListStylePosition`
- `CssProperty::ListStyleImage`
- `CssProperty::ListStyle`
- `CssProperty::CounterReset`
- `CssProperty::CounterIncrement`
- `CssProperty::CounterSet`

## Boundary Rules

- Do not add a `surgeist-css` dependency.
- Do not add a style-to-render, style-to-retained, style-to-layout, or style-to-CSS adapter.
- Do not materialize `::before`, `::after`, or `::marker` as tree nodes in this crate.
- Preserve generated content as style-owned policy/data. Retained projection, marker text generation, quote depth evaluation, counter formatting, attr lookup, image loading, and render resources remain outside this crate.
- Root lowers CSS syntax into these style-owned front doors. Style does not parse CSS.
- No compatibility aliases are required.

## File Responsibilities

- `src/value.rs`: generated content, counter, list marker, and list shorthand value types plus validation.
- `src/property.rs`: Operation 11 `Property` variants, metadata, inheritance, impact, `Value` acceptance, and validation domain wiring.
- `src/declaration.rs`: builder APIs, canonical longhand expansion for `list-style`, CSS-wide shorthand expansion, and stable `value_hash` coverage.
- `src/resolver.rs`: typed getters on `Resolved` for generated content, list style, and counter mutations.
- `src/authored.rs`: authored CSS-wide shorthand test coverage for `list-style`.
- `src/lib.rs`: public front-door reexports for new style-owned types.
- `tests/compile_pass/typed_public_construction.rs`: public API construction smoke coverage.
- `tests/compile_fail/*.rs`: compile-fail coverage for private fields and invalid direct construction.
- `plans/2026-07-05-css-property-coverage-ledger.md`: rebase only Operation 11 rows and family rollup after implementation.

---

### Task 1: Add Generated Content And Counter Vocabulary

**Files:**
- Modify: `src/value.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_content_item_list_empty_literal.rs`
- Create: `tests/compile_fail/invalid_counter_name_literal.rs`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: the branch may be ahead of origin, but there should be no unrelated working-tree edits.

- [ ] **Step 2: Add style-owned content string and counter names**

In `src/value.rs`, add these types near the existing symbolic/string-list models. Keep fields private.

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContentString {
    value: String,
}

impl ContentString {
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.contains('\0') {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "content string cannot contain U+0000",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CounterName {
    value: String,
}

impl CounterName {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self> {
        let value = validate_generated_ident(value.as_ref(), "counter name")?;
        if value.eq_ignore_ascii_case("none") {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "counter name cannot be `none`",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CounterStyleName {
    value: String,
}

impl CounterStyleName {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self> {
        let value = validate_generated_ident(value.as_ref(), "counter style name")?;
        if value.eq_ignore_ascii_case("none") {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "counter style name cannot be `none`",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

fn validate_generated_ident(value: &str, label: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} cannot be empty"),
        ));
    }
    if is_css_wide_keyword(trimmed) {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} cannot be a CSS-wide keyword"),
        ));
    }
    let mut chars = trimmed.chars();
    let first = chars.next().expect("trimmed value is not empty");
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} must start with an identifier character"),
        ));
    }
    for ch in chars {
        let valid = ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-');
        if !valid {
            return Err(Error::new(
                ErrorCode::InvalidString,
                format!("{label} contains unsupported character `{ch}`"),
            ));
        }
    }
    Ok(trimmed.to_owned())
}

fn is_css_wide_keyword(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    )
}
```

This is an intentional style-owned ASCII identifier subset, not a CSS parser clone. Root may parse a broader CSS identifier surface; when a parsed counter name or counter-style name cannot pass this style front door, root must report an unsupported-integration diagnostic instead of bypassing validation or adding an unchecked constructor.

- [ ] **Step 3: Add counter style and generated content payload types**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BuiltInCounterStyle {
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerAlpha,
    UpperAlpha,
    LowerLatin,
    UpperLatin,
    LowerRoman,
    UpperRoman,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CounterStyle {
    BuiltIn(BuiltInCounterStyle),
    Named(CounterStyleName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CounterFunction {
    name: CounterName,
    style: Option<CounterStyle>,
}

impl CounterFunction {
    #[must_use]
    pub const fn new(name: CounterName, style: Option<CounterStyle>) -> Self {
        Self { name, style }
    }

    #[must_use]
    pub const fn name(&self) -> &CounterName {
        &self.name
    }

    #[must_use]
    pub const fn style(&self) -> Option<&CounterStyle> {
        self.style.as_ref()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CountersFunction {
    name: CounterName,
    separator: ContentString,
    style: Option<CounterStyle>,
}

impl CountersFunction {
    #[must_use]
    pub const fn new(
        name: CounterName,
        separator: ContentString,
        style: Option<CounterStyle>,
    ) -> Self {
        Self {
            name,
            separator,
            style,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &CounterName {
        &self.name
    }

    #[must_use]
    pub const fn separator(&self) -> &ContentString {
        &self.separator
    }

    #[must_use]
    pub const fn style(&self) -> Option<&CounterStyle> {
        self.style.as_ref()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ContentItem {
    String(ContentString),
    Url(StyleUrl),
    Counter(CounterFunction),
    Counters(CountersFunction),
    Attr(StyleAttributeName),
    OpenQuote,
    CloseQuote,
    NoOpenQuote,
    NoCloseQuote,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContentItemList {
    items: Vec<ContentItem>,
}

impl ContentItemList {
    pub fn try_new(items: impl IntoIterator<Item = ContentItem>) -> Result<Self> {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "content item list cannot be empty",
            ));
        }
        Ok(Self { items })
    }

    #[must_use]
    pub fn items(&self) -> &[ContentItem] {
        &self.items
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Content {
    Normal,
    None,
    Items(ContentItemList),
}

impl Default for Content {
    fn default() -> Self {
        Self::Normal
    }
}
```

Use `StyleAttributeName` from `src/identity.rs` for `attr(...)`. Do not create a separate attribute-name type for generated content.

- [ ] **Step 4: Add validation tests for the new vocabulary**

In the `#[cfg(test)]` module in `src/value.rs`, add:

```rust
#[test]
fn generated_content_values_preserve_symbolic_payloads() {
    let counter = CounterName::try_new("chapter").unwrap();
    let style = CounterStyle::BuiltIn(BuiltInCounterStyle::UpperRoman);
    let content = Content::Items(
        ContentItemList::try_new([
            ContentItem::String(ContentString::try_new("Section ").unwrap()),
            ContentItem::Counter(CounterFunction::new(counter.clone(), Some(style.clone()))),
            ContentItem::String(ContentString::try_new(": ").unwrap()),
            ContentItem::Attr(StyleAttributeName::new("data-title").unwrap()),
            ContentItem::Url(StyleUrl::new("marker.svg").unwrap()),
            ContentItem::OpenQuote,
            ContentItem::CloseQuote,
        ])
        .unwrap(),
    );

    let Content::Items(items) = content else {
        panic!("content items should be preserved");
    };
    assert_eq!(items.items().len(), 7);
    assert_eq!(counter.as_str(), "chapter");
}

#[test]
fn generated_content_names_and_lists_validate_invariants() {
    assert_eq!(ContentString::try_new("\0").unwrap_err().code(), ErrorCode::InvalidString);
    assert_eq!(
        ContentItemList::try_new([]).unwrap_err().code(),
        ErrorCode::InvalidValue
    );
    for rejected in ["", "-", " two words ", "inherit", "initial", "unset", "revert", "revert-layer", "none", "1bad"] {
        assert_eq!(
            CounterName::try_new(rejected).unwrap_err().code(),
            ErrorCode::InvalidString
        );
    }
    assert_eq!(CounterName::try_new("section-1").unwrap().as_str(), "section-1");
    assert_eq!(
        CounterStyleName::try_new("legal").unwrap().as_str(),
        "legal"
    );
}
```

- [ ] **Step 5: Reexport public front doors**

In `src/lib.rs`, add the new types to the `pub use value::{ ... }` list:

```rust
BuiltInCounterStyle, Content, ContentItem, ContentItemList, ContentString, CounterFunction,
CounterName, CounterStyle, CounterStyleName, CountersFunction,
```

- [ ] **Step 6: Add compile-pass coverage**

In `tests/compile_pass/typed_public_construction.rs`, import and construct:

```rust
let counter_name = CounterName::try_new("figure")?;
let content = Content::Items(ContentItemList::try_new([
    ContentItem::String(ContentString::try_new("Figure ")?),
    ContentItem::Counter(CounterFunction::new(
        counter_name.clone(),
        Some(CounterStyle::BuiltIn(BuiltInCounterStyle::Decimal)),
    )),
    ContentItem::Counters(CountersFunction::new(
        counter_name,
        ContentString::try_new(".")?,
        Some(CounterStyle::Named(CounterStyleName::try_new("legal")?)),
    )),
])?);
let _ = content;
```

Place the snippet near other value-construction smoke tests.

- [ ] **Step 7: Add compile-fail tests for private fields**

Create `tests/compile_fail/invalid_content_item_list_empty_literal.rs`:

```rust
use surgeist_style::ContentItemList;

fn main() {
    let _list = ContentItemList { items: Vec::new() };
}
```

Create `tests/compile_fail/invalid_counter_name_literal.rs`:

```rust
use surgeist_style::CounterName;

fn main() {
    let _name = CounterName {
        value: String::from("bad"),
    };
}
```

Generate and review the matching trybuild expectations:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_content_item_list_empty_literal.stderr tests/compile_fail/invalid_counter_name_literal.stderr
```

Expected: two `.stderr` files are created, and their diagnostics show private-field construction is rejected.

- [ ] **Step 8: Run focused checks**

Run:

```sh
cargo test -p surgeist-style generated_content_values
cargo test -p surgeist-style generated_content_names
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass; only Task 1 files and the two matching `.stderr` expectation files are modified.

- [ ] **Step 9: Commit after worker/reviewer clean**

After the coordinator receives a clean scoped reviewer result:

```sh
git add src/value.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_content_item_list_empty_literal.rs tests/compile_fail/invalid_content_item_list_empty_literal.stderr tests/compile_fail/invalid_counter_name_literal.rs tests/compile_fail/invalid_counter_name_literal.stderr
git commit -m "style: add generated content vocabulary"
```

---

### Task 2: Add List Marker And Counter Mutation Models

**Files:**
- Modify: `src/value.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_counter_change_list_empty_literal.rs`

- [ ] **Step 1: Add list marker models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ListStyleType {
    None,
    CounterStyle(CounterStyle),
    String(ContentString),
}

impl Default for ListStyleType {
    fn default() -> Self {
        Self::CounterStyle(CounterStyle::BuiltIn(BuiltInCounterStyle::Disc))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ListStylePosition {
    Inside,
    #[default]
    Outside,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ListStyleImage {
    #[default]
    None,
    Url(StyleUrl),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListStyle {
    style_type: Option<ListStyleType>,
    position: Option<ListStylePosition>,
    image: Option<ListStyleImage>,
}

impl ListStyle {
    pub fn try_new(
        style_type: Option<ListStyleType>,
        position: Option<ListStylePosition>,
        image: Option<ListStyleImage>,
    ) -> Result<Self> {
        if style_type.is_none() && position.is_none() && image.is_none() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "list-style shorthand cannot be empty",
            ));
        }
        Ok(Self {
            style_type,
            position,
            image,
        })
    }

    #[must_use]
    pub const fn style_type(&self) -> Option<&ListStyleType> {
        self.style_type.as_ref()
    }

    #[must_use]
    pub const fn position(&self) -> Option<ListStylePosition> {
        self.position
    }

    #[must_use]
    pub const fn image(&self) -> Option<&ListStyleImage> {
        self.image.as_ref()
    }
}
```

Root should lower ambiguous CSS `none` in the CSS shorthand into explicit `ListStyleType::None`, `ListStyleImage::None`, or both before constructing `ListStyle`.

- [ ] **Step 2: Add counter mutation models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CounterChange {
    name: CounterName,
    value: i32,
}

impl CounterChange {
    #[must_use]
    pub const fn new(name: CounterName, value: i32) -> Self {
        Self { name, value }
    }

    #[must_use]
    pub const fn name(&self) -> &CounterName {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> i32 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CounterChangeList {
    changes: Vec<CounterChange>,
}

impl CounterChangeList {
    pub fn try_new(changes: impl IntoIterator<Item = CounterChange>) -> Result<Self> {
        let changes = changes.into_iter().collect::<Vec<_>>();
        if changes.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "counter change list cannot be empty",
            ));
        }
        Ok(Self { changes })
    }

    #[must_use]
    pub fn changes(&self) -> &[CounterChange] {
        &self.changes
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum CounterChanges {
    #[default]
    None,
    Changes(CounterChangeList),
}
```

Do not preserve CSS's `Option<i32>` value in style. Root must lower omitted CSS counter values into explicit `i32` values according to the destination property:

- `counter-reset`: omitted value becomes `0`
- `counter-increment`: omitted value becomes `1`
- `counter-set`: omitted value becomes `0`

- [ ] **Step 3: Add validation tests**

In `src/value.rs` tests, add:

```rust
#[test]
fn list_style_models_preserve_marker_policy() {
    let marker = ListStyle::try_new(
        Some(ListStyleType::String(ContentString::try_new("->").unwrap())),
        Some(ListStylePosition::Inside),
        Some(ListStyleImage::Url(StyleUrl::new("marker.svg").unwrap())),
    )
    .unwrap();

    assert!(matches!(marker.style_type(), Some(ListStyleType::String(_))));
    assert_eq!(marker.position(), Some(ListStylePosition::Inside));
    assert!(matches!(marker.image(), Some(ListStyleImage::Url(_))));
    assert!(ListStyle::try_new(None, None, None).is_err());
}

#[test]
fn counter_changes_require_explicit_values_and_non_empty_lists() {
    let name = CounterName::try_new("section").unwrap();
    let changes = CounterChangeList::try_new([
        CounterChange::new(name.clone(), 0),
        CounterChange::new(name, 1),
    ])
    .unwrap();

    assert_eq!(changes.changes()[0].value(), 0);
    assert_eq!(
        CounterChangeList::try_new([]).unwrap_err().code(),
        ErrorCode::InvalidValue
    );
    assert_eq!(CounterChanges::default(), CounterChanges::None);
}
```

- [ ] **Step 4: Reexport public front doors**

In `src/lib.rs`, add:

```rust
CounterChange, CounterChangeList, CounterChanges, ListStyle, ListStyleImage,
ListStylePosition, ListStyleType,
```

- [ ] **Step 5: Add compile-pass coverage**

In `tests/compile_pass/typed_public_construction.rs`, import and construct:

```rust
let marker_type = ListStyleType::CounterStyle(CounterStyle::BuiltIn(
    BuiltInCounterStyle::LowerRoman,
));
let list_style = ListStyle::try_new(
    Some(marker_type),
    Some(ListStylePosition::Outside),
    Some(ListStyleImage::Url(StyleUrl::new("marker.svg")?)),
)?;
let _ = list_style;

let counter_changes = CounterChanges::Changes(CounterChangeList::try_new([
    CounterChange::new(CounterName::try_new("figure")?, 0),
])?);
let _ = counter_changes;
```

- [ ] **Step 6: Add compile-fail coverage**

Create `tests/compile_fail/invalid_counter_change_list_empty_literal.rs`:

```rust
use surgeist_style::CounterChangeList;

fn main() {
    let _list = CounterChangeList {
        changes: Vec::new(),
    };
}
```

Generate and review the matching trybuild expectation:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_counter_change_list_empty_literal.stderr
```

Expected: one `.stderr` file is created, and its diagnostic shows private-field construction is rejected.

- [ ] **Step 7: Run focused checks**

Run:

```sh
cargo test -p surgeist-style list_style_models
cargo test -p surgeist-style counter_changes
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass; only Task 2 files and the matching `.stderr` expectation file are modified.

- [ ] **Step 8: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_counter_change_list_empty_literal.rs tests/compile_fail/invalid_counter_change_list_empty_literal.stderr
git commit -m "style: add list and counter models"
```

---

### Task 3: Wire Operation 11 Properties Through Declarations

**Files:**
- Modify: `src/property.rs`
- Modify: `src/value.rs`
- Modify: `src/declaration.rs`
- Modify: `src/authored.rs`

- [ ] **Step 1: Add property variants**

In `src/property.rs`, add these variants near `ContentVisibility` and before layout sizing rows:

```rust
Content,
ListStyleType,
ListStylePosition,
ListStyleImage,
ListStyle,
CounterReset,
CounterIncrement,
CounterSet,
```

Add each variant to `Property::ALL`. Mark only `ListStyle` as non-canonical in `is_canonical`.

- [ ] **Step 2: Add value variants**

In `src/value.rs`, add these `Value` variants:

```rust
Content(Content),
ListStyleType(ListStyleType),
ListStylePosition(ListStylePosition),
ListStyleImage(ListStyleImage),
ListStyle(ListStyle),
CounterChanges(CounterChanges),
```

Update `Value::interpolation()` so all six are `Interpolation::Discrete`.

Update `Value::validate()`:

```rust
Self::Content(value) => value.validate(),
Self::ListStyleType(_) | Self::ListStylePosition(_) | Self::ListStyleImage(_) => Ok(()),
Self::ListStyle(value) => value.validate(),
Self::CounterChanges(value) => value.validate(),
```

Add these validation methods:

```rust
impl Content {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Normal | Self::None => Ok(()),
            Self::Items(items) => {
                if items.items().is_empty() {
                    Err(Error::new(
                        ErrorCode::InvalidValue,
                        "content item list cannot be empty",
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl ListStyle {
    pub fn validate(&self) -> Result<()> {
        if self.style_type.is_none() && self.position.is_none() && self.image.is_none() {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "list-style shorthand cannot be empty",
            ))
        } else {
            Ok(())
        }
    }
}

impl CounterChanges {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::Changes(changes) => {
                if changes.changes().is_empty() {
                    Err(Error::new(
                        ErrorCode::InvalidValue,
                        "counter change list cannot be empty",
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}
```

These methods validate structural invariants only. They must not perform counter formatting, attr lookup, quote depth evaluation, or image loading.

- [ ] **Step 3: Add metadata, acceptance, and defaults**

In `src/property.rs`, update `metadata()`:

```rust
Self::Content => Metadata::new(Value::Content(Content::Normal))
    .impact(Impact::empty().layout().text().paint()),
Self::ListStyleType => Metadata::new(Value::ListStyleType(ListStyleType::default()))
    .inherited(true)
    .impact(Impact::empty().layout().text().paint()),
Self::ListStylePosition => Metadata::new(Value::ListStylePosition(ListStylePosition::Outside))
    .inherited(true)
    .impact(Impact::empty().layout().paint()),
Self::ListStyleImage => Metadata::new(Value::ListStyleImage(ListStyleImage::None))
    .inherited(true)
    .impact(Impact::empty().layout().paint()),
Self::ListStyle => Metadata::new(Value::ListStyle(
    ListStyle::try_new(
        Some(ListStyleType::default()),
        Some(ListStylePosition::Outside),
        Some(ListStyleImage::None),
    )
    .unwrap(),
))
.inherited(true)
.impact(Impact::empty().layout().text().paint()),
Self::CounterReset | Self::CounterIncrement | Self::CounterSet => {
    Metadata::new(Value::CounterChanges(CounterChanges::None))
        .impact(Impact::empty().layout().text().paint())
}
```

Update `accepts()`:

```rust
Self::Content => matches!(value, Value::Content(_)),
Self::ListStyleType => matches!(value, Value::ListStyleType(_)),
Self::ListStylePosition => matches!(value, Value::ListStylePosition(_)),
Self::ListStyleImage => matches!(value, Value::ListStyleImage(_)),
Self::ListStyle => matches!(value, Value::ListStyle(_)),
Self::CounterReset | Self::CounterIncrement | Self::CounterSet => {
    matches!(value, Value::CounterChanges(_))
}
```

Update `value_kind()` with labels:

```rust
Value::Content(_) => "content",
Value::ListStyleType(_) => "list style type",
Value::ListStylePosition(_) => "list style position",
Value::ListStyleImage(_) => "list style image",
Value::ListStyle(_) => "list style shorthand",
Value::CounterChanges(_) => "counter changes",
```

- [ ] **Step 4: Add declaration builders and shorthand lowering**

In `src/declaration.rs`, add builder methods on `Declarations`:

```rust
pub fn content(self, value: Content) -> Result<Self> {
    self.try_set(Property::Content, Value::Content(value))
}

pub fn list_style_type(self, value: ListStyleType) -> Self {
    self.set(Property::ListStyleType, Value::ListStyleType(value))
}

pub fn list_style_position(self, value: ListStylePosition) -> Self {
    self.set(Property::ListStylePosition, Value::ListStylePosition(value))
}

pub fn list_style_image(self, value: ListStyleImage) -> Self {
    self.set(Property::ListStyleImage, Value::ListStyleImage(value))
}

pub fn list_style(self, value: ListStyle) -> Result<Self> {
    self.try_set(Property::ListStyle, Value::ListStyle(value))
}

pub fn counter_reset(self, value: CounterChanges) -> Result<Self> {
    self.try_set(Property::CounterReset, Value::CounterChanges(value))
}

pub fn counter_increment(self, value: CounterChanges) -> Result<Self> {
    self.try_set(Property::CounterIncrement, Value::CounterChanges(value))
}

pub fn counter_set(self, value: CounterChanges) -> Result<Self> {
    self.try_set(Property::CounterSet, Value::CounterChanges(value))
}
```

Update `canonical_properties()`:

```rust
Property::ListStyle => vec![
    Property::ListStyleType,
    Property::ListStylePosition,
    Property::ListStyleImage,
],
```

Update `canonical_declarations()`:

```rust
(Property::ListStyle, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::ListStyle),
    Value::Keyword(keyword),
),
(Property::ListStyle, Value::ListStyle(value)) => list_style_declarations(value),
```

Add:

```rust
fn list_style_declarations(value: ListStyle) -> Vec<Declaration> {
    vec![
        Declaration::new(
            Property::ListStyleType,
            Value::ListStyleType(value.style_type().cloned().unwrap_or_default()),
        ),
        Declaration::new(
            Property::ListStylePosition,
            Value::ListStylePosition(value.position().unwrap_or_default()),
        ),
        Declaration::new(
            Property::ListStyleImage,
            Value::ListStyleImage(value.image().cloned().unwrap_or_default()),
        ),
    ]
}
```

- [ ] **Step 5: Add stable hashing**

In `src/declaration.rs`, update `hash_value()` to dispatch the new value variants:

```rust
Value::Content(value) => hash_content(value, state),
Value::ListStyleType(value) => hash_list_style_type(value, state),
Value::ListStylePosition(value) => value.hash(state),
Value::ListStyleImage(value) => hash_list_style_image(value, state),
Value::ListStyle(value) => hash_list_style(value, state),
Value::CounterChanges(value) => hash_counter_changes(value, state),
```

Add helper functions that write discriminants and recursively hash nested strings, URLs, counter names, styles, and explicit counter values. Do not use `Debug` strings for hash identity.

- [ ] **Step 6: Add focused declaration tests**

In `src/declaration.rs` tests, add:

```rust
#[test]
fn list_style_shorthand_lowers_to_marker_longhands() {
    let marker_type = ListStyleType::String(ContentString::try_new("->").unwrap());
    let image = ListStyleImage::Url(StyleUrl::new("marker.svg").unwrap());
    let declarations = Declarations::new()
        .list_style(
            ListStyle::try_new(
                Some(marker_type.clone()),
                Some(ListStylePosition::Inside),
                Some(image.clone()),
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(declarations.get(Property::ListStyle), None);
    assert_eq!(
        declarations.get(Property::ListStyleType),
        Some(&Value::ListStyleType(marker_type))
    );
    assert_eq!(
        declarations.get(Property::ListStylePosition),
        Some(&Value::ListStylePosition(ListStylePosition::Inside))
    );
    assert_eq!(
        declarations.get(Property::ListStyleImage),
        Some(&Value::ListStyleImage(image))
    );
}

#[test]
fn list_style_shorthand_resets_omitted_components_to_defaults() {
    let declarations = Declarations::new()
        .list_style(
            ListStyle::try_new(Some(ListStyleType::None), None, None).unwrap(),
        )
        .unwrap();

    assert_eq!(
        declarations.get(Property::ListStyleType),
        Some(&Value::ListStyleType(ListStyleType::None))
    );
    assert_eq!(
        declarations.get(Property::ListStylePosition),
        Some(&Value::ListStylePosition(ListStylePosition::Outside))
    );
    assert_eq!(
        declarations.get(Property::ListStyleImage),
        Some(&Value::ListStyleImage(ListStyleImage::None))
    );
}

#[test]
fn generated_content_and_counters_accept_typed_values() {
    let counter_name = CounterName::try_new("section").unwrap();
    let content = Content::Items(
        ContentItemList::try_new([
            ContentItem::String(ContentString::try_new("Section ").unwrap()),
            ContentItem::Counter(CounterFunction::new(
                counter_name.clone(),
                Some(CounterStyle::BuiltIn(BuiltInCounterStyle::Decimal)),
            )),
        ])
        .unwrap(),
    );
    let changes = CounterChanges::Changes(
        CounterChangeList::try_new([CounterChange::new(counter_name, 1)]).unwrap(),
    );

    let declarations = Declarations::new()
        .content(content.clone())
        .unwrap()
        .counter_increment(changes.clone())
        .unwrap();

    assert_eq!(declarations.get(Property::Content), Some(&Value::Content(content)));
    assert_eq!(
        declarations.get(Property::CounterIncrement),
        Some(&Value::CounterChanges(changes))
    );
}
```

In `src/authored.rs` tests, add CSS-wide shorthand coverage:

```rust
#[test]
fn list_style_css_wide_keyword_expands_to_marker_longhands() {
    let mut declarations = AuthoredDeclarations::new();
    declarations.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::Property(Property::ListStyle),
        CssWideKeyword::Unset,
    ));

    let canonical = declarations.to_rule_declarations().unwrap();

    assert_eq!(canonical.get(Property::ListStyle), None);
    for property in [
        Property::ListStyleType,
        Property::ListStylePosition,
        Property::ListStyleImage,
    ] {
        assert_eq!(
            canonical.get(property),
            Some(&AuthoredCascadeValue::CssWideKeyword(CssWideKeyword::Unset))
        );
    }
}
```

- [ ] **Step 7: Run focused checks**

Run:

```sh
cargo test -p surgeist-style list_style_shorthand
cargo test -p surgeist-style generated_content_and_counters
cargo test -p surgeist-style list_style_css_wide
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 8: Commit after worker/reviewer clean**

```sh
git add src/property.rs src/value.rs src/declaration.rs src/authored.rs
git commit -m "style: wire generated content properties"
```

---

### Task 4: Add Resolved Getters, Bucket Smoke Coverage, And Public API Artifacts

**Files:**
- Modify: `src/resolver.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Add typed resolved getters**

In `src/resolver.rs`, import the new value types and add getters on `Resolved`:

```rust
#[must_use]
pub fn content(&self) -> &Content {
    match self.get(Property::Content) {
        Value::Content(value) => value,
        _ => unreachable!("resolved content stores generated content"),
    }
}

#[must_use]
pub fn list_style_type(&self) -> &ListStyleType {
    match self.get(Property::ListStyleType) {
        Value::ListStyleType(value) => value,
        _ => unreachable!("resolved list-style-type stores marker type"),
    }
}

#[must_use]
pub fn list_style_position(&self) -> ListStylePosition {
    match self.get(Property::ListStylePosition) {
        Value::ListStylePosition(value) => *value,
        _ => ListStylePosition::Outside,
    }
}

#[must_use]
pub fn list_style_image(&self) -> &ListStyleImage {
    match self.get(Property::ListStyleImage) {
        Value::ListStyleImage(value) => value,
        _ => unreachable!("resolved list-style-image stores marker image"),
    }
}

#[must_use]
pub fn counter_reset(&self) -> &CounterChanges {
    self.counter_changes(Property::CounterReset)
}

#[must_use]
pub fn counter_increment(&self) -> &CounterChanges {
    self.counter_changes(Property::CounterIncrement)
}

#[must_use]
pub fn counter_set(&self) -> &CounterChanges {
    self.counter_changes(Property::CounterSet)
}

fn counter_changes(&self, property: Property) -> &CounterChanges {
    match self.get(property) {
        Value::CounterChanges(value) => value,
        _ => unreachable!("resolved counter property stores counter changes"),
    }
}
```

- [ ] **Step 2: Add resolver tests for generated-content buckets**

In `src/resolver.rs` tests, add:

```rust
#[test]
fn generated_content_values_resolve_on_pseudo_buckets_without_tree_materialization() {
    let tree = Tree::new(vec![Node::new(StyleKey::new("root").unwrap())]);
    let content = Content::Items(
        ContentItemList::try_new([ContentItem::String(
            ContentString::try_new("New").unwrap(),
        )])
        .unwrap(),
    );
    let sheet = Sheet::new().push_targeted_rule(
        RuleTarget::new(Selector::key("root").unwrap(), StyleBucket::Before),
        Declarations::new().content(content.clone()).unwrap(),
    );

    let style = Resolver::new(&sheet)
        .resolve(Context::new(&tree, 0).style_bucket(StyleBucket::Before))
        .unwrap();
    let element_style = Resolver::new(&sheet)
        .resolve(Context::new(&tree, 0))
        .unwrap();

    assert_eq!(style.content(), &content);
    assert_eq!(element_style.content(), &Content::Normal);
    assert_eq!(tree.len(), 1);
}

#[test]
fn list_marker_and_counter_values_resolve_together() {
    let tree = Tree::new(vec![Node::new(StyleKey::new("item").unwrap())]);
    let counter_name = CounterName::try_new("item").unwrap();
    let changes = CounterChanges::Changes(
        CounterChangeList::try_new([CounterChange::new(counter_name, 1)]).unwrap(),
    );
    let marker_image = ListStyleImage::Url(StyleUrl::new("marker.svg").unwrap());
    let sheet = Sheet::new().push_targeted_rule(
        RuleTarget::element(Selector::key("item").unwrap()),
        Declarations::new()
            .list_style(
                ListStyle::try_new(
                    Some(ListStyleType::CounterStyle(CounterStyle::BuiltIn(
                        BuiltInCounterStyle::Decimal,
                    ))),
                    Some(ListStylePosition::Inside),
                    Some(marker_image.clone()),
                )
                .unwrap(),
            )
            .unwrap()
            .counter_increment(changes.clone())
            .unwrap(),
    );

    let style = Resolver::new(&sheet).resolve(Context::new(&tree, 0)).unwrap();

    assert!(matches!(
        style.list_style_type(),
        ListStyleType::CounterStyle(CounterStyle::BuiltIn(BuiltInCounterStyle::Decimal))
    ));
    assert_eq!(style.list_style_position(), ListStylePosition::Inside);
    assert_eq!(style.list_style_image(), &marker_image);
    assert_eq!(style.counter_increment(), &changes);
}
```

- [ ] **Step 3: Extend compile-pass public construction**

In `tests/compile_pass/typed_public_construction.rs`, after building a `Declarations`, add:

```rust
let generated_content = Content::Items(ContentItemList::try_new([
    ContentItem::String(ContentString::try_new("Item ")?),
    ContentItem::Counter(CounterFunction::new(
        CounterName::try_new("item")?,
        Some(CounterStyle::BuiltIn(BuiltInCounterStyle::Decimal)),
    )),
])?);

let declarations = Declarations::new()
    .content(generated_content)?
    .list_style(ListStyle::try_new(
        Some(ListStyleType::CounterStyle(CounterStyle::BuiltIn(
            BuiltInCounterStyle::Disc,
        ))),
        Some(ListStylePosition::Outside),
        Some(ListStyleImage::None),
    )?)?
    .counter_reset(CounterChanges::None)?
    .counter_increment(CounterChanges::Changes(CounterChangeList::try_new([
        CounterChange::new(CounterName::try_new("item")?, 1),
    ])?))?
    .counter_set(CounterChanges::None)?;
let _ = declarations;
```

- [ ] **Step 4: Run focused checks**

Run:

```sh
cargo test -p surgeist-style generated_content_values_resolve
cargo test -p surgeist-style list_marker_and_counter
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 5: Commit after worker/reviewer clean**

```sh
git add src/resolver.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: resolve generated content values"
```

---

### Task 5: Rebase Operation 11 Ledger And Run Final Checks

**Files:**
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Rebase ledger rows**

Update only Operation 11-affected rows:

- `Content`: `Existing style property`, target `Property::Content` + `Value::Content`.
- `ListStyleType`: `Existing style property`, target `Property::ListStyleType` + `Value::ListStyleType`.
- `ListStylePosition`: `Existing style property`, target `Property::ListStylePosition` + `Value::ListStylePosition`.
- `ListStyleImage`: `Existing style property`, target `Property::ListStyleImage` + `Value::ListStyleImage`; note URLs remain symbolic and resource loading remains outside style.
- `ListStyle`: `Existing style shorthand`, target `Property::ListStyle` + `Value::ListStyle`; note lowering to type, position, and image longhands.
- `CounterReset`: `Existing style property`, target `Property::CounterReset` + `Value::CounterChanges`.
- `CounterIncrement`: `Existing style property`, target `Property::CounterIncrement` + `Value::CounterChanges`.
- `CounterSet`: `Existing style property`, target `Property::CounterSet` + `Value::CounterChanges`.

Update the family rollup:

```markdown
| Generated content and lists | `content`, list marker type/position/image/shorthand, counter reset/increment/set, counter/counters content functions, quote payloads, attr payloads, and pseudo-element style buckets have typed style targets. | Counter formatting, quote depth evaluation, attr lookup, marker materialization, retained projection, image loading, and render resources remain outside style. | No property implementation |
```

Update `Next Sequence Context` so Operation 12 timing/animation/keyframes comes next.

- [ ] **Step 2: Run ledger consistency**

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
print(f'source={len(source)} rows={len(rows)} missing={missing} extra={extra} duplicates={duplicates}')
raise SystemExit(0 if len(source) == len(rows) == 180 and not missing and not extra and not duplicates else 1)
PY
```

Expected: `source=180 rows=180 missing=[] extra=[] duplicates=[]`.

- [ ] **Step 3: Run full final checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
```

Expected: all pass, and the branch is clean except this task's ledger edit before commit.

- [ ] **Step 4: Commit after worker/reviewer clean**

```sh
git add plans/2026-07-05-css-property-coverage-ledger.md
git commit -m "style: rebase generated content ledger"
```

---

## Root Handoff Notes

After this plan is implemented, root can lower CSS generated-content syntax into these style-owned APIs:

- `CssContent::Normal` -> `Content::Normal`
- `CssContent::None` -> `Content::None`
- `CssContent::Items` -> `Content::Items(ContentItemList)`
- `CssContentItem::String` -> `ContentItem::String(ContentString)`
- `CssContentItem::Url` -> `ContentItem::Url(StyleUrl)`
- `CssContentItem::Counter` -> `ContentItem::Counter(CounterFunction)`
- `CssContentItem::Counters` -> `ContentItem::Counters(CountersFunction)`
- `CssContentItem::Attr` -> `ContentItem::Attr(StyleAttributeName)`
- quote keywords -> matching `ContentItem` variants
- `CssListStyleType` -> `ListStyleType`
- `CssListStylePosition` -> `ListStylePosition`
- `CssListStyleImage` -> `ListStyleImage`
- `CssListStyle` -> `ListStyle`
- `CssCounterChanges::None` -> `CounterChanges::None`
- CSS counter changes with omitted values -> explicit `CounterChange` values before style receipt
- parsed CSS counter or counter-style identifiers outside style's ASCII front-door subset -> root unsupported-integration diagnostic

Root remains responsible for CSS parse errors, CSS source locations, ambiguous shorthand `none` disambiguation before constructing `ListStyle`, rejecting parsed counter identifiers outside style's documented subset, image loading, actual counter formatting, quote depth, attr lookup, retained pseudo-element projection, and render materialization.

## Final Holistic Review Prompt

Use a clean-context reviewer with this prompt:

```text
You are a final holistic reviewer for surgeist-style Operation 11 generated content/counters/lists. Do not edit files.

Repo: /Users/codex/Development/surgeist-style

Read:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger.md
- plans/2026-07-06-generated-content-counters-lists-implementation.md
- read-only /Users/codex/Development/surgeist-css/src/syntax.rs
- read-only /Users/codex/Development/surgeist-css/src/parser/generated_content.rs

Review the completed implementation against:
- style owns typed generated content, list marker, and counter mutation models;
- style does not import or depend on surgeist-css;
- URLs, counter/counters functions, attr payloads, and quote payloads remain symbolic style data;
- actual counter formatting, quote depth evaluation, attr lookup, image loading, retained projection, and render materialization remain outside style;
- `content` can be targeted to pseudo-element style buckets without making pseudo-elements tree nodes;
- `list-style` lowers to list-style-type, list-style-position, and list-style-image longhands;
- CSS-wide keywords for `list-style` expand to canonical longhands;
- counter reset/increment/set store explicit integer changes and do not preserve ambiguous omitted CSS integers;
- public APIs have front doors and invalid states are hard to construct;
- Operation 11 ledger rows were rebased honestly and Operation 12 remains the next plan;
- implementation follows the Rust modeling guide.

Run:
- cargo fmt --check
- cargo test -p surgeist-style
- cargo clippy -p surgeist-style --all-targets -- -D warnings
- ! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
- git diff --check
- git status --short --branch
- ledger consistency script from the plan

Report findings first with file/line references. If clean, say clean and include commands run.
```

## Final Verification

After all task commits and the final holistic review are clean, run:

```sh
cargo fmt --check
cargo test -p surgeist-style
cargo clippy -p surgeist-style --all-targets -- -D warnings
! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
git diff --check
git status --short --branch
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
print(f'source={len(source)} rows={len(rows)} missing={missing} extra={extra} duplicates={duplicates}')
raise SystemExit(0 if len(source) == len(rows) == 180 and not missing and not extra and not duplicates else 1)
PY
```

Expected final state:

- no direct `surgeist-css` or `surgeist-text` dependency,
- clean formatting,
- all tests pass,
- clippy has no warnings,
- ledger still has 180 rows with no missing/extra/duplicate property rows,
- working tree is clean except the branch being ahead by new task commits.

## This Will Come Next

After Operation 11 lands and reviewers are clean, write the next sequential implementation plan for Operation 12: timing, animation, and keyframe style data.

Operation 12 should start from the rebased ledger and cover:

- transition property/duration/delay/timing function/shorthand,
- animation name/duration/delay/timing function/iteration/direction/fill mode/play state/shorthand,
- typed list models for time and animation component lists,
- symbolic easing functions,
- keyframe references and keyframe declaration blocks.

Operation 12 must keep style as the owner of style data and invalidation impact while leaving runtime animation scheduling, interpolation execution, clock ownership, and render updates outside this crate.
