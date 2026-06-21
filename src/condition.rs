#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Condition {
    Viewport(Viewport),
    Container(Container),
}

impl Condition {
    #[must_use]
    pub const fn viewport(viewport: Viewport) -> Self {
        Self::Viewport(viewport)
    }

    #[must_use]
    pub const fn container(container: Container) -> Self {
        Self::Container(container)
    }

    #[must_use]
    pub fn matches(self, viewport: Viewport, container: Option<Container>) -> bool {
        match self {
            Self::Viewport(query) => query.matches(viewport),
            Self::Container(query) => container.is_some_and(|container| query.matches(container)),
        }
    }

    #[must_use]
    pub const fn is_viewport(self) -> bool {
        matches!(self, Self::Viewport(_))
    }

    #[must_use]
    pub const fn is_container(self) -> bool {
        matches!(self, Self::Container(_))
    }

    #[must_use]
    pub fn matches_all(
        conditions: &[Self],
        viewport: Viewport,
        container: Option<Container>,
    ) -> bool {
        conditions
            .iter()
            .all(|condition| condition.matches(viewport, container))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    width: Option<f32>,
    height: Option<f32>,
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
}

impl Viewport {
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    #[must_use]
    pub const fn min_width(width: f32) -> Self {
        Self {
            min_width: Some(width),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn max_width(width: f32) -> Self {
        Self {
            max_width: Some(width),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn min_height(height: f32) -> Self {
        Self {
            min_height: Some(height),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn max_height(height: f32) -> Self {
        Self {
            max_height: Some(height),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn query() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    #[must_use]
    pub const fn width(self) -> Option<f32> {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> Option<f32> {
        self.height
    }

    fn matches(self, viewport: Self) -> bool {
        let width = viewport.width.unwrap_or(0.0);
        let height = viewport.height.unwrap_or(0.0);
        self.min_width.is_none_or(|min| width >= min)
            && self.max_width.is_none_or(|max| width <= max)
            && self.min_height.is_none_or(|min| height >= min)
            && self.max_height.is_none_or(|max| height <= max)
    }

    pub(crate) fn cache_values(self) -> [Option<u32>; 6] {
        [
            self.width.map(f32::to_bits),
            self.height.map(f32::to_bits),
            self.min_width.map(f32::to_bits),
            self.max_width.map(f32::to_bits),
            self.min_height.map(f32::to_bits),
            self.max_height.map(f32::to_bits),
        ]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Container {
    width: Option<f32>,
    height: Option<f32>,
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
}

impl Container {
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    #[must_use]
    pub const fn min_width(width: f32) -> Self {
        Self {
            min_width: Some(width),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn max_width(width: f32) -> Self {
        Self {
            max_width: Some(width),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn min_height(height: f32) -> Self {
        Self {
            min_height: Some(height),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn max_height(height: f32) -> Self {
        Self {
            max_height: Some(height),
            ..Self::query()
        }
    }

    #[must_use]
    pub const fn query() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }

    #[must_use]
    pub const fn width(self) -> Option<f32> {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> Option<f32> {
        self.height
    }

    fn matches(self, container: Self) -> bool {
        let width = container.width.unwrap_or(0.0);
        let height = container.height.unwrap_or(0.0);
        self.min_width.is_none_or(|min| width >= min)
            && self.max_width.is_none_or(|max| width <= max)
            && self.min_height.is_none_or(|min| height >= min)
            && self.max_height.is_none_or(|max| height <= max)
    }

    pub(crate) fn cache_values(self) -> [Option<u32>; 6] {
        [
            self.width.map(f32::to_bits),
            self.height.map(f32::to_bits),
            self.min_width.map(f32::to_bits),
            self.max_width.map(f32::to_bits),
            self.min_height.map(f32::to_bits),
            self.max_height.map(f32::to_bits),
        ]
    }
}
