use std::{
    collections::{BTreeMap, BTreeSet},
    sync::atomic::{AtomicU64, Ordering},
};

use super::{Change, Condition, Declarations, Result, Selector, Tree, selector::PrimaryKey};
use surgeist_retained::{Class, Key, Tag};

static NEXT_VERSION: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Version(u64);

impl Version {
    fn next() -> Self {
        Self(NEXT_VERSION.fetch_add(1, Ordering::Relaxed))
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    selector: Selector,
    declarations: Declarations,
    conditions: Vec<Condition>,
    order: u32,
}

impl Rule {
    #[must_use]
    pub fn new(selector: Selector, declarations: Declarations) -> Self {
        Self::with_order(selector, declarations, 0)
    }

    #[must_use]
    pub(crate) fn with_order(selector: Selector, declarations: Declarations, order: u32) -> Self {
        Self {
            selector,
            declarations,
            conditions: Vec::new(),
            order,
        }
    }

    #[must_use]
    pub fn when(mut self, conditions: impl IntoIterator<Item = Condition>) -> Self {
        self.conditions = conditions.into_iter().collect();
        self
    }

    #[must_use]
    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    #[must_use]
    pub fn declarations(&self) -> &Declarations {
        &self.declarations
    }

    #[must_use]
    pub fn conditions(&self) -> &[Condition] {
        &self.conditions
    }

    #[must_use]
    pub const fn order(&self) -> u32 {
        self.order
    }
}

#[derive(Clone, Debug)]
pub struct Sheet {
    rules: Vec<Rule>,
    index: RuleIndex,
    version: Version,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            index: RuleIndex::default(),
            version: Version::next(),
        }
    }
}

impl PartialEq for Sheet {
    fn eq(&self, other: &Self) -> bool {
        self.rules == other.rules
    }
}

impl Sheet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn rule(mut self, selector: Selector, declarations: Declarations) -> Self {
        self.push_rule(selector, declarations);
        self
    }

    #[must_use]
    pub fn conditional_rule(
        mut self,
        selector: Selector,
        declarations: Declarations,
        conditions: impl IntoIterator<Item = Condition>,
    ) -> Self {
        self.push_conditional_rule(selector, declarations, conditions);
        self
    }

    pub fn push_rule(&mut self, selector: Selector, declarations: Declarations) -> &mut Self {
        let order = self.rules.len() as u32;
        self.push(Rule::with_order(selector, declarations, order));
        self
    }

    pub fn push_conditional_rule(
        &mut self,
        selector: Selector,
        declarations: Declarations,
        conditions: impl IntoIterator<Item = Condition>,
    ) -> &mut Self {
        let order = self.rules.len() as u32;
        self.push(Rule::with_order(selector, declarations, order).when(conditions));
        self
    }

    pub fn extend(&mut self, rules: impl IntoIterator<Item = Rule>) -> &mut Self {
        for mut rule in rules {
            rule.order = self.rules.len() as u32;
            self.push(rule);
        }
        self
    }

    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    #[must_use]
    pub const fn version(&self) -> Version {
        self.version
    }

    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    pub fn rules_for_selector<'a>(
        &'a self,
        selector: &'a Selector,
    ) -> impl Iterator<Item = &'a Rule> + 'a {
        self.rules
            .iter()
            .filter(move |rule| rule.selector() == selector)
    }

    pub fn rules_for_class<'a>(&'a self, class: &'a Class) -> impl Iterator<Item = &'a Rule> + 'a {
        self.index
            .by_class
            .get(class)
            .into_iter()
            .flat_map(|indices| indices.iter().map(|index| &self.rules[*index]))
    }

    pub fn rules_for_tag<'a>(&'a self, tag: &'a Tag) -> impl Iterator<Item = &'a Rule> + 'a {
        self.index
            .by_tag
            .get(tag)
            .into_iter()
            .flat_map(|indices| indices.iter().map(|index| &self.rules[*index]))
    }

    pub fn rules_for_key<'a>(&'a self, key: &'a Key) -> impl Iterator<Item = &'a Rule> + 'a {
        self.index
            .by_key
            .get(key)
            .into_iter()
            .flat_map(|indices| indices.iter().map(|index| &self.rules[*index]))
    }

    pub fn conditional_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules
            .iter()
            .filter(|rule| !rule.conditions().is_empty())
    }

    pub fn unconditional_rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.conditions().is_empty())
    }

    #[must_use]
    pub fn viewport_change(&self) -> Change {
        self.condition_change(Condition::is_viewport)
    }

    #[must_use]
    pub fn container_change(&self) -> Change {
        self.condition_change(Condition::is_container)
    }

    pub fn candidate_rule_count<T: Tree>(&self, tree: &T, id: T::Id) -> Result<usize> {
        Ok(self.candidate_indices(tree, id)?.len())
    }

    pub(crate) fn candidate_rules<T: Tree>(&self, tree: &T, id: T::Id) -> Result<Vec<&Rule>> {
        Ok(self
            .candidate_indices(tree, id)?
            .into_iter()
            .map(|index| &self.rules[index])
            .collect())
    }

    fn push(&mut self, rule: Rule) {
        let index = self.rules.len();
        self.index.insert(index, rule.selector.primary_key());
        self.rules.push(rule);
        self.version = Version::next();
    }

    fn condition_change(&self, predicate: impl Fn(Condition) -> bool) -> Change {
        let mut change = Change::empty();
        for rule in &self.rules {
            if !rule.conditions().iter().copied().any(&predicate) {
                continue;
            }
            change.rematch = true;
            change.scope.include_subtree();
            for declaration in rule.declarations().iter() {
                change.invalidation.include_property(declaration.property);
            }
        }
        change
    }

    fn candidate_indices<T: Tree>(&self, tree: &T, id: T::Id) -> Result<Vec<usize>> {
        let node = tree.node(id)?;
        let mut candidates = BTreeSet::new();
        candidates.extend(self.index.universal.iter().copied());
        if let Some(key) = node.key
            && let Some(indices) = self.index.by_key.get(key)
        {
            candidates.extend(indices.iter().copied());
        }
        if let Some(tag) = node.tag
            && let Some(indices) = self.index.by_tag.get(tag)
        {
            candidates.extend(indices.iter().copied());
        }
        for class in node.classes {
            if let Some(indices) = self.index.by_class.get(class) {
                candidates.extend(indices.iter().copied());
            }
        }
        Ok(candidates.into_iter().collect())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RuleIndex {
    universal: Vec<usize>,
    by_key: BTreeMap<Key, Vec<usize>>,
    by_class: BTreeMap<Class, Vec<usize>>,
    by_tag: BTreeMap<Tag, Vec<usize>>,
}

impl RuleIndex {
    fn insert(&mut self, index: usize, key: PrimaryKey) {
        match key {
            PrimaryKey::Universal => self.universal.push(index),
            PrimaryKey::Key(key) => self.by_key.entry(key).or_default().push(index),
            PrimaryKey::Class(class) => self.by_class.entry(class).or_default().push(index),
            PrimaryKey::Tag(tag) => self.by_tag.entry(tag).or_default().push(index),
        }
    }
}
