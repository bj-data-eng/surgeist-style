use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    CalcLength, CalcLengthTerm, Color, Corners, Cursor, DimensionLength, Display, Edges,
    GridAreaPlacement, GridAutoFlow, GridDefinition, GridFlowTolerance, GridLine, GridPlacement,
    GridTemplate, GridTemplateAreas, GridTrackComponent, GridTrackList, Length, MaxTrackSizing,
    MinTrackSizing, Opacity, PointerEvents, Property, Result, Shadow, Size,
    SubgridLineNameComponent, TrackRepeatCount, TrackSizing, Transform, Value, Visibility,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub(crate) property: Property,
    pub(crate) value: Value,
}

impl Declaration {
    #[must_use]
    pub(crate) fn new(property: Property, value: Value) -> Self {
        Self { property, value }
    }

    pub fn try_new(property: Property, value: Value) -> Result<Self> {
        property.validate_value(&value)?;
        Ok(Self::new(property, value))
    }

    #[must_use]
    pub const fn property(&self) -> Property {
        self.property
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedDeclaration(Declaration);

impl TypedDeclaration {
    #[must_use]
    pub fn width(width: DimensionLength) -> Self {
        Self(Declaration::new(
            Property::Width,
            Value::Length(width.into_length()),
        ))
    }

    #[must_use]
    pub fn opacity(opacity: Opacity) -> Self {
        Self(Declaration::new(
            Property::Opacity,
            Value::Number(opacity.get()),
        ))
    }

    #[must_use]
    pub fn text_color(color: Color) -> Self {
        Self(Declaration::new(Property::Color, Value::Color(color)))
    }

    fn into_declaration(self) -> Declaration {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Fingerprint(u64);

impl Fingerprint {
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Declarations {
    values: Vec<Declaration>,
}

impl Declarations {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_typed(declarations: impl IntoIterator<Item = TypedDeclaration>) -> Result<Self> {
        let mut values = Self::new();
        for declaration in declarations {
            let Declaration { property, value } = declaration.into_declaration();
            values.try_insert(property, value)?;
        }
        Ok(values)
    }

    fn set(mut self, property: Property, value: Value) -> Self {
        self.insert(property, value);
        self
    }

    pub fn try_set(mut self, property: Property, value: Value) -> Result<Self> {
        self.try_insert(property, value)?;
        Ok(self)
    }

    fn insert(&mut self, property: Property, value: Value) -> &mut Self {
        for declaration in canonical_declarations(property, value) {
            self.insert_canonical(declaration.property, declaration.value);
        }
        self
    }

    fn insert_canonical(&mut self, property: Property, value: Value) {
        if let Some(existing) = self
            .values
            .iter_mut()
            .find(|declaration| declaration.property == property)
        {
            existing.value = value;
        } else {
            self.values.push(Declaration::new(property, value));
        }
    }

    pub fn try_insert(&mut self, property: Property, value: Value) -> Result<&mut Self> {
        property.validate_value(&value)?;
        Ok(self.insert(property, value))
    }

    #[must_use]
    pub fn get(&self, property: Property) -> Option<&Value> {
        self.values
            .iter()
            .find(|declaration| declaration.property == property)
            .map(|declaration| &declaration.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Declaration> {
        self.values.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn fingerprint(&self) -> Fingerprint {
        let mut hasher = DefaultHasher::new();
        for declaration in &self.values {
            declaration.property.hash(&mut hasher);
            hash_value(&declaration.value, &mut hasher);
        }
        Fingerprint(hasher.finish())
    }

    #[must_use]
    pub fn bg(self, color: Color) -> Self {
        self.set(Property::Background, Value::Color(color))
    }

    #[must_use]
    pub fn text_color(self, color: Color) -> Self {
        self.set(Property::Color, Value::Color(color))
    }

    #[must_use]
    pub fn width(self, width: Length) -> Self {
        self.set(Property::Width, Value::Length(width))
    }

    #[must_use]
    pub fn height(self, height: Length) -> Self {
        self.set(Property::Height, Value::Length(height))
    }

    #[must_use]
    pub fn padding(self, edges: Edges) -> Self {
        self.set(Property::Padding, Value::Edges(edges))
    }

    #[must_use]
    pub fn margin(self, edges: Edges) -> Self {
        self.set(Property::Margin, Value::Edges(edges))
    }

    #[must_use]
    pub fn radius(self, corners: Corners) -> Self {
        self.set(Property::Radius, Value::Corners(corners))
    }

    #[must_use]
    pub fn shadow(self, shadow: Shadow) -> Self {
        self.set(Property::Shadow, Value::ShadowList(vec![shadow]))
    }

    #[must_use]
    pub fn border_width(self, edges: Edges) -> Self {
        self.set(Property::BorderWidth, Value::Edges(edges))
    }

    #[must_use]
    pub fn border_color(self, color: Color) -> Self {
        self.set(Property::BorderColor, Value::Color(color))
    }

    #[must_use]
    pub fn opacity(self, opacity: f32) -> Self {
        self.set(Property::Opacity, Value::Number(opacity))
    }

    #[must_use]
    pub fn font_size(self, size: Length) -> Self {
        self.set(Property::FontSize, Value::Length(size))
    }

    #[must_use]
    pub fn cursor(self, cursor: Cursor) -> Self {
        self.set(Property::Cursor, Value::Cursor(cursor))
    }

    #[must_use]
    pub fn pointer_events(self, pointer_events: PointerEvents) -> Self {
        self.set(
            Property::PointerEvents,
            Value::PointerEvents(pointer_events),
        )
    }

    #[must_use]
    pub fn visibility(self, visibility: Visibility) -> Self {
        self.set(Property::Visibility, Value::Visibility(visibility))
    }

    #[must_use]
    pub fn transform(self, transform: Transform) -> Self {
        self.set(Property::Transform, Value::Transform(transform))
    }

    #[must_use]
    pub fn transform_origin(self, origin: Size) -> Self {
        self.set(Property::TransformOrigin, Value::Size(origin))
    }

    #[must_use]
    pub fn transition_properties(self, properties: Vec<Property>) -> Self {
        self.set(
            Property::TransitionProperty,
            Value::PropertyList(properties),
        )
    }

    #[must_use]
    pub fn transition_duration(self, duration: f32) -> Self {
        self.set(Property::TransitionDuration, Value::Number(duration))
    }

    #[must_use]
    pub fn transition_delay(self, delay: f32) -> Self {
        self.set(Property::TransitionDelay, Value::Number(delay))
    }

    #[must_use]
    pub fn display(self, display: Display) -> Self {
        self.set(Property::Display, Value::Display(display))
    }

    #[must_use]
    pub fn grid_template_rows(self, tracks: GridTrackList) -> Self {
        self.set(Property::GridTemplateRows, Value::GridTrackList(tracks))
    }

    #[must_use]
    pub fn grid_template_columns(self, tracks: GridTrackList) -> Self {
        self.set(Property::GridTemplateColumns, Value::GridTrackList(tracks))
    }

    #[must_use]
    pub fn grid_template_areas(self, areas: GridTemplateAreas) -> Self {
        self.set(Property::GridTemplateAreas, Value::GridTemplateAreas(areas))
    }

    #[must_use]
    pub fn grid_template(self, template: GridTemplate) -> Self {
        self.set(Property::GridTemplate, Value::GridTemplate(template))
    }

    #[must_use]
    pub fn grid_auto_rows(self, tracks: GridTrackList) -> Self {
        self.set(Property::GridAutoRows, Value::GridTrackList(tracks))
    }

    #[must_use]
    pub fn grid_auto_columns(self, tracks: GridTrackList) -> Self {
        self.set(Property::GridAutoColumns, Value::GridTrackList(tracks))
    }

    #[must_use]
    pub fn grid_auto_flow(self, flow: GridAutoFlow) -> Self {
        self.set(Property::GridAutoFlow, Value::GridAutoFlow(flow))
    }

    #[must_use]
    pub fn grid_flow_tolerance(self, tolerance: GridFlowTolerance) -> Self {
        self.set(
            Property::GridFlowTolerance,
            Value::GridFlowTolerance(tolerance),
        )
    }

    #[must_use]
    pub fn grid(self, grid: GridDefinition) -> Self {
        self.set(Property::Grid, Value::GridDefinition(grid))
    }

    #[must_use]
    pub fn grid_row_start(self, line: GridLine) -> Self {
        self.set(Property::GridRowStart, Value::GridLine(line))
    }

    #[must_use]
    pub fn grid_row_end(self, line: GridLine) -> Self {
        self.set(Property::GridRowEnd, Value::GridLine(line))
    }

    #[must_use]
    pub fn grid_column_start(self, line: GridLine) -> Self {
        self.set(Property::GridColumnStart, Value::GridLine(line))
    }

    #[must_use]
    pub fn grid_column_end(self, line: GridLine) -> Self {
        self.set(Property::GridColumnEnd, Value::GridLine(line))
    }

    #[must_use]
    pub fn grid_row(self, placement: GridPlacement) -> Self {
        self.set(Property::GridRow, Value::GridPlacement(placement))
    }

    #[must_use]
    pub fn grid_column(self, placement: GridPlacement) -> Self {
        self.set(Property::GridColumn, Value::GridPlacement(placement))
    }

    #[must_use]
    pub fn grid_area(self, area: GridAreaPlacement) -> Self {
        self.set(Property::GridArea, Value::GridAreaPlacement(area))
    }

    #[must_use]
    pub fn background(&self) -> Option<Color> {
        match self.get(Property::Background) {
            Some(Value::Color(color)) => Some(*color),
            _ => None,
        }
    }

    #[must_use]
    pub fn padding_edges(&self) -> Option<Edges> {
        match self.get(Property::Padding) {
            Some(Value::Edges(edges)) => Some(edges.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn margin_edges(&self) -> Option<Edges> {
        match self.get(Property::Margin) {
            Some(Value::Edges(edges)) => Some(edges.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn opacity_number(&self) -> Option<f32> {
        match self.get(Property::Opacity) {
            Some(Value::Number(opacity)) => Some(*opacity),
            _ => None,
        }
    }

    #[must_use]
    pub fn font_size_length(&self) -> Option<Length> {
        match self.get(Property::FontSize) {
            Some(Value::Length(size)) => Some(size.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn cursor_kind(&self) -> Option<Cursor> {
        match self.get(Property::Cursor) {
            Some(Value::Cursor(cursor)) => Some(*cursor),
            _ => None,
        }
    }

    #[must_use]
    pub fn pointer_events_kind(&self) -> Option<PointerEvents> {
        match self.get(Property::PointerEvents) {
            Some(Value::PointerEvents(pointer_events)) => Some(*pointer_events),
            _ => None,
        }
    }

    #[must_use]
    pub fn width_length(&self) -> Option<Length> {
        match self.get(Property::Width) {
            Some(Value::Length(length)) => Some(length.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn height_length(&self) -> Option<Length> {
        match self.get(Property::Height) {
            Some(Value::Length(length)) => Some(length.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn border_width_edges(&self) -> Option<Edges> {
        match self.get(Property::BorderWidth) {
            Some(Value::Edges(edges)) => Some(edges.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn visibility_state(&self) -> Option<Visibility> {
        match self.get(Property::Visibility) {
            Some(Value::Visibility(visibility)) => Some(*visibility),
            _ => None,
        }
    }

    #[must_use]
    pub fn transform_value(&self) -> Option<&Transform> {
        match self.get(Property::Transform) {
            Some(Value::Transform(transform)) => Some(transform),
            _ => None,
        }
    }

    #[must_use]
    pub fn transform_origin_size(&self) -> Option<Size> {
        match self.get(Property::TransformOrigin) {
            Some(Value::Size(origin)) => Some(origin.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn transition_property_list(&self) -> Option<&[Property]> {
        match self.get(Property::TransitionProperty) {
            Some(Value::PropertyList(properties)) => Some(properties),
            _ => None,
        }
    }

    #[must_use]
    pub fn transition_duration_number(&self) -> Option<f32> {
        match self.get(Property::TransitionDuration) {
            Some(Value::Number(duration)) => Some(*duration),
            _ => None,
        }
    }

    #[must_use]
    pub fn transition_delay_number(&self) -> Option<f32> {
        match self.get(Property::TransitionDelay) {
            Some(Value::Number(delay)) => Some(*delay),
            _ => None,
        }
    }
}

fn canonical_declarations(property: Property, value: Value) -> Vec<Declaration> {
    match (property, value) {
        (Property::MinSize, Value::Keyword(keyword)) => same_value_declarations(
            [Property::MinWidth, Property::MinHeight],
            Value::Keyword(keyword),
        ),
        (Property::MinSize, value) => vec![
            Declaration::new(Property::MinWidth, value.clone()),
            Declaration::new(Property::MinHeight, value),
        ],
        (Property::MaxSize, Value::Keyword(keyword)) => same_value_declarations(
            [Property::MaxWidth, Property::MaxHeight],
            Value::Keyword(keyword),
        ),
        (Property::MaxSize, value) => vec![
            Declaration::new(Property::MaxWidth, value.clone()),
            Declaration::new(Property::MaxHeight, value),
        ],
        (Property::Overflow, Value::Keyword(keyword)) => same_value_declarations(
            [Property::OverflowX, Property::OverflowY],
            Value::Keyword(keyword),
        ),
        (Property::Overflow, Value::OverflowAxes(axes)) => vec![
            Declaration::new(Property::OverflowX, Value::Overflow(axes.x)),
            Declaration::new(Property::OverflowY, Value::Overflow(axes.y)),
        ],
        (Property::Overflow, value) => vec![
            Declaration::new(Property::OverflowX, value.clone()),
            Declaration::new(Property::OverflowY, value),
        ],
        (Property::Align, Value::Keyword(keyword)) => same_value_declarations(
            [Property::AlignItems, Property::AlignSelf],
            Value::Keyword(keyword),
        ),
        (Property::Align, value) => vec![
            Declaration::new(Property::AlignItems, value.clone()),
            Declaration::new(Property::AlignSelf, value),
        ],
        (Property::Justify, Value::Keyword(keyword)) => same_value_declarations(
            [Property::JustifyItems, Property::JustifySelf],
            Value::Keyword(keyword),
        ),
        (Property::Justify, value) => vec![
            Declaration::new(Property::JustifyItems, value.clone()),
            Declaration::new(Property::JustifySelf, value),
        ],
        (Property::Gap, Value::Keyword(keyword)) => same_value_declarations(
            [Property::RowGap, Property::ColumnGap],
            Value::Keyword(keyword),
        ),
        (Property::Gap, value) => vec![
            Declaration::new(Property::RowGap, value.clone()),
            Declaration::new(Property::ColumnGap, value),
        ],
        (Property::GridTemplate, Value::Keyword(keyword)) => same_value_declarations(
            [
                Property::GridTemplateRows,
                Property::GridTemplateColumns,
                Property::GridTemplateAreas,
            ],
            Value::Keyword(keyword),
        ),
        (Property::GridTemplate, Value::GridTemplate(template)) => {
            let GridTemplate {
                rows,
                columns,
                areas,
            } = template;
            vec![
                Declaration::new(Property::GridTemplateRows, Value::GridTrackList(rows)),
                Declaration::new(Property::GridTemplateColumns, Value::GridTrackList(columns)),
                Declaration::new(Property::GridTemplateAreas, Value::GridTemplateAreas(areas)),
            ]
        }
        (Property::Grid, Value::Keyword(keyword)) => same_value_declarations(
            [
                Property::GridTemplateRows,
                Property::GridTemplateColumns,
                Property::GridTemplateAreas,
                Property::GridAutoRows,
                Property::GridAutoColumns,
                Property::GridAutoFlow,
            ],
            Value::Keyword(keyword),
        ),
        (Property::Grid, Value::GridDefinition(grid)) => {
            let GridDefinition {
                template,
                auto_rows,
                auto_columns,
                auto_flow,
            } = grid;
            let GridTemplate {
                rows,
                columns,
                areas,
            } = template;
            vec![
                Declaration::new(Property::GridTemplateRows, Value::GridTrackList(rows)),
                Declaration::new(Property::GridTemplateColumns, Value::GridTrackList(columns)),
                Declaration::new(Property::GridTemplateAreas, Value::GridTemplateAreas(areas)),
                Declaration::new(Property::GridAutoRows, Value::GridTrackList(auto_rows)),
                Declaration::new(
                    Property::GridAutoColumns,
                    Value::GridTrackList(auto_columns),
                ),
                Declaration::new(Property::GridAutoFlow, Value::GridAutoFlow(auto_flow)),
            ]
        }
        (Property::GridRow, Value::Keyword(keyword)) => same_value_declarations(
            [Property::GridRowStart, Property::GridRowEnd],
            Value::Keyword(keyword),
        ),
        (Property::GridRow, Value::GridPlacement(placement)) => {
            let GridPlacement { start, end } = placement;
            let end = grid_placement_end_for_shorthand(&start, end);
            vec![
                Declaration::new(Property::GridRowStart, Value::GridLine(start)),
                Declaration::new(Property::GridRowEnd, Value::GridLine(end)),
            ]
        }
        (Property::GridColumn, Value::Keyword(keyword)) => same_value_declarations(
            [Property::GridColumnStart, Property::GridColumnEnd],
            Value::Keyword(keyword),
        ),
        (Property::GridColumn, Value::GridPlacement(placement)) => {
            let GridPlacement { start, end } = placement;
            let end = grid_placement_end_for_shorthand(&start, end);
            vec![
                Declaration::new(Property::GridColumnStart, Value::GridLine(start)),
                Declaration::new(Property::GridColumnEnd, Value::GridLine(end)),
            ]
        }
        (Property::GridArea, Value::Keyword(keyword)) => same_value_declarations(
            [
                Property::GridRowStart,
                Property::GridColumnStart,
                Property::GridRowEnd,
                Property::GridColumnEnd,
            ],
            Value::Keyword(keyword),
        ),
        (Property::GridArea, Value::GridAreaPlacement(area)) => {
            let GridAreaPlacement {
                row_start,
                column_start,
                row_end,
                column_end,
            } = area;
            let column_start = if matches!(column_start, GridLine::Auto) {
                grid_area_omitted_line(&row_start)
            } else {
                column_start
            };
            let row_end = if matches!(row_end, GridLine::Auto) {
                grid_area_omitted_line(&row_start)
            } else {
                row_end
            };
            let column_end = if matches!(column_end, GridLine::Auto) {
                grid_area_omitted_line(&column_start)
            } else {
                column_end
            };
            vec![
                Declaration::new(Property::GridRowStart, Value::GridLine(row_start)),
                Declaration::new(Property::GridColumnStart, Value::GridLine(column_start)),
                Declaration::new(Property::GridRowEnd, Value::GridLine(row_end)),
                Declaration::new(Property::GridColumnEnd, Value::GridLine(column_end)),
            ]
        }
        (property, value) => vec![Declaration::new(property, value)],
    }
}

fn grid_placement_end_for_shorthand(start: &GridLine, end: GridLine) -> GridLine {
    match (&start, end) {
        (GridLine::BareIdent(name), GridLine::Auto) => GridLine::BareIdent(name.clone()),
        (_, end) => end,
    }
}

fn grid_area_omitted_line(reference: &GridLine) -> GridLine {
    match reference {
        GridLine::BareIdent(name) => GridLine::BareIdent(name.clone()),
        _ => GridLine::Auto,
    }
}

fn same_value_declarations<const N: usize>(
    properties: [Property; N],
    value: Value,
) -> Vec<Declaration> {
    properties
        .into_iter()
        .map(|property| Declaration::new(property, value.clone()))
        .collect()
}

pub(crate) fn hash_value(value: &Value, state: &mut DefaultHasher) {
    match value {
        Value::Keyword(value) => {
            0u8.hash(state);
            value.hash(state);
        }
        Value::Display(value) => {
            20u8.hash(state);
            value.hash(state);
        }
        Value::BoxSizing(value) => {
            26u8.hash(state);
            value.hash(state);
        }
        Value::Position(value) => {
            27u8.hash(state);
            value.hash(state);
        }
        Value::Direction(value) => {
            28u8.hash(state);
            value.hash(state);
        }
        Value::Overflow(value) => {
            29u8.hash(state);
            value.hash(state);
        }
        Value::OverflowAxes(value) => {
            38u8.hash(state);
            value.hash(state);
        }
        Value::Float(value) => {
            30u8.hash(state);
            value.hash(state);
        }
        Value::Clear(value) => {
            31u8.hash(state);
            value.hash(state);
        }
        Value::TextAlign(value) => {
            32u8.hash(state);
            value.hash(state);
        }
        Value::WritingMode(value) => {
            33u8.hash(state);
            value.hash(state);
        }
        Value::FlexDirection(value) => {
            34u8.hash(state);
            value.hash(state);
        }
        Value::FlexWrap(value) => {
            35u8.hash(state);
            value.hash(state);
        }
        Value::AlignItems(value) => {
            36u8.hash(state);
            value.hash(state);
        }
        Value::AlignContent(value) => {
            37u8.hash(state);
            value.hash(state);
        }
        Value::Number(value) => {
            1u8.hash(state);
            hash_f32(*value, state);
        }
        Value::Length(value) => {
            2u8.hash(state);
            hash_length(value, state);
        }
        Value::Size(value) => {
            3u8.hash(state);
            hash_length(&value.width, state);
            hash_length(&value.height, state);
        }
        Value::Edges(value) => {
            4u8.hash(state);
            hash_length(&value.top, state);
            hash_length(&value.right, state);
            hash_length(&value.bottom, state);
            hash_length(&value.left, state);
        }
        Value::GridTrackList(value) => {
            16u8.hash(state);
            hash_grid_track_list(value, state);
        }
        Value::GridTemplateAreas(value) => {
            21u8.hash(state);
            hash_grid_template_areas(value, state);
        }
        Value::GridTemplate(value) => {
            23u8.hash(state);
            hash_grid_template(value, state);
        }
        Value::GridDefinition(value) => {
            24u8.hash(state);
            hash_grid_definition(value, state);
        }
        Value::GridLine(value) => {
            22u8.hash(state);
            hash_grid_line(value, state);
        }
        Value::GridPlacement(value) => {
            17u8.hash(state);
            hash_grid_placement(value, state);
        }
        Value::GridAreaPlacement(value) => {
            25u8.hash(state);
            hash_grid_area_placement(value, state);
        }
        Value::GridAutoFlow(value) => {
            18u8.hash(state);
            value.hash(state);
        }
        Value::GridFlowTolerance(value) => {
            39u8.hash(state);
            hash_grid_flow_tolerance(value, state);
        }
        Value::Color(value) => {
            5u8.hash(state);
            hash_color(*value, state);
        }
        Value::Corners(value) => {
            6u8.hash(state);
            hash_length(&value.top_left, state);
            hash_length(&value.top_right, state);
            hash_length(&value.bottom_right, state);
            hash_length(&value.bottom_left, state);
        }
        Value::FontFamilyList(value) => {
            7u8.hash(state);
            value.hash(state);
        }
        Value::AnimationNameList(value) => {
            40u8.hash(state);
            value.hash(state);
        }
        Value::PropertyList(value) => {
            8u8.hash(state);
            value.hash(state);
        }
        Value::ShadowList(value) => {
            9u8.hash(state);
            value.len().hash(state);
            for shadow in value {
                hash_length(&shadow.x, state);
                hash_length(&shadow.y, state);
                hash_length(&shadow.blur, state);
                hash_length(&shadow.spread, state);
                hash_color(shadow.color, state);
                shadow.inset.hash(state);
            }
        }
        Value::Stroke(value) => {
            10u8.hash(state);
            hash_length(&value.width, state);
            hash_color(value.color, state);
            value.style.hash(state);
            value.sides.top.hash(state);
            value.sides.right.hash(state);
            value.sides.bottom.hash(state);
            value.sides.left.hash(state);
            if let Some(dash) = value.dash {
                true.hash(state);
                hash_f32(dash.density, state);
                hash_f32(dash.phase, state);
                dash.rounded.hash(state);
                dash.circular.hash(state);
            } else {
                false.hash(state);
            }
            value.align.hash(state);
        }
        Value::Text(value) => {
            11u8.hash(state);
            value.font_family.hash(state);
            hash_length(&value.font_size, state);
            value.font_weight.hash(state);
            hash_slant(value.font_style, state);
            hash_length(&value.line_height, state);
            hash_color(value.color, state);
            value.alignment.hash(state);
            value.wrap.hash(state);
            value.white_space.hash(state);
            value.word_break.hash(state);
            value.overflow_wrap.hash(state);
            hash_decoration(value.underline, state);
            hash_decoration(value.strikethrough, state);
            hash_color(value.selection_color, state);
        }
        Value::Transform(value) => {
            12u8.hash(state);
            value.operations.len().hash(state);
            for operation in &value.operations {
                hash_transform_op(operation, state);
            }
        }
        Value::Cursor(value) => {
            13u8.hash(state);
            value.hash(state);
        }
        Value::PointerEvents(value) => {
            14u8.hash(state);
            value.hash(state);
        }
        Value::Visibility(value) => {
            15u8.hash(state);
            value.hash(state);
        }
    }
}

fn hash_grid_track_list(value: &GridTrackList, state: &mut DefaultHasher) {
    value.components.len().hash(state);
    for component in &value.components {
        hash_grid_track_component(component, state);
    }
}

fn hash_grid_template_areas(value: &GridTemplateAreas, state: &mut DefaultHasher) {
    value.rows.len().hash(state);
    for row in &value.rows {
        row.cells.len().hash(state);
        for cell in &row.cells {
            cell.hash(state);
        }
    }
}

fn hash_grid_template(value: &GridTemplate, state: &mut DefaultHasher) {
    hash_grid_track_list(&value.rows, state);
    hash_grid_track_list(&value.columns, state);
    hash_grid_template_areas(&value.areas, state);
}

fn hash_grid_definition(value: &GridDefinition, state: &mut DefaultHasher) {
    hash_grid_template(&value.template, state);
    hash_grid_track_list(&value.auto_rows, state);
    hash_grid_track_list(&value.auto_columns, state);
    value.auto_flow.hash(state);
}

fn hash_grid_track_component(component: &GridTrackComponent, state: &mut DefaultHasher) {
    match component {
        GridTrackComponent::Track(track) => {
            0u8.hash(state);
            hash_track_sizing(track, state);
        }
        GridTrackComponent::Repeat(repeat) => {
            1u8.hash(state);
            match repeat.count {
                TrackRepeatCount::Count(count) => {
                    0u8.hash(state);
                    count.hash(state);
                }
                TrackRepeatCount::AutoFill => 1u8.hash(state),
                TrackRepeatCount::AutoFit => 2u8.hash(state),
            }
            repeat.components.len().hash(state);
            for component in &repeat.components {
                hash_grid_track_component(component, state);
            }
        }
        GridTrackComponent::LineNames(names) => {
            2u8.hash(state);
            names.hash(state);
        }
        GridTrackComponent::Subgrid(subgrid) => {
            3u8.hash(state);
            subgrid.name_components().len().hash(state);
            for component in subgrid.name_components() {
                match component {
                    SubgridLineNameComponent::LineNames(names) => {
                        0u8.hash(state);
                        names.hash(state);
                    }
                    SubgridLineNameComponent::Repeat {
                        count,
                        line_name_sets,
                    } => {
                        1u8.hash(state);
                        count.hash(state);
                        line_name_sets.hash(state);
                    }
                }
            }
        }
    }
}

fn hash_grid_flow_tolerance(value: &GridFlowTolerance, state: &mut DefaultHasher) {
    match value {
        GridFlowTolerance::Normal => 0u8.hash(state),
        GridFlowTolerance::Length(length) => {
            1u8.hash(state);
            hash_length(length, state);
        }
        GridFlowTolerance::Percent(value) => {
            2u8.hash(state);
            hash_f32(*value, state);
        }
        GridFlowTolerance::Infinite => 3u8.hash(state),
    }
}

fn hash_track_sizing(value: &TrackSizing, state: &mut DefaultHasher) {
    hash_min_track_sizing(&value.min, state);
    hash_max_track_sizing(&value.max, state);
}

fn hash_min_track_sizing(value: &MinTrackSizing, state: &mut DefaultHasher) {
    match value {
        MinTrackSizing::Length(length) => {
            0u8.hash(state);
            hash_length(length, state);
        }
        MinTrackSizing::Auto => 1u8.hash(state),
        MinTrackSizing::MinContent => 2u8.hash(state),
        MinTrackSizing::MaxContent => 3u8.hash(state),
    }
}

fn hash_max_track_sizing(value: &MaxTrackSizing, state: &mut DefaultHasher) {
    match value {
        MaxTrackSizing::Length(length) => {
            0u8.hash(state);
            hash_length(length, state);
        }
        MaxTrackSizing::Flex(flex) => {
            1u8.hash(state);
            hash_f32(*flex, state);
        }
        MaxTrackSizing::Auto => 2u8.hash(state),
        MaxTrackSizing::MinContent => 3u8.hash(state),
        MaxTrackSizing::MaxContent => 4u8.hash(state),
        MaxTrackSizing::FitContent(length) => {
            5u8.hash(state);
            hash_length(length, state);
        }
    }
}

fn hash_grid_placement(value: &GridPlacement, state: &mut DefaultHasher) {
    hash_grid_line(&value.start, state);
    hash_grid_line(&value.end, state);
}

fn hash_grid_area_placement(value: &GridAreaPlacement, state: &mut DefaultHasher) {
    hash_grid_line(&value.row_start, state);
    hash_grid_line(&value.column_start, state);
    hash_grid_line(&value.row_end, state);
    hash_grid_line(&value.column_end, state);
}

fn hash_grid_line(value: &GridLine, state: &mut DefaultHasher) {
    match value {
        GridLine::Auto => 0u8.hash(state),
        GridLine::Line(line) => {
            1u8.hash(state);
            line.hash(state);
        }
        GridLine::Span(span) => {
            2u8.hash(state);
            span.hash(state);
        }
        GridLine::BareIdent(name) => {
            3u8.hash(state);
            name.hash(state);
        }
        GridLine::NamedLine { name, index } => {
            4u8.hash(state);
            name.hash(state);
            index.hash(state);
        }
        GridLine::NamedSpan { name, index } => {
            5u8.hash(state);
            name.hash(state);
            index.hash(state);
        }
    }
}

fn hash_length(value: &super::Length, state: &mut DefaultHasher) {
    match value {
        super::Length::Normal => 7u8.hash(state),
        super::Length::Px(value) => {
            0u8.hash(state);
            hash_f32(*value, state);
        }
        super::Length::Percent(value) => {
            1u8.hash(state);
            hash_f32(*value, state);
        }
        super::Length::Calc(value) => {
            8u8.hash(state);
            hash_calc_length(value, state);
        }
        super::Length::Fill => 2u8.hash(state),
        super::Length::Fit => 3u8.hash(state),
        super::Length::MinContent => 4u8.hash(state),
        super::Length::MaxContent => 5u8.hash(state),
        super::Length::Auto => 6u8.hash(state),
    }
}

fn hash_calc_length(value: &CalcLength, state: &mut DefaultHasher) {
    match value {
        CalcLength::Px(value) => {
            0u8.hash(state);
            hash_f32(*value, state);
        }
        CalcLength::Percent(value) => {
            1u8.hash(state);
            hash_f32(*value, state);
        }
        CalcLength::Sum(terms) => {
            2u8.hash(state);
            terms.len().hash(state);
            for term in terms {
                hash_calc_term(term, state);
            }
        }
    }
}

fn hash_calc_term(term: &CalcLengthTerm, state: &mut DefaultHasher) {
    term.operator.hash(state);
    hash_calc_length(&term.value, state);
}

fn hash_transform_op(value: &super::TransformOp, state: &mut DefaultHasher) {
    match value {
        super::TransformOp::Translate { x, y } => {
            0u8.hash(state);
            hash_length(x, state);
            hash_length(y, state);
        }
        super::TransformOp::Scale { x, y } => {
            1u8.hash(state);
            hash_f32(*x, state);
            hash_f32(*y, state);
        }
        super::TransformOp::Rotate { radians } => {
            2u8.hash(state);
            hash_f32(*radians, state);
        }
    }
}

fn hash_color(value: Color, state: &mut DefaultHasher) {
    hash_f32(value.r, state);
    hash_f32(value.g, state);
    hash_f32(value.b, state);
    hash_f32(value.a, state);
}

fn hash_slant(value: surgeist_text::Slant, state: &mut DefaultHasher) {
    match value {
        surgeist_text::Slant::Normal => 0u8.hash(state),
        surgeist_text::Slant::Italic => 1u8.hash(state),
        surgeist_text::Slant::Oblique(angle) => {
            2u8.hash(state);
            angle.map(f32::to_bits).hash(state);
        }
    }
}

fn hash_decoration(value: surgeist_text::Decoration, state: &mut DefaultHasher) {
    value.enabled.hash(state);
    value.offset.map(f32::to_bits).hash(state);
    value.size.map(f32::to_bits).hash(state);
    if let Some(brush) = value.brush {
        true.hash(state);
        hash_f32(brush.r, state);
        hash_f32(brush.g, state);
        hash_f32(brush.b, state);
        hash_f32(brush.a, state);
    } else {
        false.hash(state);
    }
}

fn hash_f32(value: f32, state: &mut DefaultHasher) {
    value.to_bits().hash(state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BoxSizing, CalcLength, CalcLengthTerm, ErrorCode, GridFlowTolerance};

    fn value_hash(value: &Value) -> u64 {
        let mut hasher = DefaultHasher::new();
        hash_value(value, &mut hasher);
        hasher.finish()
    }

    #[test]
    fn value_hash_distinguishes_grid_flow_tolerance_from_box_sizing() {
        assert_ne!(
            value_hash(&Value::GridFlowTolerance(GridFlowTolerance::Normal)),
            value_hash(&Value::BoxSizing(BoxSizing::ContentBox))
        );
    }

    #[test]
    fn value_hash_distinguishes_calc_lengths() {
        let calc_a = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::px(20.0)),
            [CalcLengthTerm::add(CalcLength::percent(10.0))],
        );
        let calc_b = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::px(21.0)),
            [CalcLengthTerm::add(CalcLength::percent(10.0))],
        );

        assert_ne!(
            value_hash(&Value::Length(Length::Calc(calc_a))),
            value_hash(&Value::Length(Length::Calc(calc_b)))
        );
    }

    #[test]
    fn calc_lengths_validate_through_length_properties() {
        let calc = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::px(20.0)),
            [CalcLengthTerm::add(CalcLength::percent(10.0))],
        );

        Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap();
    }

    #[test]
    fn calc_px_only_negative_results_are_rejected_for_non_negative_properties() {
        let calc = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::px(0.0)),
            [CalcLengthTerm::sub(CalcLength::px(1.0))],
        );

        let error =
            Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn calc_percent_only_negative_results_are_rejected_for_non_negative_properties() {
        let calc = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::percent(0.0)),
            [CalcLengthTerm::sub(CalcLength::percent(1.0))],
        );

        let error =
            Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn mixed_all_nonpositive_calc_lengths_are_rejected_for_non_negative_properties() {
        let calc = CalcLength::sum(
            CalcLengthTerm::sub(CalcLength::px(1.0)),
            [CalcLengthTerm::sub(CalcLength::percent(1.0))],
        );

        let error =
            Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn indefinite_mixed_calc_lengths_remain_valid_for_non_negative_properties() {
        let calc = CalcLength::sum(
            CalcLengthTerm::sub(CalcLength::px(1.0)),
            [CalcLengthTerm::add(CalcLength::percent(10.0))],
        );

        Declaration::try_new(Property::Width, Value::Length(Length::Calc(calc))).unwrap();
    }

    #[test]
    fn grid_flow_tolerance_calc_reaches_property_domain_validation() {
        let calc = CalcLength::sum(
            CalcLengthTerm::add(CalcLength::px(8.0)),
            [CalcLengthTerm::add(CalcLength::percent(2.0))],
        );

        let error = Declaration::try_new(
            Property::GridFlowTolerance,
            Value::GridFlowTolerance(GridFlowTolerance::Length(Length::Calc(calc))),
        )
        .unwrap_err();
        assert!(error.to_string().contains("grid flow tolerance length"));
    }
}
