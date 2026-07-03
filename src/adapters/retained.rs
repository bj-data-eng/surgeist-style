//! Adapter from retained document snapshots and changes into style facts.

use crate::{
    Change, Error, ErrorCode, Node, Resolver, Result, StyleAttribute, StyleAttributeName,
    StyleAttributeValue, StyleClass, StyleKey, StyleRole, StyleState, StyleTag, Traversal, Tree,
};
use surgeist_retained::{self as retained, Id, Kind, ProjectionSlot, Snapshot};

impl<'a> Tree for Snapshot<'a> {
    type Id = Id;

    fn version_hint(&self) -> Option<u64> {
        Some(self.revision().get())
    }

    fn node(&self, id: Self::Id) -> Result<Node<Self::Id>> {
        let node = self
            .get(id)
            .ok_or_else(|| Error::new(ErrorCode::MissingNode, format!("missing node {id:?}")))?;
        Ok(Node {
            id,
            tag: tag_for_kind(node.kind()),
            key: node.key().map(style_key_from_retained).transpose()?,
            classes: node
                .classes()
                .iter()
                .map(style_class_from_retained)
                .collect::<Result<Vec<_>>>()?,
            attributes: node
                .attributes()
                .iter()
                .map(style_attribute_from_retained)
                .collect::<Result<Vec<_>>>()?,
            role: style_role_from_retained(node.role())?,
            state: style_state_from_retained(node.state()),
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

fn tag_for_kind(kind: &Kind) -> Option<StyleTag> {
    match kind {
        Kind::Element(tag) | Kind::Slot(tag) | Kind::Widget(tag) => {
            Some(style_tag_from_retained(tag).expect("retained tag should be style-valid"))
        }
        Kind::Root | Kind::Text | Kind::Canvas | Kind::Fragment => None,
        _ => None,
    }
}

fn style_tag_from_retained(tag: &retained::Tag) -> Result<StyleTag> {
    StyleTag::new(tag.as_str()).map_err(map_style_identity_error)
}

fn style_key_from_retained(key: &retained::Key) -> Result<StyleKey> {
    StyleKey::new(key.as_str()).map_err(map_style_identity_error)
}

fn style_class_from_retained(class: &retained::Class) -> Result<StyleClass> {
    StyleClass::new(class.as_str()).map_err(map_style_identity_error)
}

fn style_attribute_from_retained(attribute: &retained::Attribute) -> Result<StyleAttribute> {
    Ok(StyleAttribute::new(
        StyleAttributeName::new(attribute.name.as_str()).map_err(map_style_identity_error)?,
        StyleAttributeValue::new(attribute.value.as_str()).map_err(map_style_identity_error)?,
    ))
}

fn style_role_from_retained(role: retained::Role) -> Result<StyleRole> {
    Ok(match role {
        retained::Role::Generic => StyleRole::Generic,
        retained::Role::Application => StyleRole::Application,
        retained::Role::Button => StyleRole::Button,
        retained::Role::Text => StyleRole::Text,
        retained::Role::List => StyleRole::List,
        retained::Role::ListItem => StyleRole::ListItem,
        retained::Role::Checkbox => StyleRole::Checkbox,
        retained::Role::Textbox => StyleRole::Textbox,
        retained::Role::Image => StyleRole::Image,
        retained::Role::Canvas => StyleRole::Canvas,
        retained::Role::Widget => StyleRole::Widget,
        retained::Role::Custom(tag) => StyleRole::Custom(style_tag_from_retained(&tag)?),
        _ => StyleRole::Generic,
    })
}

fn style_state_from_retained(state: &retained::State) -> StyleState {
    StyleState::default()
        .with_disabled(state.disabled())
        .with_hovered(state.hovered())
        .with_active(state.active())
        .with_focused(state.focused())
        .with_focus_within(state.focus_within())
        .with_pointer_captured(state.pointer_captured())
        .with_selected(state.selected())
        .with_pressed(state.pressed())
        .with_checked(state.checked())
        .with_expanded(state.expanded())
}

fn map_style_identity_error(error: Error) -> Error {
    Error::new(ErrorCode::InvalidString, error.message().to_owned())
}

fn map_retained_error(error: retained::Error) -> Error {
    Error::new(ErrorCode::Traversal, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Context, Declarations, Sheet};
    use surgeist_retained::{Class, Element, Model, Patch, Tag, Text};

    #[test]
    fn clear_cache_for_changes_distinguishes_local_and_broad_style_scopes() {
        let mut model = Model::empty();
        let root = model.root();
        let id_one = model
            .apply(Patch::Insert {
                parent: root,
                index: 0,
                element: element("one"),
            })
            .unwrap()
            .changes()
            .inserted()[0];
        let id_two = model
            .apply(Patch::Insert {
                parent: root,
                index: 1,
                element: element("two"),
            })
            .unwrap()
            .changes()
            .inserted()[0];
        let local_one = Declarations::new().try_text_color(Color::BLACK).unwrap();
        let local_two = Declarations::new().try_bg(Color::BLACK).unwrap();
        let mut resolver = Resolver::new(Sheet::new());

        let tree = model.snapshot();
        resolver
            .resolve(Context::new(&tree, id_one).local(&local_one))
            .unwrap();
        resolver
            .resolve(Context::new(&tree, id_two).local(&local_two))
            .unwrap();
        resolver
            .resolve(Context::new(&tree, id_one).local(&local_one))
            .unwrap();
        resolver
            .resolve(Context::new(&tree, id_two).local(&local_two))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 2);

        let local_change = model
            .apply(Patch::SetLabel {
                id: id_one,
                label: Some(Text::new("updated label").unwrap()),
            })
            .unwrap();
        resolver.clear_cache_for_changes(local_change.changes());
        let tree = model.snapshot();
        resolver
            .resolve(Context::new(&tree, id_two).local(&local_two))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 2);
        resolver
            .resolve(Context::new(&tree, id_one).local(&local_one))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 2);

        resolver
            .resolve(Context::new(&tree, id_one).local(&local_one))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 3);
        let broad_change = model
            .apply(Patch::SetClasses {
                id: id_one,
                classes: vec![Class::new("featured").unwrap()],
            })
            .unwrap();
        resolver.clear_cache_for_changes(broad_change.changes());
        assert_eq!(resolver.cache_hits(), 0);
        let tree = model.snapshot();
        resolver
            .resolve(Context::new(&tree, id_two).local(&local_two))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 0);
        resolver
            .resolve(Context::new(&tree, id_two).local(&local_two))
            .unwrap();
        assert_eq!(resolver.cache_hits(), 1);
    }

    fn element(name: &str) -> Element {
        Element::tagged(Tag::new(name).unwrap())
    }
}
