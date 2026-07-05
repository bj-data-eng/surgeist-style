use super::{
    AlignContent, AlignItems, AnimationNameList, AspectRatio, BoxSizing, CalcLength, CalcOperator,
    Clear, Color, ContentVisibility, Corners, Direction, Edges, Error, ErrorCode, Flex,
    FlexDirection, FlexFactor, FlexWrap, Float, Font, FontFamilyList, FontFeatureSettings,
    FontStretch, FontVariant, FontWeight, GridFlowTolerance, LayoutPosition, Length, LetterSpacing,
    Order, Overflow, OverflowWrap, PlaceContentAlignment, PlaceItemsAlignment, Result,
    ScrollbarWidth, StyleColor, StyleTextAlign, TextAlignLast, TextDecoration, TextDecorationLine,
    TextDecorationStyle, TextDecorationThickness, TextIndent, TextOverflow, TextSlant,
    TextTransform, TextWrap, Value, VerticalAlign, Visibility, WhiteSpace, WordBreak, WritingMode,
    ZIndex,
    value::{validate_font_size_length, validate_line_height_length},
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Property {
    Display,
    BoxSizing,
    Position,
    Inset,
    Top,
    Right,
    Bottom,
    Left,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MinSize,
    MaxWidth,
    MaxHeight,
    MaxSize,
    AspectRatio,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Overflow,
    OverflowX,
    OverflowY,
    ScrollbarWidth,
    ContentVisibility,
    ZIndex,
    Direction,
    WritingMode,
    TextAlign,
    TextAlignLast,
    TextIndent,
    VerticalAlign,
    LetterSpacing,
    TextTransform,
    Float,
    Clear,
    FlexDirection,
    FlexWrap,
    Flex,
    Order,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Align,
    AlignItems,
    AlignSelf,
    AlignContent,
    PlaceContent,
    Justify,
    JustifyItems,
    JustifySelf,
    JustifyContent,
    PlaceItems,
    PlaceSelf,
    JustifyTracks,
    AlignTracks,
    Gap,
    RowGap,
    ColumnGap,
    GridTemplateRows,
    GridTemplateColumns,
    GridTemplateAreas,
    GridTemplate,
    GridAutoRows,
    GridAutoColumns,
    GridAutoFlow,
    GridFlowTolerance,
    GridRowStart,
    GridRowEnd,
    GridColumnStart,
    GridColumnEnd,
    GridRow,
    GridColumn,
    GridArea,
    Grid,
    Background,
    Foreground,
    Color,
    BorderColor,
    BorderWidth,
    BorderTopWidth,
    BorderRightWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderStyle,
    Radius,
    Shadow,
    Opacity,
    Visibility,
    Font,
    FontFamily,
    FontSize,
    FontWeight,
    FontStyle,
    FontStretch,
    FontVariant,
    FontFeatureSettings,
    LineHeight,
    TextWrap,
    WhiteSpace,
    WordBreak,
    OverflowWrap,
    TextOverflow,
    TextDecoration,
    TextDecorationLine,
    TextDecorationColor,
    TextDecorationStyle,
    TextDecorationThickness,
    SelectionColor,
    Cursor,
    PointerEvents,
    FocusOutline,
    SelectionPaint,
    Transform,
    TransformOrigin,
    Filter,
    TransitionProperty,
    TransitionDuration,
    TransitionDelay,
    TransitionTiming,
    AnimationName,
}

impl Property {
    pub const ALL: &'static [Self] = &[
        Self::Display,
        Self::BoxSizing,
        Self::Position,
        Self::Inset,
        Self::Top,
        Self::Right,
        Self::Bottom,
        Self::Left,
        Self::Width,
        Self::Height,
        Self::MinWidth,
        Self::MinHeight,
        Self::MinSize,
        Self::MaxWidth,
        Self::MaxHeight,
        Self::MaxSize,
        Self::AspectRatio,
        Self::Margin,
        Self::MarginTop,
        Self::MarginRight,
        Self::MarginBottom,
        Self::MarginLeft,
        Self::Padding,
        Self::PaddingTop,
        Self::PaddingRight,
        Self::PaddingBottom,
        Self::PaddingLeft,
        Self::Overflow,
        Self::OverflowX,
        Self::OverflowY,
        Self::ScrollbarWidth,
        Self::ContentVisibility,
        Self::ZIndex,
        Self::Direction,
        Self::WritingMode,
        Self::TextAlign,
        Self::TextAlignLast,
        Self::TextIndent,
        Self::VerticalAlign,
        Self::LetterSpacing,
        Self::TextTransform,
        Self::Float,
        Self::Clear,
        Self::FlexDirection,
        Self::FlexWrap,
        Self::Flex,
        Self::Order,
        Self::FlexGrow,
        Self::FlexShrink,
        Self::FlexBasis,
        Self::Align,
        Self::AlignItems,
        Self::AlignSelf,
        Self::AlignContent,
        Self::PlaceContent,
        Self::Justify,
        Self::JustifyItems,
        Self::JustifySelf,
        Self::JustifyContent,
        Self::PlaceItems,
        Self::PlaceSelf,
        Self::JustifyTracks,
        Self::AlignTracks,
        Self::Gap,
        Self::RowGap,
        Self::ColumnGap,
        Self::GridTemplateRows,
        Self::GridTemplateColumns,
        Self::GridTemplateAreas,
        Self::GridTemplate,
        Self::GridAutoRows,
        Self::GridAutoColumns,
        Self::GridAutoFlow,
        Self::GridFlowTolerance,
        Self::GridRowStart,
        Self::GridRowEnd,
        Self::GridColumnStart,
        Self::GridColumnEnd,
        Self::GridRow,
        Self::GridColumn,
        Self::GridArea,
        Self::Grid,
        Self::Background,
        Self::Foreground,
        Self::Color,
        Self::BorderColor,
        Self::BorderWidth,
        Self::BorderTopWidth,
        Self::BorderRightWidth,
        Self::BorderBottomWidth,
        Self::BorderLeftWidth,
        Self::BorderStyle,
        Self::Radius,
        Self::Shadow,
        Self::Opacity,
        Self::Visibility,
        Self::Font,
        Self::FontFamily,
        Self::FontSize,
        Self::FontWeight,
        Self::FontStyle,
        Self::FontStretch,
        Self::FontVariant,
        Self::FontFeatureSettings,
        Self::LineHeight,
        Self::TextWrap,
        Self::WhiteSpace,
        Self::WordBreak,
        Self::OverflowWrap,
        Self::TextOverflow,
        Self::TextDecoration,
        Self::TextDecorationLine,
        Self::TextDecorationColor,
        Self::TextDecorationStyle,
        Self::TextDecorationThickness,
        Self::SelectionColor,
        Self::Cursor,
        Self::PointerEvents,
        Self::FocusOutline,
        Self::SelectionPaint,
        Self::Transform,
        Self::TransformOrigin,
        Self::Filter,
        Self::TransitionProperty,
        Self::TransitionDuration,
        Self::TransitionDelay,
        Self::TransitionTiming,
        Self::AnimationName,
    ];

    #[must_use]
    pub const fn is_canonical(self) -> bool {
        !matches!(
            self,
            Self::Inset
                | Self::Margin
                | Self::Padding
                | Self::BorderWidth
                | Self::Gap
                | Self::MinSize
                | Self::MaxSize
                | Self::Overflow
                | Self::Align
                | Self::Justify
                | Self::PlaceContent
                | Self::PlaceItems
                | Self::PlaceSelf
                | Self::Flex
                | Self::Font
                | Self::TextDecoration
                | Self::GridTemplate
                | Self::Grid
                | Self::GridRow
                | Self::GridColumn
                | Self::GridArea
        )
    }

    #[must_use]
    pub fn metadata(self) -> Metadata {
        match self {
            Self::Color => Metadata::new(Value::StyleColor(StyleColor::rgba(Color::BLACK)))
                .inherited(true)
                .impact(Impact::empty().text().paint())
                .interpolation(Interpolation::Color)
                .animatable(true),
            Self::Background => {
                Metadata::new(Value::StyleColor(StyleColor::rgba(Color::TRANSPARENT)))
                    .impact(Impact::empty().paint())
                    .interpolation(Interpolation::Color)
                    .animatable(true)
            }
            Self::Foreground => Metadata::new(Value::Color(Color::BLACK))
                .impact(Impact::empty().paint())
                .interpolation(Interpolation::Color)
                .animatable(true),
            Self::Padding => Metadata::new(Value::Edges(Edges::default()))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Edges),
            Self::PaddingTop | Self::PaddingRight | Self::PaddingBottom | Self::PaddingLeft => {
                Metadata::new(Value::Length(Length::ZERO))
                    .impact(Impact::empty().layout())
                    .interpolation(Interpolation::Length)
            }
            Self::Margin => Metadata::new(Value::Edges(Edges::default()))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Edges),
            Self::MarginTop | Self::MarginRight | Self::MarginBottom | Self::MarginLeft => {
                Metadata::new(Value::Length(Length::ZERO))
                    .impact(Impact::empty().layout())
                    .interpolation(Interpolation::Length)
            }
            Self::Inset => Metadata::new(Value::Edges(Edges::all(Length::Auto)))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Edges),
            Self::Top | Self::Right | Self::Bottom | Self::Left => {
                Metadata::new(Value::Length(Length::Auto))
                    .impact(Impact::empty().layout())
                    .interpolation(Interpolation::Length)
            }
            Self::Radius => Metadata::new(Value::Corners(Corners::default()))
                .impact(Impact::empty().paint().effect())
                .interpolation(Interpolation::Corners)
                .animatable(true),
            Self::Shadow => Metadata::new(Value::ShadowList(Vec::new()))
                .impact(Impact::empty().effect().paint())
                .interpolation(Interpolation::ShadowList)
                .animatable(true),
            Self::Opacity => Metadata::new(Value::Number(1.0))
                .impact(Impact::empty().paint())
                .interpolation(Interpolation::Number)
                .animatable(true),
            Self::Visibility => Metadata::new(Value::Visibility(Visibility::Visible))
                .impact(Impact::empty().layout().paint()),
            Self::Width
            | Self::Height
            | Self::MinWidth
            | Self::MinHeight
            | Self::MinSize
            | Self::MaxWidth
            | Self::MaxHeight
            | Self::MaxSize
            | Self::FlexBasis => Metadata::new(Value::Length(Length::Auto))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Length),
            Self::Gap | Self::RowGap | Self::ColumnGap => {
                Metadata::new(Value::Length(Length::NORMAL))
                    .impact(Impact::empty().layout())
                    .interpolation(Interpolation::Length)
            }
            Self::GridTemplateRows
            | Self::GridTemplateColumns
            | Self::GridAutoRows
            | Self::GridAutoColumns => {
                Metadata::new(Value::GridTrackList(super::GridTrackList::default()))
                    .impact(Impact::empty().layout())
            }
            Self::GridTemplateAreas => {
                Metadata::new(Value::GridTemplateAreas(super::GridTemplateAreas::default()))
                    .impact(Impact::empty().layout())
            }
            Self::GridTemplate => {
                Metadata::new(Value::GridTemplate(super::GridTemplate::default()))
                    .impact(Impact::empty().layout())
            }
            Self::Grid => Metadata::new(Value::GridDefinition(super::GridDefinition::default()))
                .impact(Impact::empty().layout()),
            Self::GridRowStart | Self::GridRowEnd | Self::GridColumnStart | Self::GridColumnEnd => {
                Metadata::new(Value::GridLine(super::GridLine::Auto))
                    .impact(Impact::empty().layout())
            }
            Self::GridRow | Self::GridColumn => {
                Metadata::new(Value::GridPlacement(super::GridPlacement::default()))
                    .impact(Impact::empty().layout())
            }
            Self::GridArea => {
                Metadata::new(Value::GridAreaPlacement(super::GridAreaPlacement::default()))
                    .impact(Impact::empty().layout())
            }
            Self::GridAutoFlow => {
                Metadata::new(Value::GridAutoFlow(super::GridAutoFlow::default()))
                    .impact(Impact::empty().layout())
            }
            Self::GridFlowTolerance => {
                Metadata::new(Value::GridFlowTolerance(super::GridFlowTolerance::default()))
                    .impact(Impact::empty().layout())
            }
            Self::Flex => Metadata::new(Value::Flex(Flex::none())).impact(Impact::empty().layout()),
            Self::FlexGrow => Metadata::new(Value::FlexFactor(FlexFactor::zero()))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Number),
            Self::FlexShrink => Metadata::new(Value::FlexFactor(FlexFactor::one()))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Number),
            Self::AspectRatio => Metadata::new(Value::AspectRatio(AspectRatio::default()))
                .impact(Impact::empty().layout())
                .interpolation(Interpolation::Number),
            Self::ScrollbarWidth => Metadata::new(Value::ScrollbarWidth(ScrollbarWidth::default()))
                .impact(Impact::empty().layout()),
            Self::ContentVisibility => {
                Metadata::new(Value::ContentVisibility(ContentVisibility::default()))
                    .impact(Impact::empty().layout().paint())
            }
            Self::Order => {
                Metadata::new(Value::Order(Order::default())).impact(Impact::empty().layout())
            }
            Self::BorderColor => Metadata::new(Value::Color(Color::TRANSPARENT))
                .impact(Impact::empty().paint())
                .interpolation(Interpolation::Color)
                .animatable(true),
            Self::BorderWidth => Metadata::new(Value::Edges(Edges::default()))
                .impact(Impact::empty().layout().paint())
                .interpolation(Interpolation::Edges),
            Self::BorderTopWidth
            | Self::BorderRightWidth
            | Self::BorderBottomWidth
            | Self::BorderLeftWidth => Metadata::new(Value::Length(Length::ZERO))
                .impact(Impact::empty().layout().paint())
                .interpolation(Interpolation::Length),
            Self::Font => Metadata::new(Value::Font(
                Font::try_new(
                    None,
                    None,
                    None,
                    None,
                    Length::Px(16.0),
                    None,
                    FontFamilyList::new(["serif"]).unwrap(),
                )
                .unwrap(),
            ))
            .inherited(true)
            .impact(Impact::empty().text().layout()),
            Self::FontSize | Self::LineHeight => Metadata::new(Value::Length(Length::Px(16.0)))
                .inherited(true)
                .impact(Impact::empty().text().layout())
                .interpolation(Interpolation::Length),
            Self::FontFamily => Metadata::new(Value::FontFamilyList(FontFamilyList::empty()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::FontWeight => Metadata::new(Value::FontWeight(FontWeight::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout())
                .interpolation(Interpolation::Number),
            Self::FontStyle => Metadata::new(Value::TextSlant(TextSlant::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::FontStretch => Metadata::new(Value::FontStretch(FontStretch::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::FontVariant => Metadata::new(Value::FontVariant(FontVariant::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::FontFeatureSettings => {
                Metadata::new(Value::FontFeatureSettings(FontFeatureSettings::default()))
                    .inherited(true)
                    .impact(Impact::empty().text().layout())
            }
            Self::TextWrap => Metadata::new(Value::TextWrap(TextWrap::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::WhiteSpace => Metadata::new(Value::WhiteSpace(WhiteSpace::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::WordBreak => Metadata::new(Value::WordBreak(WordBreak::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::OverflowWrap => Metadata::new(Value::OverflowWrap(OverflowWrap::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::TextOverflow => Metadata::new(Value::TextOverflow(TextOverflow::default()))
                .impact(Impact::empty().text().layout()),
            Self::TextDecoration => Metadata::new(Value::TextDecoration(
                TextDecoration::try_new(Some(TextDecorationLine::default()), None, None, None)
                    .unwrap(),
            ))
            .impact(Impact::empty().text().layout()),
            Self::TextDecorationLine => {
                Metadata::new(Value::TextDecorationLine(TextDecorationLine::default()))
                    .impact(Impact::empty().text().layout())
            }
            Self::TextDecorationColor => {
                Metadata::new(Value::StyleColor(StyleColor::current_color()))
                    .impact(Impact::empty().paint().text())
                    .interpolation(Interpolation::Color)
            }
            Self::TextDecorationStyle => {
                Metadata::new(Value::TextDecorationStyle(TextDecorationStyle::default()))
                    .impact(Impact::empty().text().layout())
            }
            Self::TextDecorationThickness => Metadata::new(Value::TextDecorationThickness(
                TextDecorationThickness::default(),
            ))
            .impact(Impact::empty().text().layout())
            .interpolation(Interpolation::Length),
            Self::Transform => Metadata::new(Value::Transform(super::Transform::default()))
                .impact(Impact::empty().paint())
                .interpolation(Interpolation::Transform)
                .animatable(true),
            Self::TransformOrigin => Metadata::new(Value::Size(super::Size::new(
                Length::Percent(50.0),
                Length::Percent(50.0),
            )))
            .impact(Impact::empty().paint())
            .interpolation(Interpolation::Length),
            Self::Cursor => {
                Metadata::new(Value::Cursor(super::Cursor::Default)).impact(Impact::empty().paint())
            }
            Self::PointerEvents => Metadata::new(Value::PointerEvents(super::PointerEvents::Auto)),
            Self::FocusOutline | Self::SelectionPaint => Metadata::new(Value::Stroke(
                super::Stroke::new(Length::ZERO, Color::TRANSPARENT),
            ))
            .impact(Impact::empty().paint())
            .interpolation(Interpolation::Stroke),
            Self::SelectionColor => Metadata::new(Value::Color(Color::TRANSPARENT))
                .impact(Impact::empty().paint())
                .interpolation(Interpolation::Color),
            Self::TransitionProperty => {
                Metadata::new(Value::PropertyList(Vec::new())).impact(Impact::empty().animation())
            }
            Self::TransitionDuration | Self::TransitionDelay => Metadata::new(Value::Number(0.0))
                .impact(Impact::empty().animation())
                .interpolation(Interpolation::Number),
            Self::TransitionTiming => Metadata::new(Value::Keyword(super::value::Keyword::Initial))
                .impact(Impact::empty().animation()),
            Self::AnimationName => {
                Metadata::new(Value::AnimationNameList(AnimationNameList::empty()))
                    .impact(Impact::empty().animation())
            }
            Self::Display => Metadata::new(Value::Display(super::Display::default()))
                .impact(Impact::empty().layout().paint()),
            Self::BoxSizing => Metadata::new(Value::BoxSizing(BoxSizing::default()))
                .impact(Impact::empty().layout()),
            Self::Position => Metadata::new(Value::Position(LayoutPosition::default()))
                .impact(Impact::empty().layout()),
            Self::Overflow | Self::OverflowX | Self::OverflowY => {
                Metadata::new(Value::Overflow(Overflow::default())).impact(Impact::empty().layout())
            }
            Self::Direction => Metadata::new(Value::Direction(Direction::default()))
                .inherited(true)
                .impact(Impact::empty().layout()),
            Self::WritingMode => Metadata::new(Value::WritingMode(WritingMode::default()))
                .inherited(true)
                .impact(Impact::empty().layout()),
            Self::TextAlign => Metadata::new(Value::TextAlign(StyleTextAlign::default()))
                .inherited(true)
                .impact(Impact::empty().layout()),
            Self::TextAlignLast => Metadata::new(Value::TextAlignLast(TextAlignLast::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::TextIndent => Metadata::new(Value::TextIndent(TextIndent::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout())
                .interpolation(Interpolation::Length),
            Self::VerticalAlign => Metadata::new(Value::VerticalAlign(VerticalAlign::default()))
                .impact(Impact::empty().text().layout()),
            Self::LetterSpacing => Metadata::new(Value::LetterSpacing(LetterSpacing::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout())
                .interpolation(Interpolation::Length),
            Self::TextTransform => Metadata::new(Value::TextTransform(TextTransform::default()))
                .inherited(true)
                .impact(Impact::empty().text().layout()),
            Self::Float => {
                Metadata::new(Value::Float(Float::default())).impact(Impact::empty().layout())
            }
            Self::Clear => {
                Metadata::new(Value::Clear(Clear::default())).impact(Impact::empty().layout())
            }
            Self::FlexDirection => Metadata::new(Value::FlexDirection(FlexDirection::default()))
                .impact(Impact::empty().layout()),
            Self::FlexWrap => {
                Metadata::new(Value::FlexWrap(FlexWrap::default())).impact(Impact::empty().layout())
            }
            Self::Align | Self::AlignItems | Self::AlignSelf => {
                Metadata::new(Value::AlignItems(AlignItems::default()))
                    .impact(Impact::empty().layout())
            }
            Self::AlignContent => Metadata::new(Value::AlignContent(AlignContent::default()))
                .impact(Impact::empty().layout()),
            Self::PlaceContent => Metadata::new(Value::PlaceContentAlignment(
                PlaceContentAlignment::all(AlignContent::default()),
            ))
            .impact(Impact::empty().layout()),
            Self::Justify | Self::JustifyItems | Self::JustifySelf => {
                Metadata::new(Value::AlignItems(AlignItems::default()))
                    .impact(Impact::empty().layout())
            }
            Self::JustifyContent => Metadata::new(Value::AlignContent(AlignContent::default()))
                .impact(Impact::empty().layout()),
            Self::PlaceItems | Self::PlaceSelf => Metadata::new(Value::PlaceItemsAlignment(
                PlaceItemsAlignment::all(AlignItems::default()),
            ))
            .impact(Impact::empty().layout()),
            Self::AlignTracks | Self::JustifyTracks => {
                Metadata::new(Value::AlignContent(AlignContent::default()))
                    .impact(Impact::empty().layout())
            }
            Self::ZIndex => Metadata::new(Value::ZIndex(ZIndex::default()))
                .impact(Impact::empty().layout().paint()),
            Self::BorderStyle | Self::Filter => {
                Metadata::new(Value::Keyword(super::value::Keyword::Initial))
                    .impact(Impact::empty().layout().paint())
            }
        }
    }

    pub fn validate_value(self, value: &Value) -> Result<()> {
        value.validate()?;
        if !self.accepts(value) {
            return Err(Error::new(
                ErrorCode::InvalidProperty,
                format!("{self:?} does not accept {}", value_kind(value)),
            ));
        }
        self.validate_domain(value)
    }

    fn accepts(self, value: &Value) -> bool {
        if matches!(value, Value::Keyword(_)) {
            return true;
        }
        match self {
            Self::BorderStyle | Self::Filter | Self::TransitionTiming => false,
            Self::Display => matches!(value, Value::Display(_)),
            Self::BoxSizing => matches!(value, Value::BoxSizing(_)),
            Self::Position => matches!(value, Value::Position(_)),
            Self::Overflow => matches!(value, Value::Overflow(_) | Value::OverflowAxes(_)),
            Self::OverflowX | Self::OverflowY => matches!(value, Value::Overflow(_)),
            Self::Direction => matches!(value, Value::Direction(_)),
            Self::WritingMode => matches!(value, Value::WritingMode(_)),
            Self::TextAlign => matches!(value, Value::TextAlign(_)),
            Self::TextAlignLast => matches!(value, Value::TextAlignLast(_)),
            Self::TextIndent => matches!(value, Value::TextIndent(_)),
            Self::VerticalAlign => matches!(value, Value::VerticalAlign(_)),
            Self::LetterSpacing => matches!(value, Value::LetterSpacing(_)),
            Self::TextTransform => matches!(value, Value::TextTransform(_)),
            Self::TextDecoration => matches!(value, Value::TextDecoration(_)),
            Self::TextDecorationLine => matches!(value, Value::TextDecorationLine(_)),
            Self::TextDecorationColor => matches!(value, Value::StyleColor(_)),
            Self::TextDecorationStyle => matches!(value, Value::TextDecorationStyle(_)),
            Self::TextDecorationThickness => matches!(value, Value::TextDecorationThickness(_)),
            Self::TextWrap => matches!(value, Value::TextWrap(_)),
            Self::WhiteSpace => matches!(value, Value::WhiteSpace(_)),
            Self::WordBreak => matches!(value, Value::WordBreak(_)),
            Self::OverflowWrap => matches!(value, Value::OverflowWrap(_)),
            Self::TextOverflow => matches!(value, Value::TextOverflow(_)),
            Self::Float => matches!(value, Value::Float(_)),
            Self::Clear => matches!(value, Value::Clear(_)),
            Self::FlexDirection => matches!(value, Value::FlexDirection(_)),
            Self::FlexWrap => matches!(value, Value::FlexWrap(_)),
            Self::Order => matches!(value, Value::Order(_)),
            Self::Flex => matches!(value, Value::Flex(_)),
            Self::FlexGrow | Self::FlexShrink => matches!(value, Value::FlexFactor(_)),
            Self::Align | Self::AlignItems | Self::AlignSelf => {
                matches!(value, Value::AlignItems(_))
            }
            Self::AlignContent => matches!(value, Value::AlignContent(_)),
            Self::PlaceContent => matches!(value, Value::PlaceContentAlignment(_)),
            Self::Justify | Self::JustifyItems | Self::JustifySelf => {
                matches!(value, Value::AlignItems(_))
            }
            Self::JustifyContent => matches!(value, Value::AlignContent(_)),
            Self::PlaceItems | Self::PlaceSelf => matches!(value, Value::PlaceItemsAlignment(_)),
            Self::AlignTracks | Self::JustifyTracks => matches!(value, Value::AlignContent(_)),
            Self::Inset | Self::Margin | Self::Padding | Self::BorderWidth => {
                matches!(value, Value::Edges(_))
            }
            Self::Top
            | Self::Right
            | Self::Bottom
            | Self::Left
            | Self::MarginTop
            | Self::MarginRight
            | Self::MarginBottom
            | Self::MarginLeft
            | Self::PaddingTop
            | Self::PaddingRight
            | Self::PaddingBottom
            | Self::PaddingLeft
            | Self::BorderTopWidth
            | Self::BorderRightWidth
            | Self::BorderBottomWidth
            | Self::BorderLeftWidth => matches!(value, Value::Length(_)),
            Self::Width
            | Self::Height
            | Self::MinWidth
            | Self::MinHeight
            | Self::MinSize
            | Self::MaxWidth
            | Self::MaxHeight
            | Self::MaxSize
            | Self::FlexBasis
            | Self::Gap
            | Self::RowGap
            | Self::ColumnGap
            | Self::FontSize
            | Self::LineHeight => matches!(value, Value::Length(_)),
            Self::GridTemplateRows
            | Self::GridTemplateColumns
            | Self::GridAutoRows
            | Self::GridAutoColumns => matches!(value, Value::GridTrackList(_)),
            Self::GridTemplateAreas => matches!(value, Value::GridTemplateAreas(_)),
            Self::GridTemplate => matches!(value, Value::GridTemplate(_)),
            Self::Grid => matches!(value, Value::GridDefinition(_)),
            Self::GridRowStart | Self::GridRowEnd | Self::GridColumnStart | Self::GridColumnEnd => {
                matches!(value, Value::GridLine(_))
            }
            Self::GridRow | Self::GridColumn => matches!(value, Value::GridPlacement(_)),
            Self::GridArea => matches!(value, Value::GridAreaPlacement(_)),
            Self::ZIndex => matches!(value, Value::ZIndex(_)),
            Self::ScrollbarWidth => matches!(value, Value::ScrollbarWidth(_)),
            Self::ContentVisibility => matches!(value, Value::ContentVisibility(_)),
            Self::AspectRatio => matches!(value, Value::AspectRatio(_)),
            Self::Opacity | Self::TransitionDuration | Self::TransitionDelay => {
                matches!(value, Value::Number(_))
            }
            Self::Background | Self::Color => matches!(value, Value::StyleColor(_)),
            Self::Foreground | Self::BorderColor | Self::SelectionColor => {
                matches!(value, Value::Color(_))
            }
            Self::Radius => matches!(value, Value::Corners(_)),
            Self::Shadow => matches!(value, Value::ShadowList(_)),
            Self::Visibility => matches!(value, Value::Visibility(_)),
            Self::Font => matches!(value, Value::Font(_)),
            Self::FontFamily => matches!(value, Value::FontFamilyList(_)),
            Self::FontWeight => matches!(value, Value::FontWeight(_)),
            Self::FontStyle => matches!(value, Value::TextSlant(_)),
            Self::FontStretch => matches!(value, Value::FontStretch(_)),
            Self::FontVariant => matches!(value, Value::FontVariant(_)),
            Self::FontFeatureSettings => matches!(value, Value::FontFeatureSettings(_)),
            Self::AnimationName => matches!(value, Value::AnimationNameList(_)),
            Self::Cursor => matches!(value, Value::Cursor(_)),
            Self::PointerEvents => matches!(value, Value::PointerEvents(_)),
            Self::FocusOutline | Self::SelectionPaint => matches!(value, Value::Stroke(_)),
            Self::Transform => matches!(value, Value::Transform(_)),
            Self::TransformOrigin => matches!(value, Value::Size(_)),
            Self::TransitionProperty => matches!(value, Value::PropertyList(_)),
            Self::GridAutoFlow => matches!(value, Value::GridAutoFlow(_)),
            Self::GridFlowTolerance => matches!(value, Value::GridFlowTolerance(_)),
        }
    }

    fn validate_domain(self, value: &Value) -> Result<()> {
        match (self, value) {
            (
                Self::Width
                | Self::Height
                | Self::MinSize
                | Self::MinWidth
                | Self::MinHeight
                | Self::MaxSize
                | Self::MaxWidth
                | Self::MaxHeight
                | Self::FlexBasis
                | Self::Gap
                | Self::RowGap
                | Self::ColumnGap,
                Value::Length(length),
            ) => {
                validate_normal_length_scope(length, self)?;
                validate_non_negative_length(length, self)
            }
            (Self::FontSize, Value::Length(length)) => {
                validate_font_size_length(length)?;
                validate_non_negative_length(length, self)
            }
            (Self::LineHeight, Value::Length(length)) => {
                validate_line_height_length(length)?;
                validate_non_negative_length(length, self)
            }
            (Self::Padding | Self::BorderWidth, Value::Edges(edges)) => {
                validate_non_negative_edges(edges, self)
            }
            (
                Self::PaddingTop
                | Self::PaddingRight
                | Self::PaddingBottom
                | Self::PaddingLeft
                | Self::BorderTopWidth
                | Self::BorderRightWidth
                | Self::BorderBottomWidth
                | Self::BorderLeftWidth,
                Value::Length(length),
            ) => {
                validate_normal_length_scope(length, self)?;
                validate_auto_length_scope(length, self)?;
                validate_non_negative_length(length, self)
            }
            (
                Self::MarginTop | Self::MarginRight | Self::MarginBottom | Self::MarginLeft,
                Value::Length(length),
            )
            | (Self::Top | Self::Right | Self::Bottom | Self::Left, Value::Length(length)) => {
                validate_normal_length_scope(length, self)
            }
            (Self::GridTemplateRows | Self::GridTemplateColumns, Value::GridTrackList(value)) => {
                value.validate()
            }
            (Self::GridAutoRows | Self::GridAutoColumns, Value::GridTrackList(value)) => {
                validate_grid_auto_track_list(value, self)
            }
            (Self::GridTemplateAreas, Value::GridTemplateAreas(value)) => value.validate(),
            (Self::GridTemplate, Value::GridTemplate(value)) => value.validate(),
            (Self::Grid, Value::GridDefinition(value)) => value.validate(),
            (Self::GridFlowTolerance, Value::GridFlowTolerance(value)) => {
                validate_grid_flow_tolerance(value, self)
            }
            (
                Self::GridRowStart | Self::GridRowEnd | Self::GridColumnStart | Self::GridColumnEnd,
                Value::GridLine(value),
            ) => value.validate(),
            (Self::GridRow | Self::GridColumn, Value::GridPlacement(value)) => value.validate(),
            (Self::GridArea, Value::GridAreaPlacement(value)) => value.validate(),
            (Self::Radius, Value::Corners(corners)) => {
                validate_non_negative_length(&corners.top_left, self)?;
                validate_non_negative_length(&corners.top_right, self)?;
                validate_non_negative_length(&corners.bottom_right, self)?;
                validate_non_negative_length(&corners.bottom_left, self)
            }
            (Self::Opacity, Value::Number(value)) => validate_unit_number(*value, self),
            (Self::TransitionDuration | Self::TransitionDelay, Value::Number(value)) => {
                validate_non_negative_number(*value, self)
            }
            _ => Ok(()),
        }
    }
}

fn validate_non_negative_edges(edges: &Edges, property: Property) -> Result<()> {
    validate_non_negative_length(&edges.top, property)?;
    validate_non_negative_length(&edges.right, property)?;
    validate_non_negative_length(&edges.bottom, property)?;
    validate_non_negative_length(&edges.left, property)
}

fn validate_normal_length_scope(length: &Length, property: Property) -> Result<()> {
    if matches!(length, Length::Normal)
        && !matches!(
            property,
            Property::Gap | Property::RowGap | Property::ColumnGap
        )
    {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} does not accept normal length"),
        ));
    }
    Ok(())
}

fn validate_auto_length_scope(length: &Length, property: Property) -> Result<()> {
    if matches!(length, Length::Auto)
        && matches!(
            property,
            Property::PaddingTop
                | Property::PaddingRight
                | Property::PaddingBottom
                | Property::PaddingLeft
                | Property::BorderTopWidth
                | Property::BorderRightWidth
                | Property::BorderBottomWidth
                | Property::BorderLeftWidth
        )
    {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} does not accept auto length"),
        ));
    }
    Ok(())
}

fn validate_non_negative_length(length: &Length, property: Property) -> Result<()> {
    match length {
        Length::Px(value) | Length::Percent(value) if *value < 0.0 => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        Length::Calc(calc) if calc_is_definitely_negative(calc) => Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        )),
        _ => Ok(()),
    }
}

fn calc_is_definitely_negative(calc: &CalcLength) -> bool {
    calc_coefficients(calc, 1.0).is_some_and(|coefficients| {
        coefficients.px < 0.0 && coefficients.percent <= 0.0
            || coefficients.px <= 0.0 && coefficients.percent < 0.0
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct CalcCoefficients {
    px: f32,
    percent: f32,
}

fn calc_coefficients(calc: &CalcLength, sign: f32) -> Option<CalcCoefficients> {
    match calc {
        CalcLength::Px(value) => Some(CalcCoefficients {
            px: sign * *value,
            percent: 0.0,
        }),
        CalcLength::Percent(value) => Some(CalcCoefficients {
            px: 0.0,
            percent: sign * *value,
        }),
        CalcLength::Sum(terms) => {
            let mut total = CalcCoefficients::default();
            for term in terms {
                let term_sign = match term.operator {
                    CalcOperator::Add => sign,
                    CalcOperator::Sub => -sign,
                };
                let term = calc_coefficients(&term.value, term_sign)?;
                total.px += term.px;
                total.percent += term.percent;
            }
            Some(total)
        }
    }
}

fn validate_non_negative_number(value: f32, property: Property) -> Result<()> {
    if value < 0.0 {
        Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be non-negative"),
        ))
    } else {
        Ok(())
    }
}

fn validate_grid_flow_tolerance(value: &GridFlowTolerance, property: Property) -> Result<()> {
    value.validate()?;
    match value {
        GridFlowTolerance::Length(Length::Px(value)) => {
            validate_non_negative_number(*value, property)
        }
        GridFlowTolerance::Length(_) => Err(Error::new(
            ErrorCode::InvalidValue,
            "grid flow tolerance length must be a concrete px length",
        )),
        GridFlowTolerance::Percent(value) => validate_non_negative_number(*value, property),
        GridFlowTolerance::Normal | GridFlowTolerance::Infinite => Ok(()),
    }
}

fn validate_grid_auto_track_list(value: &super::GridTrackList, property: Property) -> Result<()> {
    value.validate()?;
    if value.contains_subgrid() {
        return Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} cannot contain subgrid tracks"),
        ));
    }
    Ok(())
}

fn validate_unit_number(value: f32, property: Property) -> Result<()> {
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(Error::new(
            ErrorCode::InvalidValue,
            format!("{property:?} must be between 0 and 1"),
        ))
    }
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Keyword(_) => "keyword",
        Value::Display(_) => "display",
        Value::BoxSizing(_) => "box sizing",
        Value::Position(_) => "position",
        Value::ZIndex(_) => "z-index",
        Value::ScrollbarWidth(_) => "scrollbar width",
        Value::ContentVisibility(_) => "content visibility",
        Value::Order(_) => "order",
        Value::FlexFactor(_) => "flex factor",
        Value::Flex(_) => "flex shorthand",
        Value::AspectRatio(_) => "aspect ratio",
        Value::Direction(_) => "direction",
        Value::Overflow(_) => "overflow",
        Value::OverflowAxes(_) => "overflow axes",
        Value::Float(_) => "float",
        Value::Clear(_) => "clear",
        Value::TextAlign(_) => "text align",
        Value::TextAlignLast(_) => "text align last",
        Value::TextIndent(_) => "text indent",
        Value::VerticalAlign(_) => "vertical align",
        Value::LetterSpacing(_) => "letter spacing",
        Value::TextTransform(_) => "text transform",
        Value::TextDecoration(_) => "text decoration shorthand",
        Value::TextDecorationLine(_) => "text decoration line",
        Value::TextDecorationStyle(_) => "text decoration style",
        Value::TextDecorationThickness(_) => "text decoration thickness",
        Value::TextWrap(_) => "text wrap",
        Value::WhiteSpace(_) => "white space",
        Value::WordBreak(_) => "word break",
        Value::OverflowWrap(_) => "overflow wrap",
        Value::TextOverflow(_) => "text overflow",
        Value::WritingMode(_) => "writing mode",
        Value::FlexDirection(_) => "flex direction",
        Value::FlexWrap(_) => "flex wrap",
        Value::AlignItems(_) => "alignment",
        Value::AlignContent(_) => "content alignment",
        Value::PlaceContentAlignment(_) => "place content alignment",
        Value::PlaceItemsAlignment(_) => "place items alignment",
        Value::Number(_) => "number",
        Value::Length(_) => "length",
        Value::Size(_) => "size",
        Value::Edges(_) => "edges",
        Value::GridTrackList(_) => "grid track list",
        Value::GridTemplateAreas(_) => "grid template areas",
        Value::GridTemplate(_) => "grid template",
        Value::GridDefinition(_) => "grid definition",
        Value::GridLine(_) => "grid line",
        Value::GridPlacement(_) => "grid placement",
        Value::GridAreaPlacement(_) => "grid area placement",
        Value::GridAutoFlow(_) => "grid auto flow",
        Value::GridFlowTolerance(_) => "grid flow tolerance",
        Value::StyleColor(_) => "style color",
        Value::Color(_) => "color",
        Value::Corners(_) => "corners",
        Value::FontFamilyList(_) => "font family list",
        Value::FontWeight(_) => "font weight",
        Value::TextSlant(_) => "font style",
        Value::FontStretch(_) => "font stretch",
        Value::FontVariant(_) => "font variant",
        Value::FontFeatureSettings(_) => "font feature settings",
        Value::Font(_) => "font shorthand",
        Value::AnimationNameList(_) => "animation name list",
        Value::PropertyList(_) => "property list",
        Value::ShadowList(_) => "shadow list",
        Value::Stroke(_) => "stroke",
        Value::Text(_) => "text value",
        Value::Transform(_) => "transform",
        Value::Cursor(_) => "cursor",
        Value::PointerEvents(_) => "pointer events",
        Value::Visibility(_) => "visibility",
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    default: Value,
    inherited: bool,
    impact: Impact,
    animatable: bool,
    interpolation: Interpolation,
}

impl Metadata {
    #[must_use]
    pub(crate) fn new(default: Value) -> Self {
        Self {
            default,
            inherited: false,
            impact: Impact::empty(),
            animatable: false,
            interpolation: Interpolation::Discrete,
        }
    }

    #[must_use]
    pub fn default(&self) -> &Value {
        &self.default
    }

    #[must_use]
    pub const fn is_inherited(&self) -> bool {
        self.inherited
    }

    #[must_use]
    pub const fn impact_flags(&self) -> Impact {
        self.impact
    }

    #[must_use]
    pub const fn is_animatable(&self) -> bool {
        self.animatable
    }

    #[must_use]
    pub const fn interpolation_kind(&self) -> Interpolation {
        self.interpolation
    }

    #[must_use]
    pub(crate) const fn inherited(mut self, inherited: bool) -> Self {
        self.inherited = inherited;
        self
    }

    #[must_use]
    pub(crate) const fn impact(mut self, impact: Impact) -> Self {
        self.impact = impact;
        self
    }

    #[must_use]
    pub(crate) const fn animatable(mut self, animatable: bool) -> Self {
        self.animatable = animatable;
        self
    }

    #[must_use]
    pub(crate) const fn interpolation(mut self, interpolation: Interpolation) -> Self {
        self.interpolation = interpolation;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Interpolation {
    #[default]
    Discrete,
    Number,
    Length,
    Edges,
    Color,
    Corners,
    Stroke,
    ShadowList,
    Transform,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Impact {
    layout: bool,
    paint: bool,
    text: bool,
    effect: bool,
    animation: bool,
}

impl Impact {
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
    pub const fn affects_layout(self) -> bool {
        self.layout
    }

    #[must_use]
    pub const fn affects_paint(self) -> bool {
        self.paint
    }

    #[must_use]
    pub const fn affects_text(self) -> bool {
        self.text
    }

    #[must_use]
    pub const fn affects_effect(self) -> bool {
        self.effect
    }

    #[must_use]
    pub const fn affects_animation(self) -> bool {
        self.animation
    }

    #[must_use]
    pub(crate) const fn layout(mut self) -> Self {
        self.layout = true;
        self
    }

    #[must_use]
    pub(crate) const fn paint(mut self) -> Self {
        self.paint = true;
        self
    }

    #[must_use]
    pub(crate) const fn text(mut self) -> Self {
        self.text = true;
        self
    }

    #[must_use]
    pub(crate) const fn effect(mut self) -> Self {
        self.effect = true;
        self
    }

    #[must_use]
    pub(crate) const fn animation(mut self) -> Self {
        self.animation = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AlignContent, AlignItems, AspectRatio, ContentVisibility, Declarations, FlexFactor,
        LayoutPosition, Order, ScrollbarWidth, ZIndex,
    };

    #[test]
    fn core_layout_properties_accept_typed_values() {
        Property::Position
            .validate_value(&Value::Position(LayoutPosition::Fixed))
            .unwrap();
        Property::ZIndex
            .validate_value(&Value::ZIndex(ZIndex::Auto))
            .unwrap();
        Property::ZIndex
            .validate_value(&Value::ZIndex(ZIndex::integer(-2)))
            .unwrap();
        Property::ScrollbarWidth
            .validate_value(&Value::ScrollbarWidth(ScrollbarWidth::Thin))
            .unwrap();
        Property::ContentVisibility
            .validate_value(&Value::ContentVisibility(ContentVisibility::Auto))
            .unwrap();
        Property::AspectRatio
            .validate_value(&Value::AspectRatio(AspectRatio::ratio(16.0 / 9.0).unwrap()))
            .unwrap();
        Property::Order
            .validate_value(&Value::Order(Order::new(-2)))
            .unwrap();
        Property::FlexGrow
            .validate_value(&Value::FlexFactor(FlexFactor::new(2.0).unwrap()))
            .unwrap();
    }

    #[test]
    fn semantic_numbers_are_not_interchangeable() {
        assert!(
            Property::ZIndex
                .validate_value(&Value::Number(1.0))
                .is_err()
        );
        assert!(
            Property::AspectRatio
                .validate_value(&Value::Number(1.0))
                .is_err()
        );
        assert!(
            Property::FlexGrow
                .validate_value(&Value::Number(1.0))
                .is_err()
        );
        assert!(Property::Order.validate_value(&Value::Number(1.0)).is_err());
        assert!(
            Property::ScrollbarWidth
                .validate_value(&Value::Number(1.0))
                .is_err()
        );
    }

    #[test]
    fn track_alignment_uses_content_alignment_value() {
        Property::AlignTracks
            .validate_value(&Value::AlignContent(AlignContent::Center))
            .unwrap();
        Property::JustifyTracks
            .validate_value(&Value::AlignContent(AlignContent::SpaceAround))
            .unwrap();
        assert!(
            Property::AlignTracks
                .validate_value(&Value::AlignItems(AlignItems::Center))
                .is_err()
        );
        assert!(
            Declarations::new()
                .try_set(Property::AlignTracks, Value::AlignItems(AlignItems::Center))
                .is_err()
        );
    }
}
