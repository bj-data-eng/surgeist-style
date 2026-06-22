use std::{
    collections::{BTreeMap, BTreeSet, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

use super::{
    Condition, Container, Corners, Cursor, Declarations, Display, Edges, Length, PointerEvents,
    Property, Result, Sheet, Size, Transform, Traversal, Tree, Value, Version, Viewport,
    Visibility, declaration::hash_value,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Resolved {
    values: BTreeMap<Property, Value>,
}

impl Resolved {
    #[must_use]
    pub fn new() -> Self {
        let mut values = BTreeMap::new();
        for property in Property::ALL {
            if property.is_canonical() {
                values.insert(*property, property.metadata().default);
            }
        }
        Self { values }
    }

    #[must_use]
    pub fn get(&self, property: Property) -> &Value {
        self.values
            .get(&property)
            .expect("resolved style stores every canonical property")
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Property, &Value)> {
        self.values.iter()
    }

    #[must_use]
    pub fn background(&self) -> super::Color {
        match self.get(Property::Background) {
            Value::Color(color) => *color,
            _ => super::Color::TRANSPARENT,
        }
    }

    #[must_use]
    pub fn text_color(&self) -> super::Color {
        match self.get(Property::Color) {
            Value::Color(color) => *color,
            _ => super::Color::BLACK,
        }
    }

    #[must_use]
    pub fn width(&self) -> Length {
        match self.get(Property::Width) {
            Value::Length(value) => *value,
            _ => Length::Auto,
        }
    }

    #[must_use]
    pub fn height(&self) -> Length {
        match self.get(Property::Height) {
            Value::Length(value) => *value,
            _ => Length::Auto,
        }
    }

    #[must_use]
    pub fn padding_edges(&self) -> Edges {
        match self.get(Property::Padding) {
            Value::Edges(edges) => *edges,
            _ => Edges::default(),
        }
    }

    #[must_use]
    pub fn margin_edges(&self) -> Edges {
        match self.get(Property::Margin) {
            Value::Edges(edges) => *edges,
            _ => Edges::default(),
        }
    }

    #[must_use]
    pub fn radius_corners(&self) -> Corners {
        match self.get(Property::Radius) {
            Value::Corners(corners) => *corners,
            _ => Corners::default(),
        }
    }

    #[must_use]
    pub fn opacity(&self) -> f32 {
        match self.get(Property::Opacity) {
            Value::Number(value) => *value,
            _ => 1.0,
        }
    }

    #[must_use]
    pub fn font_size(&self) -> Length {
        match self.get(Property::FontSize) {
            Value::Length(value) => *value,
            _ => Length::Px(16.0),
        }
    }

    #[must_use]
    pub fn cursor(&self) -> Cursor {
        match self.get(Property::Cursor) {
            Value::Cursor(cursor) => *cursor,
            _ => Cursor::Default,
        }
    }

    #[must_use]
    pub fn pointer_events(&self) -> PointerEvents {
        match self.get(Property::PointerEvents) {
            Value::PointerEvents(pointer_events) => *pointer_events,
            _ => PointerEvents::Auto,
        }
    }

    #[must_use]
    pub fn border_width_edges(&self) -> Edges {
        match self.get(Property::BorderWidth) {
            Value::Edges(edges) => *edges,
            _ => Edges::default(),
        }
    }

    #[must_use]
    pub fn border_color(&self) -> super::Color {
        match self.get(Property::BorderColor) {
            Value::Color(color) => *color,
            _ => super::Color::TRANSPARENT,
        }
    }

    #[must_use]
    pub fn visibility(&self) -> Visibility {
        match self.get(Property::Visibility) {
            Value::Visibility(visibility) => *visibility,
            _ => Visibility::Visible,
        }
    }

    #[must_use]
    pub fn transform(&self) -> &Transform {
        match self.get(Property::Transform) {
            Value::Transform(transform) => transform,
            _ => unreachable!("resolved transform stores a transform value"),
        }
    }

    #[must_use]
    pub fn transform_origin(&self) -> Size {
        match self.get(Property::TransformOrigin) {
            Value::Size(origin) => *origin,
            _ => Size::new(Length::Percent(50.0), Length::Percent(50.0)),
        }
    }

    #[must_use]
    pub fn transition_properties(&self) -> &[Property] {
        match self.get(Property::TransitionProperty) {
            Value::PropertyList(properties) => properties,
            _ => &[],
        }
    }

    #[must_use]
    pub fn transition_duration(&self) -> f32 {
        match self.get(Property::TransitionDuration) {
            Value::Number(duration) => *duration,
            _ => 0.0,
        }
    }

    #[must_use]
    pub fn transition_delay(&self) -> f32 {
        match self.get(Property::TransitionDelay) {
            Value::Number(delay) => *delay,
            _ => 0.0,
        }
    }

    #[must_use]
    pub fn display(&self) -> Display {
        match self.get(Property::Display) {
            Value::Display(display) => *display,
            _ => Display::default(),
        }
    }

    fn inherit_from(&mut self, parent: &Self) {
        for property in Property::ALL {
            if property.is_canonical() && property.metadata().inherited {
                self.values.insert(*property, parent.get(*property).clone());
            }
        }
    }

    fn fingerprint(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for (property, value) in &self.values {
            property.hash(&mut hasher);
            hash_value(value, &mut hasher);
        }
        hasher.finish()
    }

    fn apply(&mut self, declarations: &Declarations, parent: Option<&Self>) -> Result<()> {
        for declaration in declarations.iter() {
            declaration.property.validate_value(&declaration.value)?;
            let value = resolve_keyword(declaration.property, &declaration.value, parent);
            self.values.insert(declaration.property, value);
        }
        Ok(())
    }
}

impl Default for Resolved {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Context<'a, T: Tree> {
    pub tree: &'a T,
    pub node: T::Id,
    pub traversal: Traversal,
    pub viewport: Viewport,
    pub container: Option<Container>,
    pub parent: Option<&'a Resolved>,
    pub local: Option<&'a Declarations>,
    pub animated: Option<&'a Declarations>,
}

impl<'a, T: Tree> Context<'a, T> {
    #[must_use]
    pub fn new(tree: &'a T, node: T::Id) -> Self {
        Self {
            tree,
            node,
            traversal: Traversal::Projected,
            viewport: Viewport::default(),
            container: None,
            parent: None,
            local: None,
            animated: None,
        }
    }

    #[must_use]
    pub const fn traversal(mut self, traversal: Traversal) -> Self {
        self.traversal = traversal;
        self
    }

    #[must_use]
    pub const fn viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = viewport;
        self
    }

    #[must_use]
    pub const fn container(mut self, container: Container) -> Self {
        self.container = Some(container);
        self
    }

    #[must_use]
    pub const fn parent(mut self, parent: &'a Resolved) -> Self {
        self.parent = Some(parent);
        self
    }

    #[must_use]
    pub const fn local(mut self, local: &'a Declarations) -> Self {
        self.local = Some(local);
        self
    }

    #[must_use]
    pub const fn animated(mut self, animated: &'a Declarations) -> Self {
        self.animated = Some(animated);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Resolver {
    sheet: Sheet,
    cache: BTreeMap<u64, CacheEntry>,
    cache_by_node: BTreeMap<u64, BTreeSet<u64>>,
    cache_hits: usize,
}

impl Resolver {
    #[must_use]
    pub fn new(sheet: Sheet) -> Self {
        Self {
            sheet,
            cache: BTreeMap::new(),
            cache_by_node: BTreeMap::new(),
            cache_hits: 0,
        }
    }

    #[must_use]
    pub fn sheet(&self) -> &Sheet {
        &self.sheet
    }

    #[must_use]
    pub const fn cache_hits(&self) -> usize {
        self.cache_hits
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.cache_by_node.clear();
        self.cache_hits = 0;
    }

    pub fn clear_cache_for_sheet(&mut self, version: Version) {
        if self.sheet.version() == version {
            self.clear_cache();
        }
    }

    pub fn clear_cache_for_node<T: Hash>(&mut self, node: T) {
        let node_hash = hash_node(&node);
        let Some(keys) = self.cache_by_node.remove(&node_hash) else {
            return;
        };
        for key in keys {
            self.cache.remove(&key);
        }
    }

    pub fn resolve<T: Tree>(&mut self, context: Context<'_, T>) -> Result<Resolved> {
        let cache_key = self.cache_key(&context)?;
        if let Some(key) = cache_key
            && let Some(entry) = self.cache.get(&key.value)
        {
            self.cache_hits += 1;
            return Ok(entry.resolved.clone());
        }

        let mut resolved = Resolved::new();
        if let Some(parent) = context.parent {
            resolved.inherit_from(parent);
        }

        for rule in self.sheet.candidate_rules(context.tree, context.node)? {
            if !Condition::matches_all(rule.conditions(), context.viewport, context.container) {
                continue;
            }
            if rule
                .selector()
                .matches(context.tree, context.node, context.traversal)?
            {
                resolved.apply(rule.declarations(), context.parent)?;
            }
        }

        if let Some(local) = context.local {
            resolved.apply(local, context.parent)?;
        }
        if let Some(animated) = context.animated {
            resolved.apply(animated, context.parent)?;
        }

        if let Some(key) = cache_key {
            self.cache_by_node
                .entry(key.node)
                .or_default()
                .insert(key.value);
            self.cache.insert(
                key.value,
                CacheEntry {
                    resolved: resolved.clone(),
                },
            );
        }
        Ok(resolved)
    }

    fn cache_key<T: Tree>(&self, context: &Context<'_, T>) -> Result<Option<CacheKey>> {
        let Some(tree_version) = context.tree.version_hint() else {
            return Ok(None);
        };
        let node = context.tree.node(context.node)?;
        let node_hash = hash_node(&context.node);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.sheet.version().hash(&mut hasher);
        tree_version.hash(&mut hasher);
        node_hash.hash(&mut hasher);
        context.traversal.hash(&mut hasher);
        hash_state(node.state, &mut hasher);
        context.viewport.cache_values().hash(&mut hasher);
        context
            .container
            .map(Container::cache_values)
            .hash(&mut hasher);
        context.parent.map(Resolved::fingerprint).hash(&mut hasher);
        context
            .local
            .map(Declarations::fingerprint)
            .hash(&mut hasher);
        context
            .animated
            .map(Declarations::fingerprint)
            .hash(&mut hasher);
        Ok(Some(CacheKey {
            value: hasher.finish(),
            node: node_hash,
        }))
    }
}

fn resolve_keyword(property: Property, value: &Value, parent: Option<&Resolved>) -> Value {
    match value {
        Value::Keyword(super::Keyword::Initial) => property.metadata().default,
        Value::Keyword(super::Keyword::Inherit) => parent
            .map(|parent| parent.get(property).clone())
            .unwrap_or_else(|| property.metadata().default),
        Value::Keyword(super::Keyword::Unset) => {
            if property.metadata().inherited {
                parent
                    .map(|parent| parent.get(property).clone())
                    .unwrap_or_else(|| property.metadata().default)
            } else {
                property.metadata().default
            }
        }
        _ => value.clone(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CacheKey {
    value: u64,
    node: u64,
}

#[derive(Clone, Debug)]
struct CacheEntry {
    resolved: Resolved,
}

fn hash_state(state: &surgeist_retained::State, hasher: &mut impl Hasher) {
    state.disabled.hash(hasher);
    state.hovered.hash(hasher);
    state.active.hash(hasher);
    state.focused.hash(hasher);
    state.focus_within.hash(hasher);
    state.pointer_captured.hash(hasher);
    state.selected.hash(hasher);
    state.pressed.hash(hasher);
    state.checked.hash(hasher);
    state.expanded.hash(hasher);
}

fn hash_node<T: Hash>(node: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    node.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let local_one = Declarations::new().text_color(super::super::Color::BLACK);
        let local_two = Declarations::new().bg(super::super::Color::BLACK);
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
