use std::{
    collections::{BTreeMap, BTreeSet, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

use super::{
    Condition, Container, Corners, CssWideKeyword, Cursor, Declarations, Display, Edges, Length,
    PointerEvents, Property, Result, RulePrecedence, Sheet, Size, Transform, Traversal, Tree,
    Value, Version, Viewport, Visibility, declaration::hash_value,
};
use crate::{
    authored::AuthoredCascadeValue,
    sheet::{RuleDeclarationOrigin, RuleDeclarationValue},
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
                values.insert(*property, property.metadata().default().clone());
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
            Value::Length(value) => value.clone(),
            _ => Length::Auto,
        }
    }

    #[must_use]
    pub fn height(&self) -> Length {
        match self.get(Property::Height) {
            Value::Length(value) => value.clone(),
            _ => Length::Auto,
        }
    }

    #[must_use]
    pub fn padding_edges(&self) -> Edges {
        match self.get(Property::Padding) {
            Value::Edges(edges) => edges.clone(),
            _ => Edges::default(),
        }
    }

    #[must_use]
    pub fn margin_edges(&self) -> Edges {
        match self.get(Property::Margin) {
            Value::Edges(edges) => edges.clone(),
            _ => Edges::default(),
        }
    }

    #[must_use]
    pub fn radius_corners(&self) -> Corners {
        match self.get(Property::Radius) {
            Value::Corners(corners) => corners.clone(),
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
            Value::Length(value) => value.clone(),
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
            Value::Edges(edges) => edges.clone(),
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
            Value::Size(origin) => origin.clone(),
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
            if property.is_canonical() && property.metadata().is_inherited() {
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
            let value = resolve_legacy_value(declaration.property, &declaration.value, parent);
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

        let mut rule_candidates = BTreeMap::<Property, Vec<RuleCandidate>>::new();
        for rule in self.sheet.candidate_rules(context.tree, context.node)? {
            if !Condition::matches_all(rule.conditions(), context.viewport, context.container) {
                continue;
            }
            if rule
                .selector()
                .matches(context.tree, context.node, context.traversal)?
            {
                for declaration in rule.declaration_items() {
                    let candidate = RuleCandidate::try_from_declaration(
                        declaration.property(),
                        rule.precedence(),
                        declaration.origin(),
                        declaration.value(),
                    )?;
                    rule_candidates
                        .entry(candidate.property)
                        .or_default()
                        .push(candidate);
                }
            }
        }
        for (property, candidates) in &mut rule_candidates {
            candidates.sort_by_key(|candidate| candidate.precedence);
            let value = resolve_rule_candidates(*property, candidates, context.parent);
            resolved.values.insert(*property, value);
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
        hash_state(&node.state, &mut hasher);
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

#[derive(Clone, Debug)]
struct RuleCandidate {
    property: Property,
    precedence: RulePrecedence,
    origin: RuleDeclarationOrigin,
    value: RuleCandidateValue,
}

impl RuleCandidate {
    fn try_from_declaration(
        property: Property,
        precedence: RulePrecedence,
        origin: RuleDeclarationOrigin,
        value: RuleDeclarationValue<'_>,
    ) -> Result<Self> {
        let value = match value {
            RuleDeclarationValue::Value(value) => {
                property.validate_value(value)?;
                RuleCandidateValue::Value(value.clone())
            }
            RuleDeclarationValue::Authored(value) => match value {
                AuthoredCascadeValue::Value(value) => {
                    property.validate_value(value)?;
                    RuleCandidateValue::Value(value.clone())
                }
                AuthoredCascadeValue::CssWideKeyword(keyword) => {
                    RuleCandidateValue::CssWideKeyword(*keyword)
                }
            },
        };
        Ok(Self {
            property,
            precedence,
            origin,
            value,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RuleCandidateValue {
    Value(Value),
    CssWideKeyword(CssWideKeyword),
}

fn resolve_legacy_value(property: Property, value: &Value, parent: Option<&Resolved>) -> Value {
    match value {
        Value::Keyword(super::Keyword::Initial) => {
            resolve_css_wide_keyword(property, CssWideKeyword::Initial, parent, None, None, None)
        }
        Value::Keyword(super::Keyword::Inherit) => {
            resolve_css_wide_keyword(property, CssWideKeyword::Inherit, parent, None, None, None)
        }
        Value::Keyword(super::Keyword::Unset) => {
            resolve_css_wide_keyword(property, CssWideKeyword::Unset, parent, None, None, None)
        }
        _ => value.clone(),
    }
}

fn resolve_rule_candidates(
    property: Property,
    candidates: &[RuleCandidate],
    parent: Option<&Resolved>,
) -> Value {
    let Some(index) = candidates.len().checked_sub(1) else {
        return property.metadata().default().clone();
    };
    let mut visited = BTreeSet::new();
    resolve_rule_candidate_at(property, candidates, index, parent, &mut visited)
}

fn resolve_rule_candidate_at(
    property: Property,
    candidates: &[RuleCandidate],
    index: usize,
    parent: Option<&Resolved>,
    visited: &mut BTreeSet<usize>,
) -> Value {
    if !visited.insert(index) {
        return resolve_unset(property, parent);
    }
    let candidate = &candidates[index];
    let value = match &candidate.value {
        RuleCandidateValue::Value(value) => resolve_legacy_value(property, value, parent),
        RuleCandidateValue::CssWideKeyword(keyword) => {
            if matches!(keyword, CssWideKeyword::RevertLayer) {
                debug_assert_eq!(candidate.origin, RuleDeclarationOrigin::Authored);
            }
            resolve_css_wide_keyword(
                property,
                *keyword,
                parent,
                Some(candidates),
                Some(index),
                Some(visited),
            )
        }
    };
    visited.remove(&index);
    value
}

// Style-owned CSS-wide keyword resolution over root-supplied layer/source precedence.
fn resolve_css_wide_keyword(
    property: Property,
    keyword: CssWideKeyword,
    parent: Option<&Resolved>,
    candidates: Option<&[RuleCandidate]>,
    index: Option<usize>,
    visited: Option<&mut BTreeSet<usize>>,
) -> Value {
    match keyword {
        CssWideKeyword::Initial => property.metadata().default().clone(),
        CssWideKeyword::Inherit => resolve_inherit(property, parent),
        CssWideKeyword::Unset => resolve_unset(property, parent),
        CssWideKeyword::RevertLayer => {
            let (Some(candidates), Some(index)) = (candidates, index) else {
                return resolve_unset(property, parent);
            };
            let Some(visited) = visited else {
                return resolve_unset(property, parent);
            };
            let layer = candidates[index].precedence.layer_order();
            candidates
                .iter()
                .enumerate()
                .rev()
                .find(|(_, candidate)| candidate.precedence.layer_order() < layer)
                .map(|(lower_index, _)| {
                    resolve_rule_candidate_at(property, candidates, lower_index, parent, visited)
                })
                .unwrap_or_else(|| resolve_unset(property, parent))
        }
    }
}

fn resolve_inherit(property: Property, parent: Option<&Resolved>) -> Value {
    parent
        .map(|parent| parent.get(property).clone())
        .unwrap_or_else(|| property.metadata().default().clone())
}

fn resolve_unset(property: Property, parent: Option<&Resolved>) -> Value {
    if property.metadata().is_inherited() {
        resolve_inherit(property, parent)
    } else {
        property.metadata().default().clone()
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

fn hash_state(state: &super::StyleState, hasher: &mut impl Hasher) {
    state.disabled().hash(hasher);
    state.hovered().hash(hasher);
    state.active().hash(hasher);
    state.focused().hash(hasher);
    state.focus_within().hash(hasher);
    state.pointer_captured().hash(hasher);
    state.selected().hash(hasher);
    state.pressed().hash(hasher);
    state.checked().hash(hasher);
    state.expanded().hash(hasher);
}

fn hash_node<T: Hash>(node: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    node.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, Color,
        CssWideKeyword, Error, ErrorCode, LayerOrder, Node, RulePrecedence, Selector, SourceOrder,
        StyleRole, StyleState, StyleTag,
    };

    fn precedence(layer: u32, source: u32) -> RulePrecedence {
        RulePrecedence::new(LayerOrder::new(layer), SourceOrder::new(source))
    }

    fn authored_color(value: Color) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Color),
                    AuthoredValue::Value(Value::Color(value)),
                )
                .unwrap(),
            )
            .unwrap();
        declarations
    }

    fn authored_width(value: Length) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Width),
                    AuthoredValue::Value(Value::Length(value)),
                )
                .unwrap(),
            )
            .unwrap();
        declarations
    }

    fn authored_keyword(property: Property, keyword: CssWideKeyword) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Property(property),
            keyword,
        ));
        declarations
    }

    fn resolve_child(sheet: Sheet, parent: Option<&Resolved>) -> Resolved {
        let tree = TestTree::new(vec![
            TestNode::new(0, "stack").children([1]),
            TestNode::new(1, "button"),
        ]);
        let mut resolver = Resolver::new(sheet);
        let context = Context::new(&tree, 1);
        let context = if let Some(parent) = parent {
            context.parent(parent)
        } else {
            context
        };

        resolver.resolve(context).unwrap()
    }

    fn parent_color(color: Color) -> Resolved {
        let mut parent = Resolved::new();
        parent
            .apply(&Declarations::new().try_text_color(color).unwrap(), None)
            .unwrap();
        parent
    }

    fn push_authored(
        sheet: &mut Sheet,
        declarations: AuthoredDeclarations,
        precedence: RulePrecedence,
    ) {
        sheet
            .push_authored_rule(Selector::tag("button").unwrap(), declarations, precedence)
            .unwrap();
    }

    #[test]
    fn higher_layer_wins_over_later_source_order() {
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_color(Color::rgba(1.0, 0.0, 0.0, 1.0)),
            precedence(1, 100),
        );
        push_authored(&mut sheet, authored_color(Color::BLACK), precedence(2, 0));
        let parent = parent_color(Color::rgba(0.0, 1.0, 0.0, 1.0));

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn source_order_wins_within_same_layer() {
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_color(Color::rgba(1.0, 0.0, 0.0, 1.0)),
            precedence(7, 0),
        );
        push_authored(&mut sheet, authored_color(Color::BLACK), precedence(7, 1));
        let parent = parent_color(Color::rgba(0.0, 1.0, 0.0, 1.0));

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn inherit_uses_parent_value() {
        let parent = parent_color(Color::BLACK);
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::Inherit),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn initial_uses_property_default() {
        let mut parent = Resolved::new();
        parent
            .apply(
                &Declarations::new()
                    .try_text_color(Color::rgba(1.0, 0.0, 0.0, 1.0))
                    .unwrap(),
                None,
            )
            .unwrap();
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::Initial),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn unset_inherits_inherited_properties_and_initializes_non_inherited_properties() {
        let mut parent = Resolved::new();
        parent
            .apply(
                &Declarations::new()
                    .try_text_color(Color::rgba(1.0, 0.0, 0.0, 1.0))
                    .unwrap()
                    .try_set(Property::Width, Value::Length(Length::Px(88.0)))
                    .unwrap(),
                None,
            )
            .unwrap();
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::Unset),
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            authored_keyword(Property::Width, CssWideKeyword::Unset),
            precedence(1, 1),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::rgba(1.0, 0.0, 0.0, 1.0));
        assert_eq!(resolved.width(), Length::Auto);
    }

    #[test]
    fn revert_layer_uses_lower_layer_candidate() {
        let mut sheet = Sheet::new();
        push_authored(&mut sheet, authored_color(Color::BLACK), precedence(1, 0));
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::RevertLayer),
            precedence(2, 0),
        );
        let parent = parent_color(Color::rgba(0.0, 1.0, 0.0, 1.0));

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn revert_layer_ignores_same_layer_earlier_source_order() {
        let mut sheet = Sheet::new();
        push_authored(&mut sheet, authored_color(Color::BLACK), precedence(1, 0));
        push_authored(
            &mut sheet,
            authored_color(Color::rgba(1.0, 0.0, 0.0, 1.0)),
            precedence(2, 0),
        );
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::RevertLayer),
            precedence(2, 1),
        );
        let parent = parent_color(Color::rgba(0.0, 1.0, 0.0, 1.0));

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn revert_layer_resolves_as_unset_without_lower_layer() {
        let mut parent = Resolved::new();
        parent
            .apply(
                &Declarations::new()
                    .try_text_color(Color::rgba(1.0, 0.0, 0.0, 1.0))
                    .unwrap()
                    .try_set(Property::Width, Value::Length(Length::Px(88.0)))
                    .unwrap(),
                None,
            )
            .unwrap();
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_keyword(Property::Color, CssWideKeyword::RevertLayer),
            precedence(2, 0),
        );
        push_authored(
            &mut sheet,
            authored_keyword(Property::Width, CssWideKeyword::RevertLayer),
            precedence(2, 1),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.text_color(), Color::rgba(1.0, 0.0, 0.0, 1.0));
        assert_eq!(resolved.width(), Length::Auto);
    }

    #[test]
    fn local_declarations_still_override_sheet_rules() {
        let tree = TestTree::new(vec![TestNode::new(0, "button")]);
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            authored_width(Length::Px(24.0)),
            precedence(2, 0),
        );
        let local = Declarations::new()
            .try_set(Property::Width, Value::Length(Length::Px(48.0)))
            .unwrap();
        let mut resolver = Resolver::new(sheet);

        let resolved = resolver
            .resolve(Context::new(&tree, 0).local(&local))
            .unwrap();

        assert_eq!(resolved.width(), Length::Px(48.0));
    }

    #[test]
    fn legacy_sheet_rules_keep_flat_source_order() {
        let tree = TestTree::new(vec![TestNode::new(0, "button")]);
        let mut sheet = Sheet::new();
        sheet.push_rule(
            Selector::tag("button").unwrap(),
            Declarations::new()
                .try_text_color(Color::rgba(1.0, 0.0, 0.0, 1.0))
                .unwrap(),
        );
        sheet.push_rule(
            Selector::tag("button").unwrap(),
            Declarations::new().try_text_color(Color::BLACK).unwrap(),
        );
        let mut resolver = Resolver::new(sheet);

        let resolved = resolver.resolve(Context::new(&tree, 0)).unwrap();

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[derive(Clone, Debug)]
    struct TestNode {
        id: usize,
        tag: StyleTag,
        children: Vec<usize>,
    }

    impl TestNode {
        fn new(id: usize, tag: &str) -> Self {
            Self {
                id,
                tag: StyleTag::new(tag).unwrap(),
                children: Vec::new(),
            }
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
            let node = self
                .nodes
                .get(id)
                .ok_or_else(|| Error::new(ErrorCode::MissingNode, "missing test node"))?;
            Ok(Node {
                id: node.id,
                tag: Some(node.tag.clone()),
                key: None,
                classes: Vec::new(),
                attributes: Vec::new(),
                role: StyleRole::default(),
                state: StyleState::default(),
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
