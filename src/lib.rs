//! Typed rule, cascade, and resolved-style boundary for Surgeist.
//!
//! This module owns Rust-authored style values, declarations, property
//! metadata, selector matching, sheets, resolution, and invalidation facts. CSS
//! parsing is specified separately and does not live in this first contract.

mod authored;
mod bucket;
mod calc;
mod condition;
mod custom;
mod declaration;
mod error;
mod identity;
mod invalidation;
mod precedence;
mod property;
mod resolver;
mod selector;
mod sheet;
mod state;
mod tree;
mod value;

pub use authored::{
    AuthoredDeclaration, AuthoredDeclarations, AuthoredProperty, AuthoredValue, CssWideKeyword,
};
pub use bucket::{PseudoElement, StyleBucket, StyleBucketPolicy};
pub use calc::{CalcLength, CalcLengthTerm, CalcOperator};
pub use condition::{Condition, Container, Viewport};
pub use custom::{
    AuthoredTokens, CustomPropertyDependencies, CustomPropertyName, CustomPropertyResolution,
    CustomPropertyTypedValue, CustomPropertyValue, VariableDependentValue, VariableExpression,
    VariableFallback, VariableReference,
};
pub use declaration::{Declaration, Declarations, Fingerprint, TypedDeclaration};
pub use error::{Error, ErrorCode, Result};
pub use identity::{
    RangeState, StyleAttribute, StyleAttributeName, StyleAttributeValue, StyleClass, StyleKey,
    StyleRole, StyleState, StyleTag,
};
pub use invalidation::{Change, Invalidation, Scope, SelectorFactChange};
pub use precedence::{LayerOrder, RulePrecedence, SourceOrder};
pub use property::{Impact, Interpolation, Metadata, Property};
pub use resolver::{Context, Resolved, Resolver};
pub use selector::{
    AttributeCaseSensitivity, AttributeMatcher, AttributeSelector, Combinator, ComplexSelector,
    ComplexSelectorPart, Compound, Nth, NthPattern, NthSelector, Position as SelectorPosition,
    PositionSelector, PseudoClassSelector, RelativeSelector, RelativeSelectorList,
    RuntimePseudoClass, Selector, SelectorList, SelectorListPseudoClass, SelectorMatchContext,
    SelectorSpecificity, StructuralSelector,
};
pub use sheet::{Rule, RuleTarget, Sheet, Version};
pub use state::StateFlag;
pub use tree::{Node, Traversal, Tree};
pub use value::{
    AlignContent, AlignItems, Alpha, AnimationNameList, AspectRatio, BackgroundAttachment,
    BackgroundAttachmentList, BackgroundBox, BackgroundRepeat, BackgroundRepeatList,
    BackgroundRepeatStyle, BackgroundSize, BackgroundSizeComponent, BackgroundSizeList, BasicShape,
    Border, BorderLineStyle, BorderRadii, BorderSide, BorderStyles, BoxDecorationBreak, BoxSizing,
    BuiltInCounterStyle, Clear, ClipPath, Color, ColorComponent, ColorFunction,
    ColorInterpolationMethod, ColorInterpolationSpace, ColorMix, ColorMixComponent, Content,
    ContentItem, ContentItemList, ContentString, ContentVisibility, CornerRadius, Corners,
    CounterFunction, CounterName, CounterStyle, CounterStyleName, CountersFunction, CssPx, Cursor,
    Dash, Decoration, DimensionLength, Direction, Display, DurationSeconds, Edges, Filter,
    FilterFunction, FilterFunctionList, Flex, FlexDirection, FlexFactor, FlexWrap, Float, Font,
    FontFamilyList, FontFeature, FontFeatureSettings, FontFeatureTag, FontFeatureValue,
    FontStretch, FontVariant, FontWeight, FontWeightNumber, GridAreaPlacement, GridAutoFlow,
    GridDefinition, GridFlowTolerance, GridLine, GridPlacement, GridTemplate, GridTemplateAreaRow,
    GridTemplateAreas, GridTrackComponent, GridTrackList, HorizontalPositionKeyword,
    HueInterpolationMethod, ImageLayer, ImageLayerList, Keyword, LayoutPosition, Length,
    LetterSpacing, LetterSpacingLength, LineStyle, MaskLayer, MaskLayerList, MaxTrackSizing,
    MinTrackSizing, Opacity, Order, Outline, OutlineStyle, OutlineWidth, OutlineWidthLength,
    Overflow, OverflowAxes, OverflowWrap, PlaceContentAlignment, PlaceItemsAlignment,
    PointerEvents, Position, PositionComponent, PositionList, PredefinedColorSpace, RelativeColor,
    RelativeColorFunction, Rotate, Scale, ScaleValues, ScrollbarWidth, Shadow, SideSet, Size,
    Stroke, StrokeAlign, StyleColor, StyleTextAlign, StyleUrl, SubgridLineNameComponent,
    SubgridLineNameRepeatCount, SubgridTrack, SymbolicComponentExpression, SymbolicFunctionValue,
    SystemColor, TextAlignLast, TextDecoration, TextDecorationLine, TextDecorationLineComponent,
    TextDecorationStyle, TextDecorationThickness, TextDecorationThicknessLength, TextIndent,
    TextOverflow, TextSlant, TextTransform, TextValue, TextWeight, TextWrap, TrackRepeat,
    TrackRepeatCount, TrackSizing, Transform, TransformOp, Translate, TranslateValues, UserSelect,
    Value, VerticalAlign, VerticalAlignLength, VerticalPositionKeyword, Visibility, WhiteSpace,
    WordBreak, WritingMode, ZIndex,
};

#[must_use]
pub fn color(rgba: u32) -> Color {
    let r = ((rgba >> 24) & 0xff) as f32 / 255.0;
    let g = ((rgba >> 16) & 0xff) as f32 / 255.0;
    let b = ((rgba >> 8) & 0xff) as f32 / 255.0;
    let a = (rgba & 0xff) as f32 / 255.0;
    Color::raw_rgba(r, g, b, a)
}
