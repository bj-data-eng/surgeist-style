use surgeist_style::{
    AlignContent, AlignItems, Alpha, AnimationNameList, AspectRatio, AttributeCaseSensitivity,
    AttributeMatcher, AttributeSelector, AuthoredDeclaration, AuthoredDeclarations,
    AuthoredProperty, AuthoredTokens, AuthoredValue, BackgroundAttachment,
    BackgroundAttachmentList, BackgroundBox, BackgroundRepeat, BackgroundRepeatList,
    BackgroundRepeatStyle, BackgroundSize, BackgroundSizeComponent, BackgroundSizeList,
    BasicShape, Border, BorderLineStyle, BorderRadii, BorderStyles, BoxDecorationBreak, Change,
    ClipPath, Color, ColorComponent, Combinator, ContentVisibility, Context, CornerRadius, CssPx,
    CssWideKeyword, CustomPropertyName, CustomPropertyTypedValue, CustomPropertyValue,
    Declarations, DimensionLength, DurationSeconds, Filter, FilterFunction, FilterFunctionList,
    Flex, FlexFactor, Font, FontFamilyList, FontFeature, FontFeatureSettings, FontFeatureTag,
    FontFeatureValue, FontStretch, FontVariant, FontWeight, GridTrackList,
    HorizontalPositionKeyword, ImageLayer, ImageLayerList, LayerOrder, LayoutPosition, Length,
    LetterSpacing, LetterSpacingLength, MaskLayer, MaskLayerList, Node, NthPattern, NthSelector,
    Opacity, Order, Outline, OutlineStyle, OutlineWidth, OutlineWidthLength, OverflowWrap,
    PlaceContentAlignment, PlaceItemsAlignment, Position, PositionComponent, PositionList,
    Property, PseudoClassSelector, PseudoElement, RangeState, RelativeSelector,
    RelativeSelectorList, Rotate, RulePrecedence, RuleTarget, RuntimePseudoClass, Scale,
    ScaleValues, ScrollbarWidth, Selector, SelectorList, SelectorListPseudoClass,
    SelectorSpecificity, SelectorFactChange, Sheet, SourceOrder, StateFlag, StructuralSelector,
    StyleAttributeValue, StyleBucket, StyleBucketPolicy, StyleColor, StyleRole, StyleState,
    StyleTag, StyleUrl, SymbolicFunctionValue, TextAlignLast, TextDecoration, TextDecorationLine,
    TextDecorationLineComponent, TextDecorationStyle, TextDecorationThickness,
    TextDecorationThicknessLength, TextIndent, TextOverflow, TextSlant, TextTransform, TextWrap,
    Translate, TranslateValues, Traversal, Tree, TypedDeclaration, Value, VariableDependentValue,
    VariableExpression, VariableFallback, VariableReference, VerticalAlign, VerticalAlignLength,
    VerticalPositionKeyword, WhiteSpace, WordBreak, ZIndex,
};

fn main() -> surgeist_style::Result<()> {
    let width = TypedDeclaration::width(DimensionLength::px(CssPx::new(120.0)?)?);
    let opacity = TypedDeclaration::opacity(Opacity::new(0.75)?);
    let color = TypedDeclaration::try_text_color(StyleColor::rgba(Color::try_rgba(
        0.0, 0.0, 0.0, 1.0,
    )?))?;

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

    let font_features = FontFeatureSettings::features([FontFeature::new(
        FontFeatureTag::new("kern")?,
        Some(FontFeatureValue::On),
    )])?;
    let font = Font::try_new(
        Some(TextSlant::Italic),
        Some(FontVariant::SmallCaps),
        Some(FontWeight::Bold),
        Some(FontStretch::Expanded),
        Length::Px(16.0),
        Some(Length::Percent(120.0)),
        FontFamilyList::new(["Inter", "serif"])?,
    )?;
    let declarations = Declarations::new()
        .try_font_family(FontFamilyList::new(["Inter", "serif"])?)?
        .try_font_size(Length::Px(16.0))?
        .try_line_height(Length::Percent(120.0))?
        .font_weight(FontWeight::Bold)
        .try_font_style(TextSlant::Italic)?
        .font_stretch(FontStretch::Expanded)
        .font_variant(FontVariant::SmallCaps)
        .try_font_feature_settings(font_features)?
        .try_font(font)?;
    assert_eq!(declarations.len(), 8);

    let text_indent = TextIndent::new(Length::Percent(10.0), true, false)?;
    assert_eq!(text_indent.length(), &Length::Percent(10.0));
    assert!(text_indent.hanging());
    assert!(!text_indent.each_line());
    let vertical_align_length = VerticalAlignLength::new(Length::Px(-1.0))?;
    assert_eq!(vertical_align_length.length(), &Length::Px(-1.0));
    let letter_spacing_length = LetterSpacingLength::new(Length::Px(0.5))?;
    assert_eq!(letter_spacing_length.length(), &Length::Px(0.5));
    let declarations = Declarations::new()
        .text_align_last(TextAlignLast::End)
        .try_text_indent(text_indent)?
        .try_vertical_align(VerticalAlign::Length(vertical_align_length))?
        .try_letter_spacing(LetterSpacing::Length(letter_spacing_length))?
        .text_transform(TextTransform::Lowercase);
    assert_eq!(declarations.len(), 5);

    let declarations = Declarations::new()
        .text_wrap(TextWrap::Balance)
        .white_space(WhiteSpace::BreakSpaces)
        .word_break(WordBreak::BreakWord)
        .overflow_wrap(OverflowWrap::Anywhere)
        .text_overflow(TextOverflow::Ellipsis);
    assert_eq!(declarations.len(), 5);

    let text_decoration_line = TextDecorationLine::try_new([
        TextDecorationLineComponent::Underline,
        TextDecorationLineComponent::Overline,
    ])?;
    assert!(!text_decoration_line.is_none());
    assert_eq!(text_decoration_line.components().len(), 2);
    assert!(TextDecorationLine::none().components().is_empty());
    let text_decoration_thickness_length =
        TextDecorationThicknessLength::new(Length::Px(2.0))?;
    assert_eq!(
        text_decoration_thickness_length.length(),
        &Length::Px(2.0)
    );
    let text_decoration_thickness =
        TextDecorationThickness::Length(text_decoration_thickness_length);
    let text_decoration = TextDecoration::try_new(
        Some(text_decoration_line.clone()),
        None,
        Some(TextDecorationStyle::Double),
        Some(text_decoration_thickness.clone()),
    )?;
    assert!(text_decoration.line().is_some());
    assert_eq!(text_decoration.style(), Some(TextDecorationStyle::Double));
    assert!(text_decoration.thickness().is_some());
    let declarations = Declarations::new()
        .try_text_decoration(text_decoration)?
        .try_text_decoration_line(text_decoration_line)?
        .text_decoration_style(TextDecorationStyle::Wavy)
        .try_text_decoration_thickness(text_decoration_thickness)?;
    assert_eq!(declarations.len(), 4);

    let filter_functions =
        FilterFunctionList::try_new([FilterFunction::Blur(SymbolicFunctionValue::new("4px")?)])?;
    assert_eq!(filter_functions.functions().len(), 1);
    let clip_path = ClipPath::BasicShape(BasicShape::Circle(SymbolicFunctionValue::new("50%")?));
    let translate_values = TranslateValues::try_new([Length::Px(10.0), Length::Percent(5.0)])?;
    assert_eq!(translate_values.values().len(), 2);
    let scale_values = ScaleValues::try_new([1.0, 2.0])?;
    assert_eq!(scale_values.values().len(), 2);
    let declarations = Declarations::new()
        .box_decoration_break(BoxDecorationBreak::Clone)
        .filter(Filter::Functions(filter_functions))
        .backdrop_filter(Filter::None)
        .clip_path(clip_path)
        .translate(Translate::try_values([Length::Px(10.0)])?)
        .translate(Translate::Values(translate_values))
        .rotate(Rotate::Value(SymbolicFunctionValue::new("45deg")?))
        .scale(Scale::try_values([1.0, 2.0])?)
        .scale(Scale::Values(scale_values));
    assert_eq!(declarations.len(), 7);

    let alpha = Alpha::new(0.5)?;
    let color = StyleColor::Hsl {
        hue: ColorComponent::new(Some(210.0))?,
        saturation: ColorComponent::new(Some(60.0))?,
        lightness: ColorComponent::new(Some(40.0))?,
        alpha: Some(alpha),
    };
    let declarations = Declarations::new()
        .try_text_color(color.clone())?
        .try_text_decoration_color(StyleColor::current_color())?;
    assert_eq!(declarations.len(), 2);

    let hero_url = StyleUrl::new("hero.png")?;
    assert_eq!(hero_url.as_str(), "hero.png");
    let image_layers = ImageLayerList::try_new([ImageLayer::url(hero_url)])?;
    assert_eq!(image_layers.layers().len(), 1);
    assert!(ImageLayerList::try_new([]).is_err());
    let position = Position::try_new([
        PositionComponent::Horizontal(HorizontalPositionKeyword::Left),
        PositionComponent::Length(Length::Percent(25.0)),
    ])?;
    assert_eq!(position.components().len(), 2);
    assert_eq!(
        Position::origin().components(),
        &[
            PositionComponent::Length(Length::Percent(0.0)),
            PositionComponent::Length(Length::Percent(0.0)),
        ]
    );
    assert!(Position::try_new([]).is_err());
    assert!(
        Position::try_new([
            PositionComponent::Horizontal(HorizontalPositionKeyword::Left),
            PositionComponent::Horizontal(HorizontalPositionKeyword::Right),
        ])
        .is_err()
    );
    assert!(
        Position::try_new([
            PositionComponent::Vertical(VerticalPositionKeyword::Top),
            PositionComponent::Vertical(VerticalPositionKeyword::Bottom),
        ])
        .is_err()
    );
    let position_layers = PositionList::try_new([position.clone()])?;
    let size = BackgroundSize::Explicit {
        width: BackgroundSizeComponent::Length(Length::Percent(100.0)),
        height: Some(BackgroundSizeComponent::Auto),
    };
    let size_layers = BackgroundSizeList::try_new([size.clone()])?;
    let repeat = BackgroundRepeat::Axes {
        x: BackgroundRepeatStyle::NoRepeat,
        y: BackgroundRepeatStyle::NoRepeat,
    };
    let repeat_layers = BackgroundRepeatList::try_new([repeat])?;
    let attachments = BackgroundAttachmentList::try_new([BackgroundAttachment::Fixed])?;
    let mask_layer = MaskLayer::try_new(
        Some(ImageLayer::None),
        Some(Position::try_new([PositionComponent::Vertical(
            VerticalPositionKeyword::Top,
        )])?),
        Some(BackgroundSize::Contain),
        Some(BackgroundRepeat::RepeatX),
    )?;
    assert!(MaskLayer::try_new(None, None, None, None).is_err());
    let mask_layers = MaskLayerList::try_new([mask_layer])?;
    let declarations = Declarations::new()
        .background_image(image_layers.clone())
        .background_position(position_layers.clone())
        .background_size(size_layers.clone())
        .background_repeat(repeat_layers.clone())
        .background_origin(BackgroundBox::PaddingBox)
        .background_clip(BackgroundBox::ContentBox)
        .background_attachment(attachments)
        .mask(mask_layers)?
        .mask_image(image_layers)
        .mask_position(position_layers)
        .mask_size(size_layers)
        .mask_repeat(repeat_layers);
    assert_eq!(declarations.get(Property::Mask), None);
    assert!(matches!(
        declarations.get(Property::BackgroundImage),
        Some(Value::ImageLayerList(_))
    ));
    assert!(matches!(
        declarations.get(Property::MaskPosition),
        Some(Value::PositionList(_))
    ));

    let declarations = Declarations::new()
        .try_inset_top(Length::Auto)?
        .try_margin_left(Length::Px(-4.0))?
        .try_padding_right(Length::Px(8.0))?
        .try_border_bottom_width(Length::Px(2.0))?;
    assert_eq!(declarations.len(), 4);

    let border = Border::try_new(
        Some(Length::Px(2.0)),
        Some(BorderLineStyle::Solid),
        Some(StyleColor::current_color()),
    )?;
    let border_styles = BorderStyles::new(
        BorderLineStyle::Solid,
        BorderLineStyle::Dashed,
        BorderLineStyle::Dotted,
        BorderLineStyle::Double,
    );
    let corner = CornerRadius::new(Length::Px(4.0), Length::Percent(50.0))?;
    let radii = BorderRadii::all(corner.clone());
    let outline = Outline::try_new(
        Some(OutlineWidth::Length(OutlineWidthLength::new(
            Length::Px(3.0),
        )?)),
        Some(OutlineStyle::Border(BorderLineStyle::Dotted)),
        Some(StyleColor::current_color()),
    )?;
    let declarations = Declarations::new()
        .try_border(border)?
        .border_style(border_styles)
        .try_border_color(StyleColor::current_color())?
        .try_border_top_left_radius(corner)?
        .try_border_radius(radii)?
        .try_outline(outline)?;
    assert_eq!(declarations.len(), 19);

    let declarations = Declarations::new()
        .position(LayoutPosition::Fixed)
        .z_index(ZIndex::integer(-1))
        .scrollbar_width(ScrollbarWidth::Thin)
        .content_visibility(ContentVisibility::Auto)
        .order(Order::new(2))
        .try_flex_grow(FlexFactor::new(2.0)?)?
        .try_flex_shrink(FlexFactor::new(0.0)?)?
        .try_aspect_ratio(AspectRatio::ratio(16.0 / 9.0)?)?;
    assert_eq!(declarations.len(), 8);

    let declarations = Declarations::new()
        .try_flex(Flex::components(
            FlexFactor::new(2.0)?,
            None,
            Some(Length::Px(10.0)),
        ))?
        .place_content(PlaceContentAlignment::all(AlignContent::Center))
        .place_items(PlaceItemsAlignment::new(
            AlignItems::Start,
            AlignItems::Stretch,
        ))
        .place_self(PlaceItemsAlignment::all(AlignItems::Center))
        .align_tracks(AlignContent::SpaceBetween)
        .justify_tracks(AlignContent::SpaceAround);
    assert_eq!(declarations.len(), 11);

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
                        VariableExpression::Value(Value::StyleColor(StyleColor::rgba(Color::BLACK))),
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
                VariableExpression::Value(Value::StyleColor(StyleColor::rgba(Color::BLACK))),
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
        AuthoredValue::Value(Value::StyleColor(StyleColor::rgba(Color::BLACK))),
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
