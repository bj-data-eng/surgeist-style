# Timing, Animation, And Keyframes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add style-owned timing, transition, animation, and keyframe data models for Operation 12.

**Architecture:** `surgeist-style` receives root-lowered timing and animation syntax as typed style-owned data. The implementation replaces scalar transition timing storage with list-valued models, adds symbolic easing payloads and animation longhand/shorthand models, stores keyframe declaration blocks on `Sheet`, and leaves runtime scheduling, interpolation sampling, and render effects outside this crate.

**Tech Stack:** Rust 2024, `surgeist-style` value/property/declaration/resolver/sheet model, trybuild compile-fail tests, Cargo baseline checks.

---

## Source Context

- Crate: `surgeist-style`
- Current branch: `main`
- Crate workflow: `AGENTS.md`
- Modeling guidance: `guidance/surgeist-rust-modeling-guide.md`
- Sequence source: `plans/2026-07-05-css-surface-style-operations-sequence.md`
- Coverage ledger: `plans/2026-07-05-css-property-coverage-ledger.md`
- CSS source inspected read-only: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- CSS timing parser inspected read-only: `/Users/codex/Development/surgeist-css/src/parser/timing.rs`
- CSS keyframes parser inspected read-only: `/Users/codex/Development/surgeist-css/src/parser/keyframes.rs`

Operation 12 owns these ledger rows:

- `CssProperty::TransitionProperty`
- `CssProperty::TransitionDuration`
- `CssProperty::TransitionDelay`
- `CssProperty::TransitionTimingFunction`
- `CssProperty::Transition`
- `CssProperty::AnimationName`
- `CssProperty::AnimationDuration`
- `CssProperty::AnimationDelay`
- `CssProperty::AnimationTimingFunction`
- `CssProperty::AnimationIterationCount`
- `CssProperty::AnimationDirection`
- `CssProperty::AnimationFillMode`
- `CssProperty::AnimationPlayState`
- `CssProperty::Animation`
- `@keyframes` rule data from `CssRule::Keyframes`

## Boundary Rules

- Do not add a `surgeist-css` dependency.
- Do not add a style-to-render, style-to-retained, style-to-layout, or style-to-CSS adapter.
- Do not evaluate easing curves, sample animations, schedule transitions, resolve frame times, choose compositor/render behavior, or interpolate keyframes in this crate.
- Preserve easing function arguments and keyframe names as style-owned symbolic data.
- Root lowers CSS syntax into these style-owned front doors. Style does not parse CSS property names, timing functions, keyframe names, or keyframe selectors.
- No compatibility aliases are required. Breaking changes are fine when they make the model more accurate.

## File Responsibilities

- `src/value.rs`: timing lists, easing lists, transition-property lists, transition/animation shorthand items, animation longhand lists, keyframe names, offsets, blocks, and rule models.
- `src/property.rs`: Operation 12 `Property` variants, metadata, inheritance, impact, value acceptance, and validation-domain wiring.
- `src/declaration.rs`: builder APIs, transition and animation shorthand lowering, CSS-wide shorthand expansion, stable value hashing, and focused declaration tests.
- `src/resolver.rs`: typed getters on `Resolved` for transition and animation longhands plus resolver smoke coverage.
- `src/sheet.rs`: keyframe rule storage on `Sheet` without affecting selector matching or resolver scheduling.
- `src/lib.rs`: public front-door reexports for new style-owned types.
- `tests/compile_pass/typed_public_construction.rs`: public API construction smoke coverage.
- `tests/compile_fail/*.rs`: compile-fail coverage for private fields and invalid direct construction.
- `plans/2026-07-05-css-property-coverage-ledger.md`: rebase only Operation 12 rows and family rollup after implementation.

---

### Task 1: Add Timing, Easing, And Transition Vocabulary

**Files:**
- Modify: `src/value.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_time_list_empty_literal.rs`
- Create: `tests/compile_fail/invalid_transition_property_name_literal.rs`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: the branch may be ahead of origin, but there should be no unrelated working-tree edits.

- [ ] **Step 2: Add nonempty time lists**

In `src/value.rs`, keep `DurationSeconds` as the scalar non-negative seconds primitive and add a list wrapper near it:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct TimeList {
    values: Vec<DurationSeconds>,
}

impl TimeList {
    pub fn try_new(values: impl IntoIterator<Item = DurationSeconds>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "time list cannot be empty",
            ));
        }
        Ok(Self { values })
    }

    #[must_use]
    pub fn seconds(&self) -> &[DurationSeconds] {
        &self.values
    }

    #[must_use]
    pub fn single_zero() -> Self {
        Self {
            values: vec![DurationSeconds::new(0.0).expect("zero seconds is valid")],
        }
    }
}
```

Do not preserve CSS seconds versus milliseconds in style. Root must lower both CSS units into `DurationSeconds` before constructing `TimeList`.

- [ ] **Step 3: Add symbolic easing values**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EasingArguments {
    authored: String,
}

impl EasingArguments {
    pub fn try_new(authored: impl Into<String>) -> Result<Self> {
        let authored = authored.into();
        if authored.trim().is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "easing arguments cannot be empty",
            ));
        }
        if authored.contains('\0') {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "easing arguments cannot contain U+0000",
            ));
        }
        Ok(Self { authored })
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        &self.authored
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EasingFunction {
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    CubicBezier(EasingArguments),
    Steps(EasingArguments),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EasingList {
    values: Vec<EasingFunction>,
}

impl EasingList {
    pub fn try_new(values: impl IntoIterator<Item = EasingFunction>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "easing list cannot be empty",
            ));
        }
        Ok(Self { values })
    }

    #[must_use]
    pub fn values(&self) -> &[EasingFunction] {
        &self.values
    }

    #[must_use]
    pub fn single_ease() -> Self {
        Self {
            values: vec![EasingFunction::Ease],
        }
    }
}
```

These are symbolic style values. They must not parse, validate, or evaluate cubic-bezier or steps arguments beyond the style-owned string invariants above.

- [ ] **Step 4: Add transition property and shorthand models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransitionPropertyName {
    value: String,
}

impl TransitionPropertyName {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self> {
        let value = validate_timing_ident(value.as_ref(), "transition property name")?;
        if value.eq_ignore_ascii_case("none") || value.eq_ignore_ascii_case("all") {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "transition property name cannot be `none` or `all`",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TransitionPropertyTarget {
    All,
    None,
    Property(Property),
    Custom(TransitionPropertyName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransitionPropertyList {
    values: Vec<TransitionPropertyTarget>,
}

impl TransitionPropertyList {
    pub fn try_new(values: impl IntoIterator<Item = TransitionPropertyTarget>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "transition property list cannot be empty",
            ));
        }
        Ok(Self { values })
    }

    #[must_use]
    pub fn values(&self) -> &[TransitionPropertyTarget] {
        &self.values
    }

    #[must_use]
    pub fn single_all() -> Self {
        Self {
            values: vec![TransitionPropertyTarget::All],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionItem {
    property: Option<TransitionPropertyTarget>,
    duration: Option<DurationSeconds>,
    delay: Option<DurationSeconds>,
    timing_function: Option<EasingFunction>,
}

impl TransitionItem {
    pub fn try_new(
        property: Option<TransitionPropertyTarget>,
        duration: Option<DurationSeconds>,
        delay: Option<DurationSeconds>,
        timing_function: Option<EasingFunction>,
    ) -> Result<Self> {
        if property.is_none() && duration.is_none() && delay.is_none() && timing_function.is_none() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "transition item cannot be empty",
            ));
        }
        Ok(Self {
            property,
            duration,
            delay,
            timing_function,
        })
    }

    #[must_use]
    pub const fn property(&self) -> Option<&TransitionPropertyTarget> {
        self.property.as_ref()
    }

    #[must_use]
    pub const fn duration(&self) -> Option<DurationSeconds> {
        self.duration
    }

    #[must_use]
    pub const fn delay(&self) -> Option<DurationSeconds> {
        self.delay
    }

    #[must_use]
    pub const fn timing_function(&self) -> Option<&EasingFunction> {
        self.timing_function.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionList {
    items: Vec<TransitionItem>,
}

impl TransitionList {
    pub fn try_new(items: impl IntoIterator<Item = TransitionItem>) -> Result<Self> {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "transition list cannot be empty",
            ));
        }
        Ok(Self { items })
    }

    #[must_use]
    pub fn items(&self) -> &[TransitionItem] {
        &self.items
    }
}
```

Add this helper near `validate_generated_ident` or another local string-validation helper. This helper intentionally mirrors root-lowered `CssCustomIdent` semantics instead of imposing a CSS parser grammar or an ASCII-only subset:

```rust
fn validate_timing_ident(value: &str, label: &str) -> Result<String> {
    if value.is_empty() {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} cannot be empty"),
        ));
    }
    if value.contains('\0') {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} cannot contain U+0000"),
        ));
    }
    if is_css_wide_keyword(value)
        || matches!(value.to_ascii_lowercase().as_str(), "span" | "auto")
    {
        return Err(Error::new(
            ErrorCode::InvalidString,
            format!("{label} uses a reserved CSS identifier"),
        ));
    }
    Ok(value.to_owned())
}

fn is_css_wide_keyword(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    )
}
```

`TransitionPropertyName::try_new` must still reject the type-specific reserved identifiers `none` and `all`. `KeyframesIdent::try_new` must still reject the type-specific reserved identifier `none`. Do not trim, normalize, ASCII-filter, or parse the identifier in style; root owns CSS tokenization and source diagnostics.

- [ ] **Step 5: Add validation tests**

In `src/value.rs` tests, add:

```rust
#[test]
fn timing_lists_and_easing_values_preserve_symbolic_payloads() {
    let times = TimeList::try_new([
        DurationSeconds::new(0.2).unwrap(),
        DurationSeconds::new(1.0).unwrap(),
    ])
    .unwrap();
    assert_eq!(times.seconds().len(), 2);
    assert!(TimeList::try_new([]).is_err());

    let easing = EasingFunction::CubicBezier(EasingArguments::try_new("0.4, 0, 0.2, 1").unwrap());
    let list = EasingList::try_new([easing.clone()]).unwrap();
    assert_eq!(list.values(), &[easing]);
    assert_eq!(
        EasingArguments::try_new("").unwrap_err().code(),
        ErrorCode::InvalidString
    );
}

#[test]
fn transition_models_require_non_empty_lists_and_items() {
    let property = TransitionPropertyTarget::Custom(
        TransitionPropertyName::try_new("opacity").unwrap(),
    );
    let item = TransitionItem::try_new(
        Some(property.clone()),
        Some(DurationSeconds::new(0.15).unwrap()),
        None,
        Some(EasingFunction::EaseOut),
    )
    .unwrap();
    let list = TransitionList::try_new([item]).unwrap();
    assert!(matches!(list.items()[0].property(), Some(value) if value == &property));
    assert!(TransitionItem::try_new(None, None, None, None).is_err());
    assert!(TransitionList::try_new([]).is_err());
    assert!(TransitionPropertyList::try_new([]).is_err());
    assert!(TransitionPropertyName::try_new("none").is_err());
    assert!(TransitionPropertyName::try_new("-webkit-transform").is_ok());
    assert!(TransitionPropertyName::try_new("フェード").is_ok());
    assert!(TransitionPropertyName::try_new("auto").is_err());
}
```

- [ ] **Step 6: Reexport public front doors**

In `src/lib.rs`, add:

```rust
EasingArguments, EasingFunction, EasingList, TimeList, TransitionItem,
TransitionList, TransitionPropertyList, TransitionPropertyName,
TransitionPropertyTarget,
```

- [ ] **Step 7: Add compile-pass coverage**

In `tests/compile_pass/typed_public_construction.rs`, import and construct:

```rust
let time_list = TimeList::try_new([
    DurationSeconds::new(0.2)?,
    DurationSeconds::new(0.4)?,
])?;
let easing_list = EasingList::try_new([
    EasingFunction::EaseInOut,
    EasingFunction::CubicBezier(EasingArguments::try_new("0.4, 0, 0.2, 1")?),
])?;
let transition_properties = TransitionPropertyList::try_new([
    TransitionPropertyTarget::All,
    TransitionPropertyTarget::Custom(TransitionPropertyName::try_new("opacity")?),
])?;
let transition = TransitionList::try_new([TransitionItem::try_new(
    Some(TransitionPropertyTarget::Property(Property::Opacity)),
    Some(DurationSeconds::new(0.2)?),
    Some(DurationSeconds::new(0.05)?),
    Some(EasingFunction::EaseOut),
)?])?;
let _ = (time_list, easing_list, transition_properties, transition);
```

- [ ] **Step 8: Add compile-fail coverage**

Create `tests/compile_fail/invalid_time_list_empty_literal.rs`:

```rust
use surgeist_style::TimeList;

fn main() {
    let _list = TimeList { values: Vec::new() };
}
```

Create `tests/compile_fail/invalid_transition_property_name_literal.rs`:

```rust
use surgeist_style::TransitionPropertyName;

fn main() {
    let _name = TransitionPropertyName {
        value: String::from("opacity"),
    };
}
```

Generate and review the matching trybuild expectations:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_time_list_empty_literal.stderr tests/compile_fail/invalid_transition_property_name_literal.stderr
```

Expected: two `.stderr` files are created, and diagnostics show private-field construction is rejected.

- [ ] **Step 9: Run focused checks**

Run:

```sh
cargo test -p surgeist-style timing_lists_and_easing
cargo test -p surgeist-style transition_models
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass; only Task 1 files and matching `.stderr` expectation files are modified.

- [ ] **Step 10: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_time_list_empty_literal.rs tests/compile_fail/invalid_time_list_empty_literal.stderr tests/compile_fail/invalid_transition_property_name_literal.rs tests/compile_fail/invalid_transition_property_name_literal.stderr
git commit -m "style: add timing transition vocabulary"
```

---

### Task 2: Replace Transition Longhands And Add Transition Shorthand Lowering

**Files:**
- Modify: `src/property.rs`
- Modify: `src/value.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Modify: existing tests that assume transition duration/delay are bare numbers

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: clean after the Task 1 commit.

- [ ] **Step 2: Add transition value variants**

In `src/value.rs`, add these `Value` variants and remove transition use of `PropertyList(Vec<Property)` and `Number(f32)` for transition longhands:

```rust
TransitionPropertyList(TransitionPropertyList),
TimeList(TimeList),
EasingList(EasingList),
TransitionList(TransitionList),
```

If `Value::PropertyList(Vec<Property>)` has no non-transition users after this change, remove that variant, its validation arm, interpolation arm, `value_kind` label, and `hash_value` arm instead of leaving a compatibility-only value bag behind.

Update `Value::interpolation()` so all four are `Interpolation::Discrete`. Time values remain discrete style data in this pass; animation sampling/interpolation belongs outside style.

Update `Value::validate()`:

```rust
Self::TransitionPropertyList(value) => value.validate(),
Self::TimeList(value) => value.validate(),
Self::EasingList(value) => value.validate(),
Self::TransitionList(value) => value.validate(),
```

Add validation methods:

```rust
impl TimeList {
    pub fn validate(&self) -> Result<()> {
        if self.values.is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "time list cannot be empty"))
        } else {
            Ok(())
        }
    }
}

impl EasingList {
    pub fn validate(&self) -> Result<()> {
        if self.values.is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "easing list cannot be empty"))
        } else {
            Ok(())
        }
    }
}

impl TransitionPropertyList {
    pub fn validate(&self) -> Result<()> {
        if self.values.is_empty() {
            Err(Error::new(
                ErrorCode::InvalidValue,
                "transition property list cannot be empty",
            ))
        } else {
            Ok(())
        }
    }
}

impl TransitionList {
    pub fn validate(&self) -> Result<()> {
        if self.items.is_empty() {
            Err(Error::new(ErrorCode::InvalidValue, "transition list cannot be empty"))
        } else {
            Ok(())
        }
    }
}
```

- [ ] **Step 3: Update transition properties**

In `src/property.rs`:

- Rename `Property::TransitionTiming` to `Property::TransitionTimingFunction`.
- Add `Property::Transition` near the other transition properties.
- Add `TransitionTimingFunction` and `Transition` to `Property::ALL`.
- Mark only `Transition` as non-canonical.
- Update metadata:

```rust
Self::TransitionProperty => {
    Metadata::new(Value::TransitionPropertyList(TransitionPropertyList::single_all()))
        .impact(Impact::empty().animation())
}
Self::TransitionDuration | Self::TransitionDelay => {
    Metadata::new(Value::TimeList(TimeList::single_zero()))
        .impact(Impact::empty().animation())
}
Self::TransitionTimingFunction => {
    Metadata::new(Value::EasingList(EasingList::single_ease()))
        .impact(Impact::empty().animation())
}
Self::Transition => Metadata::new(Value::TransitionList(
    TransitionList::try_new([TransitionItem::try_new(
        Some(TransitionPropertyTarget::All),
        Some(DurationSeconds::new(0.0).unwrap()),
        Some(DurationSeconds::new(0.0).unwrap()),
        Some(EasingFunction::Ease),
    )
    .unwrap()])
    .unwrap(),
))
.impact(Impact::empty().animation()),
```

Update `accepts()`:

```rust
Self::TransitionProperty => matches!(value, Value::TransitionPropertyList(_)),
Self::TransitionDuration | Self::TransitionDelay => matches!(value, Value::TimeList(_)),
Self::TransitionTimingFunction => matches!(value, Value::EasingList(_)),
Self::Transition => matches!(value, Value::TransitionList(_)),
```

Remove transition duration/delay from `Value::Number` acceptance and `validate_non_negative_number` domain validation. `DurationSeconds` and `TimeList` own non-negative validation now.

Update `value_kind()` with labels:

```rust
Value::TransitionPropertyList(_) => "transition property list",
Value::TimeList(_) => "time list",
Value::EasingList(_) => "easing list",
Value::TransitionList(_) => "transition shorthand",
```

- [ ] **Step 4: Update declaration builders and shorthand lowering**

In `src/declaration.rs`, replace transition builders with typed list APIs:

```rust
pub fn transition_property(self, properties: TransitionPropertyList) -> Result<Self> {
    self.try_set(
        Property::TransitionProperty,
        Value::TransitionPropertyList(properties),
    )
}

pub fn transition_duration(self, durations: TimeList) -> Result<Self> {
    self.try_set(Property::TransitionDuration, Value::TimeList(durations))
}

pub fn transition_delay(self, delays: TimeList) -> Result<Self> {
    self.try_set(Property::TransitionDelay, Value::TimeList(delays))
}

pub fn transition_timing_function(self, easings: EasingList) -> Result<Self> {
    self.try_set(Property::TransitionTimingFunction, Value::EasingList(easings))
}

pub fn transition(self, transitions: TransitionList) -> Result<Self> {
    self.try_set(Property::Transition, Value::TransitionList(transitions))
}
```

Remove or replace the old scalar/list escape-hatch helpers if they no longer have a typed meaning:

```rust
try_transition_properties(self, properties: Vec<Property>)
transition_property_list(&self) -> Option<&[Property]>
transition_duration_number(&self) -> Option<f32>
transition_delay_number(&self) -> Option<f32>
```

Update `TypedDeclaration::transition_duration` to accept a `TimeList`, or remove it if there is no focused typed-declaration use case for list-valued timing declarations.

Update `canonical_properties()`:

```rust
Property::Transition => vec![
    Property::TransitionProperty,
    Property::TransitionDuration,
    Property::TransitionDelay,
    Property::TransitionTimingFunction,
],
```

Update `canonical_declarations()`:

```rust
(Property::Transition, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::Transition),
    Value::Keyword(keyword),
),
(Property::Transition, Value::TransitionList(value)) => transition_declarations(value),
```

Add:

```rust
fn transition_declarations(value: TransitionList) -> Vec<Declaration> {
    let mut properties = Vec::new();
    let mut durations = Vec::new();
    let mut delays = Vec::new();
    let mut easings = Vec::new();

    for item in value.items() {
        properties.push(
            item.property()
                .cloned()
                .unwrap_or(TransitionPropertyTarget::All),
        );
        durations.push(item.duration().unwrap_or_else(|| DurationSeconds::new(0.0).unwrap()));
        delays.push(item.delay().unwrap_or_else(|| DurationSeconds::new(0.0).unwrap()));
        easings.push(item.timing_function().cloned().unwrap_or(EasingFunction::Ease));
    }

    vec![
        Declaration::new(
            Property::TransitionProperty,
            Value::TransitionPropertyList(TransitionPropertyList::try_new(properties).unwrap()),
        ),
        Declaration::new(
            Property::TransitionDuration,
            Value::TimeList(TimeList::try_new(durations).unwrap()),
        ),
        Declaration::new(
            Property::TransitionDelay,
            Value::TimeList(TimeList::try_new(delays).unwrap()),
        ),
        Declaration::new(
            Property::TransitionTimingFunction,
            Value::EasingList(EasingList::try_new(easings).unwrap()),
        ),
    ]
}
```

Remove or update `transition_duration_number()` and `transition_delay_number()` helpers so declarations expose `Option<&TimeList>` for transition duration/delay.

- [ ] **Step 5: Add stable hashing**

In `src/declaration.rs`, update `hash_value()` with new top-level discriminants after the current highest tag:

```rust
Value::TransitionPropertyList(value) => {
    99u8.hash(state);
    hash_transition_property_list(value, state);
}
Value::TimeList(value) => {
    100u8.hash(state);
    hash_time_list(value, state);
}
Value::EasingList(value) => {
    101u8.hash(state);
    hash_easing_list(value, state);
}
Value::TransitionList(value) => {
    102u8.hash(state);
    hash_transition_list(value, state);
}
```

Add helper functions that explicitly hash list lengths, scalar seconds, transition target discriminants, property enum values, custom names, easing discriminants, and symbolic argument strings. Do not use `Debug` strings for hash identity.

- [ ] **Step 6: Update resolved getters**

In `src/resolver.rs`, update the transition getters:

```rust
pub fn transition_properties(&self) -> &TransitionPropertyList {
    match self.get(Property::TransitionProperty) {
        Value::TransitionPropertyList(properties) => properties,
        _ => unreachable!("resolved transition-property stores transition property list"),
    }
}

pub fn transition_duration(&self) -> &TimeList {
    match self.get(Property::TransitionDuration) {
        Value::TimeList(durations) => durations,
        _ => unreachable!("resolved transition-duration stores time list"),
    }
}

pub fn transition_delay(&self) -> &TimeList {
    match self.get(Property::TransitionDelay) {
        Value::TimeList(delays) => delays,
        _ => unreachable!("resolved transition-delay stores time list"),
    }
}

pub fn transition_timing_function(&self) -> &EasingList {
    match self.get(Property::TransitionTimingFunction) {
        Value::EasingList(easings) => easings,
        _ => unreachable!("resolved transition-timing-function stores easing list"),
    }
}
```

- [ ] **Step 7: Add focused tests**

In `src/declaration.rs` tests, add:

```rust
#[test]
fn transition_shorthand_lowers_to_typed_lists() {
    let declarations = Declarations::new()
        .transition(
            TransitionList::try_new([
                TransitionItem::try_new(
                    Some(TransitionPropertyTarget::Property(Property::Opacity)),
                    Some(DurationSeconds::new(0.2).unwrap()),
                    Some(DurationSeconds::new(0.05).unwrap()),
                    Some(EasingFunction::EaseOut),
                )
                .unwrap(),
            ])
            .unwrap(),
        )
        .unwrap();

    assert_eq!(declarations.get(Property::Transition), None);
    assert!(matches!(
        declarations.get(Property::TransitionProperty),
        Some(Value::TransitionPropertyList(_))
    ));
    assert!(matches!(
        declarations.get(Property::TransitionDuration),
        Some(Value::TimeList(_))
    ));
    assert!(matches!(
        declarations.get(Property::TransitionTimingFunction),
        Some(Value::EasingList(_))
    ));
}

#[test]
fn transition_shorthand_resets_omitted_components_to_defaults() {
    let declarations = Declarations::new()
        .transition(
            TransitionList::try_new([
                TransitionItem::try_new(
                    Some(TransitionPropertyTarget::Property(Property::Opacity)),
                    None,
                    None,
                    None,
                )
                .unwrap(),
            ])
            .unwrap(),
        )
        .unwrap();

    let durations = declarations
        .get(Property::TransitionDuration)
        .and_then(|value| match value {
            Value::TimeList(values) => Some(values),
            _ => None,
        })
        .unwrap();
    assert_eq!(durations.seconds()[0].get(), 0.0);
}
```

In `src/authored.rs` tests, add CSS-wide coverage for `transition`.

- [ ] **Step 8: Update compile-pass construction**

In `tests/compile_pass/typed_public_construction.rs`, update existing transition construction to use the new typed list APIs:

```rust
let declarations = Declarations::new()
    .transition_property(TransitionPropertyList::try_new([
        TransitionPropertyTarget::Property(Property::Opacity),
    ])?)?
    .transition_duration(TimeList::try_new([DurationSeconds::new(0.2)?])?)?
    .transition_delay(TimeList::try_new([DurationSeconds::new(0.05)?])?)?
    .transition_timing_function(EasingList::try_new([EasingFunction::EaseInOut])?)?
    .transition(TransitionList::try_new([TransitionItem::try_new(
        Some(TransitionPropertyTarget::Property(Property::Opacity)),
        Some(DurationSeconds::new(0.2)?),
        Some(DurationSeconds::new(0.05)?),
        Some(EasingFunction::EaseInOut),
    )?])?)?;
let _ = declarations;
```

Update compile-fail expectations for `invalid_declarations_builders.rs` if diagnostics move from `DurationSeconds` to `TimeList`.

- [ ] **Step 9: Run focused checks**

Run:

```sh
cargo test -p surgeist-style transition_shorthand
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start + 1)
body = text[start:end]
arms = []
for match in re.finditer(r'Value::([A-Za-z0-9_]+)(?:\([^=]*?\))?\s*=>\s*\{\s*(\d+)u8\.hash\(state\);', body, re.S):
    arms.append((match.group(1), int(match.group(2))))
seen = {}
dups = {}
for name, tag in arms:
    if tag in seen:
        dups.setdefault(tag, [seen[tag]]).append(name)
    seen[tag] = name
print(f'top-level tagged value arms={len(arms)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
git diff --check
git status --short --branch
```

Expected: all pass; hash duplicate check reports no duplicate top-level value tags.

- [ ] **Step 10: Commit after worker/reviewer clean**

```sh
git add src/property.rs src/value.rs src/declaration.rs src/resolver.rs src/authored.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_declarations_builders.stderr
git commit -m "style: model transition timing lists"
```

---

### Task 3: Add Animation Longhand Models

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/resolver.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_animation_iteration_number_literal.rs`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: clean after the Task 2 commit.

- [ ] **Step 2: Replace animation-name storage with typed animation names**

In `src/value.rs`, replace the string-list-only `AnimationNameList` model with:

```rust
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyframesIdent {
    value: String,
}

impl KeyframesIdent {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self> {
        let value = validate_timing_ident(value.as_ref(), "keyframes name")?;
        if value.eq_ignore_ascii_case("none") {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "keyframes name cannot be `none`",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KeyframesString {
    value: String,
}

impl KeyframesString {
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "keyframes string name cannot be empty",
            ));
        }
        if value.contains('\0') {
            return Err(Error::new(
                ErrorCode::InvalidString,
                "keyframes string name cannot contain U+0000",
            ));
        }
        Ok(Self { value })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum KeyframesName {
    Ident(KeyframesIdent),
    String(KeyframesString),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AnimationName {
    None,
    Keyframes(KeyframesName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AnimationNameList {
    names: Vec<AnimationName>,
}

impl AnimationNameList {
    pub fn try_new(names: impl IntoIterator<Item = AnimationName>) -> Result<Self> {
        let names = names.into_iter().collect::<Vec<_>>();
        if names.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "animation name list cannot be empty",
            ));
        }
        Ok(Self { names })
    }

    #[must_use]
    pub fn names(&self) -> &[AnimationName] {
        &self.names
    }

    #[must_use]
    pub fn single_none() -> Self {
        Self {
            names: vec![AnimationName::None],
        }
    }
}
```

Update existing tests that expect `AnimationNameList::empty()` or string iteration. CSS initial animation name is a one-item `none` list, not an empty list.

- [ ] **Step 3: Add animation longhand list models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationIterationCount {
    Infinite,
    Number(AnimationIterationNumber),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationIterationNumber {
    value: f32,
}

impl AnimationIterationNumber {
    pub fn try_new(value: f32) -> Result<Self> {
        validate_non_negative(value, "animation iteration count")?;
        Ok(Self { value })
    }

    #[must_use]
    pub const fn get(self) -> f32 {
        self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationIterationCountList {
    values: Vec<AnimationIterationCount>,
}

impl AnimationIterationCountList {
    pub fn try_new(values: impl IntoIterator<Item = AnimationIterationCount>) -> Result<Self> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "animation iteration count list cannot be empty",
            ));
        }
        Ok(Self { values })
    }

    #[must_use]
    pub fn values(&self) -> &[AnimationIterationCount] {
        &self.values
    }

    #[must_use]
    pub fn single_one() -> Self {
        Self {
            values: vec![AnimationIterationCount::Number(
                AnimationIterationNumber::try_new(1.0).unwrap(),
            )],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AnimationDirection {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AnimationFillMode {
    #[default]
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AnimationPlayState {
    #[default]
    Running,
    Paused,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AnimationDirectionList {
    values: Vec<AnimationDirection>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AnimationFillModeList {
    values: Vec<AnimationFillMode>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AnimationPlayStateList {
    values: Vec<AnimationPlayState>,
}
```

Implement `try_new`, `values`, and single-default constructors for each list:

```rust
AnimationDirectionList::single_normal()
AnimationFillModeList::single_none()
AnimationPlayStateList::single_running()
```

Each `try_new` must reject an empty iterator and keep fields private.

- [ ] **Step 4: Add animation value variants**

In `src/value.rs`, add:

```rust
AnimationIterationCountList(AnimationIterationCountList),
AnimationDirectionList(AnimationDirectionList),
AnimationFillModeList(AnimationFillModeList),
AnimationPlayStateList(AnimationPlayStateList),
```

Reuse `Value::TimeList` for `animation-duration` and `animation-delay`, and reuse `Value::EasingList` for `animation-timing-function`.

Update `Value::interpolation()` and `Value::validate()` for all new animation list variants.

- [ ] **Step 5: Add animation property variants**

In `src/property.rs`, add these variants after `AnimationName`:

```rust
AnimationDuration,
AnimationDelay,
AnimationTimingFunction,
AnimationIterationCount,
AnimationDirection,
AnimationFillMode,
AnimationPlayState,
```

Add them to `Property::ALL`.

Update metadata:

```rust
Self::AnimationName => Metadata::new(Value::AnimationNameList(AnimationNameList::single_none()))
    .impact(Impact::empty().animation()),
Self::AnimationDuration | Self::AnimationDelay => {
    Metadata::new(Value::TimeList(TimeList::single_zero()))
        .impact(Impact::empty().animation())
}
Self::AnimationTimingFunction => Metadata::new(Value::EasingList(EasingList::single_ease()))
    .impact(Impact::empty().animation()),
Self::AnimationIterationCount => Metadata::new(Value::AnimationIterationCountList(
    AnimationIterationCountList::single_one(),
))
.impact(Impact::empty().animation()),
Self::AnimationDirection => Metadata::new(Value::AnimationDirectionList(
    AnimationDirectionList::single_normal(),
))
.impact(Impact::empty().animation()),
Self::AnimationFillMode => Metadata::new(Value::AnimationFillModeList(
    AnimationFillModeList::single_none(),
))
.impact(Impact::empty().animation()),
Self::AnimationPlayState => Metadata::new(Value::AnimationPlayStateList(
    AnimationPlayStateList::single_running(),
))
.impact(Impact::empty().animation()),
```

Update `accepts()` and `value_kind()` for each variant.

- [ ] **Step 6: Add declaration builders and resolved getters**

In `src/declaration.rs`, add builder methods:

```rust
pub fn animation_name(self, names: AnimationNameList) -> Result<Self> {
    self.try_set(Property::AnimationName, Value::AnimationNameList(names))
}

pub fn animation_duration(self, durations: TimeList) -> Result<Self> {
    self.try_set(Property::AnimationDuration, Value::TimeList(durations))
}

pub fn animation_delay(self, delays: TimeList) -> Result<Self> {
    self.try_set(Property::AnimationDelay, Value::TimeList(delays))
}

pub fn animation_timing_function(self, easings: EasingList) -> Result<Self> {
    self.try_set(Property::AnimationTimingFunction, Value::EasingList(easings))
}

pub fn animation_iteration_count(self, values: AnimationIterationCountList) -> Result<Self> {
    self.try_set(
        Property::AnimationIterationCount,
        Value::AnimationIterationCountList(values),
    )
}

pub fn animation_direction(self, values: AnimationDirectionList) -> Result<Self> {
    self.try_set(Property::AnimationDirection, Value::AnimationDirectionList(values))
}

pub fn animation_fill_mode(self, values: AnimationFillModeList) -> Result<Self> {
    self.try_set(Property::AnimationFillMode, Value::AnimationFillModeList(values))
}

pub fn animation_play_state(self, values: AnimationPlayStateList) -> Result<Self> {
    self.try_set(Property::AnimationPlayState, Value::AnimationPlayStateList(values))
}
```

In `src/resolver.rs`, add typed getters for all animation longhands.

- [ ] **Step 7: Reexport public front doors**

In `src/lib.rs`, add:

```rust
AnimationDirection, AnimationDirectionList, AnimationFillMode,
AnimationFillModeList, AnimationIterationCount, AnimationIterationCountList,
AnimationIterationNumber, AnimationName, AnimationPlayState,
AnimationPlayStateList, KeyframesIdent, KeyframesName, KeyframesString,
```

- [ ] **Step 8: Add stable hashing**

In `src/declaration.rs`, update `hash_value()` with new top-level discriminants after Task 2:

```rust
Value::AnimationIterationCountList(value) => { 103u8.hash(state); hash_animation_iteration_count_list(value, state); }
Value::AnimationDirectionList(value) => { 104u8.hash(state); value.hash(state); }
Value::AnimationFillModeList(value) => { 105u8.hash(state); value.hash(state); }
Value::AnimationPlayStateList(value) => { 106u8.hash(state); value.hash(state); }
```

Update `Value::AnimationNameList` hashing so it hashes `AnimationName::None`, custom ident names, and string names with explicit discriminants. Do not hash `Debug` output.

- [ ] **Step 9: Add focused tests**

In `src/value.rs` tests, add:

```rust
#[test]
fn animation_longhand_lists_preserve_css_timing_shape() {
    let names = AnimationNameList::try_new([
        AnimationName::None,
        AnimationName::Keyframes(KeyframesName::Ident(
            KeyframesIdent::try_new("fade-in").unwrap(),
        )),
    ])
    .unwrap();
    assert_eq!(names.names().len(), 2);

    let iterations = AnimationIterationCountList::try_new([
        AnimationIterationCount::Number(AnimationIterationNumber::try_new(2.5).unwrap()),
        AnimationIterationCount::Infinite,
    ])
    .unwrap();
    assert_eq!(iterations.values().len(), 2);
    assert!(AnimationIterationNumber::try_new(-1.0).is_err());
}
```

In `src/resolver.rs` tests, add an animation longhand smoke test that resolves name, duration, timing function, iteration, direction, fill mode, and play state together.

- [ ] **Step 10: Add compile-fail coverage**

Create `tests/compile_fail/invalid_animation_iteration_number_literal.rs`:

```rust
use surgeist_style::AnimationIterationNumber;

fn main() {
    let _count = AnimationIterationNumber { value: 1.0 };
}
```

Generate and review the matching trybuild expectation:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_animation_iteration_number_literal.stderr
```

- [ ] **Step 11: Run focused checks**

Run:

```sh
cargo test -p surgeist-style animation_longhand_lists
cargo test -p surgeist-style animation_longhand
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 12: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/resolver.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_animation_iteration_number_literal.rs tests/compile_fail/invalid_animation_iteration_number_literal.stderr
git commit -m "style: add animation longhand models"
```

---

### Task 4: Add Animation Shorthand Lowering

**Files:**
- Modify: `src/value.rs`
- Modify: `src/property.rs`
- Modify: `src/declaration.rs`
- Modify: `src/authored.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: clean after the Task 3 commit.

- [ ] **Step 2: Add animation shorthand models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationItem {
    name: Option<AnimationName>,
    duration: Option<DurationSeconds>,
    delay: Option<DurationSeconds>,
    timing_function: Option<EasingFunction>,
    iteration_count: Option<AnimationIterationCount>,
    direction: Option<AnimationDirection>,
    fill_mode: Option<AnimationFillMode>,
    play_state: Option<AnimationPlayState>,
}

impl AnimationItem {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        name: Option<AnimationName>,
        duration: Option<DurationSeconds>,
        delay: Option<DurationSeconds>,
        timing_function: Option<EasingFunction>,
        iteration_count: Option<AnimationIterationCount>,
        direction: Option<AnimationDirection>,
        fill_mode: Option<AnimationFillMode>,
        play_state: Option<AnimationPlayState>,
    ) -> Result<Self> {
        if name.is_none()
            && duration.is_none()
            && delay.is_none()
            && timing_function.is_none()
            && iteration_count.is_none()
            && direction.is_none()
            && fill_mode.is_none()
            && play_state.is_none()
        {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "animation item cannot be empty",
            ));
        }
        Ok(Self {
            name,
            duration,
            delay,
            timing_function,
            iteration_count,
            direction,
            fill_mode,
            play_state,
        })
    }

    #[must_use]
    pub const fn name(&self) -> Option<&AnimationName> { self.name.as_ref() }
    #[must_use]
    pub const fn duration(&self) -> Option<DurationSeconds> { self.duration }
    #[must_use]
    pub const fn delay(&self) -> Option<DurationSeconds> { self.delay }
    #[must_use]
    pub const fn timing_function(&self) -> Option<&EasingFunction> { self.timing_function.as_ref() }
    #[must_use]
    pub const fn iteration_count(&self) -> Option<AnimationIterationCount> { self.iteration_count }
    #[must_use]
    pub const fn direction(&self) -> Option<AnimationDirection> { self.direction }
    #[must_use]
    pub const fn fill_mode(&self) -> Option<AnimationFillMode> { self.fill_mode }
    #[must_use]
    pub const fn play_state(&self) -> Option<AnimationPlayState> { self.play_state }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationList {
    items: Vec<AnimationItem>,
}

impl AnimationList {
    pub fn try_new(items: impl IntoIterator<Item = AnimationItem>) -> Result<Self> {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.is_empty() {
            return Err(Error::new(ErrorCode::InvalidValue, "animation list cannot be empty"));
        }
        Ok(Self { items })
    }

    #[must_use]
    pub fn items(&self) -> &[AnimationItem] {
        &self.items
    }
}
```

Add `Value::AnimationList(AnimationList)` and validate it as a nonempty list.

- [ ] **Step 3: Reexport public front doors**

In `src/lib.rs`, add:

```rust
AnimationItem, AnimationList,
```

- [ ] **Step 4: Add animation shorthand property**

In `src/property.rs`, add `Property::Animation` after animation longhands, add it to `Property::ALL`, and mark it non-canonical.

Metadata:

```rust
Self::Animation => Metadata::new(Value::AnimationList(
    AnimationList::try_new([AnimationItem::try_new(
        Some(AnimationName::None),
        Some(DurationSeconds::new(0.0).unwrap()),
        Some(DurationSeconds::new(0.0).unwrap()),
        Some(EasingFunction::Ease),
        Some(AnimationIterationCount::Number(AnimationIterationNumber::try_new(1.0).unwrap())),
        Some(AnimationDirection::Normal),
        Some(AnimationFillMode::None),
        Some(AnimationPlayState::Running),
    )
    .unwrap()])
    .unwrap(),
))
.impact(Impact::empty().animation()),
```

Acceptance:

```rust
Self::Animation => matches!(value, Value::AnimationList(_)),
```

- [ ] **Step 5: Add animation shorthand lowering**

In `src/declaration.rs`, add:

```rust
pub fn animation(self, animations: AnimationList) -> Result<Self> {
    self.try_set(Property::Animation, Value::AnimationList(animations))
}
```

Update `canonical_properties()`:

```rust
Property::Animation => vec![
    Property::AnimationName,
    Property::AnimationDuration,
    Property::AnimationDelay,
    Property::AnimationTimingFunction,
    Property::AnimationIterationCount,
    Property::AnimationDirection,
    Property::AnimationFillMode,
    Property::AnimationPlayState,
],
```

Update `canonical_declarations()`:

```rust
(Property::Animation, Value::Keyword(keyword)) => same_value_declarations(
    canonical_properties(Property::Animation),
    Value::Keyword(keyword),
),
(Property::Animation, Value::AnimationList(value)) => animation_declarations(value),
```

Add `animation_declarations` that lowers each `AnimationItem` to parallel longhand lists, filling omitted components with CSS initial defaults:

- name: `AnimationName::None`
- duration: `0s`
- delay: `0s`
- timing function: `EasingFunction::Ease`
- iteration count: `1`
- direction: `Normal`
- fill mode: `None`
- play state: `Running`

- [ ] **Step 6: Add stable hashing**

In `src/declaration.rs`, update `hash_value()` with:

```rust
Value::AnimationList(value) => {
    107u8.hash(state);
    hash_animation_list(value, state);
}
```

Hash item lists explicitly and do not use `Debug` strings.

- [ ] **Step 7: Add focused tests**

In `src/declaration.rs` tests, add:

```rust
#[test]
fn animation_shorthand_lowers_to_typed_longhand_lists() {
    let fade = AnimationName::Keyframes(KeyframesName::Ident(
        KeyframesIdent::try_new("fade-in").unwrap(),
    ));
    let declarations = Declarations::new()
        .animation(
            AnimationList::try_new([AnimationItem::try_new(
                Some(fade.clone()),
                Some(DurationSeconds::new(0.3).unwrap()),
                Some(DurationSeconds::new(0.1).unwrap()),
                Some(EasingFunction::EaseInOut),
                Some(AnimationIterationCount::Infinite),
                Some(AnimationDirection::Alternate),
                Some(AnimationFillMode::Both),
                Some(AnimationPlayState::Paused),
            )
            .unwrap()])
            .unwrap(),
        )
        .unwrap();

    assert_eq!(declarations.get(Property::Animation), None);
    assert!(matches!(
        declarations.get(Property::AnimationName),
        Some(Value::AnimationNameList(_))
    ));
    assert!(matches!(
        declarations.get(Property::AnimationDuration),
        Some(Value::TimeList(_))
    ));
    assert!(matches!(
        declarations.get(Property::AnimationIterationCount),
        Some(Value::AnimationIterationCountList(_))
    ));
}

#[test]
fn animation_shorthand_resets_omitted_components_to_defaults() {
    let declarations = Declarations::new()
        .animation(
            AnimationList::try_new([AnimationItem::try_new(
                Some(AnimationName::Keyframes(KeyframesName::Ident(
                    KeyframesIdent::try_new("fade-in").unwrap(),
                ))),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .unwrap()])
            .unwrap(),
        )
        .unwrap();

    assert!(matches!(
        declarations.get(Property::AnimationTimingFunction),
        Some(Value::EasingList(values)) if matches!(values.values(), [EasingFunction::Ease])
    ));
}
```

In `src/authored.rs` tests, add CSS-wide `animation` expansion coverage.

- [ ] **Step 8: Run focused checks**

Run:

```sh
cargo test -p surgeist-style animation_shorthand
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 9: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/property.rs src/declaration.rs src/authored.rs src/lib.rs tests/compile_pass/typed_public_construction.rs
git commit -m "style: lower animation shorthand"
```

---

### Task 5: Add Keyframe Rule Models And Sheet Storage

**Files:**
- Modify: `src/value.rs`
- Modify: `src/sheet.rs`
- Modify: `src/lib.rs`
- Modify: `tests/compile_pass/typed_public_construction.rs`
- Create: `tests/compile_fail/invalid_keyframe_offset_literal.rs`

- [ ] **Step 1: Check status**

Run:

```sh
git status --short --branch
```

Expected: clean after the Task 4 commit.

- [ ] **Step 2: Add keyframe offset and selector list models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyframeOffset {
    percent: f32,
}

impl KeyframeOffset {
    pub fn try_new(percent: f32) -> Result<Self> {
        validate_finite(percent, "keyframe offset")?;
        if !(0.0..=100.0).contains(&percent) {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "keyframe offset must be between 0 and 100 percent",
            ));
        }
        Ok(Self { percent })
    }

    #[must_use]
    pub fn from() -> Self {
        Self::try_new(0.0).unwrap()
    }

    #[must_use]
    pub fn to() -> Self {
        Self::try_new(100.0).unwrap()
    }

    #[must_use]
    pub const fn percent(self) -> f32 {
        self.percent
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeSelectorList {
    offsets: Vec<KeyframeOffset>,
}

impl KeyframeSelectorList {
    pub fn try_new(offsets: impl IntoIterator<Item = KeyframeOffset>) -> Result<Self> {
        let offsets = offsets.into_iter().collect::<Vec<_>>();
        if offsets.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "keyframe selector list cannot be empty",
            ));
        }
        let mut seen = Vec::new();
        for offset in &offsets {
            let percent = offset.percent();
            if seen.contains(&percent) {
                return Err(Error::new(
                    ErrorCode::InvalidValue,
                    "keyframe selector list cannot contain duplicate offsets",
                ));
            }
            seen.push(percent);
        }
        Ok(Self { offsets })
    }

    #[must_use]
    pub fn offsets(&self) -> &[KeyframeOffset] {
        &self.offsets
    }
}
```

- [ ] **Step 3: Add keyframe block and rule models**

In `src/value.rs`, add:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeBlock {
    selectors: KeyframeSelectorList,
    declarations: AuthoredDeclarations,
}

impl KeyframeBlock {
    pub fn try_new(selectors: KeyframeSelectorList, declarations: AuthoredDeclarations) -> Result<Self> {
        if declarations.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "keyframe block declarations cannot be empty",
            ));
        }
        Ok(Self {
            selectors,
            declarations,
        })
    }

    #[must_use]
    pub const fn selectors(&self) -> &KeyframeSelectorList {
        &self.selectors
    }

    #[must_use]
    pub const fn declarations(&self) -> &AuthoredDeclarations {
        &self.declarations
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyframesRule {
    name: KeyframesName,
    blocks: Vec<KeyframeBlock>,
}

impl KeyframesRule {
    pub fn try_new(name: KeyframesName, blocks: impl IntoIterator<Item = KeyframeBlock>) -> Result<Self> {
        let blocks = blocks.into_iter().collect::<Vec<_>>();
        if blocks.is_empty() {
            return Err(Error::new(
                ErrorCode::InvalidValue,
                "keyframes rule must contain at least one block",
            ));
        }
        let mut seen = Vec::new();
        for block in &blocks {
            for offset in block.selectors().offsets() {
                let percent = offset.percent();
                if seen.contains(&percent) {
                    return Err(Error::new(
                        ErrorCode::InvalidValue,
                        "keyframes rule cannot contain duplicate offsets",
                    ));
                }
                seen.push(percent);
            }
        }
        Ok(Self { name, blocks })
    }

    #[must_use]
    pub const fn name(&self) -> &KeyframesName {
        &self.name
    }

    #[must_use]
    pub fn blocks(&self) -> &[KeyframeBlock] {
        &self.blocks
    }
}
```

Import `AuthoredDeclarations` at the top of `src/value.rs` for this model.

- [ ] **Step 4: Store keyframe rules on Sheet**

In `src/sheet.rs`, add a `keyframes: Vec<KeyframesRule>` field to `Sheet`.

Update `Default`:

```rust
Self {
    rules: Vec::new(),
    keyframes: Vec::new(),
    index: RuleIndex::default(),
    version: Version::next(),
}
```

Update `PartialEq` so it compares both `rules` and `keyframes`.

Add public APIs:

```rust
pub fn push_keyframes_rule(&mut self, rule: KeyframesRule) -> &mut Self {
    self.keyframes.push(rule);
    self.version = Version::next();
    self
}

#[must_use]
pub fn keyframes_rule(mut self, rule: KeyframesRule) -> Self {
    self.push_keyframes_rule(rule);
    self
}

#[must_use]
pub fn keyframes_rules(&self) -> &[KeyframesRule] {
    &self.keyframes
}

#[must_use]
pub fn keyframes_rule_count(&self) -> usize {
    self.keyframes.len()
}
```

Do not insert keyframes into `RuleIndex`. Keyframes are not selector-matched style rules.

- [ ] **Step 5: Add public reexports and compile-pass coverage**

In `src/lib.rs`, reexport:

```rust
KeyframeBlock, KeyframeOffset, KeyframeSelectorList, KeyframesIdent,
KeyframesName, KeyframesRule, KeyframesString,
```

In `tests/compile_pass/typed_public_construction.rs`, construct a keyframes rule:

```rust
let mut keyframe_declarations = AuthoredDeclarations::new();
keyframe_declarations.try_push(AuthoredDeclaration::try_new(
    AuthoredProperty::Property(Property::Opacity),
    AuthoredValue::Value(Value::Number(1.0)),
)?)?;
let keyframes = KeyframesRule::try_new(
    KeyframesName::Ident(KeyframesIdent::try_new("fade-in")?),
    [KeyframeBlock::try_new(
        KeyframeSelectorList::try_new([KeyframeOffset::from(), KeyframeOffset::to()])?,
        keyframe_declarations,
    )?],
)?;
let sheet = Sheet::new().keyframes_rule(keyframes);
assert_eq!(sheet.keyframes_rule_count(), 1);
```

- [ ] **Step 6: Add tests**

In `src/value.rs` tests, add:

```rust
#[test]
fn keyframes_require_non_empty_unique_offsets_and_declarations() {
    let mut declarations = AuthoredDeclarations::new();
    declarations
        .try_push(AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Opacity),
            AuthoredValue::Value(Value::Number(1.0)),
        )
        .unwrap())
        .unwrap();

    let selectors = KeyframeSelectorList::try_new([
        KeyframeOffset::from(),
        KeyframeOffset::to(),
    ])
    .unwrap();
    let block = KeyframeBlock::try_new(selectors, declarations).unwrap();
    let rule = KeyframesRule::try_new(
        KeyframesName::Ident(KeyframesIdent::try_new("fade-in").unwrap()),
        [block],
    )
    .unwrap();

    assert_eq!(rule.blocks().len(), 1);
    assert!(KeyframeOffset::try_new(101.0).is_err());
    assert!(KeyframeSelectorList::try_new([KeyframeOffset::from(), KeyframeOffset::from()]).is_err());
}
```

In `src/sheet.rs` tests, add:

```rust
#[test]
fn keyframes_store_on_sheet_without_entering_rule_index() {
    let mut declarations = AuthoredDeclarations::new();
    declarations
        .try_push(AuthoredDeclaration::try_new(
            AuthoredProperty::Property(Property::Opacity),
            AuthoredValue::Value(Value::Number(1.0)),
        )
        .unwrap())
        .unwrap();
    let keyframes = KeyframesRule::try_new(
        KeyframesName::Ident(KeyframesIdent::try_new("fade-in").unwrap()),
        [KeyframeBlock::try_new(
            KeyframeSelectorList::try_new([KeyframeOffset::to()]).unwrap(),
            declarations,
        )
        .unwrap()],
    )
    .unwrap();
    let sheet = Sheet::new().keyframes_rule(keyframes);

    assert_eq!(sheet.rule_count(), 0);
    assert_eq!(sheet.keyframes_rule_count(), 1);
    assert_eq!(sheet.keyframes_rules()[0].blocks().len(), 1);
}
```

- [ ] **Step 7: Add compile-fail coverage**

Create `tests/compile_fail/invalid_keyframe_offset_literal.rs`:

```rust
use surgeist_style::KeyframeOffset;

fn main() {
    let _offset = KeyframeOffset { percent: 50.0 };
}
```

Generate and review the matching expectation:

```sh
TRYBUILD=overwrite cargo test -p surgeist-style public_type_safety_contract
git diff -- tests/compile_fail/invalid_keyframe_offset_literal.stderr
```

- [ ] **Step 8: Run focused checks**

Run:

```sh
cargo test -p surgeist-style keyframes_require
cargo test -p surgeist-style keyframes_store
cargo test -p surgeist-style public_type_safety_contract
cargo fmt --check
git diff --check
git status --short --branch
```

Expected: all pass.

- [ ] **Step 9: Commit after worker/reviewer clean**

```sh
git add src/value.rs src/sheet.rs src/lib.rs tests/compile_pass/typed_public_construction.rs tests/compile_fail/invalid_keyframe_offset_literal.rs tests/compile_fail/invalid_keyframe_offset_literal.stderr
git commit -m "style: store keyframe style data"
```

---

### Task 6: Rebase Operation 12 Ledger And Run Final Checks

**Files:**
- Modify: `plans/2026-07-05-css-property-coverage-ledger.md`

- [ ] **Step 1: Rebase ledger rows**

Update only Operation 12-affected rows:

- `TransitionProperty`: `Existing style property`, target `Property::TransitionProperty` + `Value::TransitionPropertyList`.
- `TransitionDuration`: `Existing style property`, target `Property::TransitionDuration` + `Value::TimeList`.
- `TransitionDelay`: `Existing style property`, target `Property::TransitionDelay` + `Value::TimeList`.
- `TransitionTimingFunction`: `Existing style property`, target `Property::TransitionTimingFunction` + `Value::EasingList`; note easing evaluation remains outside style.
- `Transition`: `Existing style shorthand`, target `Property::Transition` + `Value::TransitionList`; note lowering to property, duration, delay, and timing-function longhands.
- `AnimationName`: `Existing style property`, target `Property::AnimationName` + `Value::AnimationNameList`.
- `AnimationDuration`: `Existing style property`, target `Property::AnimationDuration` + `Value::TimeList`.
- `AnimationDelay`: `Existing style property`, target `Property::AnimationDelay` + `Value::TimeList`.
- `AnimationTimingFunction`: `Existing style property`, target `Property::AnimationTimingFunction` + `Value::EasingList`.
- `AnimationIterationCount`: `Existing style property`, target `Property::AnimationIterationCount` + `Value::AnimationIterationCountList`.
- `AnimationDirection`: `Existing style property`, target `Property::AnimationDirection` + `Value::AnimationDirectionList`.
- `AnimationFillMode`: `Existing style property`, target `Property::AnimationFillMode` + `Value::AnimationFillModeList`.
- `AnimationPlayState`: `Existing style property`, target `Property::AnimationPlayState` + `Value::AnimationPlayStateList`.
- `Animation`: `Existing style shorthand`, target `Property::Animation` + `Value::AnimationList`; note lowering to animation longhands.

Update the family rollup:

```markdown
| Timing and animation | Transition property/duration/delay/timing-function/shorthand, animation name/duration/delay/timing-function/iteration/direction/fill-mode/play-state/shorthand, symbolic easing payloads, and keyframe style data have typed style targets. | Runtime scheduling, animation sampling, easing evaluation, compositor/render decisions, and final keyframe interpolation remain outside style. | No property implementation |
```

Update `Next Sequence Context` so Operation 13 conditions/layers/scope integration comes next, with Operation 14 cache/invalidation integration still following after the condition/layer/scope model exists.

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
git commit -m "style: rebase timing animation ledger"
```

---

## Root Handoff Notes

After this plan is implemented, root can lower CSS timing and keyframe syntax into these style-owned APIs:

- CSS `s` and `ms` values -> `DurationSeconds`, normalized to seconds.
- `CssTimeList` -> `TimeList`.
- `CssEasing` keywords -> `EasingFunction` keyword variants.
- `CssEasing::CubicBezier(arguments)` -> `EasingFunction::CubicBezier(EasingArguments)`.
- `CssEasing::Steps(arguments)` -> `EasingFunction::Steps(EasingArguments)`.
- `CssTransitionProperty::All` -> `TransitionPropertyTarget::All`.
- `CssTransitionProperty::None` -> `TransitionPropertyTarget::None`.
- Parsed CSS property identifiers known to style -> `TransitionPropertyTarget::Property(Property)`.
- Parsed CSS transition property identifiers not mapped to a style property -> `TransitionPropertyTarget::Custom(TransitionPropertyName)` or root unsupported-integration diagnostic, depending on root policy.
- `CssTransition` -> `TransitionItem`; omitted components stay `None` in shorthand items and style lowering fills initial defaults.
- `CssTransitionList` -> `TransitionList`.
- `CssAnimationName::None` -> `AnimationName::None`.
- `CssAnimationName::Custom` -> `AnimationName::Keyframes(KeyframesName::Ident(KeyframesIdent))`.
- `CssAnimationName::String` -> `AnimationName::Keyframes(KeyframesName::String(KeyframesString))`.
- Keyframe rule custom identifiers -> `KeyframesName::Ident(KeyframesIdent)`.
- Keyframe rule string names -> `KeyframesName::String(KeyframesString)`.
- `CssAnimation*List` longhands -> matching style-owned list wrappers.
- `CssAnimation` -> `AnimationItem`; omitted components stay `None` in shorthand items and style lowering fills initial defaults.
- `CssAnimationList` -> `AnimationList`.
- `CssKeyframesRule` -> `KeyframesRule`.
- `CssKeyframeBlock` -> `KeyframeBlock` with root-lowered `AuthoredDeclarations`.
- `from` / `to` / percent selectors -> `KeyframeOffset::from()`, `KeyframeOffset::to()`, or `KeyframeOffset::try_new(percent)`.

Root remains responsible for CSS parse errors, CSS source locations, property-name parsing, milliseconds-to-seconds conversion, rejecting parsed identifiers outside style's documented subset if root does not want custom transition names, and any runtime animation scheduler/interpolator integration.

## This Comes Next

The next implementation plan after Operation 12 should cover Operation 13: conditions, layers, and scope integration. It should replace or extend the current basic `Condition::Viewport` / `Condition::Container` shape with style-owned media/container/scope/layer inputs root can lower into. Operation 14 resolver cache and invalidation integration should follow after those models exist, so cache keys are based on real style-owned condition and layer facts rather than guessed placeholders.

## Final Holistic Review Prompt

Use a clean-context reviewer with this prompt:

```text
You are a final holistic reviewer for surgeist-style Operation 12 timing/animation/keyframes. Do not edit files.

Repo: /Users/codex/Development/surgeist-style

Read:
- AGENTS.md
- guidance/surgeist-rust-modeling-guide.md
- plans/2026-07-05-css-surface-style-operations-sequence.md
- plans/2026-07-05-css-property-coverage-ledger.md
- plans/2026-07-06-timing-animation-keyframes-implementation.md
- read-only /Users/codex/Development/surgeist-css/src/syntax.rs
- read-only /Users/codex/Development/surgeist-css/src/parser/timing.rs
- read-only /Users/codex/Development/surgeist-css/src/parser/keyframes.rs

Review the completed implementation against:
- style owns typed timing, transition, animation, and keyframe models;
- style does not import or depend on surgeist-css;
- transition and animation time longhands are list-valued style models, not bare numbers;
- easing payloads remain symbolic and are not evaluated in style;
- transition and animation shorthands lower to canonical typed longhands with omitted components reset to CSS initial defaults;
- keyframes are stored as style-owned sheet data without entering selector rule indexes or resolver scheduling;
- public APIs have front doors and invalid states are hard to construct;
- Operation 12 ledger rows were rebased honestly and Operation 13 remains the next plan;
- implementation follows the Rust modeling guide.

Run:
- cargo fmt --check
- cargo test -p surgeist-style
- cargo clippy -p surgeist-style --all-targets -- -D warnings
- ! rg -n "surgeist_css|surgeist-css|surgeist_text|surgeist-text" Cargo.toml src tests
- git diff --check
- git status --short --branch
- ledger consistency script from the plan
- hash duplicate check for `hash_value`

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
python3 - <<'PY'
from pathlib import Path
import re
text = Path('src/declaration.rs').read_text()
start = text.index('pub(crate) fn hash_value')
end = text.index('\nfn hash_', start + 1)
body = text[start:end]
arms = []
for match in re.finditer(r'Value::([A-Za-z0-9_]+)(?:\([^=]*?\))?\s*=>\s*\{\s*(\d+)u8\.hash\(state\);', body, re.S):
    arms.append((match.group(1), int(match.group(2))))
seen = {}
dups = {}
for name, tag in arms:
    if tag in seen:
        dups.setdefault(tag, [seen[tag]]).append(name)
    seen[tag] = name
print(f'top-level tagged value arms={len(arms)} duplicates={dups}')
raise SystemExit(1 if dups else 0)
PY
```

Expected: all pass, ledger remains consistent, no duplicate top-level `hash_value` tags, and the final git status is clean.
