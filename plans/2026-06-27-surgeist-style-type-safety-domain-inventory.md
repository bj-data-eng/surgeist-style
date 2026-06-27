# surgeist-style Type Safety Domain Inventory

This inventory is source-derived from `Property::ALL`, `Property::metadata`,
`Property::accepts`, `Property::validate_domain`, `Value`, and `src/lib.rs`.
It records the intended typed public front door even where later plan tasks still
need to introduce the named semantic wrapper.

| Property | Authored Type | Normalized Declaration Type | Resolved Type | Public Builder | Notes |
| --- | --- | --- | --- | --- | --- |
| Display | `Display` | `Display` | `Display` | `TypedDeclaration::display` | Closed enum for layout display choice. |
| BoxSizing | `BoxSizing` | `BoxSizing` | `BoxSizing` | `TypedDeclaration::box_sizing` | Closed enum; authored, normalized, and resolved phases use `BoxSizing`. |
| Position | `LayoutPosition` | `LayoutPosition` | `LayoutPosition` | `TypedDeclaration::position` | Closed enum for layout positioning mode. |
| Inset | `EdgeLengths` | `EdgeLengths` | `EdgeLengths` | `TypedDeclaration::inset` | Edge shorthand accepts `auto`; each edge must be a valid style length. |
| Width | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::width` | Non-negative dimension domain; ordinary typed builder does not accept color or text values. |
| Height | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::height` | Non-negative dimension domain. |
| MinWidth | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::min_width` | Non-negative dimension domain. |
| MinHeight | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::min_height` | Non-negative dimension domain. |
| MinSize | `SizeLengths` | `MinWidth`/`MinHeight` `DimensionLength` | `DimensionLength` per canonical property | `TypedDeclaration::min_size` | Shorthand normalizes to min width and min height. |
| MaxWidth | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::max_width` | Non-negative dimension domain. |
| MaxHeight | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::max_height` | Non-negative dimension domain. |
| MaxSize | `SizeLengths` | `MaxWidth`/`MaxHeight` `DimensionLength` | `DimensionLength` per canonical property | `TypedDeclaration::max_size` | Shorthand normalizes to max width and max height. |
| AspectRatio | `AspectRatio` | `AspectRatio` | `AspectRatio` | `TypedDeclaration::aspect_ratio` | Non-negative finite ratio, not a generic number. |
| Margin | `MarginEdges` | `MarginEdges` | `MarginEdges` | `TypedDeclaration::margin` | Edge lengths; negative values may be intentionally allowed for margin if modeled explicitly. |
| Padding | `EdgeLengths` | `EdgeLengths` | `EdgeLengths` | `TypedDeclaration::padding` | Edge lengths must be non-negative. |
| Overflow | `OverflowAxes` | `Overflow` for `OverflowX`/`OverflowY` | `Overflow` per canonical axis | `TypedDeclaration::overflow` | Shorthand normalizes to x and y axes. |
| OverflowX | `Overflow` | `Overflow` | `Overflow` | `TypedDeclaration::overflow_x` | Closed enum for a single axis. |
| OverflowY | `Overflow` | `Overflow` | `Overflow` | `TypedDeclaration::overflow_y` | Closed enum for a single axis. |
| ScrollbarWidth | `NonNegativeNumber` | `NonNegativeNumber` | `NonNegativeNumber` | `TypedDeclaration::scrollbar_width` | Non-negative finite numeric width until a unit-specific wrapper replaces it. |
| ZIndex | `ZIndex` | `ZIndex` | `ZIndex` | `TypedDeclaration::z_index` | Should distinguish integer stack order from generic keywords. |
| Direction | `Direction` | `Direction` | `Direction` | `TypedDeclaration::direction` | Closed enum; inherited. |
| WritingMode | `WritingMode` | `WritingMode` | `WritingMode` | `TypedDeclaration::writing_mode` | Closed enum; inherited. |
| TextAlign | `StyleTextAlign` | `StyleTextAlign` | `StyleTextAlign` | `TypedDeclaration::text_align` | Style-owned text alignment enum, separate from text crate layout alignment. |
| Float | `Float` | `Float` | `Float` | `TypedDeclaration::float` | Closed enum for float behavior. |
| Clear | `Clear` | `Clear` | `Clear` | `TypedDeclaration::clear` | Closed enum for clear behavior. |
| FlexDirection | `FlexDirection` | `FlexDirection` | `FlexDirection` | `TypedDeclaration::flex_direction` | Closed enum for flex main axis. |
| FlexWrap | `FlexWrap` | `FlexWrap` | `FlexWrap` | `TypedDeclaration::flex_wrap` | Closed enum for flex wrap behavior. |
| FlexGrow | `NonNegativeNumber` | `NonNegativeNumber` | `NonNegativeNumber` | `TypedDeclaration::flex_grow` | Non-negative finite scalar, not a broad `Number`. |
| FlexShrink | `NonNegativeNumber` | `NonNegativeNumber` | `NonNegativeNumber` | `TypedDeclaration::flex_shrink` | Non-negative finite scalar. |
| FlexBasis | `DimensionLength` | `DimensionLength` | `DimensionLength` | `TypedDeclaration::flex_basis` | Non-negative dimension length; accepts `auto`. |
| Align | `AlignItems` | `AlignItems`/`AlignSelf` `AlignItems` | `AlignItems` per canonical property | `TypedDeclaration::align` | Shorthand normalizes to items and self alignment. |
| AlignItems | `AlignItems` | `AlignItems` | `AlignItems` | `TypedDeclaration::align_items` | Closed enum for cross-axis item alignment. |
| AlignSelf | `AlignItems` | `AlignItems` | `AlignItems` | `TypedDeclaration::align_self` | Currently shares the `AlignItems` domain. |
| AlignContent | `AlignContent` | `AlignContent` | `AlignContent` | `TypedDeclaration::align_content` | Closed enum for content distribution. |
| Justify | `AlignItems` | `JustifyItems`/`JustifySelf` `AlignItems` | `AlignItems` per canonical property | `TypedDeclaration::justify` | Current source uses `AlignItems`; inventory flags the domain for later naming refinement. |
| JustifyItems | `AlignItems` | `AlignItems` | `AlignItems` | `TypedDeclaration::justify_items` | Current source uses `AlignItems`; may become `JustifyItems`. |
| JustifySelf | `AlignItems` | `AlignItems` | `AlignItems` | `TypedDeclaration::justify_self` | Current source uses `AlignItems`; may become `JustifySelf`. |
| JustifyContent | `AlignContent` | `AlignContent` | `AlignContent` | `TypedDeclaration::justify_content` | Current source uses `AlignContent`; may become `JustifyContent`. |
| Gap | `GapLength` | `RowGap`/`ColumnGap` `GapLength` | `GapLength` per canonical property | `TypedDeclaration::gap` | Allows `normal`; shorthand normalizes to row and column gap. |
| RowGap | `GapLength` | `GapLength` | `GapLength` | `TypedDeclaration::row_gap` | Allows `normal`; non-negative length otherwise. |
| ColumnGap | `GapLength` | `GapLength` | `GapLength` | `TypedDeclaration::column_gap` | Allows `normal`; non-negative length otherwise. |
| GridTemplateRows | `GridTrackList` | `GridTrackList` | `GridTrackList` | `TypedDeclaration::grid_template_rows` | Track list with validated repeat, line-name, and subgrid contents. |
| GridTemplateColumns | `GridTrackList` | `GridTrackList` | `GridTrackList` | `TypedDeclaration::grid_template_columns` | Track list with validated repeat, line-name, and subgrid contents. |
| GridTemplateAreas | `GridTemplateAreas` | `GridTemplateAreas` | `GridTemplateAreas` | `TypedDeclaration::grid_template_areas` | Rows must be non-empty, equal width, and named cells valid. |
| GridTemplate | `GridTemplate` | `GridTemplateRows`/`GridTemplateColumns`/`GridTemplateAreas` | Grid template canonical properties | `TypedDeclaration::grid_template` | Shorthand normalizes into row tracks, column tracks, and areas. |
| GridAutoRows | `GridTrackList` | `GridTrackList` | `GridTrackList` | `TypedDeclaration::grid_auto_rows` | Auto tracks cannot contain subgrid tracks. |
| GridAutoColumns | `GridTrackList` | `GridTrackList` | `GridTrackList` | `TypedDeclaration::grid_auto_columns` | Auto tracks cannot contain subgrid tracks. |
| GridAutoFlow | `GridAutoFlow` | `GridAutoFlow` | `GridAutoFlow` | `TypedDeclaration::grid_auto_flow` | Closed enum for auto-placement flow. |
| GridFlowTolerance | `GridFlowTolerance` | `GridFlowTolerance` | `GridFlowTolerance` | `TypedDeclaration::grid_flow_tolerance` | Normal, infinite, percent, or concrete non-negative px length. |
| GridRowStart | `GridLine` | `GridLine` | `GridLine` | `TypedDeclaration::grid_row_start` | Validated line index, span, and line-name domain. |
| GridRowEnd | `GridLine` | `GridLine` | `GridLine` | `TypedDeclaration::grid_row_end` | Validated line index, span, and line-name domain. |
| GridColumnStart | `GridLine` | `GridLine` | `GridLine` | `TypedDeclaration::grid_column_start` | Validated line index, span, and line-name domain. |
| GridColumnEnd | `GridLine` | `GridLine` | `GridLine` | `TypedDeclaration::grid_column_end` | Validated line index, span, and line-name domain. |
| GridRow | `GridPlacement` | `GridRowStart`/`GridRowEnd` `GridLine` | `GridLine` per canonical property | `TypedDeclaration::grid_row` | Shorthand normalizes start/end lines and expands omitted end names. |
| GridColumn | `GridPlacement` | `GridColumnStart`/`GridColumnEnd` `GridLine` | `GridLine` per canonical property | `TypedDeclaration::grid_column` | Shorthand normalizes start/end lines and expands omitted end names. |
| GridArea | `GridAreaPlacement` | Four grid line canonical properties | `GridLine` per canonical property | `TypedDeclaration::grid_area` | Shorthand normalizes row and column line placement. |
| Grid | `GridDefinition` | Grid template and auto canonical properties | Grid canonical properties | `TypedDeclaration::grid` | Shorthand normalizes template, auto tracks, and auto-flow. |
| Background | `Color` | `Color` | `Color` | `TypedDeclaration::background` | Finite RGBA color channels. |
| Foreground | `Color` | `Color` | `Color` | `TypedDeclaration::foreground` | Finite RGBA color channels. |
| Color | `Color` | `Color` | `Color` | `TypedDeclaration::text_color` | Text color front door uses color domain. |
| BorderColor | `Color` | `Color` | `Color` | `TypedDeclaration::border_color` | Finite RGBA color channels. |
| BorderWidth | `EdgeLengths` | `EdgeLengths` | `EdgeLengths` | `TypedDeclaration::border_width` | Edge lengths must be non-negative. |
| BorderStyle | `BorderStyle` | `BorderStyle` | `BorderStyle` | `TypedDeclaration::border_style` | Currently keyword-backed; should become a closed style enum. |
| Radius | `CornerLengths` | `CornerLengths` | `CornerLengths` | `TypedDeclaration::radius` | Corner lengths must be non-negative. |
| Shadow | `ShadowList` | `ShadowList` | `ShadowList` | `TypedDeclaration::shadow` | List wrapper should validate each shadow and whether empty lists are meaningful. |
| Opacity | `Opacity` | `Opacity` | `Opacity` | `TypedDeclaration::opacity` | Finite unit interval value. |
| Visibility | `Visibility` | `Visibility` | `Visibility` | `TypedDeclaration::visibility` | Closed enum for paint/layout retention state. |
| FontFamily | `FontFamilyList` | `FontFamilyList` | `FontFamilyList` | `TypedDeclaration::font_family` | String list needs semantic name/list validation. |
| FontSize | `FontSizeLength` | `FontSizeLength` | `FontSizeLength` | `TypedDeclaration::font_size` | Non-negative text size length. |
| FontWeight | `TextWeight` | `TextWeight` | `TextWeight` | `TypedDeclaration::font_weight` | Currently keyword-backed; public front door should use text crate weight. |
| FontStyle | `TextSlant` | `TextSlant` | `TextSlant` | `TypedDeclaration::font_style` | Currently keyword-backed; public front door should validate oblique angles. |
| LineHeight | `LineHeightLength` | `LineHeightLength` | `LineHeightLength` | `TypedDeclaration::line_height` | Non-negative text line-height length. |
| TextWrap | `TextWrap` | `TextWrap` | `TextWrap` | `TypedDeclaration::text_wrap` | Currently keyword-backed; should use text crate wrap enum. |
| WhiteSpace | `WhiteSpace` | `WhiteSpace` | `WhiteSpace` | `TypedDeclaration::white_space` | Currently keyword-backed; should use text crate whitespace enum. |
| WordBreak | `WordBreak` | `WordBreak` | `WordBreak` | `TypedDeclaration::word_break` | Currently keyword-backed; should use text crate word-break enum. |
| OverflowWrap | `OverflowWrap` | `OverflowWrap` | `OverflowWrap` | `TypedDeclaration::overflow_wrap` | Currently keyword-backed; should use text crate overflow-wrap enum. |
| TextOverflow | `TextOverflow` | `TextOverflow` | `TextOverflow` | `TypedDeclaration::text_overflow` | Currently keyword-backed; needs a style-owned closed enum or text crate type. |
| TextDecoration | `TextDecoration` | `TextDecoration` | `TextDecoration` | `TypedDeclaration::text_decoration` | Currently keyword-backed; should validate decoration offsets, sizes, and brushes. |
| SelectionColor | `Color` | `Color` | `Color` | `TypedDeclaration::selection_color` | Finite RGBA color channels. |
| Cursor | `Cursor` | `Cursor` | `Cursor` | `TypedDeclaration::cursor` | Closed enum for cursor intent. |
| PointerEvents | `PointerEvents` | `PointerEvents` | `PointerEvents` | `TypedDeclaration::pointer_events` | Closed enum for pointer hit testing. |
| FocusOutline | `Stroke` | `Stroke` | `Stroke` | `TypedDeclaration::focus_outline` | Stroke validates width, color, dash, and non-empty side set. |
| SelectionPaint | `Stroke` | `Stroke` | `Stroke` | `TypedDeclaration::selection_paint` | Stroke validates width, color, dash, and non-empty side set. |
| Transform | `Transform` | `Transform` | `Transform` | `TypedDeclaration::transform` | Operations validate finite scale/rotate values and length translations. |
| TransformOrigin | `Size` | `Size` | `Size` | `TypedDeclaration::transform_origin` | Two-axis length pair with valid lengths. |
| Filter | `FilterList` | `FilterList` | `FilterList` | `TypedDeclaration::filter` | Currently keyword-backed; should not remain a generic keyword. |
| TransitionProperty | `TransitionPropertyList` | `TransitionPropertyList` | `TransitionPropertyList` | `TypedDeclaration::transition_property` | Property list should reject invalid or non-animatable entries if that is the intended contract. |
| TransitionDuration | `DurationSeconds` | `DurationSeconds` | `DurationSeconds` | `TypedDeclaration::transition_duration` | Non-negative finite duration, not a generic number. |
| TransitionDelay | `DurationSeconds` | `DurationSeconds` | `DurationSeconds` | `TypedDeclaration::transition_delay` | Non-negative finite duration; delay policy should document whether negative delays are allowed. |
| TransitionTiming | `TimingFunction` | `TimingFunction` | `TimingFunction` | `TypedDeclaration::transition_timing` | Currently keyword-backed; should become a timing function domain. |
| AnimationName | `AnimationNameList` | `AnimationNameList` | `AnimationNameList` | `TypedDeclaration::animation_name` | String list needs semantic animation-name validation. |

## Value Variant Decisions

| Variant | Keep Public? | Single Semantic Domain | Replacement/Reason |
| --- | --- | --- | --- |
| `Keyword` | Temporarily | No | Explicit parser/interop escape hatch for cascade keywords; typed builders should model `inherit`, `initial`, and `unset` deliberately. |
| `Display` | Yes | Yes | Closed `Display` enum maps directly to display properties. |
| `BoxSizing` | Yes | Yes | Closed `BoxSizing` enum maps directly to box sizing. |
| `Position` | Yes | Yes | Closed `LayoutPosition` enum maps directly to position. |
| `Direction` | Yes | Yes | Closed `Direction` enum maps directly to direction. |
| `Overflow` | Yes | Yes | Single-axis overflow domain. |
| `OverflowAxes` | Yes, shorthand only | Yes | Two-axis overflow shorthand domain. |
| `Float` | Yes | Yes | Closed float domain. |
| `Clear` | Yes | Yes | Closed clear domain. |
| `TextAlign` | Yes | Yes | Style text alignment domain. |
| `WritingMode` | Yes | Yes | Closed writing mode domain. |
| `FlexDirection` | Yes | Yes | Closed flex direction domain. |
| `FlexWrap` | Yes | Yes | Closed flex wrap domain. |
| `AlignItems` | Yes | Mostly | Shared by align and justify item/self domains today; later tasks may split justify naming. |
| `AlignContent` | Yes | Mostly | Shared by align-content and justify-content today; later tasks may split justify naming. |
| `Number` | No | No | Replace with `Opacity`, `AspectRatio`, `DurationSeconds`, `NonNegativeNumber`, `ZIndex`, or property-specific types. |
| `Length` | Temporarily | No | Prefer context wrappers such as `DimensionLength`, `EdgeLength`, `GapLength`, and `FontSizeLength`. |
| `Size` | Temporarily | Yes | Transform-origin pair; should use a named `TransformOrigin` or `SizeLengths` wrapper. |
| `Edges` | Temporarily | No | Replace with `EdgeLengths`, `MarginEdges`, or border/padding-specific edge wrappers. |
| `GridTrackList` | Yes | Yes | Grid track list domain, but fields should become private or builder-only in later tasks. |
| `GridTemplateAreas` | Yes | Yes | Grid template area domain, but row construction needs semantic validation at the boundary. |
| `GridTemplate` | Yes | Yes | Grid template shorthand domain. |
| `GridDefinition` | Yes | Yes | Grid shorthand domain. |
| `GridLine` | Yes | Yes | Grid placement line domain; invalid direct variants must be closed in later tasks. |
| `GridPlacement` | Yes | Yes | Grid row/column shorthand domain. |
| `GridAreaPlacement` | Yes | Yes | Grid-area shorthand domain. |
| `GridAutoFlow` | Yes | Yes | Closed grid auto-flow enum. |
| `GridFlowTolerance` | Yes | Yes | Grid flow tolerance domain with additional concrete px validation for the property. |
| `Color` | Yes | Yes | Finite RGBA color domain. |
| `Corners` | Temporarily | Yes | Radius corner domain; fields should be private or use typed corner constructors. |
| `StringList` | No | No | Replace with `FontFamilyList`, `AnimationNameList`, or other named containers. |
| `PropertyList` | Temporarily | Yes | Transition-property list domain; should become a semantic list with animatability policy. |
| `ShadowList` | Temporarily | Yes | Shadow list domain; should use a semantic wrapper for empty-list policy. |
| `Stroke` | Yes | Yes | Stroke domain for outlines and selection paint. |
| `Text` | Temporarily | Yes | Composite text value domain; public field mutation should be replaced by typed builders. |
| `Transform` | Yes | Yes | Transform operation list domain; list field should become builder-only. |
| `Cursor` | Yes | Yes | Closed cursor enum. |
| `PointerEvents` | Yes | Yes | Closed pointer-events enum. |
| `Visibility` | Yes | Yes | Closed visibility enum. |

## Public Reexport Surface Decisions

External reexports from `surgeist_retained` and `surgeist_text` are intentionally
not listed here because this inventory covers `surgeist-style` owned public
reexports from `src/lib.rs`.

| Reexport | Owning File | Invariant/Unit Risk | Action |
| --- | --- | --- | --- |
| `CalcLength` | `src/calc.rs` | Public `Sum(Vec<_>)` permits empty calc sums. | Replace direct construction with non-empty/fallible constructors. |
| `CalcLengthTerm` | `src/calc.rs` | Public fields permit arbitrary nested invalid terms. | Make fields private and keep `add`/`sub` constructors. |
| `CalcOperator` | `src/calc.rs` | Closed operator enum has no invalid states. | Keep public. |
| `Condition` | `src/condition.rs` | Composes viewport/container queries over raw sizes. | Keep with finite query wrappers. |
| `Container` | `src/condition.rs` | Raw `f32` width/height query values. | Replace with semantic container size values or document finite-only query boundary. |
| `Viewport` | `src/condition.rs` | Raw `f32` width/height query values. | Replace with semantic viewport size values or document finite-only query boundary. |
| `Declaration` | `src/declaration.rs` | Public fields allow property/value mismatches. | Demote to fallible interop or make fields private. |
| `Declarations` | `src/declaration.rs` | `insert`/`set` accept broad `Value` without validation. | Prefer `from_typed` and typed builders; make unchecked insertion explicit later. |
| `Fingerprint` | `src/declaration.rs` | Opaque value with getter only. | Keep public. |
| `TypedDeclaration` | `src/declaration.rs` | Private payload preserves property/value pairing. | Keep and expand property-specific constructors. |
| `Error` | `src/error.rs` | Message string can become test coupling. | Keep code/message getters; add structured details when needed. |
| `ErrorCode` | `src/error.rs` | Non-exhaustive code enum supports stable matching. | Keep public. |
| `Result` | `src/error.rs` | Alias has no invariant risk. | Keep public. |
| `Change` | `src/invalidation.rs` | Public invalidation facts need bool-combination audit. | Keep only if every combination is meaningful; otherwise add constructors. |
| `Invalidation` | `src/invalidation.rs` | Public invalidation facts need bool-combination audit. | Keep only if every combination is meaningful; otherwise add constructors. |
| `Scope` | `src/invalidation.rs` | Public invalidation scope needs closed-domain audit. | Keep if closed choices are intentional. |
| `Impact` | `src/property.rs` | Public boolean bit struct. | Keep only if every bit combination is meaningful; otherwise use constructors. |
| `Interpolation` | `src/property.rs` | Closed enum has no invalid states. | Keep public. |
| `Metadata` | `src/property.rs` | Public mutable fields and arbitrary defaults. | Make read-only/private-field. |
| `Property` | `src/property.rs` | Public enum is the parser/interop property identity. | Keep but pair with typed declaration front doors. |
| `Context` | `src/resolver.rs` | Public fields permit inconsistent resolver context. | Prefer builder-only context or validate before resolve. |
| `Resolved` | `src/resolver.rs` | Stores broad `Value` by property. | Add typed getters and typed resolved domains. |
| `Resolver` | `src/resolver.rs` | Internal cache is private. | Keep public resolver API. |
| `AttributeSelector` | `src/selector.rs` | Name/value validation must remain fallible. | Keep constructors; audit public fields/variants. |
| `Combinator` | `src/selector.rs` | Closed enum has no invalid states. | Keep public. |
| `Compound` | `src/selector.rs` | Private fields with builder methods. | Keep public builder. |
| `Nth` | `src/selector.rs` | Check arithmetic invariants and zero semantics. | Keep if every pair is meaningful; otherwise add constructors. |
| `Part` | `src/selector.rs` | Complex selector part can be invalid in public construction if fields are public. | Make construction fallible or fields private. |
| `Position` | `src/selector.rs` | Position selector domain needs invalid nth audit. | Keep after constructor audit. |
| `PositionSelector` | `src/selector.rs` | Position selector domain needs invalid nth audit. | Keep after constructor audit. |
| `Selector` | `src/selector.rs` | `Complex(Vec<Part>)` permits direct invalid construction. | Keep fields/variants private or require fallible constructors. |
| `Rule` | `src/sheet.rs` | Rule construction must validate selector and declarations. | Keep with checked declaration path. |
| `Sheet` | `src/sheet.rs` | Rule collection owns cascade input. | Keep public. |
| `Version` | `src/sheet.rs` | Opaque version value. | Keep public. |
| `Node` | `src/tree.rs` | Public borrowed node fields reflect retained tree state. | Keep if it remains a read-only adapter boundary. |
| `Traversal` | `src/tree.rs` | Closed enum has no invalid states. | Keep public. |
| `Tree` | `src/tree.rs` | Host adapter trait boundary. | Keep public. |
| `AlignContent` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `AlignItems` | `src/value.rs` | Closed enum has no invalid states, but justify naming is conflated. | Keep temporarily; split justify types if needed. |
| `BoxSizing` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Clear` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Color` | `src/value.rs` | Public fields allow non-finite channels. | Prefer `try_rgba` and make fields private in a later hardening task. |
| `Corners` | `src/value.rs` | Public fields allow invalid radius lengths. | Add typed/fallible constructors and private fields. |
| `CssPx` | `src/value.rs` | Private finite pixel payload. | Keep public constructor. |
| `Cursor` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Dash` | `src/value.rs` | Public fields allow invalid density/phase. | Add fallible constructors or private fields. |
| `DimensionLength` | `src/value.rs` | Private dimension-length payload. | Keep and expand with percent/calc/auto constructors. |
| `Direction` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Display` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Edges` | `src/value.rs` | Public fields allow property-specific invalid lengths. | Replace with edge-domain wrappers. |
| `FlexDirection` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `FlexWrap` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Float` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `GridAreaPlacement` | `src/value.rs` | Public fields allow invalid grid lines. | Keep with private fields or fallible constructors. |
| `GridAutoFlow` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `GridDefinition` | `src/value.rs` | Public fields allow invalid auto subgrid contents. | Make fields private or validate at construction. |
| `GridFlowTolerance` | `src/value.rs` | Percent/length variants require finite and property-specific checks. | Add typed constructors. |
| `GridLine` | `src/value.rs` | Public variants allow zero line/span and invalid names. | Replace direct variants with fallible constructors. |
| `GridPlacement` | `src/value.rs` | Public fields allow invalid grid lines. | Make fields private or validate at construction. |
| `GridTemplate` | `src/value.rs` | Public fields allow invalid nested parts. | Make fields private or validate at construction. |
| `GridTemplateAreaRow` | `src/value.rs` | Public `cells` allows empty/invalid names. | Make fields private or validate at construction. |
| `GridTemplateAreas` | `src/value.rs` | Public rows allow inconsistent widths. | Make fields private or validate at construction. |
| `GridTrackComponent` | `src/value.rs` | Public variants allow invalid line-name lists. | Replace direct variants with fallible constructors. |
| `GridTrackList` | `src/value.rs` | Public components allow invalid repeats/subgrid. | Make fields private or validate at construction. |
| `Keyword` | `src/value.rs` | Closed cascade keyword enum. | Keep as explicit interop/cascade domain. |
| `LayoutPosition` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Length` | `src/value.rs` | Public variants allow non-finite and property-specific invalid lengths. | Demote behind semantic wrappers. |
| `LineStyle` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `MaxTrackSizing` | `src/value.rs` | Public variants allow invalid flex/fit-content values. | Add typed/fallible constructors. |
| `MinTrackSizing` | `src/value.rs` | Public variants allow invalid nested lengths. | Add typed/fallible constructors. |
| `Opacity` | `src/value.rs` | Private finite unit-interval payload. | Keep public constructor. |
| `Overflow` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `OverflowAxes` | `src/value.rs` | Two closed-axis values. | Keep public constructor. |
| `PointerEvents` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `Shadow` | `src/value.rs` | Public fields allow invalid lengths/colors. | Add typed/fallible constructors or private fields. |
| `SideSet` | `src/value.rs` | Public fields allow empty side set. | Make fields private or keep only named constructors. |
| `Size` | `src/value.rs` | Public fields allow invalid nested lengths. | Add domain-specific wrappers. |
| `Stroke` | `src/value.rs` | Public fields allow invalid width, color, dash, or empty sides. | Add typed/fallible constructors or private fields. |
| `StrokeAlign` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `StyleTextAlign` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `SubgridLineNameComponent` | `src/value.rs` | Public variants allow empty sets and zero counts. | Replace with fallible constructors/private variants. |
| `SubgridLineNameRepeatCount` | `src/value.rs` | Public `Count(0)` invalid in context. | Replace with non-zero count wrapper. |
| `SubgridTrack` | `src/value.rs` | Public field allows invalid name components. | Make fields private or validate at construction. |
| `TextValue` | `src/value.rs` | Public fields allow invalid text lengths, strings, colors, and decorations. | Replace with typed builders or private fields. |
| `TrackRepeat` | `src/value.rs` | Public fields allow zero count and empty components. | Make fields private or validate at construction. |
| `TrackRepeatCount` | `src/value.rs` | Public `Count(0)` invalid in context. | Replace with non-zero count wrapper. |
| `TrackSizing` | `src/value.rs` | Public fields allow invalid nested track sizing. | Add typed/fallible constructors. |
| `Transform` | `src/value.rs` | Public operation list allows invalid operations. | Make list private or validate at construction. |
| `TransformOp` | `src/value.rs` | Public variants allow non-finite scale/rotate values. | Add typed/fallible constructors. |
| `Value` | `src/value.rs` | Broad transport bag permits property/value mismatches. | Demote to explicit fallible parser/interop escape hatch. |
| `Visibility` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |
| `WritingMode` | `src/value.rs` | Closed enum has no invalid states. | Keep public. |

## Primitive Payload And Setter Inventory

This section inventories the primitive payloads that currently carry multiple
style domains and the setter surfaces that can admit them. It is source-derived
from `src/value.rs`, `src/declaration.rs`, and `Property::validate_value`.

| Domain | Current Payload | Public Setter Surface | Decision |
| --- | --- | --- | --- |
| Raw property/value interop | `Property` plus `Value` | `Declaration::try_new`, `Declarations::try_set`, `Declarations::try_insert` | Keep only explicit fallible public entry points; unchecked construction is crate-internal. |
| Typed authored declarations | `TypedDeclaration(Declaration)` | `TypedDeclaration::*`, `Declarations::from_typed` | Preferred Rust-authored front door; expand until ordinary authors do not need raw `Value`. |
| Concrete css pixels | `CssPx(f32)` | `CssPx::new` | Finite numeric payload only; property-specific wrappers decide whether negative values are allowed. |
| Dimension lengths | `DimensionLength(Length)` | `DimensionLength::px`, `TypedDeclaration::width` | Enforce finite non-negative dimensions at typed construction. Add percent, calc, and auto constructors only with the same domain policy. |
| Generic lengths | `Length` variants | `Length::px`, `Length::try_px`, `Length::percent`, `Length::try_percent`, legacy declaration builders | Keep temporarily as parser/intermediate data; prefer semantic wrappers for public typed setters. |
| Generic numbers | `Value::Number(f32)` | Legacy declaration builders for opacity, duration, flex, ratio, z-index, scrollbars | Replace with domain wrappers such as `Opacity`, `DurationSeconds`, `NonNegativeNumber`, `AspectRatio`, and `ZIndex`. |
| Unit interval opacity | `Opacity(f32)` | `Opacity::new`, `TypedDeclaration::opacity` | Keep finite 0..=1 constructor and use it instead of raw number setters. |
| Color channels | `Color { r, g, b, a }` | `Color::rgba`, `Color::try_rgba`, color declaration builders | Prefer `try_rgba`; later hardening should make fields private or introduce channel wrappers. |
| Edge/corner length groups | `Edges`, `Corners`, `Size` | Legacy declaration builders and direct public fields | Split into property-specific wrappers for padding, margin, radius, transform origin, and size shorthands. |
| String and property lists | `Vec<String>`, `Vec<Property>` inside `Value` variants | Raw `Value` interop and legacy builders | Replace with named list wrappers that validate emptiness, identifiers, and animatability policy. |
| Grid collections | Grid track, template, placement, repeat, and subgrid structs/enums | Direct public fields/variants and grid declaration builders | Close invalid zero, empty, and malformed states with fallible constructors or private fields. |
| Paint/effect collections | `Shadow`, `Stroke`, `Transform`, `Dash` | Direct public fields/variants and declaration builders | Add constructors that validate finite lengths, non-empty side sets, dash policy, and transform operation values. |

## Phase Boundary Decisions

These decisions identify where a value is allowed to stay broad and where it
must become typed before crossing into the next layer.

| Boundary | Accepted Shape | Validation Owner | Decision |
| --- | --- | --- | --- |
| Rust authored API to declaration storage | `TypedDeclaration` or fallible raw interop | Typed constructors and `Property::validate_value` | Ordinary public construction should be typed; raw `Property` plus `Value` must be visibly fallible. |
| Parser/lowering to style declarations | `Property` plus `Value` | `Declaration::try_new` or `Declarations::try_insert` | CSS and future DSL layers may lower through broad values, but only through checked interop. |
| Shorthand normalization | Canonical `Declaration` values | `Declarations` internal insertion after validation | Public callers validate the shorthand once; internal canonicalization may use unchecked crate-private constructors. |
| Cascade and resolver storage | Property-indexed `Value` map | Resolver plus property metadata/default validation | Broad storage is acceptable internally while resolved typed getters are added at the public boundary. |
| Style to layout/text/render handoff | Semantic wrappers or typed getters | `surgeist-style` public API | Do not ask downstream crates to reinterpret raw `Value`; expose typed contracts for each dependency boundary. |
| Generated API reports and inventory | Source-derived tables | Crate-local tooling and review | Reports document the public contract but do not become the source of truth. |
