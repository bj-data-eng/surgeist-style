use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{
    AspectRatio, CalcLength, CalcLengthTerm, Color, ContentVisibility, Corners, Cursor,
    DimensionLength, Display, DurationSeconds, Edges, FlexFactor, GridAreaPlacement, GridAutoFlow,
    GridDefinition, GridFlowTolerance, GridLine, GridPlacement, GridTemplate, GridTemplateAreas,
    GridTrackComponent, GridTrackList, LayoutPosition, Length, MaxTrackSizing, MinTrackSizing,
    Opacity, Order, PointerEvents, Property, Result, ScrollbarWidth, Shadow, Size,
    SubgridLineNameComponent, TextSlant, TrackRepeatCount, TrackSizing, Transform, Value,
    Visibility, ZIndex,
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

    pub fn try_text_color(color: Color) -> Result<Self> {
        Ok(Self(Declaration::try_new(
            Property::Color,
            Value::Color(color),
        )?))
    }

    #[must_use]
    pub fn transition_duration(duration: DurationSeconds) -> Self {
        Self(Declaration::new(
            Property::TransitionDuration,
            Value::Number(duration.get()),
        ))
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
        self.insert_validated(canonical_declarations(property, value));
        self
    }

    fn insert_validated(&mut self, declarations: Vec<Declaration>) -> &mut Self {
        for declaration in declarations {
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
        let declarations = canonical_declarations(property, value);
        for declaration in &declarations {
            declaration.property.validate_value(&declaration.value)?;
        }
        Ok(self.insert_validated(declarations))
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

    pub fn try_bg(self, color: Color) -> Result<Self> {
        self.try_set(Property::Background, Value::Color(color))
    }

    pub fn try_text_color(self, color: Color) -> Result<Self> {
        self.try_set(Property::Color, Value::Color(color))
    }

    #[must_use]
    pub fn width(self, width: DimensionLength) -> Self {
        self.set(Property::Width, Value::Length(width.into_length()))
    }

    #[must_use]
    pub fn height(self, height: DimensionLength) -> Self {
        self.set(Property::Height, Value::Length(height.into_length()))
    }

    pub fn try_padding(self, edges: Edges) -> Result<Self> {
        self.try_set(Property::Padding, Value::Edges(edges))
    }

    pub fn try_margin(self, edges: Edges) -> Result<Self> {
        self.try_set(Property::Margin, Value::Edges(edges))
    }

    pub fn try_inset(self, edges: Edges) -> Result<Self> {
        self.try_set(Property::Inset, Value::Edges(edges))
    }

    pub fn try_inset_top(self, value: Length) -> Result<Self> {
        self.try_set(Property::Top, Value::Length(value))
    }

    pub fn try_inset_right(self, value: Length) -> Result<Self> {
        self.try_set(Property::Right, Value::Length(value))
    }

    pub fn try_inset_bottom(self, value: Length) -> Result<Self> {
        self.try_set(Property::Bottom, Value::Length(value))
    }

    pub fn try_inset_left(self, value: Length) -> Result<Self> {
        self.try_set(Property::Left, Value::Length(value))
    }

    pub fn try_margin_top(self, value: Length) -> Result<Self> {
        self.try_set(Property::MarginTop, Value::Length(value))
    }

    pub fn try_margin_right(self, value: Length) -> Result<Self> {
        self.try_set(Property::MarginRight, Value::Length(value))
    }

    pub fn try_margin_bottom(self, value: Length) -> Result<Self> {
        self.try_set(Property::MarginBottom, Value::Length(value))
    }

    pub fn try_margin_left(self, value: Length) -> Result<Self> {
        self.try_set(Property::MarginLeft, Value::Length(value))
    }

    pub fn try_padding_top(self, value: Length) -> Result<Self> {
        self.try_set(Property::PaddingTop, Value::Length(value))
    }

    pub fn try_padding_right(self, value: Length) -> Result<Self> {
        self.try_set(Property::PaddingRight, Value::Length(value))
    }

    pub fn try_padding_bottom(self, value: Length) -> Result<Self> {
        self.try_set(Property::PaddingBottom, Value::Length(value))
    }

    pub fn try_padding_left(self, value: Length) -> Result<Self> {
        self.try_set(Property::PaddingLeft, Value::Length(value))
    }

    pub fn try_radius(self, corners: Corners) -> Result<Self> {
        self.try_set(Property::Radius, Value::Corners(corners))
    }

    pub fn try_shadow(self, shadow: Shadow) -> Result<Self> {
        self.try_set(Property::Shadow, Value::ShadowList(vec![shadow]))
    }

    pub fn try_border_width(self, edges: Edges) -> Result<Self> {
        self.try_set(Property::BorderWidth, Value::Edges(edges))
    }

    pub fn try_border_top_width(self, value: Length) -> Result<Self> {
        self.try_set(Property::BorderTopWidth, Value::Length(value))
    }

    pub fn try_border_right_width(self, value: Length) -> Result<Self> {
        self.try_set(Property::BorderRightWidth, Value::Length(value))
    }

    pub fn try_border_bottom_width(self, value: Length) -> Result<Self> {
        self.try_set(Property::BorderBottomWidth, Value::Length(value))
    }

    pub fn try_border_left_width(self, value: Length) -> Result<Self> {
        self.try_set(Property::BorderLeftWidth, Value::Length(value))
    }

    pub fn try_border_color(self, color: Color) -> Result<Self> {
        self.try_set(Property::BorderColor, Value::Color(color))
    }

    #[must_use]
    pub fn opacity(self, opacity: Opacity) -> Self {
        self.set(Property::Opacity, Value::Number(opacity.get()))
    }

    pub fn try_font_size(self, size: Length) -> Result<Self> {
        self.try_set(Property::FontSize, Value::Length(size))
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
    pub fn position(self, position: LayoutPosition) -> Self {
        self.set(Property::Position, Value::Position(position))
    }

    #[must_use]
    pub fn z_index(self, z_index: ZIndex) -> Self {
        self.set(Property::ZIndex, Value::ZIndex(z_index))
    }

    #[must_use]
    pub fn scrollbar_width(self, value: ScrollbarWidth) -> Self {
        self.set(Property::ScrollbarWidth, Value::ScrollbarWidth(value))
    }

    #[must_use]
    pub fn content_visibility(self, value: ContentVisibility) -> Self {
        self.set(Property::ContentVisibility, Value::ContentVisibility(value))
    }

    #[must_use]
    pub fn order(self, order: Order) -> Self {
        self.set(Property::Order, Value::Order(order))
    }

    pub fn try_flex_grow(self, value: FlexFactor) -> Result<Self> {
        self.try_set(Property::FlexGrow, Value::FlexFactor(value))
    }

    pub fn try_flex_shrink(self, value: FlexFactor) -> Result<Self> {
        self.try_set(Property::FlexShrink, Value::FlexFactor(value))
    }

    pub fn try_aspect_ratio(self, value: AspectRatio) -> Result<Self> {
        self.try_set(Property::AspectRatio, Value::AspectRatio(value))
    }

    pub fn try_transform(self, transform: Transform) -> Result<Self> {
        self.try_set(Property::Transform, Value::Transform(transform))
    }

    pub fn try_transform_origin(self, origin: Size) -> Result<Self> {
        self.try_set(Property::TransformOrigin, Value::Size(origin))
    }

    pub fn try_transition_properties(self, properties: Vec<Property>) -> Result<Self> {
        self.try_set(
            Property::TransitionProperty,
            Value::PropertyList(properties),
        )
    }

    #[must_use]
    pub fn transition_duration(self, duration: DurationSeconds) -> Self {
        self.set(Property::TransitionDuration, Value::Number(duration.get()))
    }

    #[must_use]
    pub fn transition_delay(self, delay: DurationSeconds) -> Self {
        self.set(Property::TransitionDelay, Value::Number(delay.get()))
    }

    #[must_use]
    pub fn display(self, display: Display) -> Self {
        self.set(Property::Display, Value::Display(display))
    }

    pub fn try_grid_template_rows(self, tracks: GridTrackList) -> Result<Self> {
        self.try_set(Property::GridTemplateRows, Value::GridTrackList(tracks))
    }

    pub fn try_grid_template_columns(self, tracks: GridTrackList) -> Result<Self> {
        self.try_set(Property::GridTemplateColumns, Value::GridTrackList(tracks))
    }

    pub fn try_grid_template_areas(self, areas: GridTemplateAreas) -> Result<Self> {
        self.try_set(Property::GridTemplateAreas, Value::GridTemplateAreas(areas))
    }

    pub fn try_grid_template(self, template: GridTemplate) -> Result<Self> {
        self.try_set(Property::GridTemplate, Value::GridTemplate(template))
    }

    pub fn try_grid_auto_rows(self, tracks: GridTrackList) -> Result<Self> {
        self.try_set(Property::GridAutoRows, Value::GridTrackList(tracks))
    }

    pub fn try_grid_auto_columns(self, tracks: GridTrackList) -> Result<Self> {
        self.try_set(Property::GridAutoColumns, Value::GridTrackList(tracks))
    }

    #[must_use]
    pub fn grid_auto_flow(self, flow: GridAutoFlow) -> Self {
        self.set(Property::GridAutoFlow, Value::GridAutoFlow(flow))
    }

    pub fn try_grid_flow_tolerance(self, tolerance: GridFlowTolerance) -> Result<Self> {
        self.try_set(
            Property::GridFlowTolerance,
            Value::GridFlowTolerance(tolerance),
        )
    }

    pub fn try_grid(self, grid: GridDefinition) -> Result<Self> {
        self.try_set(Property::Grid, Value::GridDefinition(grid))
    }

    pub fn try_grid_row_start(self, line: GridLine) -> Result<Self> {
        self.try_set(Property::GridRowStart, Value::GridLine(line))
    }

    pub fn try_grid_row_end(self, line: GridLine) -> Result<Self> {
        self.try_set(Property::GridRowEnd, Value::GridLine(line))
    }

    pub fn try_grid_column_start(self, line: GridLine) -> Result<Self> {
        self.try_set(Property::GridColumnStart, Value::GridLine(line))
    }

    pub fn try_grid_column_end(self, line: GridLine) -> Result<Self> {
        self.try_set(Property::GridColumnEnd, Value::GridLine(line))
    }

    pub fn try_grid_row(self, placement: GridPlacement) -> Result<Self> {
        self.try_set(Property::GridRow, Value::GridPlacement(placement))
    }

    pub fn try_grid_column(self, placement: GridPlacement) -> Result<Self> {
        self.try_set(Property::GridColumn, Value::GridPlacement(placement))
    }

    pub fn try_grid_area(self, area: GridAreaPlacement) -> Result<Self> {
        self.try_set(Property::GridArea, Value::GridAreaPlacement(area))
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
        edge_values(
            self,
            Property::PaddingTop,
            Property::PaddingRight,
            Property::PaddingBottom,
            Property::PaddingLeft,
        )
    }

    #[must_use]
    pub fn margin_edges(&self) -> Option<Edges> {
        edge_values(
            self,
            Property::MarginTop,
            Property::MarginRight,
            Property::MarginBottom,
            Property::MarginLeft,
        )
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
        edge_values(
            self,
            Property::BorderTopWidth,
            Property::BorderRightWidth,
            Property::BorderBottomWidth,
            Property::BorderLeftWidth,
        )
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

pub(crate) fn canonical_properties(property: Property) -> Vec<Property> {
    match property {
        Property::Inset => vec![
            Property::Top,
            Property::Right,
            Property::Bottom,
            Property::Left,
        ],
        Property::Margin => vec![
            Property::MarginTop,
            Property::MarginRight,
            Property::MarginBottom,
            Property::MarginLeft,
        ],
        Property::Padding => vec![
            Property::PaddingTop,
            Property::PaddingRight,
            Property::PaddingBottom,
            Property::PaddingLeft,
        ],
        Property::BorderWidth => vec![
            Property::BorderTopWidth,
            Property::BorderRightWidth,
            Property::BorderBottomWidth,
            Property::BorderLeftWidth,
        ],
        Property::MinSize => vec![Property::MinWidth, Property::MinHeight],
        Property::MaxSize => vec![Property::MaxWidth, Property::MaxHeight],
        Property::Overflow => vec![Property::OverflowX, Property::OverflowY],
        Property::Align => vec![Property::AlignItems, Property::AlignSelf],
        Property::Justify => vec![Property::JustifyItems, Property::JustifySelf],
        Property::Gap => vec![Property::RowGap, Property::ColumnGap],
        Property::GridTemplate => vec![
            Property::GridTemplateRows,
            Property::GridTemplateColumns,
            Property::GridTemplateAreas,
        ],
        Property::Grid => vec![
            Property::GridTemplateRows,
            Property::GridTemplateColumns,
            Property::GridTemplateAreas,
            Property::GridAutoRows,
            Property::GridAutoColumns,
            Property::GridAutoFlow,
        ],
        Property::GridRow => vec![Property::GridRowStart, Property::GridRowEnd],
        Property::GridColumn => vec![Property::GridColumnStart, Property::GridColumnEnd],
        Property::GridArea => vec![
            Property::GridRowStart,
            Property::GridColumnStart,
            Property::GridRowEnd,
            Property::GridColumnEnd,
        ],
        property => vec![property],
    }
}

pub(crate) fn canonical_declarations(property: Property, value: Value) -> Vec<Declaration> {
    match (property, value) {
        (Property::Inset, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::Inset),
            Value::Keyword(keyword),
        ),
        (Property::Inset, Value::Edges(edges)) => edge_declarations(
            edges,
            Property::Top,
            Property::Right,
            Property::Bottom,
            Property::Left,
        ),
        (Property::Margin, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::Margin),
            Value::Keyword(keyword),
        ),
        (Property::Margin, Value::Edges(edges)) => edge_declarations(
            edges,
            Property::MarginTop,
            Property::MarginRight,
            Property::MarginBottom,
            Property::MarginLeft,
        ),
        (Property::Padding, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::Padding),
            Value::Keyword(keyword),
        ),
        (Property::Padding, Value::Edges(edges)) => edge_declarations(
            edges,
            Property::PaddingTop,
            Property::PaddingRight,
            Property::PaddingBottom,
            Property::PaddingLeft,
        ),
        (Property::BorderWidth, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::BorderWidth),
            Value::Keyword(keyword),
        ),
        (Property::BorderWidth, Value::Edges(edges)) => edge_declarations(
            edges,
            Property::BorderTopWidth,
            Property::BorderRightWidth,
            Property::BorderBottomWidth,
            Property::BorderLeftWidth,
        ),
        (Property::MinSize, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::MinSize),
            Value::Keyword(keyword),
        ),
        (Property::MinSize, value) => vec![
            Declaration::new(Property::MinWidth, value.clone()),
            Declaration::new(Property::MinHeight, value),
        ],
        (Property::MaxSize, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::MaxSize),
            Value::Keyword(keyword),
        ),
        (Property::MaxSize, value) => vec![
            Declaration::new(Property::MaxWidth, value.clone()),
            Declaration::new(Property::MaxHeight, value),
        ],
        (Property::Overflow, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::Overflow),
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
            canonical_properties(Property::Align),
            Value::Keyword(keyword),
        ),
        (Property::Align, value) => vec![
            Declaration::new(Property::AlignItems, value.clone()),
            Declaration::new(Property::AlignSelf, value),
        ],
        (Property::Justify, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::Justify),
            Value::Keyword(keyword),
        ),
        (Property::Justify, value) => vec![
            Declaration::new(Property::JustifyItems, value.clone()),
            Declaration::new(Property::JustifySelf, value),
        ],
        (Property::Gap, Value::Keyword(keyword)) => {
            same_value_declarations(canonical_properties(Property::Gap), Value::Keyword(keyword))
        }
        (Property::Gap, value) => vec![
            Declaration::new(Property::RowGap, value.clone()),
            Declaration::new(Property::ColumnGap, value),
        ],
        (Property::GridTemplate, Value::Keyword(keyword)) => same_value_declarations(
            canonical_properties(Property::GridTemplate),
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
            canonical_properties(Property::Grid),
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
            canonical_properties(Property::GridRow),
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
            canonical_properties(Property::GridColumn),
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
            canonical_properties(Property::GridArea),
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

fn same_value_declarations(properties: Vec<Property>, value: Value) -> Vec<Declaration> {
    properties
        .into_iter()
        .map(|property| Declaration::new(property, value.clone()))
        .collect()
}

fn edge_declarations(
    edges: Edges,
    top: Property,
    right: Property,
    bottom: Property,
    left: Property,
) -> Vec<Declaration> {
    vec![
        Declaration::new(top, Value::Length(edges.top)),
        Declaration::new(right, Value::Length(edges.right)),
        Declaration::new(bottom, Value::Length(edges.bottom)),
        Declaration::new(left, Value::Length(edges.left)),
    ]
}

fn edge_values(
    declarations: &Declarations,
    top: Property,
    right: Property,
    bottom: Property,
    left: Property,
) -> Option<Edges> {
    Some(Edges::new(
        declaration_length(declarations, top)?,
        declaration_length(declarations, right)?,
        declaration_length(declarations, bottom)?,
        declaration_length(declarations, left)?,
    ))
}

fn declaration_length(declarations: &Declarations, property: Property) -> Option<Length> {
    match declarations.get(property) {
        Some(Value::Length(length)) => Some(length.clone()),
        _ => None,
    }
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
        Value::ZIndex(value) => {
            41u8.hash(state);
            match value {
                ZIndex::Auto => {
                    0u8.hash(state);
                }
                ZIndex::Integer(value) => {
                    1u8.hash(state);
                    value.hash(state);
                }
            }
        }
        Value::ScrollbarWidth(value) => {
            45u8.hash(state);
            value.hash(state);
        }
        Value::ContentVisibility(value) => {
            46u8.hash(state);
            value.hash(state);
        }
        Value::Order(value) => {
            42u8.hash(state);
            value.get().hash(state);
        }
        Value::FlexFactor(value) => {
            43u8.hash(state);
            value.get().to_bits().hash(state);
        }
        Value::AspectRatio(value) => {
            44u8.hash(state);
            match value.as_ratio() {
                Some(ratio) => {
                    1u8.hash(state);
                    ratio.to_bits().hash(state);
                }
                None => {
                    0u8.hash(state);
                }
            }
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

fn hash_slant(value: TextSlant, state: &mut DefaultHasher) {
    match value {
        TextSlant::Normal => 0u8.hash(state),
        TextSlant::Italic => 1u8.hash(state),
        TextSlant::Oblique(angle) => {
            2u8.hash(state);
            angle.map(f32::to_bits).hash(state);
        }
    }
}

fn hash_decoration(value: super::Decoration, state: &mut DefaultHasher) {
    value.enabled().hash(state);
    value.offset().map(f32::to_bits).hash(state);
    value.size().map(f32::to_bits).hash(state);
    if let Some(brush) = value.brush() {
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

    #[test]
    fn edge_shorthands_lower_to_side_longhands() {
        let edges = Edges::new(
            Length::Px(1.0),
            Length::Px(2.0),
            Length::Px(3.0),
            Length::Px(4.0),
        );

        let declarations = Declarations::new().try_margin(edges.clone()).unwrap();
        assert_eq!(declarations.get(Property::Margin), None);
        assert_eq!(
            declarations.get(Property::MarginTop),
            Some(&Value::Length(edges.top.clone()))
        );
        assert_eq!(
            declarations.get(Property::MarginRight),
            Some(&Value::Length(edges.right.clone()))
        );
        assert_eq!(
            declarations.get(Property::MarginBottom),
            Some(&Value::Length(edges.bottom.clone()))
        );
        assert_eq!(
            declarations.get(Property::MarginLeft),
            Some(&Value::Length(edges.left.clone()))
        );

        let declarations = Declarations::new().try_padding(edges.clone()).unwrap();
        assert_eq!(declarations.get(Property::Padding), None);
        assert_eq!(
            declarations.get(Property::PaddingTop),
            Some(&Value::Length(edges.top.clone()))
        );
        assert_eq!(
            declarations.get(Property::PaddingRight),
            Some(&Value::Length(edges.right.clone()))
        );
        assert_eq!(
            declarations.get(Property::PaddingBottom),
            Some(&Value::Length(edges.bottom.clone()))
        );
        assert_eq!(
            declarations.get(Property::PaddingLeft),
            Some(&Value::Length(edges.left.clone()))
        );

        let declarations = Declarations::new().try_border_width(edges.clone()).unwrap();
        assert_eq!(declarations.get(Property::BorderWidth), None);
        assert_eq!(
            declarations.get(Property::BorderTopWidth),
            Some(&Value::Length(edges.top.clone()))
        );
        assert_eq!(
            declarations.get(Property::BorderRightWidth),
            Some(&Value::Length(edges.right.clone()))
        );
        assert_eq!(
            declarations.get(Property::BorderBottomWidth),
            Some(&Value::Length(edges.bottom.clone()))
        );
        assert_eq!(
            declarations.get(Property::BorderLeftWidth),
            Some(&Value::Length(edges.left.clone()))
        );

        let declarations = Declarations::new().try_inset(edges.clone()).unwrap();
        assert_eq!(declarations.get(Property::Inset), None);
        assert_eq!(
            declarations.get(Property::Top),
            Some(&Value::Length(edges.top))
        );
        assert_eq!(
            declarations.get(Property::Right),
            Some(&Value::Length(edges.right))
        );
        assert_eq!(
            declarations.get(Property::Bottom),
            Some(&Value::Length(edges.bottom))
        );
        assert_eq!(
            declarations.get(Property::Left),
            Some(&Value::Length(edges.left))
        );
    }

    #[test]
    fn edge_shorthands_validate_canonical_longhand_domains() {
        assert!(
            Declarations::new()
                .try_padding(Edges::all(Length::Auto))
                .is_err()
        );
        assert!(
            Declarations::new()
                .try_border_width(Edges::all(Length::Normal))
                .is_err()
        );
        assert!(
            Declarations::new()
                .try_margin(Edges::all(Length::Normal))
                .is_err()
        );
        assert!(
            Declarations::new()
                .try_set(Property::PaddingTop, Value::Color(Color::BLACK))
                .is_err()
        );
    }
}
