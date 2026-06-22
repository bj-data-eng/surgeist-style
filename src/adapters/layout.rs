//! Lower resolved style values into the layout calculator surface.

use crate::{
    CalcLength, CalcOperator, Display, Edges, Error, ErrorCode, GridAutoFlow, GridFlowTolerance,
    GridLine, GridTrackComponent, GridTrackList, Length, MaxTrackSizing, MinTrackSizing, Property,
    Resolved, Result, TrackRepeat, TrackRepeatCount, TrackSizing, Value,
};
use surgeist_layout as layout;

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutLoweringOutput {
    pub node: layout::NodeInput,
    pub calc_store: layout::LayoutCalcStore,
}

#[derive(Clone, Debug, Default)]
pub struct LayoutLoweringSession {
    calc_store: layout::LayoutCalcStore,
}

pub fn lower(resolved: &Resolved) -> Result<layout::NodeInput> {
    if resolved_uses_calc(resolved) {
        return Err(unsupported("calc values require lower_with_store"));
    }
    let mut session = LayoutLoweringSession::new();
    session.lower_node(resolved)
}

pub fn lower_with_store(resolved: &Resolved) -> Result<LayoutLoweringOutput> {
    let mut session = LayoutLoweringSession::new();
    let node = session.lower_node(resolved)?;
    Ok(LayoutLoweringOutput {
        node,
        calc_store: session.finish(),
    })
}

impl LayoutLoweringSession {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_node(&mut self, resolved: &Resolved) -> Result<layout::NodeInput> {
        lower_node_with_session(resolved, self)
    }

    #[must_use]
    pub fn finish(self) -> layout::LayoutCalcStore {
        self.calc_store
    }

    fn push_calc_length(&mut self, calc: &CalcLength) -> layout::CalcId {
        let expression = lower_calc_expression(calc);
        self.calc_store.push(expression)
    }
}

fn resolved_uses_calc(resolved: &Resolved) -> bool {
    length_uses_calc(resolved.width())
        || length_uses_calc(resolved.height())
        || edges_use_calc(edges(resolved, Property::Inset))
        || length_uses_calc(length(resolved, Property::MinWidth))
        || length_uses_calc(length(resolved, Property::MinHeight))
        || length_uses_calc(length(resolved, Property::MaxWidth))
        || length_uses_calc(length(resolved, Property::MaxHeight))
        || edges_use_calc(resolved.margin_edges())
        || edges_use_calc(resolved.padding_edges())
        || edges_use_calc(resolved.border_width_edges())
        || length_uses_calc(length(resolved, Property::ColumnGap))
        || length_uses_calc(length(resolved, Property::RowGap))
        || length_uses_calc(length(resolved, Property::FlexBasis))
        || length_uses_calc(resolved.font_size())
        || grid_flow_tolerance_uses_calc(grid_flow_tolerance(resolved))
        || track_list_uses_calc(track_list(resolved, Property::GridTemplateColumns))
        || track_list_uses_calc(track_list(resolved, Property::GridTemplateRows))
        || track_list_uses_calc(track_list(resolved, Property::GridAutoColumns))
        || track_list_uses_calc(track_list(resolved, Property::GridAutoRows))
}

fn length_uses_calc(length: Length) -> bool {
    matches!(length, Length::Calc(_))
}

fn edges_use_calc(edges: Edges) -> bool {
    length_uses_calc(edges.top)
        || length_uses_calc(edges.right)
        || length_uses_calc(edges.bottom)
        || length_uses_calc(edges.left)
}

fn grid_flow_tolerance_uses_calc(tolerance: GridFlowTolerance) -> bool {
    matches!(tolerance, GridFlowTolerance::Length(Length::Calc(_)))
}

fn track_list_uses_calc(list: &GridTrackList) -> bool {
    list.components.iter().any(track_component_uses_calc)
}

fn track_component_uses_calc(component: &GridTrackComponent) -> bool {
    match component {
        GridTrackComponent::Track(track) => track_sizing_uses_calc(track),
        GridTrackComponent::Repeat(repeat) => {
            repeat.components.iter().any(track_component_uses_calc)
        }
        GridTrackComponent::LineNames(_) | GridTrackComponent::Subgrid(_) => false,
    }
}

fn track_sizing_uses_calc(track: &TrackSizing) -> bool {
    min_track_sizing_uses_calc(&track.min) || max_track_sizing_uses_calc(&track.max)
}

fn min_track_sizing_uses_calc(track: &MinTrackSizing) -> bool {
    matches!(track, MinTrackSizing::Length(Length::Calc(_)))
}

fn max_track_sizing_uses_calc(track: &MaxTrackSizing) -> bool {
    matches!(
        track,
        MaxTrackSizing::Length(Length::Calc(_)) | MaxTrackSizing::FitContent(Length::Calc(_))
    )
}

fn lower_node_with_session(
    resolved: &Resolved,
    session: &mut LayoutLoweringSession,
) -> Result<layout::NodeInput> {
    Ok(layout::NodeInput {
        display: lower_display(resolved.display())?,
        box_sizing: lower_box_sizing(resolved),
        direction: lower_direction(resolved),
        text_align: lower_text_align(resolved),
        writing_mode: lower_writing_mode(resolved),
        overflow: layout::Point::new(
            lower_overflow(resolved, Property::OverflowX),
            lower_overflow(resolved, Property::OverflowY),
        ),
        scrollbar_width: number(resolved, Property::ScrollbarWidth),
        position: lower_position(resolved),
        float: lower_float(resolved),
        clear: lower_clear(resolved),
        inset: lower_edges_auto_with_session(edges(resolved, Property::Inset), session)?,
        size: layout::Size::new(
            lower_dimension_with_session(resolved.width(), session)?,
            lower_dimension_with_session(resolved.height(), session)?,
        ),
        min_size: layout::Size::new(
            lower_dimension_with_session(length(resolved, Property::MinWidth), session)?,
            lower_dimension_with_session(length(resolved, Property::MinHeight), session)?,
        ),
        max_size: layout::Size::new(
            lower_dimension_with_session(length(resolved, Property::MaxWidth), session)?,
            lower_dimension_with_session(length(resolved, Property::MaxHeight), session)?,
        ),
        aspect_ratio: aspect_ratio(resolved),
        margin: lower_edges_auto_with_session(resolved.margin_edges(), session)?,
        padding: lower_edges_with_session(resolved.padding_edges(), session)?,
        border: lower_edges_with_session(resolved.border_width_edges(), session)?,
        gap: layout::Size::new(
            lower_gap_length_with_session(length(resolved, Property::ColumnGap), session)?,
            lower_gap_length_with_session(length(resolved, Property::RowGap), session)?,
        ),
        align_items: lower_align_items(resolved, Property::AlignItems),
        align_self: lower_align_items(resolved, Property::AlignSelf),
        justify_items: lower_align_items(resolved, Property::JustifyItems),
        justify_self: lower_align_items(resolved, Property::JustifySelf),
        align_content: lower_align_content(resolved, Property::AlignContent),
        justify_content: lower_align_content(resolved, Property::JustifyContent),
        flex_direction: lower_flex_direction(resolved),
        flex_wrap: lower_flex_wrap(resolved),
        flex_basis: lower_dimension_with_session(length(resolved, Property::FlexBasis), session)?,
        flex_grow: number(resolved, Property::FlexGrow),
        flex_shrink: number(resolved, Property::FlexShrink),
        grid_template_columns: lower_track_list_with_session(
            track_list(resolved, Property::GridTemplateColumns),
            session,
        )?,
        grid_template_rows: lower_track_list_with_session(
            track_list(resolved, Property::GridTemplateRows),
            session,
        )?,
        grid_template_areas: lower_grid_template_areas(grid_template_areas(resolved)),
        grid_auto_columns: lower_track_list_with_session(
            track_list(resolved, Property::GridAutoColumns),
            session,
        )?,
        grid_auto_rows: lower_track_list_with_session(
            track_list(resolved, Property::GridAutoRows),
            session,
        )?,
        grid_auto_flow: lower_grid_auto_flow(grid_auto_flow(resolved)),
        grid_flow_tolerance: lower_grid_flow_tolerance_with_session(resolved, session)?,
        grid_column: lower_grid_placement(
            grid_line(resolved, Property::GridColumnStart),
            grid_line(resolved, Property::GridColumnEnd),
        )?,
        grid_row: lower_grid_placement(
            grid_line(resolved, Property::GridRowStart),
            grid_line(resolved, Property::GridRowEnd),
        )?,
        raw_grid_column: lower_raw_grid_placement(
            grid_line(resolved, Property::GridColumnStart),
            grid_line(resolved, Property::GridColumnEnd),
        )?,
        raw_grid_row: lower_raw_grid_placement(
            grid_line(resolved, Property::GridRowStart),
            grid_line(resolved, Property::GridRowEnd),
        )?,
        ..layout::NodeInput::DEFAULT
    })
}

fn lower_display(display: Display) -> Result<layout::Display> {
    match display {
        Display::Block => Ok(layout::Display::Block),
        Display::Flex => Ok(layout::Display::Flex),
        Display::Grid => Ok(layout::Display::Grid),
        Display::InlineBlock => Ok(layout::Display::InlineBlock),
        Display::InlineGrid => Ok(layout::Display::InlineGrid),
        Display::GridLanes => Ok(layout::Display::GridLanes),
        Display::InlineGridLanes => Ok(layout::Display::InlineGridLanes),
        Display::None => Ok(layout::Display::None),
    }
}

fn lower_box_sizing(resolved: &Resolved) -> layout::BoxSizing {
    match resolved.get(Property::BoxSizing) {
        Value::BoxSizing(crate::BoxSizing::ContentBox) => layout::BoxSizing::ContentBox,
        Value::BoxSizing(crate::BoxSizing::BorderBox) => layout::BoxSizing::BorderBox,
        _ => layout::BoxSizing::default(),
    }
}

fn lower_position(resolved: &Resolved) -> layout::Position {
    match resolved.get(Property::Position) {
        Value::Position(crate::LayoutPosition::Relative) => layout::Position::Relative,
        Value::Position(crate::LayoutPosition::Absolute) => layout::Position::Absolute,
        _ => layout::Position::default(),
    }
}

fn lower_direction(resolved: &Resolved) -> layout::Direction {
    match resolved.get(Property::Direction) {
        Value::Direction(crate::Direction::Ltr) => layout::Direction::Ltr,
        Value::Direction(crate::Direction::Rtl) => layout::Direction::Rtl,
        _ => layout::Direction::default(),
    }
}

fn lower_overflow(resolved: &Resolved, property: Property) -> layout::Overflow {
    match resolved.get(property) {
        Value::Overflow(crate::Overflow::Visible) => layout::Overflow::Visible,
        Value::Overflow(crate::Overflow::Clip) => layout::Overflow::Clip,
        Value::Overflow(crate::Overflow::Hidden) => layout::Overflow::Hidden,
        Value::Overflow(crate::Overflow::Scroll) => layout::Overflow::Scroll,
        _ => layout::Overflow::default(),
    }
}

fn lower_float(resolved: &Resolved) -> layout::Float {
    match resolved.get(Property::Float) {
        Value::Float(crate::Float::None) => layout::Float::None,
        Value::Float(crate::Float::Left) => layout::Float::Left,
        Value::Float(crate::Float::Right) => layout::Float::Right,
        _ => layout::Float::default(),
    }
}

fn lower_clear(resolved: &Resolved) -> layout::Clear {
    match resolved.get(Property::Clear) {
        Value::Clear(crate::Clear::None) => layout::Clear::None,
        Value::Clear(crate::Clear::Left) => layout::Clear::Left,
        Value::Clear(crate::Clear::Right) => layout::Clear::Right,
        Value::Clear(crate::Clear::Both) => layout::Clear::Both,
        _ => layout::Clear::default(),
    }
}

fn lower_text_align(resolved: &Resolved) -> layout::TextAlign {
    match resolved.get(Property::TextAlign) {
        Value::TextAlign(crate::StyleTextAlign::Auto) => layout::TextAlign::Auto,
        Value::TextAlign(crate::StyleTextAlign::LegacyLeft) => layout::TextAlign::LegacyLeft,
        Value::TextAlign(crate::StyleTextAlign::LegacyRight) => layout::TextAlign::LegacyRight,
        Value::TextAlign(crate::StyleTextAlign::LegacyCenter) => layout::TextAlign::LegacyCenter,
        _ => layout::TextAlign::default(),
    }
}

fn lower_writing_mode(resolved: &Resolved) -> layout::WritingMode {
    match resolved.get(Property::WritingMode) {
        Value::WritingMode(crate::WritingMode::HorizontalTb) => layout::WritingMode::HorizontalTb,
        Value::WritingMode(crate::WritingMode::VerticalLr) => layout::WritingMode::VerticalLr,
        Value::WritingMode(crate::WritingMode::VerticalRl) => layout::WritingMode::VerticalRl,
        _ => layout::WritingMode::default(),
    }
}

fn lower_flex_direction(resolved: &Resolved) -> layout::FlexDirection {
    match resolved.get(Property::FlexDirection) {
        Value::FlexDirection(crate::FlexDirection::Row) => layout::FlexDirection::Row,
        Value::FlexDirection(crate::FlexDirection::Column) => layout::FlexDirection::Column,
        Value::FlexDirection(crate::FlexDirection::RowReverse) => layout::FlexDirection::RowReverse,
        Value::FlexDirection(crate::FlexDirection::ColumnReverse) => {
            layout::FlexDirection::ColumnReverse
        }
        _ => layout::FlexDirection::default(),
    }
}

fn lower_flex_wrap(resolved: &Resolved) -> layout::FlexWrap {
    match resolved.get(Property::FlexWrap) {
        Value::FlexWrap(crate::FlexWrap::NoWrap) => layout::FlexWrap::NoWrap,
        Value::FlexWrap(crate::FlexWrap::Wrap) => layout::FlexWrap::Wrap,
        Value::FlexWrap(crate::FlexWrap::WrapReverse) => layout::FlexWrap::WrapReverse,
        _ => layout::FlexWrap::default(),
    }
}

fn lower_align_items(resolved: &Resolved, property: Property) -> Option<layout::AlignItems> {
    match resolved.get(property) {
        Value::AlignItems(crate::AlignItems::Auto) => None,
        Value::AlignItems(crate::AlignItems::Start) => Some(layout::AlignItems::Start),
        Value::AlignItems(crate::AlignItems::End) => Some(layout::AlignItems::End),
        Value::AlignItems(crate::AlignItems::FlexStart) => Some(layout::AlignItems::FlexStart),
        Value::AlignItems(crate::AlignItems::FlexEnd) => Some(layout::AlignItems::FlexEnd),
        Value::AlignItems(crate::AlignItems::Center) => Some(layout::AlignItems::Center),
        Value::AlignItems(crate::AlignItems::SafeEnd) => Some(layout::AlignItems::SafeEnd),
        Value::AlignItems(crate::AlignItems::SafeFlexEnd) => Some(layout::AlignItems::SafeFlexEnd),
        Value::AlignItems(crate::AlignItems::SafeCenter) => Some(layout::AlignItems::SafeCenter),
        Value::AlignItems(crate::AlignItems::Baseline) => Some(layout::AlignItems::Baseline),
        Value::AlignItems(crate::AlignItems::LastBaseline) => {
            Some(layout::AlignItems::LastBaseline)
        }
        Value::AlignItems(crate::AlignItems::Stretch) => Some(layout::AlignItems::Stretch),
        _ => None,
    }
}

fn lower_align_content(resolved: &Resolved, property: Property) -> Option<layout::AlignContent> {
    match resolved.get(property) {
        Value::AlignContent(crate::AlignContent::Auto) => None,
        Value::AlignContent(crate::AlignContent::Start) => Some(layout::AlignContent::Start),
        Value::AlignContent(crate::AlignContent::End) => Some(layout::AlignContent::End),
        Value::AlignContent(crate::AlignContent::FlexStart) => {
            Some(layout::AlignContent::FlexStart)
        }
        Value::AlignContent(crate::AlignContent::FlexEnd) => Some(layout::AlignContent::FlexEnd),
        Value::AlignContent(crate::AlignContent::Center) => Some(layout::AlignContent::Center),
        Value::AlignContent(crate::AlignContent::SafeEnd) => Some(layout::AlignContent::SafeEnd),
        Value::AlignContent(crate::AlignContent::SafeFlexEnd) => {
            Some(layout::AlignContent::SafeFlexEnd)
        }
        Value::AlignContent(crate::AlignContent::SafeCenter) => {
            Some(layout::AlignContent::SafeCenter)
        }
        Value::AlignContent(crate::AlignContent::Stretch) => Some(layout::AlignContent::Stretch),
        Value::AlignContent(crate::AlignContent::SpaceBetween) => {
            Some(layout::AlignContent::SpaceBetween)
        }
        Value::AlignContent(crate::AlignContent::SpaceEvenly) => {
            Some(layout::AlignContent::SpaceEvenly)
        }
        Value::AlignContent(crate::AlignContent::SpaceAround) => {
            Some(layout::AlignContent::SpaceAround)
        }
        _ => None,
    }
}

fn lower_dimension(length: Length) -> Result<layout::Dimension> {
    Ok(match length {
        Length::Normal => return Err(unsupported("normal dimension length")),
        Length::Px(value) => layout::Dimension::px(value),
        Length::Percent(value) => layout::Dimension::percent(percent(value)),
        Length::Fill => layout::Dimension::fr(1.0),
        Length::Fit | Length::Auto => layout::Dimension::AUTO,
        Length::MinContent => layout::Dimension::MIN_CONTENT,
        Length::MaxContent => layout::Dimension::MAX_CONTENT,
        Length::Calc(_) => return Err(unsupported("calc dimension length")),
    })
}

fn lower_dimension_with_session(
    length: Length,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Dimension> {
    Ok(match length {
        Length::Calc(calc) => {
            let id = session.push_calc_length(&calc);
            layout::Dimension::calc(id)
        }
        length => lower_dimension(length)?,
    })
}

fn lower_length_auto(length: Length) -> Result<layout::LengthAuto> {
    Ok(match length {
        Length::Px(value) => layout::LengthAuto::px(value),
        Length::Percent(value) => layout::LengthAuto::percent(percent(value)),
        Length::Auto => layout::LengthAuto::AUTO,
        Length::Calc(_)
        | Length::Normal
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => {
            return Err(unsupported("intrinsic, flexible, or calc edge length"));
        }
    })
}

fn lower_length_auto_with_session(
    length: Length,
    session: &mut LayoutLoweringSession,
) -> Result<layout::LengthAuto> {
    Ok(match length {
        Length::Calc(calc) => {
            let id = session.push_calc_length(&calc);
            layout::LengthAuto::calc(id)
        }
        length => lower_length_auto(length)?,
    })
}

fn lower_length(length: Length) -> Result<layout::Length> {
    Ok(match length {
        Length::Px(value) => layout::Length::px(value),
        Length::Percent(value) => layout::Length::percent(percent(value)),
        Length::Calc(_)
        | Length::Normal
        | Length::Auto
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => {
            return Err(unsupported("non-definite length"));
        }
    })
}

fn lower_length_with_session(
    length: Length,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Length> {
    Ok(match length {
        Length::Calc(calc) => {
            let id = session.push_calc_length(&calc);
            layout::Length::calc(id)
        }
        length => lower_length(length)?,
    })
}

fn lower_gap_length_with_session(
    length: Length,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Length> {
    Ok(match length {
        Length::Normal => layout::Length::NORMAL,
        length => lower_length_with_session(length, session)?,
    })
}

fn lower_edges_with_session(
    edges: Edges,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Edges<layout::Length>> {
    Ok(layout::Edges::new(
        lower_length_with_session(edges.top, session)?,
        lower_length_with_session(edges.right, session)?,
        lower_length_with_session(edges.bottom, session)?,
        lower_length_with_session(edges.left, session)?,
    ))
}

fn lower_edges_auto_with_session(
    edges: Edges,
    session: &mut LayoutLoweringSession,
) -> Result<layout::Edges<layout::LengthAuto>> {
    Ok(layout::Edges::new(
        lower_length_auto_with_session(edges.top, session)?,
        lower_length_auto_with_session(edges.right, session)?,
        lower_length_auto_with_session(edges.bottom, session)?,
        lower_length_auto_with_session(edges.left, session)?,
    ))
}

fn lower_track_list_with_session(
    list: &GridTrackList,
    session: &mut LayoutLoweringSession,
) -> Result<Vec<layout::TrackComponent>> {
    let mut lowered = Vec::new();
    for component in &list.components {
        match component {
            GridTrackComponent::Track(track) => {
                lowered.push(layout::TrackComponent::Track(
                    lower_track_sizing_with_session(track, session)?,
                ));
            }
            GridTrackComponent::Repeat(repeat) => {
                lowered.push(layout::TrackComponent::Repeat(
                    lower_track_repeat_with_session(repeat, session)?,
                ));
            }
            GridTrackComponent::LineNames(names) => {
                lowered.push(layout::TrackComponent::LineNames(names.clone()));
            }
            GridTrackComponent::Subgrid(subgrid) => {
                lowered.push(layout::TrackComponent::Subgrid(layout::SubgridTrack {
                    name_components: lower_subgrid_line_name_components(&subgrid.name_components),
                }));
            }
        }
    }
    Ok(lowered)
}

fn lower_track_repeat_with_session(
    repeat: &TrackRepeat,
    session: &mut LayoutLoweringSession,
) -> Result<layout::TrackRepetition> {
    let mut components = Vec::new();
    for component in &repeat.components {
        match component {
            GridTrackComponent::Track(track) => {
                components.push(layout::TrackComponent::Track(
                    lower_track_sizing_with_session(track, session)?,
                ));
            }
            GridTrackComponent::LineNames(names) => {
                components.push(layout::TrackComponent::LineNames(names.clone()));
            }
            GridTrackComponent::Repeat(_) => return Err(unsupported("nested grid track repeat")),
            GridTrackComponent::Subgrid(_) => return Err(unsupported("subgrid track repeat")),
        }
    }

    match repeat.count {
        TrackRepeatCount::Count(count) => Ok(layout::TrackRepetition::count_components(
            usize::from(count),
            components,
        )),
        TrackRepeatCount::AutoFill => Ok(layout::TrackRepetition::auto_fill_components(components)),
        TrackRepeatCount::AutoFit => Ok(layout::TrackRepetition::auto_fit_components(components)),
    }
}

fn lower_subgrid_line_name_components(
    components: &[crate::SubgridLineNameComponent],
) -> Vec<layout::SubgridLineNameComponent> {
    components
        .iter()
        .map(|component| match component {
            crate::SubgridLineNameComponent::LineNames(names) => {
                layout::SubgridLineNameComponent::LineNames(names.clone())
            }
            crate::SubgridLineNameComponent::Repeat {
                count,
                line_name_sets,
            } => layout::SubgridLineNameComponent::Repeat {
                count: match count {
                    crate::SubgridLineNameRepeatCount::Count(count) => {
                        layout::SubgridLineNameRepeatCount::Count(*count)
                    }
                    crate::SubgridLineNameRepeatCount::AutoFill => {
                        layout::SubgridLineNameRepeatCount::AutoFill
                    }
                },
                line_name_sets: line_name_sets.clone(),
            },
        })
        .collect()
}

fn lower_track_sizing_with_session(
    track: &TrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::TrackSizing> {
    Ok(layout::TrackSizing::new(
        lower_min_track_sizing_with_session(&track.min, session)?,
        lower_max_track_sizing_with_session(&track.max, session)?,
    ))
}

fn lower_min_track_sizing_with_session(
    track: &MinTrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::MinTrackSizing> {
    Ok(match track {
        MinTrackSizing::Length(length) => {
            layout::MinTrackSizing::Length(lower_length_with_session(length.clone(), session)?)
        }
        MinTrackSizing::Auto => layout::MinTrackSizing::AUTO,
        MinTrackSizing::MinContent => layout::MinTrackSizing::MIN_CONTENT,
        MinTrackSizing::MaxContent => layout::MinTrackSizing::MAX_CONTENT,
    })
}

fn lower_max_track_sizing_with_session(
    track: &MaxTrackSizing,
    session: &mut LayoutLoweringSession,
) -> Result<layout::MaxTrackSizing> {
    Ok(match track {
        MaxTrackSizing::Length(length) => {
            layout::MaxTrackSizing::Length(lower_length_with_session(length.clone(), session)?)
        }
        MaxTrackSizing::Flex(value) => layout::MaxTrackSizing::fr(*value),
        MaxTrackSizing::Auto => layout::MaxTrackSizing::AUTO,
        MaxTrackSizing::MinContent => layout::MaxTrackSizing::MIN_CONTENT,
        MaxTrackSizing::MaxContent => layout::MaxTrackSizing::MAX_CONTENT,
        MaxTrackSizing::FitContent(limit) => {
            layout::MaxTrackSizing::fit_content(lower_length_with_session(limit.clone(), session)?)
        }
    })
}

fn lower_grid_auto_flow(flow: GridAutoFlow) -> layout::GridAutoFlow {
    match flow {
        GridAutoFlow::Row => layout::GridAutoFlow::Row,
        GridAutoFlow::Column => layout::GridAutoFlow::Column,
        GridAutoFlow::RowDense => layout::GridAutoFlow::RowDense,
        GridAutoFlow::ColumnDense => layout::GridAutoFlow::ColumnDense,
    }
}

fn lower_grid_flow_tolerance_with_session(
    resolved: &Resolved,
    session: &mut LayoutLoweringSession,
) -> Result<layout::GridFlowTolerance> {
    Ok(match grid_flow_tolerance(resolved) {
        GridFlowTolerance::Normal => layout::GridFlowTolerance::Normal {
            font_size: font_size_scalar(resolved.font_size())?,
        },
        GridFlowTolerance::Length(length) => {
            layout::GridFlowTolerance::Length(lower_length_with_session(length, session)?)
        }
        GridFlowTolerance::Percent(value) => layout::GridFlowTolerance::Percent(percent(value)),
        GridFlowTolerance::Infinite => layout::GridFlowTolerance::Infinite,
    })
}

fn font_size_scalar(length: Length) -> Result<f32> {
    Ok(match length {
        Length::Px(value) => value,
        Length::Percent(value) => 16.0 * percent(value),
        Length::Normal
        | Length::Auto
        | Length::Fill
        | Length::Fit
        | Length::MinContent
        | Length::MaxContent => 16.0,
        Length::Calc(_) => return Err(unsupported("calc font size")),
    })
}

fn lower_grid_placement(start: GridLine, end: GridLine) -> Result<layout::GridPlacement> {
    Ok(match (start, end) {
        (GridLine::Auto, GridLine::Auto) => layout::GridPlacement::AUTO,
        (GridLine::Line(line), GridLine::Auto) => layout::GridPlacement::line(isize::from(line)),
        (GridLine::Auto, GridLine::Line(line)) => {
            layout::GridPlacement::end_line(isize::from(line))
        }
        (GridLine::Line(start), GridLine::Line(end)) => {
            layout::GridPlacement::lines(isize::from(start), isize::from(end))
        }
        (GridLine::Line(line), GridLine::Span(span)) => {
            layout::GridPlacement::line_span(isize::from(line), usize::from(span))
        }
        (GridLine::Span(span), GridLine::Line(line)) => {
            layout::GridPlacement::span_line(usize::from(span), isize::from(line))
        }
        (GridLine::Span(span), GridLine::Auto) | (GridLine::Auto, GridLine::Span(span)) => {
            layout::GridPlacement::span(usize::from(span))
        }
        (GridLine::Span(_), GridLine::Span(_)) => {
            return Err(unsupported("span-to-span grid placement"));
        }
        (GridLine::BareIdent(_) | GridLine::NamedLine { .. } | GridLine::NamedSpan { .. }, _)
        | (_, GridLine::BareIdent(_) | GridLine::NamedLine { .. } | GridLine::NamedSpan { .. }) => {
            layout::GridPlacement::AUTO
        }
    })
}

fn lower_raw_grid_line(line: GridLine) -> Result<layout::RawGridLine> {
    Ok(match line {
        GridLine::Auto => layout::RawGridLine::Auto,
        GridLine::Line(line) => layout::RawGridLine::Line(isize::from(line)),
        GridLine::Span(span) => layout::RawGridLine::Span(usize::from(span)),
        GridLine::BareIdent(name) => layout::RawGridLine::BareIdent(name),
        GridLine::NamedLine { name, index } => layout::RawGridLine::NamedLine {
            name,
            index: isize::from(index),
        },
        GridLine::NamedSpan { name, index } => layout::RawGridLine::NamedSpan {
            name,
            index: usize::from(index),
        },
    })
}

fn lower_raw_grid_placement(start: GridLine, end: GridLine) -> Result<layout::RawGridPlacement> {
    Ok(layout::RawGridPlacement::new(
        lower_raw_grid_line(start)?,
        lower_raw_grid_line(end)?,
    ))
}

fn length(resolved: &Resolved, property: Property) -> Length {
    match resolved.get(property) {
        Value::Length(length) => length.clone(),
        _ => Length::Auto,
    }
}

fn edges(resolved: &Resolved, property: Property) -> Edges {
    match resolved.get(property) {
        Value::Edges(edges) => edges.clone(),
        _ => Edges::default(),
    }
}

fn number(resolved: &Resolved, property: Property) -> f32 {
    match resolved.get(property) {
        Value::Number(value) => *value,
        _ => 0.0,
    }
}

fn aspect_ratio(resolved: &Resolved) -> Option<f32> {
    match number(resolved, Property::AspectRatio) {
        value if value > 0.0 => Some(value),
        _ => None,
    }
}

fn track_list(resolved: &Resolved, property: Property) -> &GridTrackList {
    match resolved.get(property) {
        Value::GridTrackList(list) => list,
        _ => unreachable!("resolved grid track property stores a grid track list"),
    }
}

fn grid_template_areas(resolved: &Resolved) -> &crate::GridTemplateAreas {
    match resolved.get(Property::GridTemplateAreas) {
        Value::GridTemplateAreas(areas) => areas,
        _ => unreachable!("resolved grid-template-areas stores grid template areas"),
    }
}

fn lower_grid_template_areas(areas: &crate::GridTemplateAreas) -> layout::GridTemplateAreas {
    layout::GridTemplateAreas {
        rows: areas
            .rows
            .iter()
            .map(|row| layout::GridTemplateAreaRow {
                cells: row.cells.clone(),
            })
            .collect(),
    }
}

fn grid_line(resolved: &Resolved, property: Property) -> GridLine {
    match resolved.get(property) {
        Value::GridLine(line) => line.clone(),
        _ => GridLine::Auto,
    }
}

fn grid_auto_flow(resolved: &Resolved) -> GridAutoFlow {
    match resolved.get(Property::GridAutoFlow) {
        Value::GridAutoFlow(flow) => *flow,
        _ => GridAutoFlow::Row,
    }
}

fn grid_flow_tolerance(resolved: &Resolved) -> GridFlowTolerance {
    match resolved.get(Property::GridFlowTolerance) {
        Value::GridFlowTolerance(tolerance) => tolerance.clone(),
        _ => GridFlowTolerance::Normal,
    }
}

fn percent(value: f32) -> f32 {
    value / 100.0
}

fn lower_calc_expression(calc: &CalcLength) -> layout::CalcExpression {
    match calc {
        CalcLength::Px(value) => layout::CalcExpression::sum([layout::CalcTerm::px(*value)]),
        CalcLength::Percent(value) => {
            layout::CalcExpression::sum([layout::CalcTerm::percent(percent(*value))])
        }
        CalcLength::Sum(terms) => {
            let mut lowered = Vec::new();
            for term in terms {
                let sign = match term.operator {
                    CalcOperator::Add => 1.0,
                    CalcOperator::Sub => -1.0,
                };
                collect_calc_terms(&term.value, sign, &mut lowered);
            }
            layout::CalcExpression::sum(lowered)
        }
    }
}

fn collect_calc_terms(calc: &CalcLength, sign: f32, output: &mut Vec<layout::CalcTerm>) {
    match calc {
        CalcLength::Px(value) => output.push(layout::CalcTerm::px(sign * *value)),
        CalcLength::Percent(value) => {
            output.push(layout::CalcTerm::percent(sign * percent(*value)));
        }
        CalcLength::Sum(terms) => {
            for term in terms {
                let term_sign = match term.operator {
                    CalcOperator::Add => sign,
                    CalcOperator::Sub => -sign,
                };
                collect_calc_terms(&term.value, term_sign, output);
            }
        }
    }
}

fn unsupported(feature: &str) -> Error {
    Error::new(
        ErrorCode::InvalidValue,
        format!("{feature} cannot be lowered to layout style yet"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CalcLength;
    use surgeist_retained::{Element, Model, Patch, Tag};

    #[test]
    fn calc_font_size_is_unsupported_for_normal_grid_flow_tolerance_lowering() {
        let error = font_size_scalar(Length::Calc(CalcLength::px(12.0))).unwrap_err();

        assert!(error.to_string().contains("calc font size"));
    }

    #[test]
    fn lowers_calc_dimension_into_layout_calc_store() {
        let calc = crate::CalcLength::sum([
            crate::CalcLengthTerm::add(crate::CalcLength::px(20.0)),
            crate::CalcLengthTerm::add(crate::CalcLength::percent(10.0)),
        ]);
        let declarations = crate::Declarations::new()
            .try_set(
                crate::Property::Width,
                crate::Value::Length(crate::Length::Calc(calc)),
            )
            .unwrap();
        let mut model = Model::empty();
        let root = model.root();
        let panel = model
            .apply(Patch::Insert {
                parent: root,
                index: 0,
                element: Element::tagged(Tag::new("panel").unwrap()),
            })
            .unwrap()
            .changes()
            .inserted()[0];
        let tree = model.snapshot();
        let resolved = crate::Resolver::new(crate::Sheet::new())
            .resolve(crate::Context::new(&tree, panel).local(&declarations))
            .unwrap();

        let lowered = lower_with_store(&resolved).unwrap();

        let surgeist_layout::Dimension::Calc(id) = lowered.node.size.width else {
            panic!("expected calc width, got {:?}", lowered.node.size.width);
        };
        let expression = lowered.calc_store.get(id).unwrap();
        assert!((expression.percent_fraction() - 0.10).abs() < f32::EPSILON);
        assert_eq!(expression.resolve(Some(200.0)).value, Some(40.0));
        assert_eq!(lowered.calc_store.len(), 1);
    }
}
