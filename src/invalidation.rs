use super::{Property, Resolved};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Invalidation {
    pub layout: bool,
    pub paint: bool,
    pub text: bool,
    pub effect: bool,
    pub animation: bool,
}

impl Invalidation {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            layout: false,
            paint: false,
            text: false,
            effect: false,
            animation: false,
        }
    }

    #[must_use]
    pub fn from_properties(properties: impl IntoIterator<Item = Property>) -> Self {
        let mut invalidation = Self::empty();
        for property in properties {
            invalidation.include_property(property);
        }
        invalidation
    }

    pub fn include_property(&mut self, property: Property) {
        let impact = property.metadata().impact_flags();
        self.layout |= impact.affects_layout();
        self.paint |= impact.affects_paint();
        self.text |= impact.affects_text();
        self.effect |= impact.affects_effect();
        self.animation |= impact.affects_animation();
    }

    #[must_use]
    pub fn between(before: &Resolved, after: &Resolved) -> Self {
        let mut invalidation = Self::empty();
        for property in Property::ALL {
            if property.is_canonical() && before.get(*property) != after.get(*property) {
                invalidation.include_property(*property);
            }
        }
        invalidation
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scope {
    pub node: bool,
    pub siblings: bool,
    pub descendants: bool,
}

impl Scope {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            node: false,
            siblings: false,
            descendants: false,
        }
    }

    pub const fn include_node(&mut self) {
        self.node = true;
    }

    pub const fn include_siblings(&mut self) {
        self.siblings = true;
    }

    pub const fn include_descendants(&mut self) {
        self.descendants = true;
    }

    pub const fn include_subtree(&mut self) {
        self.node = true;
        self.descendants = true;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Change {
    pub rematch: bool,
    pub invalidation: Invalidation,
    pub scope: Scope,
}

impl Change {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            rematch: false,
            invalidation: Invalidation::empty(),
            scope: Scope::empty(),
        }
    }

    #[must_use]
    pub fn from_properties(properties: impl IntoIterator<Item = Property>) -> Self {
        let mut change = Self::empty();
        for property in properties {
            change.scope.include_node();
            change.invalidation.include_property(property);
            if property.metadata().is_inherited() {
                change.scope.include_descendants();
            }
        }
        change
    }
}
