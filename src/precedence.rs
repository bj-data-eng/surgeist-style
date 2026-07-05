#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LayerOrder(u32);

impl LayerOrder {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceOrder(u32);

impl SourceOrder {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RulePrecedence {
    layer_order: LayerOrder,
    source_order: SourceOrder,
}

impl RulePrecedence {
    #[must_use]
    pub const fn new(layer_order: LayerOrder, source_order: SourceOrder) -> Self {
        Self {
            layer_order,
            source_order,
        }
    }

    #[must_use]
    pub const fn layer_order(self) -> LayerOrder {
        self.layer_order
    }

    #[must_use]
    pub const fn source_order(self) -> SourceOrder {
        self.source_order
    }

    #[must_use]
    pub const fn with_source_order(self, source_order: SourceOrder) -> Self {
        Self {
            source_order,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_layer_outranks_later_source_order() {
        let lower_layer_late_source =
            RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(100));
        let higher_layer_early_source =
            RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(0));

        assert!(higher_layer_early_source > lower_layer_late_source);
    }

    #[test]
    fn source_order_breaks_ties_inside_same_layer() {
        let early = RulePrecedence::new(LayerOrder::new(7), SourceOrder::new(1));
        let late = RulePrecedence::new(LayerOrder::new(7), SourceOrder::new(2));

        assert!(late > early);
    }

    #[test]
    fn default_precedence_is_zero_layer_zero_source() {
        let precedence = RulePrecedence::default();

        assert_eq!(precedence.layer_order(), LayerOrder::new(0));
        assert_eq!(precedence.source_order(), SourceOrder::new(0));
    }
}
