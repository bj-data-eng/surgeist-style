//! Adapter from retained document snapshots and changes into style facts.

use crate::{Change, Error, ErrorCode, Node, Resolver, Result, Traversal, Tree};
use surgeist_retained::{self as retained, Id, Kind, ProjectionSlot, Snapshot, Tag};

impl<'a> Tree for Snapshot<'a> {
    type Id = Id;

    fn version_hint(&self) -> Option<u64> {
        None
    }

    fn node(&self, id: Self::Id) -> Result<Node<'_, Self::Id>> {
        let node = self
            .get(id)
            .ok_or_else(|| Error::new(ErrorCode::MissingNode, format!("missing node {id:?}")))?;
        Ok(Node {
            id,
            tag: tag_for_kind(node.kind()),
            key: node.key(),
            classes: node.classes(),
            attributes: node.attributes(),
            role: node.role(),
            state: node.state(),
            text: matches!(node.kind(), Kind::Text),
        })
    }

    fn parent(&self, id: Self::Id, traversal: Traversal) -> Result<Option<Self::Id>> {
        let node = self
            .get(id)
            .ok_or_else(|| Error::new(ErrorCode::MissingNode, format!("missing node {id:?}")))?;
        Ok(match traversal {
            Traversal::Canonical => node.parent(),
            Traversal::Projected => node.projected_parent().or_else(|| node.parent()),
        })
    }

    fn children(
        &self,
        id: Self::Id,
        traversal: Traversal,
    ) -> Result<impl Iterator<Item = Self::Id> + '_> {
        let children: Vec<_> = match traversal {
            Traversal::Canonical => self
                .children(id)
                .map_err(map_retained_error)?
                .collect::<Vec<_>>(),
            Traversal::Projected => self
                .projected_children(ProjectionSlot::default(id))
                .map_err(map_retained_error)?
                .collect::<Vec<_>>(),
        };
        Ok(children.into_iter())
    }

    fn previous_sibling(&self, id: Self::Id, traversal: Traversal) -> Result<Option<Self::Id>> {
        let Some(parent) = self.parent(id, traversal)? else {
            return Ok(None);
        };
        let siblings: Vec<_> = <Self as Tree>::children(self, parent, traversal)?.collect();
        Ok(siblings
            .iter()
            .position(|sibling| *sibling == id)
            .and_then(|index| index.checked_sub(1))
            .map(|index| siblings[index]))
    }
}

impl Change {
    #[must_use]
    pub fn from_retained_flags(flags: retained::ChangeFlags) -> Self {
        let mut change = Self::empty();
        if !flags.is_empty() {
            change.scope.include_node();
        }
        if flags.has_structure()
            || flags.has_kind()
            || flags.has_classes()
            || flags.has_attributes()
            || flags.has_state()
            || flags.has_focus()
            || flags.has_projection()
        {
            change.rematch = true;
        }
        if flags.has_structure() || flags.has_projection() || flags.has_presence() {
            change.scope.include_siblings();
            change.scope.include_descendants();
            change.invalidation.layout = true;
            change.invalidation.paint = true;
        }
        if flags.has_kind()
            || flags.has_classes()
            || flags.has_attributes()
            || flags.has_state()
            || flags.has_focus()
        {
            change.scope.include_descendants();
        }
        if flags.has_text() || flags.has_label() {
            change.invalidation.layout = true;
            change.invalidation.text = true;
            change.invalidation.paint = true;
        }
        if flags.has_state() || flags.has_focus() || flags.has_pointer_capture() {
            change.invalidation.paint = true;
        }
        change
    }
}

impl Resolver {
    pub fn clear_cache_for_changes(&mut self, changes: &retained::ChangeSet) {
        if !changes.inserted().is_empty()
            || !changes.removed().is_empty()
            || !changes.moved().is_empty()
            || !changes.changed_projection_slots().is_empty()
        {
            self.clear_cache();
            return;
        }
        let mut local_nodes = Vec::new();
        for (id, flags) in changes.changed() {
            let change = Change::from_retained_flags(flags);
            if change.scope.siblings || change.scope.descendants {
                self.clear_cache();
                return;
            }
            local_nodes.push(id);
        }
        for id in local_nodes {
            self.clear_cache_for_node(id);
        }
    }
}

fn tag_for_kind(kind: &Kind) -> Option<&Tag> {
    match kind {
        Kind::Element(tag) | Kind::Slot(tag) | Kind::Widget(tag) => Some(tag),
        Kind::Root | Kind::Text | Kind::Canvas | Kind::Fragment => None,
        _ => None,
    }
}

fn map_retained_error(error: retained::Error) -> Error {
    Error::new(ErrorCode::Traversal, error.to_string())
}
