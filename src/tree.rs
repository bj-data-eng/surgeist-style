use super::Result;
use std::hash::Hash;
use surgeist_retained::{Attribute, Class, Key, Role, State, StateFlag, Tag};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Traversal {
    Canonical,
    Projected,
}

pub trait Tree {
    type Id: Copy + Eq + Hash;

    fn version_hint(&self) -> Option<u64>;
    fn node(&self, id: Self::Id) -> Result<Node<'_, Self::Id>>;
    fn parent(&self, id: Self::Id, traversal: Traversal) -> Result<Option<Self::Id>>;
    fn children(
        &self,
        id: Self::Id,
        traversal: Traversal,
    ) -> Result<impl Iterator<Item = Self::Id> + '_>;
    fn previous_sibling(&self, id: Self::Id, traversal: Traversal) -> Result<Option<Self::Id>>;
}

#[derive(Clone, Debug)]
pub struct Node<'a, Id> {
    pub id: Id,
    pub tag: Option<&'a Tag>,
    pub key: Option<&'a Key>,
    pub classes: &'a [Class],
    pub attributes: &'a [Attribute],
    pub role: Role,
    pub state: &'a State,
    pub text: bool,
}

impl<'a, Id> Node<'a, Id> {
    #[must_use]
    pub fn has_state(&self, flag: StateFlag) -> bool {
        match flag {
            StateFlag::Hovered => self.state.hovered(),
            StateFlag::Active => self.state.active(),
            StateFlag::Focused => self.state.focused(),
            StateFlag::FocusWithin => self.state.focus_within(),
            StateFlag::PointerCaptured => self.state.pointer_captured(),
            StateFlag::Disabled => self.state.disabled(),
            StateFlag::Selected => self.state.selected(),
            StateFlag::Pressed => self.state.pressed(),
            StateFlag::Checked => self.state.checked() == Some(true),
            StateFlag::Expanded => self.state.expanded() == Some(true),
            _ => false,
        }
    }
}
