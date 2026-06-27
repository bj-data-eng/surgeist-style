use super::{Error, ErrorCode, Result, Traversal, Tree};
use crate::StateFlag;
use surgeist_retained::{AttributeName, Class, Key, Tag, Value as AttributeValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PrimaryKey {
    Universal,
    Key(Key),
    Class(Class),
    Tag(Tag),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
    Any,
    Tag(Tag),
    Class(Class),
    Key(Key),
    State(StateFlag),
    Attribute(AttributeSelector),
    Position(PositionSelector),
    Compound(Compound),
    Complex(Vec<Part>),
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

    pub fn complex(parts: impl IntoIterator<Item = Part>) -> Result<Self> {
        Self::try_complex(parts)
    }

    pub fn try_complex(parts: impl IntoIterator<Item = Part>) -> Result<Self> {
        let parts: Vec<_> = parts.into_iter().collect();
        validate_complex_parts(&parts)?;
        Ok(Self::Complex(parts))
    }

    pub fn matches<T: Tree>(&self, tree: &T, id: T::Id, traversal: Traversal) -> Result<bool> {
        match self {
            Self::Any => Ok(true),
            Self::Tag(tag) => Ok(tree.node(id)?.tag == Some(tag)),
            Self::Class(class) => Ok(tree.node(id)?.classes.contains(class)),
            Self::Key(key) => Ok(tree.node(id)?.key == Some(key)),
            Self::State(state) => Ok(tree.node(id)?.has_state(*state)),
            Self::Attribute(attribute) => attribute.matches(tree, id),
            Self::Position(position) => position.matches(tree, id, traversal),
            Self::Compound(compound) => compound.matches(tree, id, traversal),
            Self::Complex(parts) => {
                validate_complex_parts(parts)?;
                complex_matches(parts, tree, id, traversal)
            }
        }
    }

    pub(crate) fn primary_key(&self) -> PrimaryKey {
        match self {
            Self::Tag(tag) => PrimaryKey::Tag(tag.clone()),
            Self::Class(class) => PrimaryKey::Class(class.clone()),
            Self::Key(key) => PrimaryKey::Key(key.clone()),
            Self::Compound(compound) => compound.primary_key(),
            Self::Complex(parts) => parts
                .last()
                .map(|part| part.selector.primary_key())
                .unwrap_or(PrimaryKey::Universal),
            Self::Any | Self::State(_) | Self::Attribute(_) | Self::Position(_) => {
                PrimaryKey::Universal
            }
        }
    }
}

fn validate_complex_parts(parts: &[Part]) -> Result<()> {
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
    tag: Option<Tag>,
    key: Option<Key>,
    classes: Vec<Class>,
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
        if self.tag.as_ref().is_some_and(|tag| node.tag != Some(tag)) {
            return Ok(false);
        }
        if self.key.as_ref().is_some_and(|key| node.key != Some(key)) {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part {
    pub combinator: Option<Combinator>,
    pub selector: Compound,
}

impl Part {
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
    Exists(AttributeName),
    Equals(AttributeName, AttributeValue),
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
                .any(|attribute| attribute.name == *name),
            Self::Equals(name, value) => node
                .attributes
                .iter()
                .any(|attribute| attribute.name == *name && attribute.value == *value),
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
    parts: &[Part],
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
    parts: &[Part],
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

fn tag_from_str(value: &str) -> Result<Tag> {
    Tag::new(value).map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn class_from_str(value: &str) -> Result<Class> {
    Class::new(value).map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn key_from_str(value: &str) -> Result<Key> {
    Key::new(value).map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn attribute_name_from_str(value: &str) -> Result<AttributeName> {
    AttributeName::new(value)
        .map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}

fn attribute_value_from_str(value: &str) -> Result<AttributeValue> {
    AttributeValue::new(value)
        .map_err(|error| Error::new(ErrorCode::InvalidSelector, error.to_string()))
}
