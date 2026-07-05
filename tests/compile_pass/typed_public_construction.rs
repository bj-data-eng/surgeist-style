use surgeist_style::{
    AnimationNameList, AttributeCaseSensitivity, AttributeMatcher, AttributeSelector,
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, AuthoredTokens,
    Change, Color, Combinator, CssPx, CssWideKeyword, CustomPropertyName, CustomPropertyTypedValue,
    CustomPropertyValue, Declarations, DimensionLength, DurationSeconds, FontFamilyList,
    GridTrackList, LayerOrder, Length, Node, NthPattern, NthSelector, Opacity, Property,
    PseudoClassSelector, PseudoElement, RangeState, RelativeSelector, RelativeSelectorList,
    RulePrecedence, RuleTarget, RuntimePseudoClass, Selector, SelectorList, Context,
    SelectorListPseudoClass, SelectorSpecificity, SelectorFactChange, Sheet, SourceOrder, StateFlag,
    StructuralSelector, StyleAttributeValue, StyleBucket, StyleBucketPolicy, StyleRole,
    StyleState, StyleTag, Traversal, Tree, TypedDeclaration, Value, VariableDependentValue,
    VariableExpression, VariableFallback, VariableReference,
};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?)?);
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::try_text_color(Color::try_rgba(0.0, 0.0, 0.0, 1.0)?)?;

    let declarations = Declarations::from_typed([width, opacity, color])?;
    assert_eq!(declarations.len(), 3);

    let declarations = Declarations::new()
        .width(DimensionLength::px(CssPx::new(240.0)?)?)
        .opacity(Opacity::new(0.5)?)
        .transition_duration(DurationSeconds::new(0.2)?)
        .try_grid_template_rows(GridTrackList::new(Vec::new()))?;
    assert_eq!(declarations.len(), 4);

    let declarations = Declarations::new()
        .try_set(
            Property::FontFamily,
            Value::FontFamilyList(FontFamilyList::new(["Inter", "system-ui"])?),
        )?
        .try_set(
            Property::AnimationName,
            Value::AnimationNameList(AnimationNameList::new(["fade-in"])?),
        )?;
    assert_eq!(declarations.len(), 2);

    let precedence = RulePrecedence::new(LayerOrder::new(2), SourceOrder::new(8));
    assert_eq!(precedence.layer_order(), LayerOrder::new(2));

    let specificity = SelectorSpecificity::new(0, 1, 0);
    assert!(specificity > SelectorSpecificity::new(0, 0, 1));
    let selector_list = SelectorList::try_new([
        Selector::tag("button")?,
        Selector::class("primary")?,
    ])?;
    assert_eq!(selector_list.selectors().len(), 2);
    let attribute_selector = AttributeSelector::equals_with_case(
        "data-id",
        "submit",
        AttributeCaseSensitivity::AsciiCaseInsensitive,
    )?;
    let matcher_selector = AttributeSelector::matcher_with_case(
        "data-tags",
        AttributeMatcher::Includes(StyleAttributeValue::new("primary")?),
        AttributeCaseSensitivity::DocumentDefault,
    )?;
    assert!(matches!(attribute_selector, AttributeSelector::Matcher { .. }));
    assert!(matches!(matcher_selector, AttributeSelector::Matcher { .. }));
    let pseudo_selector = Selector::pseudo(PseudoClassSelector::runtime(RuntimePseudoClass::Hover));
    assert_eq!(pseudo_selector.specificity(), SelectorSpecificity::new(0, 1, 0));
    let nth_selector = NthSelector::new(NthPattern::odd(), Some(selector_list.clone()));
    assert_eq!(nth_selector.pattern(), NthPattern::odd());
    assert!(nth_selector.filter().is_some());
    let structural_selector = Selector::pseudo(PseudoClassSelector::structural(
        StructuralSelector::NthChild(nth_selector),
    ));
    assert_eq!(
        structural_selector.specificity(),
        SelectorSpecificity::new(0, 2, 0)
    );
    let has_child = Selector::pseudo(PseudoClassSelector::has(RelativeSelectorList::try_new(
        [RelativeSelector::new(
            Combinator::Child,
            Selector::class("child")?,
        )],
    )?));
    assert_eq!(has_child.specificity(), SelectorSpecificity::new(0, 1, 0));
    let not_disabled = Selector::pseudo(PseudoClassSelector::selector_list(
        SelectorListPseudoClass::Not(SelectorList::try_new([Selector::class("disabled")?])?),
    ));
    assert_eq!(
        not_disabled.specificity(),
        SelectorSpecificity::new(0, 1, 0)
    );
    let state = StyleState::default().with_range_state(Some(RangeState::InRange));
    assert!(state.has_flag(StateFlag::InRange));

    let before_bucket = StyleBucket::Before;
    assert_eq!(before_bucket.policy(), StyleBucketPolicy::GeneratedContentBox);
    let nested_marker_bucket =
        StyleBucket::from_pseudo_elements([PseudoElement::Before, PseudoElement::Marker])?;
    assert_eq!(nested_marker_bucket, StyleBucket::BeforeMarker);
    assert_eq!(
        StyleBucket::BeforeMarker.policy(),
        StyleBucketPolicy::GeneratedContentMarker
    );
    let target = RuleTarget::new(Selector::class("badge")?, StyleBucket::Before);
    assert_eq!(target.bucket(), StyleBucket::Before);
    let mut targeted_sheet = Sheet::new().targeted_rule(target, Declarations::new());
    targeted_sheet.push_targeted_rule(
        RuleTarget::new(Selector::tag("button")?, StyleBucket::After),
        Declarations::new(),
    );
    assert_eq!(targeted_sheet.rule_count(), 2);
    let tree = PublicTree;
    let _before_context = Context::new(&tree, 0).style_bucket(StyleBucket::Before);

    let selector_change = Change::from_selector_fact_change(SelectorFactChange::Class);
    assert!(selector_change.rematch);
    assert!(selector_change.scope.whole_tree);

    let custom_name = CustomPropertyName::try_new("--brand")?;
    let authored_tokens = AuthoredTokens::new("var(--brand, #000)");
    assert_eq!(custom_name.as_str(), "--brand");
    assert_eq!(authored_tokens.as_css(), "var(--brand, #000)");

    let space_name = CustomPropertyName::try_new("--space")?;
    let variable_gap = VariableDependentValue::try_new(
        Property::Gap,
        AuthoredTokens::new("var(--space, 8px)"),
        VariableExpression::Reference(VariableReference::new(
            space_name,
            Some(VariableFallback::new(
                AuthoredTokens::new("8px"),
                VariableExpression::Value(Value::Length(Length::px(8.0))),
            )),
        )),
    )?;
    assert_eq!(variable_gap.property(), Property::Gap);
    assert_eq!(variable_gap.dependencies().len(), 1);

    let nested_fallback_width = VariableDependentValue::try_new(
        Property::Width,
        AuthoredTokens::new("var(--space, var(--fallback, 4px))"),
        VariableExpression::Reference(VariableReference::new(
            CustomPropertyName::try_new("--space")?,
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--fallback, 4px)"),
                VariableExpression::Reference(VariableReference::new(
                    CustomPropertyName::try_new("--fallback")?,
                    Some(VariableFallback::new(
                        AuthoredTokens::new("4px"),
                        VariableExpression::Value(Value::Length(Length::px(4.0))),
                    )),
                )),
            )),
        )),
    )?;
    assert_eq!(nested_fallback_width.dependencies().len(), 2);

    let mut custom_value = CustomPropertyValue::new(
        AuthoredTokens::new("var(--brand, var(--fallback-brand, black))"),
        [VariableReference::new(
            CustomPropertyName::try_new("--brand")?,
            Some(VariableFallback::new(
                AuthoredTokens::new("var(--fallback-brand, black)"),
                VariableExpression::Reference(VariableReference::new(
                    CustomPropertyName::try_new("--fallback-brand")?,
                    Some(VariableFallback::new(
                        AuthoredTokens::new("black"),
                        VariableExpression::Value(Value::Color(Color::BLACK)),
                    )),
                )),
            )),
        )],
    );
    assert_eq!(custom_value.references()[0].name().as_str(), "--brand");
    assert_eq!(custom_value.dependencies().len(), 2);
    custom_value.try_push_typed_value(CustomPropertyTypedValue::try_new(
        Property::Color,
        VariableExpression::Reference(VariableReference::new(
            CustomPropertyName::try_new("--brand")?,
            Some(VariableFallback::new(
                AuthoredTokens::new("black"),
                VariableExpression::Value(Value::Color(Color::BLACK)),
            )),
        )),
    )?)?;
    assert!(custom_value.typed_value(Property::Color).is_some());

    let literal_custom_value = CustomPropertyValue::new(AuthoredTokens::new("8px"), [])
        .try_with_typed_value(
            Property::Width,
            VariableExpression::Value(Value::Length(Length::px(8.0))),
        )?;
    assert_eq!(literal_custom_value.authored().as_css(), "8px");

    let mut authored = AuthoredDeclarations::new();
    authored.push(AuthoredDeclaration::css_wide(
        AuthoredProperty::All,
        CssWideKeyword::Initial,
    ));
    authored.try_push(AuthoredDeclaration::try_new(
        AuthoredProperty::Custom(CustomPropertyName::try_new("--brand")?),
        AuthoredValue::CustomProperty(custom_value),
    )?)?;
    authored.try_push(AuthoredDeclaration::try_new(
        AuthoredProperty::Property(Property::Color),
        AuthoredValue::Value(Value::Color(Color::BLACK)),
    )?)?;
    assert!(authored.len() >= 2);
    Ok(())
}

struct PublicTree;

impl Tree for PublicTree {
    type Id = usize;

    fn version_hint(&self) -> Option<u64> {
        None
    }

    fn node(&self, id: Self::Id) -> surgeist_style::Result<Node<Self::Id>> {
        Ok(Node {
            id,
            tag: Some(StyleTag::new("button")?),
            key: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            role: StyleRole::default(),
            state: StyleState::default(),
            text: false,
        })
    }

    fn parent(
        &self,
        _id: Self::Id,
        _traversal: Traversal,
    ) -> surgeist_style::Result<Option<Self::Id>> {
        Ok(None)
    }

    fn children(
        &self,
        _id: Self::Id,
        _traversal: Traversal,
    ) -> surgeist_style::Result<impl Iterator<Item = Self::Id> + '_> {
        Ok(std::iter::empty())
    }

    fn previous_sibling(
        &self,
        _id: Self::Id,
        _traversal: Traversal,
    ) -> surgeist_style::Result<Option<Self::Id>> {
        Ok(None)
    }
}
