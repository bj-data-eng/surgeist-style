use super::{Error, ErrorCode, Result, Traversal, Tree};
use crate::{StateFlag, StyleAttributeName, StyleAttributeValue, StyleClass, StyleKey, StyleTag};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PrimaryKey {
    Universal,
    Key(StyleKey),
    Class(StyleClass),
    Tag(StyleTag),
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
    Any,
    Tag(StyleTag),
    Class(StyleClass),
    Key(StyleKey),
    State(StateFlag),
    Attribute(AttributeSelector),
    Position(PositionSelector),
    Compound(Compound),
    Complex(ComplexSelector),
    List(SelectorList),
}

impl Selector {
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    pub fn tag(tag: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Tag(tag_from_str(tag.as_ref())?))
    }

    pub fn class(class: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Class(class_from_str(class.as_ref())?))
    }

    pub fn key(key: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Key(key_from_str(key.as_ref())?))
    }

    #[must_use]
    pub const fn state(state: StateFlag) -> Self {
        Self::State(state)
    }

    pub fn attribute_exists(name: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Attribute(AttributeSelector::exists(name)?))
    }

    pub fn attribute_equals(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Attribute(AttributeSelector::equals(name, value)?))
    }

    #[must_use]
    pub const fn position(position: PositionSelector) -> Self {
        Self::Position(position)
    }

    #[must_use]
    pub fn compound() -> Compound {
        Compound::new()
    }

    pub fn complex(parts: impl IntoIterator<Item = ComplexSelectorPart>) -> Result<Self> {
        Self::try_complex(parts)
    }

    pub fn try_complex(parts: impl IntoIterator<Item = ComplexSelectorPart>) -> Result<Self> {
        Ok(Self::Complex(ComplexSelector::try_new(parts)?))
    }

    #[must_use]
    pub const fn complex_selector(selector: ComplexSelector) -> Self {
        Self::Complex(selector)
    }

    #[must_use]
    pub const fn list(list: SelectorList) -> Self {
        Self::List(list)
    }

    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
        self.matches_with_context(tree, SelectorMatchContext::new(id, traversal))
    }

    pub fn matches_with_context<T: Tree>(
        &self,
        tree: &T,
        context: SelectorMatchContext<T::Id>,
    ) -> Result<bool> {
        let id = context.subject();
        let traversal = context.traversal();
        match self {
            Self::Any => Ok(true),
            Self::Tag(tag) => Ok(tree.node(id)?.tag.as_ref() == Some(tag)),
            Self::Class(class) => Ok(tree.node(id)?.classes.contains(class)),
            Self::Key(key) => Ok(tree.node(id)?.key.as_ref() == Some(key)),
            Self::State(state) => Ok(tree.node(id)?.has_state(*state)),
            Self::Attribute(attribute) => attribute.matches(tree, id),
            Self::Position(position) => position.matches(tree, id, traversal),
            Self::Compound(compound) => compound.matches(tree, id, traversal),
            Self::Complex(complex) => complex.matches(tree, id, traversal),
            Self::List(list) => list.matches(tree, context),
        }
    }

    #[must_use]
    pub fn specificity(&self) -> SelectorSpecificity {
        match self {
            Self::Any => SelectorSpecificity::zero(),
            Self::Tag(_) => SelectorSpecificity::new(0, 0, 1),
            Self::Class(_) | Self::State(_) | Self::Attribute(_) | Self::Position(_) => {
                SelectorSpecificity::new(0, 1, 0)
            }
            Self::Key(_) => SelectorSpecificity::new(1, 0, 0),
            Self::Compound(compound) => compound.specificity(),
            Self::Complex(complex) => complex.specificity(),
            Self::List(list) => list.max_specificity(),
        }
    }

    pub(crate) fn primary_key(&self) -> PrimaryKey {
        match self {
            Self::Tag(tag) => PrimaryKey::Tag(tag.clone()),
            Self::Class(class) => PrimaryKey::Class(class.clone()),
            Self::Key(key) => PrimaryKey::Key(key.clone()),
            Self::Compound(compound) => compound.primary_key(),
            Self::Complex(complex) => complex
                .parts()
                .last()
                .map(|part| part.selector().primary_key())
                .unwrap_or(PrimaryKey::Universal),
            Self::Any | Self::State(_) | Self::Attribute(_) | Self::Position(_) | Self::List(_) => {
                PrimaryKey::Universal
            }
        }
    }
}

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

    #[must_use]
    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }

    pub fn matches<T: Tree>(&self, tree: &T, context: SelectorMatchContext<T::Id>) -> Result<bool> {
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

fn validate_complex_parts(parts: &[ComplexSelectorPart]) -> Result<()> {
    let Some((first, rest)) = parts.split_first() else {
        return Err(Error::new(
            ErrorCode::InvalidSelector,
            "complex selector must contain at least one part",
        ));
    };
    if first.combinator.is_some() {
        return Err(Error::new(
            ErrorCode::InvalidSelector,
            "complex selector must start with a root part",
        ));
    }
    for (index, part) in rest.iter().enumerate() {
        if part.combinator.is_none() {
            return Err(Error::new(
                ErrorCode::InvalidSelector,
                format!("complex selector part {} must be related", index + 1),
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Compound {
    tag: Option<StyleTag>,
    key: Option<StyleKey>,
    classes: Vec<StyleClass>,
    states: Vec<StateFlag>,
    attributes: Vec<AttributeSelector>,
    position: Option<PositionSelector>,
}

impl Compound {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag(mut self, tag: impl AsRef<str>) -> Result<Self> {
        self.tag = Some(tag_from_str(tag.as_ref())?);
        Ok(self)
    }

    pub fn key(mut self, key: impl AsRef<str>) -> Result<Self> {
        self.key = Some(key_from_str(key.as_ref())?);
        Ok(self)
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Result<Self> {
        self.classes.push(class_from_str(class.as_ref())?);
        Ok(self)
    }

    #[must_use]
    pub fn state(mut self, state: StateFlag) -> Self {
        self.states.push(state);
        self
    }

    pub fn attribute_exists(mut self, name: impl AsRef<str>) -> Result<Self> {
        self.attributes.push(AttributeSelector::exists(name)?);
        Ok(self)
    }

    pub fn attribute_equals(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<Self> {
        self.attributes
            .push(AttributeSelector::equals(name, value)?);
        Ok(self)
    }

    #[must_use]
    pub const fn position(mut self, position: PositionSelector) -> Self {
        self.position = Some(position);
        self
    }

    #[must_use]
    pub fn selector(self) -> Selector {
        Selector::Compound(self)
    }

    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
        let node = tree.node(id)?;
        if self
            .tag
            .as_ref()
            .is_some_and(|tag| node.tag.as_ref() != Some(tag))
        {
            return Ok(false);
        }
        if self
            .key
            .as_ref()
            .is_some_and(|key| node.key.as_ref() != Some(key))
        {
            return Ok(false);
        }
        if !self
            .classes
            .iter()
            .all(|class| node.classes.contains(class))
        {
            return Ok(false);
        }
        if !self.states.iter().all(|state| node.has_state(*state)) {
            return Ok(false);
        }
        for attribute in &self.attributes {
            if !attribute.matches(tree, id)? {
                return Ok(false);
            }
        }
        if let Some(position) = self.position
            && !position.matches(tree, id, traversal)?
        {
            return Ok(false);
        }
        Ok(true)
    }

    pub(crate) fn primary_key(&self) -> PrimaryKey {
        if let Some(key) = &self.key {
            PrimaryKey::Key(key.clone())
        } else if let Some(class) = self.classes.last() {
            PrimaryKey::Class(class.clone())
        } else if let Some(tag) = &self.tag {
            PrimaryKey::Tag(tag.clone())
        } else {
            PrimaryKey::Universal
        }
    }

    #[must_use]
    pub fn specificity(&self) -> SelectorSpecificity {
        let mut specificity = SelectorSpecificity::zero();
        if self.key.is_some() {
            specificity = specificity.saturating_add(SelectorSpecificity::new(1, 0, 0));
        }
        if self.tag.is_some() {
            specificity = specificity.saturating_add(SelectorSpecificity::new(0, 0, 1));
        }
        for _ in &self.classes {
            specificity = specificity.saturating_add(SelectorSpecificity::new(0, 1, 0));
        }
        for _ in &self.states {
            specificity = specificity.saturating_add(SelectorSpecificity::new(0, 1, 0));
        }
        for _ in &self.attributes {
            specificity = specificity.saturating_add(SelectorSpecificity::new(0, 1, 0));
        }
        if self.position.is_some() {
            specificity = specificity.saturating_add(SelectorSpecificity::new(0, 1, 0));
        }
        specificity
    }
}

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

    #[must_use]
    pub fn parts(&self) -> &[ComplexSelectorPart] {
        &self.parts
    }

    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
        complex_matches(&self.parts, tree, id, traversal)
    }

    #[must_use]
    pub fn specificity(&self) -> SelectorSpecificity {
        self.parts
            .iter()
            .map(|part| part.selector.specificity())
            .fold(
                SelectorSpecificity::zero(),
                SelectorSpecificity::saturating_add,
            )
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Combinator {
    Descendant,
    Child,
    Adjacent,
    Sibling,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeSelector {
    Exists(StyleAttributeName),
    Equals(StyleAttributeName, StyleAttributeValue),
}

impl AttributeSelector {
    pub fn exists(name: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Exists(attribute_name_from_str(name.as_ref())?))
    }

    pub fn equals(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        Ok(Self::Equals(
            attribute_name_from_str(name.as_ref())?,
            attribute_value_from_str(value.as_ref())?,
        ))
    }

    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id) -> Result<bool> {
        let node = tree.node(id)?;
        Ok(match self {
            Self::Exists(name) => node
                .attributes
                .iter()
                .any(|attribute| attribute.name() == name),
            Self::Equals(name, value) => node
                .attributes
                .iter()
                .any(|attribute| attribute.name() == name && attribute.value() == value),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionSelector {
    First,
    Last,
    Nth(Nth),
}

impl PositionSelector {
    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
        let Some(parent) = tree.parent(id, traversal)? else {
            return Ok(false);
        };
        let children: Vec<_> = tree.children(parent, traversal)?.collect();
        let Some(index) = children.iter().position(|child| *child == id) else {
            return Ok(false);
        };
        let position = Position::new(index, children.len());
        Ok(match self {
            Self::First => position.is_first(),
            Self::Last => position.is_last(),
            Self::Nth(nth) => position.matches(*nth),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub index: usize,
    pub sibling_count: usize,
}

impl Position {
    #[must_use]
    pub const fn new(index: usize, sibling_count: usize) -> Self {
        Self {
            index,
            sibling_count,
        }
    }

    #[must_use]
    pub const fn is_first(self) -> bool {
        self.index == 0
    }

    #[must_use]
    pub const fn is_last(self) -> bool {
        self.index + 1 == self.sibling_count
    }

    #[must_use]
    pub fn matches(self, nth: Nth) -> bool {
        nth.matches(self.index + 1)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Nth {
    pub step: usize,
    pub offset: usize,
}

impl Nth {
    #[must_use]
    pub const fn new(step: usize, offset: usize) -> Self {
        Self { step, offset }
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
    pub fn matches(self, position: usize) -> bool {
        if position == 0 {
            return false;
        }
        if self.step == 0 {
            return self.offset > 0 && position == self.offset;
        }
        if self.offset == 0 {
            return position.is_multiple_of(self.step);
        }
        position >= self.offset && (position - self.offset).is_multiple_of(self.step)
    }
}

fn complex_matches<T: Tree>(
    parts: &[ComplexSelectorPart],
    tree: &T,
    id: T::Id,
    traversal: Traversal,
) -> Result<bool> {
    let Some(last) = parts.last() else {
        return Ok(false);
    };
    if !last.selector.matches(tree, id, traversal)? {
        return Ok(false);
    }

    complex_prefix_matches(parts, parts.len() - 1, tree, id, traversal)
}

fn complex_prefix_matches<T: Tree>(
    parts: &[ComplexSelectorPart],
    index: usize,
    tree: &T,
    current: T::Id,
    traversal: Traversal,
) -> Result<bool> {
    if index == 0 {
        return Ok(true);
    }
    let combinator = parts[index].combinator.ok_or_else(|| {
        Error::new(
            ErrorCode::InvalidSelector,
            "complex selector part is missing a combinator",
        )
    })?;
    for candidate in related_candidates(
        combinator,
        &parts[index - 1].selector,
        tree,
        current,
        traversal,
    )? {
        if complex_prefix_matches(parts, index - 1, tree, candidate, traversal)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn related_candidates<T: Tree>(
    combinator: Combinator,
    selector: &Compound,
    tree: &T,
    id: T::Id,
    traversal: Traversal,
) -> Result<Vec<T::Id>> {
    match combinator {
        Combinator::Child => {
            let Some(parent) = tree.parent(id, traversal)? else {
                return Ok(Vec::new());
            };
            if selector.matches(tree, parent, traversal)? {
                Ok(vec![parent])
            } else {
                Ok(Vec::new())
            }
        }
        Combinator::Descendant => {
            let mut parent = tree.parent(id, traversal)?;
            let mut candidates = Vec::new();
            while let Some(candidate) = parent {
                if selector.matches(tree, candidate, traversal)? {
                    candidates.push(candidate);
                }
                parent = tree.parent(candidate, traversal)?;
            }
            Ok(candidates)
        }
        Combinator::Adjacent => {
            let Some(previous) = tree.previous_sibling(id, traversal)? else {
                return Ok(Vec::new());
            };
            if selector.matches(tree, previous, traversal)? {
                Ok(vec![previous])
            } else {
                Ok(Vec::new())
            }
        }
        Combinator::Sibling => {
            let Some(parent) = tree.parent(id, traversal)? else {
                return Ok(Vec::new());
            };
            let siblings: Vec<_> = tree.children(parent, traversal)?.collect();
            let Some(index) = siblings.iter().position(|sibling| *sibling == id) else {
                return Ok(Vec::new());
            };
            let mut candidates = Vec::new();
            for candidate in siblings[..index].iter().rev().copied() {
                if selector.matches(tree, candidate, traversal)? {
                    candidates.push(candidate);
                }
            }
            Ok(candidates)
        }
    }
}

fn tag_from_str(value: &str) -> Result<StyleTag> {
    StyleTag::new(value).map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn class_from_str(value: &str) -> Result<StyleClass> {
    StyleClass::new(value)
        .map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn key_from_str(value: &str) -> Result<StyleKey> {
    StyleKey::new(value).map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn attribute_name_from_str(value: &str) -> Result<StyleAttributeName> {
    StyleAttributeName::new(value)
        .map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn attribute_value_from_str(value: &str) -> Result<StyleAttributeValue> {
    StyleAttributeValue::new(value)
        .map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ErrorCode, Node, Result, Sheet, StyleAttribute, StyleAttributeName, StyleAttributeValue,
        StyleClass, StyleKey, StyleRole, StyleState, StyleTag, Traversal, Tree,
    };

    #[test]
    fn selector_matches_style_owned_tree_facts() {
        let tree = TestTree::new(vec![
            TestNode::new(0)
                .tag("window")
                .children([1])
                .state(StyleState::default().with_focus_within(true)),
            TestNode::new(1)
                .tag("button")
                .key("primary")
                .class("accent")
                .attribute("data-mode", "submit")
                .state(StyleState::default().with_hovered(true)),
        ]);

        let selector = Selector::compound()
            .tag("button")
            .unwrap()
            .key("primary")
            .unwrap()
            .class("accent")
            .unwrap()
            .attribute_equals("data-mode", "submit")
            .unwrap()
            .state(crate::StateFlag::Hovered)
            .selector();

        assert!(
            selector
                .matches(&tree, 1, Traversal::Canonical)
                .expect("selector should evaluate")
        );
        assert_eq!(
            Selector::tag("bad name").unwrap_err().code(),
            ErrorCode::InvalidSelector
        );
    }

    #[test]
    fn sheet_candidates_use_style_owned_index_keys() {
        let tree = TestTree::new(vec![
            TestNode::new(0)
                .tag("button")
                .key("primary")
                .class("accent"),
        ]);
        let sheet = Sheet::new()
            .rule(Selector::tag("button").unwrap(), crate::Declarations::new())
            .rule(
                Selector::class("accent").unwrap(),
                crate::Declarations::new(),
            )
            .rule(
                Selector::key("primary").unwrap(),
                crate::Declarations::new(),
            );

        assert_eq!(sheet.candidate_rule_count(&tree, 0).unwrap(), 3);
    }

    #[test]
    fn selector_list_matches_any_selector_and_rejects_empty_lists() {
        let tree = TestTree::new(vec![TestNode::new(0).tag("button").class("primary")]);
        let list = SelectorList::try_new([
            Selector::tag("label").unwrap(),
            Selector::class("primary").unwrap(),
        ])
        .unwrap();

        assert!(
            list.matches(&tree, SelectorMatchContext::for_subject(0))
                .unwrap()
        );
        assert_eq!(
            SelectorList::try_new([]).unwrap_err().code(),
            ErrorCode::InvalidSelector
        );
    }

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
            .tag("button")
            .unwrap()
            .key("submit")
            .unwrap()
            .class("primary")
            .unwrap()
            .attribute_exists("data-mode")
            .unwrap()
            .selector();
        let complex = Selector::complex([
            ComplexSelectorPart::root(Selector::compound().tag("form").unwrap()),
            ComplexSelectorPart::related(
                Combinator::Descendant,
                Selector::compound().class("primary").unwrap(),
            ),
        ])
        .unwrap();
        let list = Selector::list(
            SelectorList::try_new([
                Selector::tag("button").unwrap(),
                Selector::key("submit").unwrap(),
            ])
            .unwrap(),
        );

        assert_eq!(compound.specificity(), SelectorSpecificity::new(1, 2, 1));
        assert_eq!(complex.specificity(), SelectorSpecificity::new(0, 1, 1));
        assert_eq!(list.specificity(), SelectorSpecificity::new(1, 0, 0));
    }

    #[test]
    fn complex_selector_rejects_invalid_part_ordering() {
        assert_eq!(
            ComplexSelector::try_new([]).unwrap_err().code(),
            ErrorCode::InvalidSelector
        );
        assert_eq!(
            ComplexSelector::try_new([ComplexSelectorPart::related(
                Combinator::Child,
                Selector::compound().tag("button").unwrap(),
            )])
            .unwrap_err()
            .code(),
            ErrorCode::InvalidSelector
        );
        assert_eq!(
            ComplexSelector::try_new([
                ComplexSelectorPart::root(Selector::compound().tag("form").unwrap()),
                ComplexSelectorPart::root(Selector::compound().tag("button").unwrap()),
            ])
            .unwrap_err()
            .code(),
            ErrorCode::InvalidSelector
        );
    }

    #[derive(Clone, Debug)]
    struct TestNode {
        id: usize,
        tag: Option<StyleTag>,
        key: Option<StyleKey>,
        classes: Vec<StyleClass>,
        attributes: Vec<StyleAttribute>,
        role: StyleRole,
        state: StyleState,
        children: Vec<usize>,
    }

    impl TestNode {
        fn new(id: usize) -> Self {
            Self {
                id,
                tag: None,
                key: None,
                classes: Vec::new(),
                attributes: Vec::new(),
                role: StyleRole::default(),
                state: StyleState::default(),
                children: Vec::new(),
            }
        }

        fn tag(mut self, tag: &str) -> Self {
            self.tag = Some(StyleTag::new(tag).unwrap());
            self
        }

        fn key(mut self, key: &str) -> Self {
            self.key = Some(StyleKey::new(key).unwrap());
            self
        }

        fn class(mut self, class: &str) -> Self {
            self.classes.push(StyleClass::new(class).unwrap());
            self
        }

        fn attribute(mut self, name: &str, value: &str) -> Self {
            self.attributes.push(StyleAttribute::new(
                StyleAttributeName::new(name).unwrap(),
                StyleAttributeValue::new(value).unwrap(),
            ));
            self
        }

        fn state(mut self, state: StyleState) -> Self {
            self.state = state;
            self
        }

        fn children(mut self, children: impl IntoIterator<Item = usize>) -> Self {
            self.children = children.into_iter().collect();
            self
        }
    }

    struct TestTree {
        nodes: Vec<TestNode>,
    }

    impl TestTree {
        fn new(nodes: Vec<TestNode>) -> Self {
            Self { nodes }
        }
    }

    impl Tree for TestTree {
        type Id = usize;

        fn version_hint(&self) -> Option<u64> {
            Some(1)
        }

        fn node(&self, id: Self::Id) -> Result<Node<Self::Id>> {
            let node = self.nodes.get(id).ok_or_else(|| {
                crate::Error::new(crate::ErrorCode::MissingNode, "missing test node")
            })?;
            Ok(Node {
                id: node.id,
                tag: node.tag.clone(),
                key: node.key.clone(),
                classes: node.classes.clone(),
                attributes: node.attributes.clone(),
                role: node.role.clone(),
                state: node.state.clone(),
                text: false,
            })
        }

        fn parent(&self, id: Self::Id, _traversal: Traversal) -> Result<Option<Self::Id>> {
            Ok(self
                .nodes
                .iter()
                .find(|node| node.children.contains(&id))
                .map(|node| node.id))
        }

        fn children(
            &self,
            id: Self::Id,
            _traversal: Traversal,
        ) -> Result<impl Iterator<Item = Self::Id> + '_> {
            Ok(self.nodes[id].children.iter().copied())
        }

        fn previous_sibling(&self, id: Self::Id, traversal: Traversal) -> Result<Option<Self::Id>> {
            let Some(parent) = self.parent(id, traversal)? else {
                return Ok(None);
            };
            let siblings = &self.nodes[parent].children;
            Ok(siblings
                .iter()
                .position(|sibling| *sibling == id)
                .and_then(|index| index.checked_sub(1))
                .map(|index| siblings[index]))
        }
    }
}
