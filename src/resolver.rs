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
    CustomPropertyDependencies, CustomPropertyName, CustomPropertyResolution, CustomPropertyValue,
    VariableDependentValue, VariableExpression,
    authored::{AuthoredCascadeValue, CustomPropertyCascadeValue},
    sheet::{RuleDeclarationOrigin, RuleDeclarationValue},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Resolved {
    values: BTreeMap<Property, Value>,
    custom_properties: BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    custom_property_dependencies: CustomPropertyDependencies,
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
        Self {
            values,
            custom_properties: BTreeMap::new(),
            custom_property_dependencies: CustomPropertyDependencies::default(),
        }
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
    pub fn custom_property(&self, name: &CustomPropertyName) -> Option<&CustomPropertyValue> {
        self.custom_properties
            .get(name)
            .and_then(CustomPropertyResolution::value)
    }

    #[must_use]
    pub fn custom_property_resolution(
        &self,
        name: &CustomPropertyName,
    ) -> Option<&CustomPropertyResolution> {
        self.custom_properties.get(name)
    }

    #[must_use]
    pub const fn custom_property_dependencies(&self) -> &CustomPropertyDependencies {
        &self.custom_property_dependencies
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
        self.custom_properties = parent.custom_properties.clone();
    }

    fn fingerprint(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for (property, value) in &self.values {
            property.hash(&mut hasher);
            hash_value(value, &mut hasher);
        }
        for (name, resolution) in &self.custom_properties {
            name.hash(&mut hasher);
            hash_custom_property_resolution(resolution, &mut hasher);
        }
        for name in self.custom_property_dependencies.names() {
            name.hash(&mut hasher);
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
        let mut custom_candidates =
            BTreeMap::<CustomPropertyName, Vec<CustomPropertyCandidate>>::new();
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
                for declaration in rule.custom_declaration_items() {
                    let candidate = CustomPropertyCandidate::from_declaration(
                        rule.precedence(),
                        declaration.origin(),
                        declaration.value(),
                    );
                    custom_candidates
                        .entry(declaration.name().clone())
                        .or_default()
                        .push(candidate);
                }
            }
        }
        resolve_custom_property_candidates(
            &mut resolved.custom_properties,
            &mut custom_candidates,
            context.parent,
        );
        for (property, candidates) in &mut rule_candidates {
            candidates.sort_by_key(|candidate| candidate.precedence);
            let value = resolve_rule_candidates(
                *property,
                candidates,
                context.parent,
                &resolved.custom_properties,
                &mut resolved.custom_property_dependencies,
            );
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
                AuthoredCascadeValue::VariableDependent(value) => {
                    debug_assert_eq!(property, value.property());
                    RuleCandidateValue::VariableDependent(value.clone())
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
    VariableDependent(VariableDependentValue),
}

#[derive(Clone, Debug)]
struct CustomPropertyCandidate {
    precedence: RulePrecedence,
    origin: RuleDeclarationOrigin,
    value: CustomPropertyCandidateValue,
}

impl CustomPropertyCandidate {
    fn from_declaration(
        precedence: RulePrecedence,
        origin: RuleDeclarationOrigin,
        value: &CustomPropertyCascadeValue,
    ) -> Self {
        let value = match value {
            CustomPropertyCascadeValue::Value(value) => {
                CustomPropertyCandidateValue::Value(value.clone())
            }
            CustomPropertyCascadeValue::CssWideKeyword(keyword) => {
                CustomPropertyCandidateValue::CssWideKeyword(*keyword)
            }
        };
        Self {
            precedence,
            origin,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CustomPropertyCandidateValue {
    Value(CustomPropertyValue),
    CssWideKeyword(CssWideKeyword),
}

fn resolve_custom_property_candidates(
    environment: &mut BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    candidates_by_name: &mut BTreeMap<CustomPropertyName, Vec<CustomPropertyCandidate>>,
    parent: Option<&Resolved>,
) {
    for candidates in candidates_by_name.values_mut() {
        candidates.sort_by_key(|candidate| candidate.precedence);
    }
    let names = candidates_by_name.keys().cloned().collect::<Vec<_>>();
    for name in names {
        let Some(candidates) = candidates_by_name.get(&name) else {
            continue;
        };
        let Some(index) = candidates.len().checked_sub(1) else {
            continue;
        };
        let mut visited = BTreeSet::new();
        match resolve_custom_property_candidate_at(&name, candidates, index, parent, &mut visited) {
            Some(resolution) => {
                environment.insert(name, resolution);
            }
            None => {
                environment.remove(&name);
            }
        }
    }
    mark_custom_property_cycles(environment);
}

fn resolve_custom_property_candidate_at(
    name: &CustomPropertyName,
    candidates: &[CustomPropertyCandidate],
    index: usize,
    parent: Option<&Resolved>,
    visited: &mut BTreeSet<usize>,
) -> Option<CustomPropertyResolution> {
    if !visited.insert(index) {
        return parent
            .and_then(|parent| parent.custom_property_resolution(name))
            .cloned();
    }
    let candidate = &candidates[index];
    let resolution = match &candidate.value {
        CustomPropertyCandidateValue::Value(value) => {
            Some(CustomPropertyResolution::valid(value.clone()))
        }
        CustomPropertyCandidateValue::CssWideKeyword(keyword) => {
            if matches!(keyword, CssWideKeyword::RevertLayer) {
                debug_assert_eq!(candidate.origin, RuleDeclarationOrigin::Authored);
            }
            resolve_custom_property_css_wide_keyword(
                name, *keyword, parent, candidates, index, visited,
            )
        }
    };
    visited.remove(&index);
    resolution
}

fn resolve_custom_property_css_wide_keyword(
    name: &CustomPropertyName,
    keyword: CssWideKeyword,
    parent: Option<&Resolved>,
    candidates: &[CustomPropertyCandidate],
    index: usize,
    visited: &mut BTreeSet<usize>,
) -> Option<CustomPropertyResolution> {
    match keyword {
        CssWideKeyword::Initial => None,
        CssWideKeyword::Inherit | CssWideKeyword::Unset => parent
            .and_then(|parent| parent.custom_property_resolution(name))
            .cloned(),
        CssWideKeyword::RevertLayer => {
            let layer = candidates[index].precedence.layer_order();
            if let Some((lower_index, _)) = candidates
                .iter()
                .enumerate()
                .rev()
                .find(|(_, candidate)| candidate.precedence.layer_order() < layer)
            {
                resolve_custom_property_candidate_at(name, candidates, lower_index, parent, visited)
            } else {
                parent
                    .and_then(|parent| parent.custom_property_resolution(name))
                    .cloned()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Visiting,
    Done,
}

fn mark_custom_property_cycles(
    environment: &mut BTreeMap<CustomPropertyName, CustomPropertyResolution>,
) {
    let names = environment.keys().cloned().collect::<Vec<_>>();
    let mut states = BTreeMap::new();
    let mut stack = Vec::new();
    let mut cyclic = BTreeSet::new();
    for name in &names {
        collect_custom_property_cycles(name, environment, &mut states, &mut stack, &mut cyclic);
    }
    for name in cyclic {
        environment.insert(name, CustomPropertyResolution::invalid());
    }
}

fn collect_custom_property_cycles(
    name: &CustomPropertyName,
    environment: &BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    states: &mut BTreeMap<CustomPropertyName, VisitState>,
    stack: &mut Vec<CustomPropertyName>,
    cyclic: &mut BTreeSet<CustomPropertyName>,
) {
    match states.get(name).copied() {
        Some(VisitState::Done) => return,
        Some(VisitState::Visiting) => {
            if let Some(index) = stack.iter().position(|stacked| stacked == name) {
                cyclic.extend(stack[index..].iter().cloned());
            }
            return;
        }
        None => {}
    }
    states.insert(name.clone(), VisitState::Visiting);
    stack.push(name.clone());
    if let Some(resolution) = environment.get(name)
        && let Some(value) = resolution.value()
    {
        for dependency in required_custom_property_edges(value, environment) {
            if environment.contains_key(dependency) {
                collect_custom_property_cycles(dependency, environment, states, stack, cyclic);
            }
        }
    }
    stack.pop();
    states.insert(name.clone(), VisitState::Done);
}

fn required_custom_property_edges<'a>(
    value: &'a CustomPropertyValue,
    environment: &'a BTreeMap<CustomPropertyName, CustomPropertyResolution>,
) -> BTreeSet<&'a CustomPropertyName> {
    let mut edges = BTreeSet::new();
    for reference in value.references() {
        collect_required_reference_edges(reference, environment, &mut edges);
    }
    for property in Property::ALL {
        if let Some(typed_value) = value.typed_value(*property) {
            collect_required_expression_edges(typed_value.expression(), environment, &mut edges);
        }
    }
    edges
}

fn collect_required_expression_edges<'a>(
    expression: &'a VariableExpression,
    environment: &'a BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    edges: &mut BTreeSet<&'a CustomPropertyName>,
) {
    match expression {
        VariableExpression::Value(_) | VariableExpression::CssWideKeyword(_) => {}
        VariableExpression::Reference(reference) => {
            collect_required_reference_edges(reference, environment, edges);
        }
    }
}

fn collect_required_reference_edges<'a>(
    reference: &'a crate::VariableReference,
    environment: &'a BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    edges: &mut BTreeSet<&'a CustomPropertyName>,
) {
    edges.insert(reference.name());
    let needs_fallback = environment
        .get(reference.name())
        .is_none_or(|resolution| resolution.is_invalid() || resolution.value().is_none());
    if needs_fallback && let Some(fallback) = reference.fallback() {
        collect_required_expression_edges(fallback.expression(), environment, edges);
    }
}

fn resolve_legacy_value(property: Property, value: &Value, parent: Option<&Resolved>) -> Value {
    match value {
        Value::Keyword(super::Keyword::Initial) => {
            resolve_contextless_css_wide_keyword(property, CssWideKeyword::Initial, parent)
        }
        Value::Keyword(super::Keyword::Inherit) => {
            resolve_contextless_css_wide_keyword(property, CssWideKeyword::Inherit, parent)
        }
        Value::Keyword(super::Keyword::Unset) => {
            resolve_contextless_css_wide_keyword(property, CssWideKeyword::Unset, parent)
        }
        _ => value.clone(),
    }
}

fn resolve_rule_candidates(
    property: Property,
    candidates: &[RuleCandidate],
    parent: Option<&Resolved>,
    custom_properties: &BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    dependencies: &mut CustomPropertyDependencies,
) -> Value {
    let Some(index) = candidates.len().checked_sub(1) else {
        return property.metadata().default().clone();
    };
    let mut evaluator = RuleEvaluator::new(parent, candidates, custom_properties, dependencies);
    evaluator.resolve_candidate_at(property, index)
}

fn resolve_contextless_css_wide_keyword(
    property: Property,
    keyword: CssWideKeyword,
    parent: Option<&Resolved>,
) -> Value {
    match keyword {
        CssWideKeyword::Initial => property.metadata().default().clone(),
        CssWideKeyword::Inherit => resolve_inherit(property, parent),
        CssWideKeyword::Unset => resolve_unset(property, parent),
        CssWideKeyword::RevertLayer => resolve_unset(property, parent),
    }
}

struct RuleEvaluator<'a, 'dependencies> {
    parent: Option<&'a Resolved>,
    candidates: &'a [RuleCandidate],
    custom_properties: &'a BTreeMap<CustomPropertyName, CustomPropertyResolution>,
    dependencies: &'dependencies mut CustomPropertyDependencies,
    visited_candidates: BTreeSet<usize>,
}

impl<'a, 'dependencies> RuleEvaluator<'a, 'dependencies> {
    fn new(
        parent: Option<&'a Resolved>,
        candidates: &'a [RuleCandidate],
        custom_properties: &'a BTreeMap<CustomPropertyName, CustomPropertyResolution>,
        dependencies: &'dependencies mut CustomPropertyDependencies,
    ) -> Self {
        Self {
            parent,
            candidates,
            custom_properties,
            dependencies,
            visited_candidates: BTreeSet::new(),
        }
    }

    fn resolve_candidate_at(&mut self, property: Property, index: usize) -> Value {
        if !self.visited_candidates.insert(index) {
            return resolve_unset(property, self.parent);
        }
        let origin = self.candidates[index].origin;
        let candidate_value = self.candidates[index].value.clone();
        let value = match candidate_value {
            RuleCandidateValue::Value(value) => resolve_legacy_value(property, &value, self.parent),
            RuleCandidateValue::CssWideKeyword(keyword) => {
                if matches!(keyword, CssWideKeyword::RevertLayer) {
                    debug_assert_eq!(origin, RuleDeclarationOrigin::Authored);
                }
                self.resolve_css_wide_keyword(property, keyword, Some(index))
            }
            RuleCandidateValue::VariableDependent(value) => {
                let mut variable_stack = Vec::new();
                self.evaluate_variable_expression(
                    property,
                    value.expression(),
                    index,
                    &mut variable_stack,
                )
                .unwrap_or_else(|| resolve_unset(property, self.parent))
            }
        };
        self.visited_candidates.remove(&index);
        value
    }

    // Style-owned CSS-wide keyword resolution over root-supplied layer/source precedence.
    fn resolve_css_wide_keyword(
        &mut self,
        property: Property,
        keyword: CssWideKeyword,
        index: Option<usize>,
    ) -> Value {
        match keyword {
            CssWideKeyword::Initial => property.metadata().default().clone(),
            CssWideKeyword::Inherit => resolve_inherit(property, self.parent),
            CssWideKeyword::Unset => resolve_unset(property, self.parent),
            CssWideKeyword::RevertLayer => {
                let Some(index) = index else {
                    return resolve_unset(property, self.parent);
                };
                let layer = self.candidates[index].precedence.layer_order();
                let lower_index = self
                    .candidates
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, candidate)| candidate.precedence.layer_order() < layer)
                    .map(|(lower_index, _)| lower_index);
                lower_index
                    .map(|lower_index| self.resolve_candidate_at(property, lower_index))
                    .unwrap_or_else(|| resolve_unset(property, self.parent))
            }
        }
    }

    fn evaluate_variable_expression(
        &mut self,
        property: Property,
        expression: &VariableExpression,
        candidate_index: usize,
        variable_stack: &mut Vec<CustomPropertyName>,
    ) -> Option<Value> {
        match expression {
            VariableExpression::Value(value) => property
                .validate_value(value)
                .ok()
                .map(|()| resolve_legacy_value(property, value, self.parent)),
            VariableExpression::CssWideKeyword(keyword) => {
                Some(self.resolve_css_wide_keyword(property, *keyword, Some(candidate_index)))
            }
            VariableExpression::Reference(reference) => {
                self.dependencies.insert(reference.name().clone());
                if variable_stack.iter().any(|name| name == reference.name()) {
                    return self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    );
                }

                let Some(resolution) = self.custom_properties.get(reference.name()) else {
                    return self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    );
                };
                if resolution.is_invalid() {
                    return self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    );
                }
                let Some(value) = resolution.value() else {
                    return self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    );
                };
                let Some(typed_value) = value.typed_value(property) else {
                    return self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    );
                };

                variable_stack.push(reference.name().clone());
                let result = self.evaluate_variable_expression(
                    property,
                    typed_value.expression(),
                    candidate_index,
                    variable_stack,
                );
                variable_stack.pop();
                result.or_else(|| {
                    self.evaluate_variable_fallback(
                        property,
                        reference.fallback(),
                        candidate_index,
                        variable_stack,
                    )
                })
            }
        }
    }

    fn evaluate_variable_fallback(
        &mut self,
        property: Property,
        fallback: Option<&crate::VariableFallback>,
        candidate_index: usize,
        variable_stack: &mut Vec<CustomPropertyName>,
    ) -> Option<Value> {
        fallback.and_then(|fallback| {
            self.evaluate_variable_expression(
                property,
                fallback.expression(),
                candidate_index,
                variable_stack,
            )
        })
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

fn hash_custom_property_resolution(
    resolution: &CustomPropertyResolution,
    hasher: &mut DefaultHasher,
) {
    resolution.is_invalid().hash(hasher);
    if let Some(value) = resolution.value() {
        hash_custom_property_value(value, hasher);
    }
}

fn hash_custom_property_value(value: &CustomPropertyValue, hasher: &mut DefaultHasher) {
    value.authored().as_css().hash(hasher);
    for dependency in value.dependencies() {
        dependency.hash(hasher);
    }
    for property in Property::ALL {
        if let Some(typed_value) = value.typed_value(*property) {
            property.hash(hasher);
            hash_variable_expression(typed_value.expression(), hasher);
        }
    }
}

fn hash_variable_expression(expression: &VariableExpression, hasher: &mut DefaultHasher) {
    match expression {
        VariableExpression::Value(value) => {
            0_u8.hash(hasher);
            hash_value(value, hasher);
        }
        VariableExpression::CssWideKeyword(keyword) => {
            1_u8.hash(hasher);
            keyword.hash(hasher);
        }
        VariableExpression::Reference(reference) => {
            2_u8.hash(hasher);
            reference.name().hash(hasher);
            if let Some(fallback) = reference.fallback() {
                true.hash(hasher);
                fallback.authored().as_css().hash(hasher);
                hash_variable_expression(fallback.expression(), hasher);
            } else {
                false.hash(hasher);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredTokens, AuthoredValue,
        Color, CssWideKeyword, CustomPropertyName, CustomPropertyValue, Error, ErrorCode,
        LayerOrder, Node, RulePrecedence, Selector, SourceOrder, StyleRole, StyleState, StyleTag,
        VariableDependentValue, VariableExpression, VariableFallback, VariableReference,
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

    fn custom_name(name: &str) -> CustomPropertyName {
        CustomPropertyName::try_new(name).unwrap()
    }

    fn custom_color_declarations(name: &str, color: Color) -> AuthoredDeclarations {
        custom_value_declarations(
            custom_name(name),
            CustomPropertyValue::new(AuthoredTokens::new(format!("{color:?}")), [])
                .try_with_typed_value(
                    Property::Color,
                    VariableExpression::Value(Value::Color(color)),
                )
                .unwrap(),
        )
    }

    fn custom_value_declarations(
        name: CustomPropertyName,
        value: CustomPropertyValue,
    ) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Custom(name),
                    AuthoredValue::CustomProperty(value),
                )
                .unwrap(),
            )
            .unwrap();
        declarations
    }

    fn custom_keyword_declarations(
        name: CustomPropertyName,
        keyword: CssWideKeyword,
    ) -> AuthoredDeclarations {
        let mut declarations = AuthoredDeclarations::new();
        declarations.push(AuthoredDeclaration::css_wide(
            AuthoredProperty::Custom(name),
            keyword,
        ));
        declarations
    }

    fn variable_color_declarations(
        name: CustomPropertyName,
        fallback: Option<VariableExpression>,
    ) -> AuthoredDeclarations {
        let fallback = fallback
            .map(|expression| VariableFallback::new(AuthoredTokens::new("fallback"), expression));
        let variable = VariableDependentValue::try_new(
            Property::Color,
            AuthoredTokens::new(format!("var({})", name.as_str())),
            VariableExpression::Reference(VariableReference::new(name, fallback)),
        )
        .unwrap();
        let mut declarations = AuthoredDeclarations::new();
        declarations
            .try_push(
                AuthoredDeclaration::try_new(
                    AuthoredProperty::Property(Property::Color),
                    AuthoredValue::VariableDependent(variable),
                )
                .unwrap(),
            )
            .unwrap();
        declarations
    }

    fn push_custom_color(sheet: &mut Sheet, name: &str, color: Color, precedence: RulePrecedence) {
        push_authored(sheet, custom_color_declarations(name, color), precedence);
    }

    fn dependency_names(resolved: &Resolved) -> Vec<String> {
        resolved
            .custom_property_dependencies()
            .names()
            .map(|name| name.as_str().to_owned())
            .collect()
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

    fn parent_custom_color(name: &str, color: Color) -> Resolved {
        let mut sheet = Sheet::new();
        push_custom_color(&mut sheet, name, color, precedence(1, 0));
        resolve_child(sheet, None)
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
    fn child_styles_inherit_custom_properties_from_parent_resolved() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::rgba(0.2, 0.3, 0.4, 1.0));
        let sheet = Sheet::new();

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(
            resolved.custom_property(&name).unwrap().authored().as_css(),
            "Color { r: 0.2, g: 0.3, b: 0.4, a: 1.0 }"
        );
    }

    #[test]
    fn authored_custom_property_overrides_parent_custom_property() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::BLACK);
        let mut sheet = Sheet::new();
        push_custom_color(
            &mut sheet,
            "--brand",
            Color::rgba(0.8, 0.1, 0.1, 1.0),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(
            resolved.custom_property(&name).unwrap().authored().as_css(),
            "Color { r: 0.8, g: 0.1, b: 0.1, a: 1.0 }"
        );
    }

    #[test]
    fn initial_clears_custom_property_so_variable_uses_fallback() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::BLACK);
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::Initial),
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                name.clone(),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 1),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.custom_property(&name), None);
        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
    }

    #[test]
    fn unset_inherits_custom_property_from_parent() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::rgba(0.2, 0.3, 0.4, 1.0));
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::Unset),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(
            resolved.custom_property(&name).unwrap().authored().as_css(),
            "Color { r: 0.2, g: 0.3, b: 0.4, a: 1.0 }"
        );
    }

    #[test]
    fn revert_layer_on_custom_property_uses_lower_layer_candidate() {
        let name = custom_name("--brand");
        let mut sheet = Sheet::new();
        push_custom_color(&mut sheet, "--brand", Color::BLACK, precedence(1, 0));
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::RevertLayer),
            precedence(2, 0),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(
            resolved.custom_property(&name).unwrap().authored().as_css(),
            "Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }"
        );
    }

    #[test]
    fn revert_layer_on_custom_property_without_lower_layer_resolves_as_unset() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::rgba(0.3, 0.4, 0.5, 1.0));
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::RevertLayer),
            precedence(2, 0),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(
            resolved.custom_property(&name).unwrap().authored().as_css(),
            "Color { r: 0.3, g: 0.4, b: 0.5, a: 1.0 }"
        );
    }

    #[test]
    fn revert_layer_on_custom_property_honors_lower_layer_clear_result() {
        let name = custom_name("--brand");
        let parent = parent_custom_color("--brand", Color::BLACK);
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::Initial),
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            custom_keyword_declarations(name.clone(), CssWideKeyword::RevertLayer),
            precedence(2, 0),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                name.clone(),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(2, 1),
        );

        let resolved = resolve_child(sheet, Some(&parent));

        assert_eq!(resolved.custom_property(&name), None);
        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
    }

    #[test]
    fn variable_dependent_color_resolves_through_typed_custom_property() {
        let name = custom_name("--brand");
        let mut sheet = Sheet::new();
        push_custom_color(
            &mut sheet,
            "--brand",
            Color::rgba(0.9, 0.2, 0.1, 1.0),
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                name,
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 1),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::rgba(0.9, 0.2, 0.1, 1.0));
    }

    #[test]
    fn custom_property_dependencies_expose_only_valid_primary_reference_when_fallback_is_unused() {
        let mut sheet = Sheet::new();
        push_custom_color(&mut sheet, "--brand", Color::BLACK, precedence(1, 0));
        push_custom_color(
            &mut sheet,
            "--fallback",
            Color::TRANSPARENT,
            precedence(1, 1),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                custom_name("--brand"),
                Some(VariableExpression::Reference(VariableReference::new(
                    custom_name("--fallback"),
                    None,
                ))),
            ),
            precedence(1, 2),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::BLACK);
        assert_eq!(dependency_names(&resolved), ["--brand"]);
    }

    #[test]
    fn custom_property_dependencies_include_fallback_reference_when_primary_is_untyped() {
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_value_declarations(
                custom_name("--brand"),
                CustomPropertyValue::new(AuthoredTokens::new("not typed"), []),
            ),
            precedence(1, 0),
        );
        push_custom_color(
            &mut sheet,
            "--fallback",
            Color::TRANSPARENT,
            precedence(1, 1),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                custom_name("--brand"),
                Some(VariableExpression::Reference(VariableReference::new(
                    custom_name("--fallback"),
                    None,
                ))),
            ),
            precedence(1, 2),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
        assert_eq!(dependency_names(&resolved), ["--brand", "--fallback"]);
    }

    #[test]
    fn custom_property_dependencies_include_fallback_reference_when_primary_is_missing() {
        let mut sheet = Sheet::new();
        push_custom_color(
            &mut sheet,
            "--fallback",
            Color::TRANSPARENT,
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                custom_name("--brand"),
                Some(VariableExpression::Reference(VariableReference::new(
                    custom_name("--fallback"),
                    None,
                ))),
            ),
            precedence(1, 1),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
        assert_eq!(dependency_names(&resolved), ["--brand", "--fallback"]);
    }

    #[test]
    fn variable_dependent_color_uses_fallback_when_custom_property_is_missing() {
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            variable_color_declarations(
                custom_name("--brand"),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
    }

    #[test]
    fn variable_dependent_color_without_fallback_resolves_as_unset_when_missing() {
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            variable_color_declarations(custom_name("--brand"), None),
            precedence(1, 0),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn untyped_custom_property_uses_variable_fallback_or_unset_for_target_property() {
        let name = custom_name("--brand");
        let mut fallback_sheet = Sheet::new();
        push_authored(
            &mut fallback_sheet,
            custom_value_declarations(
                name.clone(),
                CustomPropertyValue::new(AuthoredTokens::new("not typed"), []),
            ),
            precedence(1, 0),
        );
        push_authored(
            &mut fallback_sheet,
            variable_color_declarations(
                name.clone(),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 1),
        );

        let fallback_resolved = resolve_child(fallback_sheet, None);

        assert_eq!(fallback_resolved.text_color(), Color::TRANSPARENT);

        let mut unset_sheet = Sheet::new();
        push_authored(
            &mut unset_sheet,
            custom_value_declarations(
                name.clone(),
                CustomPropertyValue::new(AuthoredTokens::new("not typed"), []),
            ),
            precedence(1, 0),
        );
        push_authored(
            &mut unset_sheet,
            variable_color_declarations(name, None),
            precedence(1, 1),
        );

        let unset_resolved = resolve_child(unset_sheet, None);

        assert_eq!(unset_resolved.text_color(), Color::BLACK);
    }

    #[test]
    fn fallback_only_custom_property_self_reference_does_not_create_cycle() {
        let a = custom_name("--a");
        let b = custom_name("--b");
        let a_color = Color::rgba(0.1, 0.6, 0.3, 1.0);
        let b_fallback_reference = VariableReference::new(b.clone(), None);
        let b_expression = VariableExpression::Reference(VariableReference::new(
            a.clone(),
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--b)"),
                VariableExpression::Reference(b_fallback_reference),
            )),
        ));
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            custom_value_declarations(
                a,
                CustomPropertyValue::new(AuthoredTokens::new("green"), [])
                    .try_with_typed_value(
                        Property::Color,
                        VariableExpression::Value(Value::Color(a_color)),
                    )
                    .unwrap(),
            ),
            precedence(1, 0),
        );
        push_authored(
            &mut sheet,
            custom_value_declarations(
                b.clone(),
                CustomPropertyValue::new(
                    AuthoredTokens::new("var(--a, var(--b))"),
                    [match &b_expression {
                        VariableExpression::Reference(reference) => reference.clone(),
                        _ => unreachable!("test expression is a reference"),
                    }],
                )
                .try_with_typed_value(Property::Color, b_expression)
                .unwrap(),
            ),
            precedence(1, 1),
        );
        push_authored(
            &mut sheet,
            variable_color_declarations(
                b.clone(),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 2),
        );

        let resolved = resolve_child(sheet, None);

        assert_eq!(resolved.text_color(), a_color);
        assert!(
            !resolved
                .custom_property_resolution(&b)
                .unwrap()
                .is_invalid()
        );
    }

    #[test]
    fn custom_property_cycle_uses_fallback_or_unset_without_unbounded_recursion() {
        let a = custom_name("--a");
        let b = custom_name("--b");
        let a_reference = VariableReference::new(b.clone(), None);
        let b_reference = VariableReference::new(a.clone(), None);
        let mut fallback_sheet = Sheet::new();
        push_authored(
            &mut fallback_sheet,
            custom_value_declarations(
                a.clone(),
                CustomPropertyValue::new(AuthoredTokens::new("var(--b)"), [a_reference.clone()])
                    .try_with_typed_value(
                        Property::Color,
                        VariableExpression::Reference(a_reference),
                    )
                    .unwrap(),
            ),
            precedence(1, 0),
        );
        push_authored(
            &mut fallback_sheet,
            custom_value_declarations(
                b.clone(),
                CustomPropertyValue::new(AuthoredTokens::new("var(--a)"), [b_reference.clone()])
                    .try_with_typed_value(
                        Property::Color,
                        VariableExpression::Reference(b_reference),
                    )
                    .unwrap(),
            ),
            precedence(1, 1),
        );
        push_authored(
            &mut fallback_sheet,
            variable_color_declarations(
                a.clone(),
                Some(VariableExpression::Value(Value::Color(Color::TRANSPARENT))),
            ),
            precedence(1, 2),
        );

        let fallback_resolved = resolve_child(fallback_sheet, None);

        assert_eq!(fallback_resolved.text_color(), Color::TRANSPARENT);
        assert!(
            fallback_resolved
                .custom_property_resolution(&a)
                .unwrap()
                .is_invalid()
        );

        let mut unset_sheet = Sheet::new();
        let a_reference = VariableReference::new(b.clone(), None);
        let b_reference = VariableReference::new(a.clone(), None);
        push_authored(
            &mut unset_sheet,
            custom_value_declarations(
                a.clone(),
                CustomPropertyValue::new(AuthoredTokens::new("var(--b)"), [a_reference.clone()])
                    .try_with_typed_value(
                        Property::Color,
                        VariableExpression::Reference(a_reference),
                    )
                    .unwrap(),
            ),
            precedence(1, 0),
        );
        push_authored(
            &mut unset_sheet,
            custom_value_declarations(
                b,
                CustomPropertyValue::new(AuthoredTokens::new("var(--a)"), [b_reference.clone()])
                    .try_with_typed_value(
                        Property::Color,
                        VariableExpression::Reference(b_reference),
                    )
                    .unwrap(),
            ),
            precedence(1, 1),
        );
        push_authored(
            &mut unset_sheet,
            variable_color_declarations(a, None),
            precedence(1, 2),
        );

        let unset_resolved = resolve_child(unset_sheet, None);

        assert_eq!(unset_resolved.text_color(), Color::BLACK);
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
    fn local_and_animated_declarations_override_variable_dependent_sheet_resolution() {
        let tree = TestTree::new(vec![TestNode::new(0, "button")]);
        let mut sheet = Sheet::new();
        push_custom_color(&mut sheet, "--brand", Color::BLACK, precedence(1, 0));
        push_authored(
            &mut sheet,
            variable_color_declarations(custom_name("--brand"), None),
            precedence(1, 1),
        );
        let local = Declarations::new()
            .try_text_color(Color::rgba(0.4, 0.4, 0.4, 1.0))
            .unwrap();
        let animated = Declarations::new()
            .try_text_color(Color::TRANSPARENT)
            .unwrap();
        let mut resolver = Resolver::new(sheet);

        let resolved = resolver
            .resolve(Context::new(&tree, 0).local(&local).animated(&animated))
            .unwrap();

        assert_eq!(resolved.text_color(), Color::TRANSPARENT);
    }

    #[test]
    fn parent_custom_property_changes_affect_cache_keys() {
        let tree = TestTree::new(vec![TestNode::new(0, "button")]);
        let mut sheet = Sheet::new();
        push_authored(
            &mut sheet,
            variable_color_declarations(custom_name("--brand"), None),
            precedence(1, 0),
        );
        let black_parent = parent_custom_color("--brand", Color::BLACK);
        let transparent_parent = parent_custom_color("--brand", Color::TRANSPARENT);
        let mut resolver = Resolver::new(sheet);

        let black_resolved = resolver
            .resolve(Context::new(&tree, 0).parent(&black_parent))
            .unwrap();
        let transparent_resolved = resolver
            .resolve(Context::new(&tree, 0).parent(&transparent_parent))
            .unwrap();

        assert_eq!(black_resolved.text_color(), Color::BLACK);
        assert_eq!(transparent_resolved.text_color(), Color::TRANSPARENT);
        assert_eq!(resolver.cache_hits(), 0);
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
