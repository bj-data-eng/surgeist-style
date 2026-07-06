# CSS Property Coverage Ledger

Date: 2026-07-05

## Source Snapshot

- `surgeist-style`: `055077b` (`style: resolve requested style buckets`)
- `surgeist-css`: `1c95d4218439f1696151e0ee9602671fab418314`
- Source CSS enum: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Source CSS parser dispatch: `/Users/codex/Development/surgeist-css/src/parser/mod.rs`
- Current style property model: `src/property.rs`
- Current style value model: `src/value.rs`
- Current authored/custom model: `src/authored.rs`, `src/custom.rs`

## Purpose

This ledger maps every parsed `surgeist-css` `CssProperty` to the current or
planned `surgeist-style` behavior. It is the handoff artifact for the next
property-family implementation plans.

This file is descriptive. It does not add Rust APIs, CSS lowering, parser
dependencies, adapters, or generated code.

## Outcome Labels

| Outcome | Meaning |
| --- | --- |
| `Existing style property` | Style already has a semantically owned longhand `Property` and `Value` model for this CSS surface. |
| `Existing style shorthand` | Style already has a semantic shorthand or aggregate `Property`/`Value` model for this CSS surface. |
| `New style property needed` | CSS accepts a longhand that style does not yet model as computed style data. |
| `New shorthand lowering needed` | CSS accepts a shorthand that should lower into existing or planned style longhands. |
| `Symbolic style data needed` | Style must preserve authored symbolic data because another owner or later context is needed. |
| `Existing authored cascade model` | Authored declarations, CSS-wide keywords, custom properties, or cascade-path code already owns the surface. |
| `Root rejection required` | Root should reject this property before normal style declaration input. |
| `Out of style` | The surface is intentionally not represented in style output. |

## Property Coverage

| CSS property | CSS value kind | Family | Outcome | Style target | Lowering or gap | Next plan |
| --- | --- | --- | --- | --- | --- | --- |
| `CssProperty::All` | `CssValue::GlobalKeyword` | Authored cascade | `Existing authored cascade model` | `AuthoredProperty::All` + `CssWideKeyword` | Authored cascade expands CSS-wide keywords across canonical style properties; no computed `Property::All` is needed. | No property implementation |
| `CssProperty::Display` | `CssValue::Display` | Display and box | `Existing style property` | `Property::Display` + `Value::Display` | Current display keywords have a style model; any future CSS-only display forms should remain root-lowering decisions. | Operation 8 layout-facing properties |
| `CssProperty::BoxSizing` | `CssValue::BoxSizing` | Display and box | `Existing style property` | `Property::BoxSizing` + `Value::BoxSizing` | Concrete box-sizing values lower to the existing style property. | Operation 8 layout-facing properties |
| `CssProperty::Position` | `CssValue::Position` | Position and stacking | `Existing style property` | `Property::Position` + `Value::Position` | Static, relative, absolute, fixed, and sticky positions have typed style support. | Operation 8 layout-facing properties |
| `CssProperty::Direction` | `CssValue::Direction` | Writing mode | `Existing style property` | `Property::Direction` + `Value::Direction` | Direction is inherited style data and already has typed LTR/RTL support. | Operation 8 layout-facing properties |
| `CssProperty::Overflow` | `CssValue::Overflow` or `CssValue::OverflowAxes` | Overflow and visibility | `Existing style shorthand` | `Property::Overflow` + `Value::Overflow` or `Value::OverflowAxes` | Existing shorthand canonicalization lowers one or two axes into `OverflowX` and `OverflowY`. | Operation 8 layout-facing properties |
| `CssProperty::OverflowX` | `CssValue::Overflow` | Overflow and visibility | `Existing style property` | `Property::OverflowX` + `Value::Overflow` | Single-axis overflow is modeled directly. | Operation 8 layout-facing properties |
| `CssProperty::OverflowY` | `CssValue::Overflow` | Overflow and visibility | `Existing style property` | `Property::OverflowY` + `Value::Overflow` | Single-axis overflow is modeled directly. | Operation 8 layout-facing properties |
| `CssProperty::FlexDirection` | `CssValue::FlexDirection` | Flex | `Existing style property` | `Property::FlexDirection` + `Value::FlexDirection` | Flex direction has a typed style enum. | Operation 8 layout-facing properties |
| `CssProperty::FlexWrap` | `CssValue::FlexWrap` | Flex | `Existing style property` | `Property::FlexWrap` + `Value::FlexWrap` | Flex wrapping has a typed style enum. | Operation 8 layout-facing properties |
| `CssProperty::Float` | `CssValue::Float` | Display and box | `Existing style property` | `Property::Float` + `Value::Float` | Float has a typed style enum for current parsed values. | Operation 8 layout-facing properties |
| `CssProperty::Clear` | `CssValue::Clear` | Display and box | `Existing style property` | `Property::Clear` + `Value::Clear` | Clear has a typed style enum for current parsed values. | Operation 8 layout-facing properties |
| `CssProperty::AlignContent` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::AlignContent` + `Value::AlignContent` | Content alignment has a typed style target; first-baseline variants need layout-pass parity review. | Operation 8 layout-facing properties |
| `CssProperty::JustifyContent` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::JustifyContent` + `Value::AlignContent` | Justify-content reuses the content-alignment value model. | Operation 8 layout-facing properties |
| `CssProperty::AlignItems` | `CssValue::AlignItems` | Alignment | `Existing style property` | `Property::AlignItems` + `Value::AlignItems` | Item alignment has a typed style target; CSS normal/baseline variants need parity review. | Operation 8 layout-facing properties |
| `CssProperty::AlignSelf` | `CssValue::AlignItems` | Alignment | `Existing style property` | `Property::AlignSelf` + `Value::AlignItems` | Align-self uses the existing item-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::JustifyItems` | `CssValue::AlignItems` | Alignment | `Existing style property` | `Property::JustifyItems` + `Value::AlignItems` | Justify-items uses the existing item-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::JustifySelf` | `CssValue::AlignItems` | Alignment | `Existing style property` | `Property::JustifySelf` + `Value::AlignItems` | Justify-self uses the existing item-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::PlaceContent` | `CssValue::PlaceAlignment` | Alignment | `Existing style shorthand` | `Property::PlaceContent` + `Value::PlaceContentAlignment` | Style shorthand canonicalization lowers to `AlignContent` and `JustifyContent`. | Operation 8 layout-facing properties |
| `CssProperty::PlaceItems` | `CssValue::PlaceAlignment` | Alignment | `Existing style shorthand` | `Property::PlaceItems` + `Value::PlaceItemsAlignment` | Style shorthand canonicalization lowers to `AlignItems` and `JustifyItems`. | Operation 8 layout-facing properties |
| `CssProperty::PlaceSelf` | `CssValue::PlaceAlignment` | Alignment | `Existing style shorthand` | `Property::PlaceSelf` + `Value::PlaceItemsAlignment` | Style shorthand canonicalization lowers to `AlignSelf` and `JustifySelf`. | Operation 8 layout-facing properties |
| `CssProperty::Visibility` | `CssValue::Visibility` | Overflow and visibility | `Existing style property` | `Property::Visibility` + `Value::Visibility` | Visible and hidden are modeled; CSS collapse needs layout-pass treatment. | Operation 8 layout-facing properties |
| `CssProperty::Content` | `CssValue::Content` | Generated content and lists | `Existing style property` | `Property::Content` + `Value::Content` | Style owns generated content policy/data; retained/tree materialization remains outside style. | No property implementation |
| `CssProperty::ContentVisibility` | `CssValue::ContentVisibility` | Overflow and visibility | `Existing style property` | `Property::ContentVisibility` + `Value::ContentVisibility` | Content visibility has typed layout/paint style data. | Operation 8 layout-facing properties |
| `CssProperty::ListStyleType` | `CssValue::ListStyleType` | Generated content and lists | `Existing style property` | `Property::ListStyleType` + `Value::ListStyleType` | List marker type has typed style data; marker text materialization remains outside style. | No property implementation |
| `CssProperty::ListStylePosition` | `CssValue::ListStylePosition` | Generated content and lists | `Existing style property` | `Property::ListStylePosition` + `Value::ListStylePosition` | Marker position has typed style data for list layout policy. | No property implementation |
| `CssProperty::ListStyleImage` | `CssValue::ListStyleImage` | Generated content and lists | `Existing style property` | `Property::ListStyleImage` + `Value::ListStyleImage` | URLs remain symbolic; image resource loading remains outside style. | No property implementation |
| `CssProperty::ListStyle` | `CssValue::ListStyle` | Generated content and lists | `Existing style shorthand` | `Property::ListStyle` + `Value::ListStyle` | Style shorthand canonicalization lowers to list-style type, position, and image longhands. | No property implementation |
| `CssProperty::CounterReset` | `CssValue::CounterChanges` | Generated content and lists | `Existing style property` | `Property::CounterReset` + `Value::CounterChanges` | Counter reset has typed counter mutation style data. | No property implementation |
| `CssProperty::CounterIncrement` | `CssValue::CounterChanges` | Generated content and lists | `Existing style property` | `Property::CounterIncrement` + `Value::CounterChanges` | Counter increment has typed counter mutation style data. | No property implementation |
| `CssProperty::CounterSet` | `CssValue::CounterChanges` | Generated content and lists | `Existing style property` | `Property::CounterSet` + `Value::CounterChanges` | Counter set has typed counter mutation style data. | No property implementation |
| `CssProperty::Width` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::Width` + `Value::Length` | Width has typed length data, including symbolic calc lengths. | Operation 8 layout-facing properties |
| `CssProperty::Height` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::Height` + `Value::Length` | Height has typed length data, including symbolic calc lengths. | Operation 8 layout-facing properties |
| `CssProperty::MinWidth` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MinWidth` + `Value::Length` | Minimum width has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::MinHeight` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MinHeight` + `Value::Length` | Minimum height has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::MaxWidth` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MaxWidth` + `Value::Length` | Maximum width has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::MaxHeight` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MaxHeight` + `Value::Length` | Maximum height has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::FlexBasis` | `CssValue::Length` | Flex | `Existing style property` | `Property::FlexBasis` + `Value::Length` | Flex basis reuses the typed length model. | Operation 8 layout-facing properties |
| `CssProperty::Gap` | `CssValue::Length` | Sizing and spacing | `Existing style shorthand` | `Property::Gap` + `Value::Length` | Existing shorthand canonicalization lowers to row-gap and column-gap. | Operation 8 layout-facing properties |
| `CssProperty::RowGap` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::RowGap` + `Value::Length` | Row gap has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::ColumnGap` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::ColumnGap` + `Value::Length` | Column gap has typed length data. | Operation 8 layout-facing properties |
| `CssProperty::GridFlowTolerance` | `CssValue::GridFlowTolerance` | Grid | `Existing style property` | `Property::GridFlowTolerance` + `Value::GridFlowTolerance` | Grid flow tolerance has a typed style model. | Operation 8 layout-facing properties |
| `CssProperty::GridTemplateRows` | `CssValue::GridTrackList` | Grid | `Existing style property` | `Property::GridTemplateRows` + `Value::GridTrackList` | Grid row tracks have typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridTemplateColumns` | `CssValue::GridTrackList` | Grid | `Existing style property` | `Property::GridTemplateColumns` + `Value::GridTrackList` | Grid column tracks have typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridTemplateAreas` | `CssValue::GridTemplateAreas` | Grid | `Existing style property` | `Property::GridTemplateAreas` + `Value::GridTemplateAreas` | Grid template areas have typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridTemplate` | `CssValue::GridTemplate` | Grid | `Existing style shorthand` | `Property::GridTemplate` + `Value::GridTemplate` | Existing canonicalization lowers rows, columns, and areas. | Operation 8 layout-facing properties |
| `CssProperty::GridAutoRows` | `CssValue::GridTrackList` | Grid | `Existing style property` | `Property::GridAutoRows` + `Value::GridTrackList` | Grid auto rows have typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridAutoColumns` | `CssValue::GridTrackList` | Grid | `Existing style property` | `Property::GridAutoColumns` + `Value::GridTrackList` | Grid auto columns have typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridAutoFlow` | `CssValue::GridAutoFlow` | Grid | `Existing style property` | `Property::GridAutoFlow` + `Value::GridAutoFlow` | Grid auto-flow has typed style data. | Operation 8 layout-facing properties |
| `CssProperty::GridRowStart` | `CssValue::GridLine` | Grid | `Existing style property` | `Property::GridRowStart` + `Value::GridLine` | Grid row start has typed placement data. | Operation 8 layout-facing properties |
| `CssProperty::GridRowEnd` | `CssValue::GridLine` | Grid | `Existing style property` | `Property::GridRowEnd` + `Value::GridLine` | Grid row end has typed placement data. | Operation 8 layout-facing properties |
| `CssProperty::GridColumnStart` | `CssValue::GridLine` | Grid | `Existing style property` | `Property::GridColumnStart` + `Value::GridLine` | Grid column start has typed placement data. | Operation 8 layout-facing properties |
| `CssProperty::GridColumnEnd` | `CssValue::GridLine` | Grid | `Existing style property` | `Property::GridColumnEnd` + `Value::GridLine` | Grid column end has typed placement data. | Operation 8 layout-facing properties |
| `CssProperty::GridRow` | `CssValue::GridLineRange` | Grid | `Existing style shorthand` | `Property::GridRow` + `Value::GridPlacement` | Existing canonicalization lowers row start and end. | Operation 8 layout-facing properties |
| `CssProperty::GridColumn` | `CssValue::GridLineRange` | Grid | `Existing style shorthand` | `Property::GridColumn` + `Value::GridPlacement` | Existing canonicalization lowers column start and end. | Operation 8 layout-facing properties |
| `CssProperty::GridArea` | `CssValue::GridArea` | Grid | `Existing style shorthand` | `Property::GridArea` + `Value::GridAreaPlacement` | Existing canonicalization lowers the four grid placement longhands. | Operation 8 layout-facing properties |
| `CssProperty::Grid` | `CssValue::Grid` | Grid | `Existing style shorthand` | `Property::Grid` + `Value::GridDefinition` | Existing canonicalization lowers template, auto tracks, and auto-flow. | Operation 8 layout-facing properties |
| `CssProperty::FontSize` | `CssValue::Length` | Text and font | `Existing style property` | `Property::FontSize` + `Value::Length` | Font size has typed length data. | Operation 9 text-facing properties |
| `CssProperty::LineHeight` | `CssValue::Length` | Text and font | `Existing style property` | `Property::LineHeight` + `Value::Length` | Line height has typed length data. | Operation 9 text-facing properties |
| `CssProperty::WritingMode` | `CssValue::WritingMode` | Writing mode | `Existing style property` | `Property::WritingMode` + `Value::WritingMode` | Writing mode has a typed style enum. | Operation 8 layout-facing properties |
| `CssProperty::TextAlign` | `CssValue::TextAlign` | Text and font | `Existing style property` | `Property::TextAlign` + `Value::TextAlign` | Text alignment has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextAlignLast` | `CssValue::TextAlignLast` | Text and font | `Existing style property` | `Property::TextAlignLast` + `Value::TextAlignLast` | Final-line text alignment has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextIndent` | `CssValue::TextIndent` | Text and font | `Existing style property` | `Property::TextIndent` + `Value::TextIndent` | Text indentation length and hanging/each-line flags have typed style data. | Operation 9 text-facing properties |
| `CssProperty::VerticalAlign` | `CssValue::VerticalAlign` | Text and font | `Existing style property` | `Property::VerticalAlign` + `Value::VerticalAlign` | Inline/table-cell vertical alignment has typed style data. | Operation 9 text-facing properties |
| `CssProperty::FontFamily` | `CssValue::FontFamily` | Text and font | `Existing style property` | `Property::FontFamily` + `Value::FontFamilyList` | Family names are preserved as symbolic font family data; final font loading remains outside style. | Operation 9 text-facing properties |
| `CssProperty::Font` | `CssValue::Font` | Text and font | `Existing style shorthand` | `Property::Font` + `Value::Font` | Font shorthand lowers across font style, variant, weight, stretch, size, line-height, and family longhands. | Operation 9 text-facing properties |
| `CssProperty::FontWeight` | `CssValue::FontWeight` | Text and font | `Existing style property` | `Property::FontWeight` + `Value::FontWeight` | Font weight keywords and numeric weights have typed style data. | Operation 9 text-facing properties |
| `CssProperty::FontStyle` | `CssValue::FontStyle` | Text and font | `Existing style property` | `Property::FontStyle` + `Value::TextSlant` | Font style and oblique slant are represented by typed text slant data. | Operation 9 text-facing properties |
| `CssProperty::FontStretch` | `CssValue::FontStretch` | Text and font | `Existing style property` | `Property::FontStretch` + `Value::FontStretch` | Font width/stretch keywords have typed style data. | Operation 9 text-facing properties |
| `CssProperty::FontVariant` | `CssValue::FontVariant` | Text and font | `Existing style property` | `Property::FontVariant` + `Value::FontVariant` | Font variant keywords have typed style data. | Operation 9 text-facing properties |
| `CssProperty::FontFeatureSettings` | `CssValue::FontFeatureSettings` | Text and font | `Existing style property` | `Property::FontFeatureSettings` + `Value::FontFeatureSettings` | Feature tags are preserved as symbolic style-owned data for later shaping. | Operation 9 text-facing properties |
| `CssProperty::LetterSpacing` | `CssValue::LetterSpacing` | Text and font | `Existing style property` | `Property::LetterSpacing` + `Value::LetterSpacing` | Letter spacing has typed normal-or-length style data. | Operation 9 text-facing properties |
| `CssProperty::TextWrap` | `CssValue::TextWrap` | Text and font | `Existing style property` | `Property::TextWrap` + `Value::TextWrap` | Text wrapping policy has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::WhiteSpace` | `CssValue::WhiteSpace` | Text and font | `Existing style property` | `Property::WhiteSpace` + `Value::WhiteSpace` | White-space handling has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::WordBreak` | `CssValue::WordBreak` | Text and font | `Existing style property` | `Property::WordBreak` + `Value::WordBreak` | Word-break handling has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::OverflowWrap` | `CssValue::OverflowWrap` | Text and font | `Existing style property` | `Property::OverflowWrap` + `Value::OverflowWrap` | Overflow wrapping policy has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextOverflow` | `CssValue::TextOverflow` | Text and font | `Existing style property` | `Property::TextOverflow` + `Value::TextOverflow` | Text overflow behavior has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextDecoration` | `CssValue::TextDecoration` | Text and font | `Existing style shorthand` | `Property::TextDecoration` + `Value::TextDecoration` | Style shorthand canonicalization lowers line, color, style, and thickness when authored components are present. | No property implementation |
| `CssProperty::TextDecorationLine` | `CssValue::TextDecorationLine` | Text and font | `Existing style property` | `Property::TextDecorationLine` + `Value::TextDecorationLine` | Text decoration line components have typed style data. | Operation 9 text-facing properties |
| `CssProperty::TextDecorationColor` | `CssValue::TextDecorationColor` | Color | `Existing style property` | `Property::TextDecorationColor` + `Value::StyleColor` | Text decoration color accepts style-owned concrete and symbolic colors; final currentColor, system-color, and color-space resolution remains outside style. | No property implementation |
| `CssProperty::TextDecorationStyle` | `CssValue::TextDecorationStyle` | Text and font | `Existing style property` | `Property::TextDecorationStyle` + `Value::TextDecorationStyle` | Text decoration stroke style has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextDecorationThickness` | `CssValue::TextDecorationThickness` | Text and font | `Existing style property` | `Property::TextDecorationThickness` + `Value::TextDecorationThickness` | Text decoration thickness has typed auto, from-font, and length data. | Operation 9 text-facing properties |
| `CssProperty::TextTransform` | `CssValue::TextTransform` | Text and font | `Existing style property` | `Property::TextTransform` + `Value::TextTransform` | Text transform has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::Inset` | `CssValue::Edges` | Position and stacking | `Existing style shorthand` | `Property::Inset` + `Value::Edges` | Style shorthand canonicalization lowers to top, right, bottom, and left longhands. | Operation 8 layout-facing properties |
| `CssProperty::Top` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Top` + `Value::Length` | Top inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Right` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Right` + `Value::Length` | Right inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Bottom` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Bottom` + `Value::Length` | Bottom inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Left` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Left` + `Value::Length` | Left inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::ZIndex` | `CssValue::ZIndex` | Position and stacking | `Existing style property` | `Property::ZIndex` + `Value::ZIndex` | Auto and integer z-index values use the style-owned `ZIndex` enum. | Operation 8 layout-facing properties |
| `CssProperty::BoxDecorationBreak` | `CssValue::BoxDecorationBreak` | Paint and effects | `Existing style property` | `Property::BoxDecorationBreak` + `Value::BoxDecorationBreak` | Fragmented-box paint policy has a typed style enum. | No property implementation |
| `CssProperty::Margin` | `CssValue::Edges` | Sizing and spacing | `Existing style shorthand` | `Property::Margin` + `Value::Edges` | Style shorthand canonicalization lowers to margin side longhands. | Operation 8 layout-facing properties |
| `CssProperty::MarginTop` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MarginTop` + `Value::Length` | Margin top has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::MarginRight` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MarginRight` + `Value::Length` | Margin right has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::MarginBottom` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MarginBottom` + `Value::Length` | Margin bottom has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::MarginLeft` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::MarginLeft` + `Value::Length` | Margin left has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Padding` | `CssValue::Edges` | Sizing and spacing | `Existing style shorthand` | `Property::Padding` + `Value::Edges` | Style shorthand canonicalization lowers to padding side longhands. | Operation 8 layout-facing properties |
| `CssProperty::PaddingTop` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::PaddingTop` + `Value::Length` | Padding top has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::PaddingRight` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::PaddingRight` + `Value::Length` | Padding right has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::PaddingBottom` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::PaddingBottom` + `Value::Length` | Padding bottom has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::PaddingLeft` | `CssValue::Length` | Sizing and spacing | `Existing style property` | `Property::PaddingLeft` + `Value::Length` | Padding left has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Border` | `CssValue::Border` | Border and outline | `Existing style shorthand` | `Property::Border` + `Value::Border` | Style shorthand canonicalization lowers to side width, style, and color longhands, resetting omitted shorthand components to defaults. | No property implementation |
| `CssProperty::BorderTop` | `CssValue::Border` | Border and outline | `Existing style shorthand` | `Property::BorderTop` + `Value::Border` | Style shorthand canonicalization lowers to top border width, style, and color longhands, resetting omitted components to defaults. | No property implementation |
| `CssProperty::BorderRight` | `CssValue::Border` | Border and outline | `Existing style shorthand` | `Property::BorderRight` + `Value::Border` | Style shorthand canonicalization lowers to right border width, style, and color longhands, resetting omitted components to defaults. | No property implementation |
| `CssProperty::BorderBottom` | `CssValue::Border` | Border and outline | `Existing style shorthand` | `Property::BorderBottom` + `Value::Border` | Style shorthand canonicalization lowers to bottom border width, style, and color longhands, resetting omitted components to defaults. | No property implementation |
| `CssProperty::BorderLeft` | `CssValue::Border` | Border and outline | `Existing style shorthand` | `Property::BorderLeft` + `Value::Border` | Style shorthand canonicalization lowers to left border width, style, and color longhands, resetting omitted components to defaults. | No property implementation |
| `CssProperty::BorderWidth` | `CssValue::Edges` | Border and outline | `Existing style shorthand` | `Property::BorderWidth` + `Value::Edges` | Style shorthand canonicalization lowers to border-width side longhands. | Operation 8 layout-facing properties |
| `CssProperty::BorderTopWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderTopWidth` + `Value::Length` | Border top width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderRightWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderRightWidth` + `Value::Length` | Border right width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderBottomWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderBottomWidth` + `Value::Length` | Border bottom width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderLeftWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderLeftWidth` + `Value::Length` | Border left width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Color` | `CssValue::Color` | Color | `Existing style property` | `Property::Color` + `Value::StyleColor` | Style owns concrete RGBA and symbolic colors including currentColor, system colors, modern color functions, color-mix, and relative colors; final color-space and system-color resolution remains outside style. | No property implementation |
| `CssProperty::Background` | `CssValue::Color` | Background | `Existing style property` | `Property::Background` + `Value::StyleColor` | Current CSS parser surface accepts color-only `background`; style stores that as the background color slot and does not claim full background shorthand layer support. | No property implementation |
| `CssProperty::BackgroundColor` | `CssValue::Color` | Background | `Existing style property` | `Property::Background` + `Value::StyleColor` | Background color maps to the style-owned background color slot and accepts concrete or symbolic style colors. | No property implementation |
| `CssProperty::BorderColor` | `CssValue::Color` | Border and outline | `Existing style shorthand` | `Property::BorderColor` + `Value::StyleColor` | Style shorthand canonicalization lowers one style-owned color to the four border side color longhands. | No property implementation |
| `CssProperty::BorderTopColor` | `CssValue::Color` | Border and outline | `Existing style property` | `Property::BorderTopColor` + `Value::StyleColor` | Top border color accepts concrete and symbolic style-owned colors; final color resolution remains outside style. | No property implementation |
| `CssProperty::BorderRightColor` | `CssValue::Color` | Border and outline | `Existing style property` | `Property::BorderRightColor` + `Value::StyleColor` | Right border color accepts concrete and symbolic style-owned colors; final color resolution remains outside style. | No property implementation |
| `CssProperty::BorderBottomColor` | `CssValue::Color` | Border and outline | `Existing style property` | `Property::BorderBottomColor` + `Value::StyleColor` | Bottom border color accepts concrete and symbolic style-owned colors; final color resolution remains outside style. | No property implementation |
| `CssProperty::BorderLeftColor` | `CssValue::Color` | Border and outline | `Existing style property` | `Property::BorderLeftColor` + `Value::StyleColor` | Left border color accepts concrete and symbolic style-owned colors; final color resolution remains outside style. | No property implementation |
| `CssProperty::BackgroundImage` | `CssValue::BackgroundImage` | Background | `Existing style property` | `Property::BackgroundImage` + `Value::ImageLayerList` | Background image layers preserve style-owned symbolic image payloads; resource loading and render realization remain unresolved outside style. | No property implementation |
| `CssProperty::BackgroundPosition` | `CssValue::BackgroundPosition` | Background | `Existing style property` | `Property::BackgroundPosition` + `Value::PositionList` | Background position layers have typed style-owned position data. | No property implementation |
| `CssProperty::BackgroundSize` | `CssValue::BackgroundSize` | Background | `Existing style property` | `Property::BackgroundSize` + `Value::BackgroundSizeList` | Background size layers have typed style-owned size data. | No property implementation |
| `CssProperty::BackgroundRepeat` | `CssValue::BackgroundRepeat` | Background | `Existing style property` | `Property::BackgroundRepeat` + `Value::BackgroundRepeatList` | Background repeat layers have typed style-owned repeat data. | No property implementation |
| `CssProperty::BackgroundOrigin` | `CssValue::BackgroundBox` | Background | `Existing style property` | `Property::BackgroundOrigin` + `Value::BackgroundBox` | Background origin box has typed style-owned box data. | No property implementation |
| `CssProperty::BackgroundClip` | `CssValue::BackgroundBox` | Background | `Existing style property` | `Property::BackgroundClip` + `Value::BackgroundBox` | Background clip box has typed style-owned box data. | No property implementation |
| `CssProperty::BackgroundAttachment` | `CssValue::BackgroundAttachment` | Background | `Existing style property` | `Property::BackgroundAttachment` + `Value::BackgroundAttachmentList` | Background attachment layers have typed style-owned attachment data. | No property implementation |
| `CssProperty::BorderStyle` | `CssValue::BorderStyles` | Border and outline | `Existing style shorthand` | `Property::BorderStyle` + `Value::BorderStyles` | Style shorthand canonicalization lowers multi-side border styles to side style longhands. | No property implementation |
| `CssProperty::BorderTopStyle` | `CssValue::BorderStyle` | Border and outline | `Existing style property` | `Property::BorderTopStyle` + `Value::BorderLineStyle` | Top border line style has typed style-owned data. | No property implementation |
| `CssProperty::BorderRightStyle` | `CssValue::BorderStyle` | Border and outline | `Existing style property` | `Property::BorderRightStyle` + `Value::BorderLineStyle` | Right border line style has typed style-owned data. | No property implementation |
| `CssProperty::BorderBottomStyle` | `CssValue::BorderStyle` | Border and outline | `Existing style property` | `Property::BorderBottomStyle` + `Value::BorderLineStyle` | Bottom border line style has typed style-owned data. | No property implementation |
| `CssProperty::BorderLeftStyle` | `CssValue::BorderStyle` | Border and outline | `Existing style property` | `Property::BorderLeftStyle` + `Value::BorderLineStyle` | Left border line style has typed style-owned data. | No property implementation |
| `CssProperty::BorderRadius` | `CssValue::BorderRadius` | Border and outline | `Existing style shorthand` | `Property::Radius` + `Value::BorderRadii` | Style shorthand canonicalization lowers corner radii to the four individual corner longhands, including elliptical radius data. | No property implementation |
| `CssProperty::BorderTopLeftRadius` | `CssValue::CornerRadius` | Border and outline | `Existing style property` | `Property::BorderTopLeftRadius` + `Value::CornerRadius` | Top-left corner radius has typed style-owned corner radius data. | No property implementation |
| `CssProperty::BorderTopRightRadius` | `CssValue::CornerRadius` | Border and outline | `Existing style property` | `Property::BorderTopRightRadius` + `Value::CornerRadius` | Top-right corner radius has typed style-owned corner radius data. | No property implementation |
| `CssProperty::BorderBottomRightRadius` | `CssValue::CornerRadius` | Border and outline | `Existing style property` | `Property::BorderBottomRightRadius` + `Value::CornerRadius` | Bottom-right corner radius has typed style-owned corner radius data. | No property implementation |
| `CssProperty::BorderBottomLeftRadius` | `CssValue::CornerRadius` | Border and outline | `Existing style property` | `Property::BorderBottomLeftRadius` + `Value::CornerRadius` | Bottom-left corner radius has typed style-owned corner radius data. | No property implementation |
| `CssProperty::BoxShadow` | `CssValue::BoxShadow` | Paint and effects | `Existing style property` | `Property::Shadow` + `Value::ShadowList` | Shadow lists have a typed style model. | No property implementation |
| `CssProperty::Opacity` | `CssValue::Opacity` | Paint and effects | `Existing style property` | `Property::Opacity` + `Value::Number` | Opacity has typed numeric style storage and validation. | No property implementation |
| `CssProperty::FlexGrow` | `CssValue::FlexGrow` | Flex | `Existing style property` | `Property::FlexGrow` + `Value::FlexFactor` | Flex grow uses the style-owned non-negative flex factor type. | Operation 8 layout-facing properties |
| `CssProperty::FlexShrink` | `CssValue::FlexShrink` | Flex | `Existing style property` | `Property::FlexShrink` + `Value::FlexFactor` | Flex shrink uses the style-owned non-negative flex factor type. | Operation 8 layout-facing properties |
| `CssProperty::Order` | `CssValue::Order` | Flex | `Existing style property` | `Property::Order` + `Value::Order` | Flex/grid item ordering has a style-owned integer newtype. | Operation 8 layout-facing properties |
| `CssProperty::Flex` | `CssValue::Flex` | Flex | `Existing style shorthand` | `Property::Flex` + `Value::Flex` | Style shorthand canonicalization lowers to flex-grow, flex-shrink, and flex-basis. | Operation 8 layout-facing properties |
| `CssProperty::JustifyTracks` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::JustifyTracks` + `Value::AlignContent` | Grid track justification uses the style-owned content-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::AlignTracks` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::AlignTracks` + `Value::AlignContent` | Grid track alignment uses the style-owned content-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::AspectRatio` | `CssValue::AspectRatio` | Sizing and spacing | `Existing style property` | `Property::AspectRatio` + `Value::AspectRatio` | Aspect ratio uses the style-owned auto-or-ratio type. | Operation 8 layout-facing properties |
| `CssProperty::ScrollbarWidth` | `CssValue::ScrollbarWidth` | Overflow and visibility | `Existing style property` | `Property::ScrollbarWidth` + `Value::ScrollbarWidth` | Scrollbar width keywords use the style-owned enum. | Operation 8 layout-facing properties |
| `CssProperty::Cursor` | `CssValue::Cursor` | Interaction | `Existing style property` | `Property::Cursor` + `Value::Cursor` | Cursor has typed style data; platform cursor realization remains outside style. | No property implementation |
| `CssProperty::PointerEvents` | `CssValue::PointerEvents` | Interaction | `Existing style property` | `Property::PointerEvents` + `Value::PointerEvents` | Pointer event participation has typed style data. | No property implementation |
| `CssProperty::UserSelect` | `CssValue::UserSelect` | Interaction | `Existing style property` | `Property::UserSelect` + `Value::UserSelect` | User selection participation has typed style-owned interaction data. | No property implementation |
| `CssProperty::Outline` | `CssValue::Outline` | Border and outline | `Existing style shorthand` | `Property::Outline` + `Value::Outline` | CSS outline shorthand lowers to outline width, style, and color longhands and remains distinct from focus outline. | No property implementation |
| `CssProperty::OutlineColor` | `CssValue::OutlineColor` | Border and outline | `Existing style property` | `Property::OutlineColor` + `Value::StyleColor` | CSS outline color accepts concrete and symbolic style-owned colors; final color resolution remains outside style. | No property implementation |
| `CssProperty::OutlineStyle` | `CssValue::OutlineStyle` | Border and outline | `Existing style property` | `Property::OutlineStyle` + `Value::OutlineStyle` | CSS outline style has typed style-owned data distinct from focus outline. | No property implementation |
| `CssProperty::OutlineWidth` | `CssValue::OutlineWidth` | Border and outline | `Existing style property` | `Property::OutlineWidth` + `Value::OutlineWidth` | CSS outline width has typed keyword-or-length style data distinct from focus outline. | No property implementation |
| `CssProperty::Transform` | `CssValue::Transform` | Transforms | `Existing style property` | `Property::Transform` + `Value::Transform` | Transform functions have a typed style transform list; unsupported render capability choices remain outside style. | No property implementation |
| `CssProperty::TransformOrigin` | `CssValue::TransformOrigin` | Transforms | `Existing style property` | `Property::TransformOrigin` + `Value::Size` | Transform origin has typed two-axis style data. | No property implementation |
| `CssProperty::Translate` | `CssValue::Translate` | Transforms | `Existing style property` | `Property::Translate` + `Value::Translate` | Individual translate has typed style-owned transform data, including symbolic lengths. | No property implementation |
| `CssProperty::Rotate` | `CssValue::Rotate` | Transforms | `Existing style property` | `Property::Rotate` + `Value::Rotate` | Individual rotate has typed style-owned transform data, including symbolic function values. | No property implementation |
| `CssProperty::Scale` | `CssValue::Scale` | Transforms | `Existing style property` | `Property::Scale` + `Value::Scale` | Individual scale has typed style-owned transform data. | No property implementation |
| `CssProperty::Filter` | `CssValue::Filter` | Paint and effects | `Existing style property` | `Property::Filter` + `Value::Filter` | Filter function payloads remain style-owned symbolic effect data until render capability resolution. | No property implementation |
| `CssProperty::BackdropFilter` | `CssValue::Filter` | Paint and effects | `Existing style property` | `Property::BackdropFilter` + `Value::Filter` | Backdrop filter function payloads remain style-owned symbolic effect data until render capability resolution. | No property implementation |
| `CssProperty::ClipPath` | `CssValue::ClipPath` | Paint and effects | `Existing style property` | `Property::ClipPath` + `Value::ClipPath` | Clip paths preserve style-owned symbolic references and basic shape payloads until render realization. | No property implementation |
| `CssProperty::Mask` | `CssValue::Mask` | Paint and effects | `Existing style shorthand` | `Property::Mask` + `Value::MaskLayerList` | Mask layer shorthand lowers to image, position, size, and repeat layer longhands; symbolic resource payloads remain unresolved outside style. | No property implementation |
| `CssProperty::MaskImage` | `CssValue::MaskImage` | Paint and effects | `Existing style property` | `Property::MaskImage` + `Value::ImageLayerList` | Mask image layers preserve style-owned symbolic resources; loading and render resources remain outside style. | No property implementation |
| `CssProperty::MaskSize` | `CssValue::MaskSize` | Paint and effects | `Existing style property` | `Property::MaskSize` + `Value::BackgroundSizeList` | Mask size layers reuse typed style-owned background-size layer data. | No property implementation |
| `CssProperty::MaskPosition` | `CssValue::MaskPosition` | Paint and effects | `Existing style property` | `Property::MaskPosition` + `Value::PositionList` | Mask position layers have typed style-owned position data. | No property implementation |
| `CssProperty::MaskRepeat` | `CssValue::MaskRepeat` | Paint and effects | `Existing style property` | `Property::MaskRepeat` + `Value::BackgroundRepeatList` | Mask repeat layers reuse typed style-owned background-repeat layer data. | No property implementation |
| `CssProperty::TransitionProperty` | `CssValue::TransitionProperty` | Timing and animation | `Existing style property` | `Property::TransitionProperty` + `Value::PropertyList` | Style has property-list storage; CSS name coverage and `all` policy need timing-plan review. | Operation 12 timing/animation/keyframes |
| `CssProperty::TransitionDuration` | `CssValue::TimeList` | Timing and animation | `Existing style property` | `Property::TransitionDuration` + `Value::Number` | Single numeric duration exists; CSS time lists need typed list expansion. | Operation 12 timing/animation/keyframes |
| `CssProperty::TransitionDelay` | `CssValue::TimeList` | Timing and animation | `Existing style property` | `Property::TransitionDelay` + `Value::Number` | Single numeric delay exists; CSS time lists need typed list expansion. | Operation 12 timing/animation/keyframes |
| `CssProperty::TransitionTimingFunction` | `CssValue::EasingList` | Timing and animation | `Symbolic style data needed` | Planned transition easing list model | Easing functions should be preserved symbolically until timing evaluation. | Operation 12 timing/animation/keyframes |
| `CssProperty::Transition` | `CssValue::Transition` | Timing and animation | `New shorthand lowering needed` | Planned transition list model plus longhand lowering | Style has partial transition longhands; shorthand/list lowering is missing. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationName` | `CssValue::AnimationName` | Timing and animation | `Existing style property` | `Property::AnimationName` + `Value::AnimationNameList` | Animation names are preserved as symbolic keyframe references. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationDuration` | `CssValue::TimeList` | Timing and animation | `New style property needed` | Planned animation duration list model | Style lacks typed animation duration data. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationDelay` | `CssValue::TimeList` | Timing and animation | `New style property needed` | Planned animation delay list model | Style lacks typed animation delay data. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationTimingFunction` | `CssValue::EasingList` | Timing and animation | `Symbolic style data needed` | Planned animation easing list model | Easing functions should be preserved symbolically until timing evaluation. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationIterationCount` | `CssValue::AnimationIterationCount` | Timing and animation | `New style property needed` | Planned animation iteration-count list model | Style lacks typed animation iteration data. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationDirection` | `CssValue::AnimationDirection` | Timing and animation | `New style property needed` | Planned animation direction list model | Style lacks typed animation direction data. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationFillMode` | `CssValue::AnimationFillMode` | Timing and animation | `New style property needed` | Planned animation fill-mode list model | Style lacks typed animation fill-mode data. | Operation 12 timing/animation/keyframes |
| `CssProperty::AnimationPlayState` | `CssValue::AnimationPlayState` | Timing and animation | `New style property needed` | Planned animation play-state list model | Style lacks typed animation play-state data. | Operation 12 timing/animation/keyframes |
| `CssProperty::Animation` | `CssValue::Animation` | Timing and animation | `New shorthand lowering needed` | Planned animation list model plus longhand lowering | Style has `AnimationName` only; timing, direction, fill, play-state, and iteration counts are missing. | Operation 12 timing/animation/keyframes |
| `CssProperty::Custom(CssCustomPropertyName)` | `CssValue::CustomProperty` | Custom properties | `Existing authored cascade model` | `CustomPropertyName`, `CustomPropertyValue`, `VariableDependentValue` | Custom property storage and variable substitution models exist; later plans may expand typed value coverage. | No property implementation |

## Family Rollup

| Family | Existing style support | Missing style support | Next implementation plan |
| --- | --- | --- | --- |
| Authored cascade | `AuthoredProperty::All`, `AuthoredDeclaration::css_wide`, custom property cascade entries, and variable-dependent declaration paths exist. | Broader cascade ordering, origins, layers, and `revert` semantics remain separate sequence work. | No property implementation |
| Display and box | `Display`, `BoxSizing`, `Position`, `Float`, and `Clear` have typed properties and values. | Future CSS-only box values need separate parity review. | No property implementation |
| Overflow and visibility | `Overflow`, `OverflowX`, `OverflowY`, `Visibility`, `ContentVisibility`, and `ScrollbarWidth` have typed style targets. | Visibility collapse behavior remains a future layout parity review. | No property implementation |
| Sizing and spacing | Width/height, min/max sizes, gap, margin/padding sides and shorthands, aspect ratio, and shared `Length`/`Edges` data exist. | No Operation 8 sizing or spacing property gap remains in this ledger. | No property implementation |
| Position and stacking | `Position`, `Inset`, top/right/bottom/left, and `ZIndex` have typed style targets. | No Operation 8 position or stacking property gap remains in this ledger. | No property implementation |
| Flex | Direction, wrap, grow, shrink, basis, order, and flex shorthand lowering exist. | No Operation 8 flex property gap remains in this ledger. | No property implementation |
| Grid | Track lists, template areas, template/grid shorthands, placement, auto-flow, flow tolerance, and track alignment properties exist. | No Operation 8 grid property gap remains in this ledger. | No property implementation |
| Alignment | Align/justify content/items/self targets, place shorthands, and track alignment properties exist. | No Operation 8 alignment property gap remains in this ledger. | No property implementation |
| Writing mode | `Direction` and `WritingMode` are inherited typed style properties. | CSS parity review for additional writing-mode values remains future parser-lowering work. | No property implementation |
| Text and font | Font family, font size, line height, text alignment, font shorthand and longhands, font feature settings, spacing, wrapping, text overflow, text decoration line/color/style/thickness, and text transform have typed style targets. | No Operation 9 text or font property gap remains in this ledger. | No property implementation |
| Generated content and lists | Generated content and lists now have typed style targets for `content`, list marker type/position/image/shorthand, counter reset/increment/set, counter/counters content functions, quote payloads, attr payloads, and pseudo-element style buckets. | Counter formatting, quote depth evaluation, attr lookup, marker materialization, retained projection, image loading, and render resources remain outside style. | No property implementation |
| Color | `Color`, `TextDecorationColor`, border colors, outline color, and background color use `Value::StyleColor` for concrete and symbolic style-owned color data. | Final currentColor, system-color, and color-space resolution remains outside style; no Operation 10 color property gap remains in this ledger. | No property implementation |
| Background | Background color and background image, position, size, repeat, origin, clip, and attachment longhands have typed style targets. | Full CSS `background` shorthand layering is not claimed; the current CSS `Background` row remains color-only. | No property implementation |
| Border and outline | Border width/style/color side longhands, border side shorthands, border radius corner longhands, radius shorthand, and CSS outline shorthand/longhands have typed style targets. | Focus outline remains a distinct style property; no Operation 10 border or outline property gap remains in this ledger. | No property implementation |
| Paint and effects | Shadow, opacity, box decoration break, filters, backdrop filters, clip paths, and mask shorthand/longhands have typed style targets. | Symbolic filter, clip, image, and mask payloads remain style-owned and unresolved until render capability/resource realization. | No property implementation |
| Transforms | `Transform`, `TransformOrigin`, and individual `Translate`, `Rotate`, and `Scale` properties have typed style targets. | Render capability choices for transform realization remain outside style; no Operation 10 transform property gap remains in this ledger. | No property implementation |
| Interaction | `Cursor`, `PointerEvents`, and `UserSelect` have typed style targets. | Platform cursor and selection behavior realization remains outside style; no Operation 10 interaction property gap remains in this ledger. | No property implementation |
| Timing and animation | Transition property/duration/delay and animation names have partial typed targets. | Time lists, easing lists, transition shorthand, animation longhands, animation shorthand, and keyframe style data are missing. | Operation 12 timing/animation/keyframes |
| Custom properties | `CustomPropertyName`, `CustomPropertyValue`, `VariableDependentValue`, and authored custom declarations exist. | Typed value coverage may expand as future property families add supported receiving models. | No property implementation |

## Coverage Audit

| Audit | Expected | Observed | Result |
| --- | --- | --- | --- |
| `CssProperty` variants in `surgeist-css` | `180` | `180` | Pass |
| Property ledger rows | `180` | `180` | Pass |
| Duplicate property rows | `0` | `0` | Pass |
| Missing property rows | `0` | `0` | Pass |

## Dependency And Boundary Check

`surgeist-style` source and tests do not depend on `surgeist-css`; this ledger
uses read-only source inspection only.

This ledger does not introduce Rust source changes, parser dependencies,
adapters, generated content materialization, layout algorithms, text shaping, or
render resources.

## Next Sequence Context

The next implementation plan should cover Operation 12: timing, animation, and
keyframe style data.

Use this ledger instead of re-inventorying the full CSS property surface. The
timing/animation/keyframes plan should start with the `Transition*`,
`Animation*`, and keyframe style data gaps that point to Operation 12.

The timing/animation/keyframes plan should implement style-owned models and
lowering front doors for time lists, easing lists, transition shorthand/list
lowering, animation longhands/shorthand, and keyframe style data. It should not
add a style-to-render adapter or Operation 14 cache/invalidation generalization.

Operation 8 layout-facing rows, Operation 9 text-facing rows, and Operation 10
paint/color/effects rows, and Operation 11 generated content/counters/lists rows
have been rebased after implementation, so Operation 12 can proceed from the
current timing/animation/keyframes gaps.
