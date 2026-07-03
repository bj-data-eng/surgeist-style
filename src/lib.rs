//! Typed rule, cascade, and resolved-style boundary for Surgeist.
//!
//! This module owns Rust-authored style values, declarations, property
//! metadata, selector matching, sheets, resolution, and invalidation facts. CSS
//! parsing is specified separately and does not live in this first contract.

pub mod adapters;
mod calc;
mod condition;
mod declaration;
mod error;
mod identity;
mod invalidation;
mod property;
mod resolver;
mod selector;
mod sheet;
mod state;
mod tree;
mod value;

pub use calc::{CalcLength, CalcLengthTerm, CalcOperator};
pub use condition::{Condition, Container, Viewport};
pub use declaration::{Declaration, Declarations, Fingerprint, TypedDeclaration};
pub use error::{Error, ErrorCode, Result};
pub use identity::{
    StyleAttribute, StyleAttributeName, StyleAttributeValue, StyleClass, StyleKey, StyleRole,
    StyleState, StyleTag,
};
pub use invalidation::{Change, Invalidation, Scope};
pub use property::{Impact, Interpolation, Metadata, Property};
pub use resolver::{Context, Resolved, Resolver};
pub use selector::{
    AttributeSelector, Combinator, Compound, Nth, Part, Position, PositionSelector, Selector,
};
pub use sheet::{Rule, Sheet, Version};
pub use state::StateFlag;
pub use surgeist_text::{
    Alignment as TextAlign, Decoration, Direction as TextDirection, OverflowWrap,
    Slant as TextSlant, Weight as TextWeight, WhiteSpace, WordBreak, Wrap as TextWrap,
};
pub use tree::{Node, Traversal, Tree};
pub use value::{
    AlignContent, AlignItems, AnimationNameList, BoxSizing, Clear, Color, Corners, CssPx, Cursor,
    Dash, DimensionLength, Direction, Display, DurationSeconds, Edges, FlexDirection, FlexWrap,
    Float, FontFamilyList, GridAreaPlacement, GridAutoFlow, GridDefinition, GridFlowTolerance,
    GridLine, GridPlacement, GridTemplate, GridTemplateAreaRow, GridTemplateAreas,
    GridTrackComponent, GridTrackList, Keyword, LayoutPosition, Length, LineStyle, MaxTrackSizing,
    MinTrackSizing, Opacity, Overflow, OverflowAxes, PointerEvents, Shadow, SideSet, Size, Stroke,
    StrokeAlign, StyleTextAlign, SubgridLineNameComponent, SubgridLineNameRepeatCount,
    SubgridTrack, TextValue, TrackRepeat, TrackRepeatCount, TrackSizing, Transform, TransformOp,
    Value, Visibility, WritingMode,
};

#[must_use]
pub fn color(rgba: u32) -> Color {
    let r = ((rgba >> 24) & 0xff) as f32 / 255.0;
    let g = ((rgba >> 16) & 0xff) as f32 / 255.0;
    let b = ((rgba >> 8) & 0xff) as f32 / 255.0;
    let a = (rgba & 0xff) as f32 / 255.0;
    Color::rgba(r, g, b, a)
}
