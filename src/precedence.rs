use crate::selector::SelectorSpecificity;

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
    specificity: SelectorSpecificity,
    source_order: SourceOrder,
}

impl RulePrecedence {
    #[must_use]
    pub const fn new(layer_order: LayerOrder, source_order: SourceOrder) -> Self {
        Self {
            layer_order,
            specificity: SelectorSpecificity::zero(),
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
    pub const fn specificity(self) -> SelectorSpecificity {
        self.specificity
    }

    #[must_use]
    pub const fn with_source_order(self, source_order: SourceOrder) -> Self {
        Self {
            source_order,
            ..self
        }
    }

    #[must_use]
    pub const fn with_specificity(self, specificity: SelectorSpecificity) -> Self {
        Self {
            specificity,
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
        assert_eq!(precedence.specificity(), SelectorSpecificity::zero());
        assert_eq!(precedence.source_order(), SourceOrder::new(0));
    }

    #[test]
    fn selector_specificity_orders_between_layer_and_source_order() {
        let low_specificity_late = RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(9))
            .with_specificity(SelectorSpecificity::new(0, 0, 1));
        let high_specificity_early = RulePrecedence::new(LayerOrder::new(1), SourceOrder::new(1))
            .with_specificity(SelectorSpecificity::new(0, 1, 0));
        let higher_layer = RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(0))
            .with_specificity(SelectorSpecificity::zero());

        assert!(high_specificity_early > low_specificity_late);
        assert!(higher_layer > high_specificity_early);
    }
}
