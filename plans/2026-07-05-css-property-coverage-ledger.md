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
| `CssProperty::Content` | `CssValue::Content` | Generated content and lists | `New style property needed` | Planned generated content model scoped to `StyleBucket` | Style should own generated content policy/data; retained/tree materialization remains outside style. | Operation 11 generated content/counters/lists |
| `CssProperty::ContentVisibility` | `CssValue::ContentVisibility` | Overflow and visibility | `Existing style property` | `Property::ContentVisibility` + `Value::ContentVisibility` | Content visibility has typed layout/paint style data. | Operation 8 layout-facing properties |
| `CssProperty::ListStyleType` | `CssValue::ListStyleType` | Generated content and lists | `New style property needed` | Planned list marker type model | Style needs typed marker style data; marker text materialization remains outside style. | Operation 11 generated content/counters/lists |
| `CssProperty::ListStylePosition` | `CssValue::ListStylePosition` | Generated content and lists | `New style property needed` | Planned list marker position model | Style lacks marker position data for list layout policy. | Operation 11 generated content/counters/lists |
| `CssProperty::ListStyleImage` | `CssValue::ListStyleImage` | Generated content and lists | `Symbolic style data needed` | Planned symbolic list marker image model | URLs and image resources should be preserved symbolically; loading and render resources stay outside style. | Operation 11 generated content/counters/lists |
| `CssProperty::ListStyle` | `CssValue::ListStyle` | Generated content and lists | `New shorthand lowering needed` | Planned lowering to list-style type, position, and image models | Style needs explicit list shorthand lowering without storing an untyped list bag. | Operation 11 generated content/counters/lists |
| `CssProperty::CounterReset` | `CssValue::CounterChanges` | Generated content and lists | `New style property needed` | Planned counter reset model | Style lacks counter mutation data for generated content. | Operation 11 generated content/counters/lists |
| `CssProperty::CounterIncrement` | `CssValue::CounterChanges` | Generated content and lists | `New style property needed` | Planned counter increment model | Style lacks counter mutation data for generated content. | Operation 11 generated content/counters/lists |
| `CssProperty::CounterSet` | `CssValue::CounterChanges` | Generated content and lists | `New style property needed` | Planned counter set model | Style lacks counter mutation data for generated content. | Operation 11 generated content/counters/lists |
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
| `CssProperty::TextDecoration` | `CssValue::TextDecoration` | Text and font | `Existing style shorthand` | `Property::TextDecoration` + `Value::TextDecoration` | Operation 9 lowers line, style, and thickness; `TextDecorationColor` remains Operation 10. | Operation 9 text-facing properties |
| `CssProperty::TextDecorationLine` | `CssValue::TextDecorationLine` | Text and font | `Existing style property` | `Property::TextDecorationLine` + `Value::TextDecorationLine` | Text decoration line components have typed style data. | Operation 9 text-facing properties |
| `CssProperty::TextDecorationColor` | `CssValue::TextDecorationColor` | Color | `New style property needed` | Planned `text-decoration-color` model using color data | Concrete colors can use style color primitives, but the property path is missing. | Operation 10 paint/color/effects |
| `CssProperty::TextDecorationStyle` | `CssValue::TextDecorationStyle` | Text and font | `Existing style property` | `Property::TextDecorationStyle` + `Value::TextDecorationStyle` | Text decoration stroke style has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::TextDecorationThickness` | `CssValue::TextDecorationThickness` | Text and font | `Existing style property` | `Property::TextDecorationThickness` + `Value::TextDecorationThickness` | Text decoration thickness has typed auto, from-font, and length data. | Operation 9 text-facing properties |
| `CssProperty::TextTransform` | `CssValue::TextTransform` | Text and font | `Existing style property` | `Property::TextTransform` + `Value::TextTransform` | Text transform has a typed style enum. | Operation 9 text-facing properties |
| `CssProperty::Inset` | `CssValue::Edges` | Position and stacking | `Existing style shorthand` | `Property::Inset` + `Value::Edges` | Style shorthand canonicalization lowers to top, right, bottom, and left longhands. | Operation 8 layout-facing properties |
| `CssProperty::Top` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Top` + `Value::Length` | Top inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Right` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Right` + `Value::Length` | Right inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Bottom` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Bottom` + `Value::Length` | Bottom inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Left` | `CssValue::Length` | Position and stacking | `Existing style property` | `Property::Left` + `Value::Length` | Left inset has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::ZIndex` | `CssValue::ZIndex` | Position and stacking | `Existing style property` | `Property::ZIndex` + `Value::ZIndex` | Auto and integer z-index values use the style-owned `ZIndex` enum. | Operation 8 layout-facing properties |
| `CssProperty::BoxDecorationBreak` | `CssValue::BoxDecorationBreak` | Paint and effects | `New style property needed` | Planned `box-decoration-break` model | Style lacks fragmented-box paint policy data. | Operation 10 paint/color/effects |
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
| `CssProperty::Border` | `CssValue::Border` | Border and outline | `New shorthand lowering needed` | Planned lowering to border width, style, and color models | CSS border shorthand needs explicit lowering; style lacks complete side/style/color data. | Operation 10 paint/color/effects |
| `CssProperty::BorderTop` | `CssValue::Border` | Border and outline | `New shorthand lowering needed` | Planned lowering to top border width, style, and color | Side border shorthand needs explicit lowering into side-specific border data. | Operation 10 paint/color/effects |
| `CssProperty::BorderRight` | `CssValue::Border` | Border and outline | `New shorthand lowering needed` | Planned lowering to right border width, style, and color | Side border shorthand needs explicit lowering into side-specific border data. | Operation 10 paint/color/effects |
| `CssProperty::BorderBottom` | `CssValue::Border` | Border and outline | `New shorthand lowering needed` | Planned lowering to bottom border width, style, and color | Side border shorthand needs explicit lowering into side-specific border data. | Operation 10 paint/color/effects |
| `CssProperty::BorderLeft` | `CssValue::Border` | Border and outline | `New shorthand lowering needed` | Planned lowering to left border width, style, and color | Side border shorthand needs explicit lowering into side-specific border data. | Operation 10 paint/color/effects |
| `CssProperty::BorderWidth` | `CssValue::Edges` | Border and outline | `Existing style shorthand` | `Property::BorderWidth` + `Value::Edges` | Style shorthand canonicalization lowers to border-width side longhands. | Operation 8 layout-facing properties |
| `CssProperty::BorderTopWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderTopWidth` + `Value::Length` | Border top width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderRightWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderRightWidth` + `Value::Length` | Border right width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderBottomWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderBottomWidth` + `Value::Length` | Border bottom width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::BorderLeftWidth` | `CssValue::Length` | Border and outline | `Existing style property` | `Property::BorderLeftWidth` + `Value::Length` | Border left width has a canonical side longhand and typed front door. | Operation 8 layout-facing properties |
| `CssProperty::Color` | `CssValue::Color` | Color | `Existing style property` | `Property::Color` + `Value::Color` | Concrete RGBA is supported; symbolic colors and variable-dependent components need symbolic style data. | Operation 10 paint/color/effects |
| `CssProperty::Background` | `CssValue::Color` | Background | `Existing style shorthand` | `Property::Background` + `Value::Color` | Current parser surface accepts color-only background; full background layers need future lowering. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundColor` | `CssValue::Color` | Background | `Existing style property` | `Property::Background` + `Value::Color` | Concrete background color lowers to the existing background color property. | Operation 10 paint/color/effects |
| `CssProperty::BorderColor` | `CssValue::Color` | Border and outline | `Existing style property` | `Property::BorderColor` + `Value::Color` | Concrete all-side border color exists; side colors and symbolic colors need expansion. | Operation 10 paint/color/effects |
| `CssProperty::BorderTopColor` | `CssValue::Color` | Border and outline | `New style property needed` | Planned top border color model | Style lacks side-specific border color data. | Operation 10 paint/color/effects |
| `CssProperty::BorderRightColor` | `CssValue::Color` | Border and outline | `New style property needed` | Planned right border color model | Style lacks side-specific border color data. | Operation 10 paint/color/effects |
| `CssProperty::BorderBottomColor` | `CssValue::Color` | Border and outline | `New style property needed` | Planned bottom border color model | Style lacks side-specific border color data. | Operation 10 paint/color/effects |
| `CssProperty::BorderLeftColor` | `CssValue::Color` | Border and outline | `New style property needed` | Planned left border color model | Style lacks side-specific border color data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundImage` | `CssValue::BackgroundImage` | Background | `Symbolic style data needed` | Planned background image layer model | URLs/images are authored symbolic resources; loading and final render resources stay outside style. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundPosition` | `CssValue::BackgroundPosition` | Background | `New style property needed` | Planned background position layer model | Style lacks typed background-position layer data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundSize` | `CssValue::BackgroundSize` | Background | `New style property needed` | Planned background size layer model | Style lacks typed background-size layer data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundRepeat` | `CssValue::BackgroundRepeat` | Background | `New style property needed` | Planned background repeat layer model | Style lacks typed background-repeat layer data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundOrigin` | `CssValue::BackgroundBox` | Background | `New style property needed` | Planned background origin box model | Style lacks typed background-origin data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundClip` | `CssValue::BackgroundBox` | Background | `New style property needed` | Planned background clip box model | Style lacks typed background-clip data. | Operation 10 paint/color/effects |
| `CssProperty::BackgroundAttachment` | `CssValue::BackgroundAttachment` | Background | `New style property needed` | Planned background attachment layer model | Style lacks typed background-attachment data. | Operation 10 paint/color/effects |
| `CssProperty::BorderStyle` | `CssValue::BorderStyles` | Border and outline | `New style property needed` | Planned multi-side border style model | A property variant exists, but non-keyword CSS border styles are not accepted typed style values. | Operation 10 paint/color/effects |
| `CssProperty::BorderTopStyle` | `CssValue::BorderStyle` | Border and outline | `New style property needed` | Planned top border style model | Style lacks side-specific border style data. | Operation 10 paint/color/effects |
| `CssProperty::BorderRightStyle` | `CssValue::BorderStyle` | Border and outline | `New style property needed` | Planned right border style model | Style lacks side-specific border style data. | Operation 10 paint/color/effects |
| `CssProperty::BorderBottomStyle` | `CssValue::BorderStyle` | Border and outline | `New style property needed` | Planned bottom border style model | Style lacks side-specific border style data. | Operation 10 paint/color/effects |
| `CssProperty::BorderLeftStyle` | `CssValue::BorderStyle` | Border and outline | `New style property needed` | Planned left border style model | Style lacks side-specific border style data. | Operation 10 paint/color/effects |
| `CssProperty::BorderRadius` | `CssValue::BorderRadius` | Border and outline | `Existing style shorthand` | `Property::Radius` + `Value::Corners` | Aggregate corner radii exist; elliptical and per-corner parity need paint-pass review. | Operation 10 paint/color/effects |
| `CssProperty::BorderTopLeftRadius` | `CssValue::CornerRadius` | Border and outline | `New style property needed` | Planned top-left radius model | Style lacks individual corner-radius storage and elliptical radius data. | Operation 10 paint/color/effects |
| `CssProperty::BorderTopRightRadius` | `CssValue::CornerRadius` | Border and outline | `New style property needed` | Planned top-right radius model | Style lacks individual corner-radius storage and elliptical radius data. | Operation 10 paint/color/effects |
| `CssProperty::BorderBottomRightRadius` | `CssValue::CornerRadius` | Border and outline | `New style property needed` | Planned bottom-right radius model | Style lacks individual corner-radius storage and elliptical radius data. | Operation 10 paint/color/effects |
| `CssProperty::BorderBottomLeftRadius` | `CssValue::CornerRadius` | Border and outline | `New style property needed` | Planned bottom-left radius model | Style lacks individual corner-radius storage and elliptical radius data. | Operation 10 paint/color/effects |
| `CssProperty::BoxShadow` | `CssValue::BoxShadow` | Paint and effects | `Existing style property` | `Property::Shadow` + `Value::ShadowList` | Concrete shadow lists have a typed style model. | Operation 10 paint/color/effects |
| `CssProperty::Opacity` | `CssValue::Opacity` | Paint and effects | `Existing style property` | `Property::Opacity` + `Value::Number` | Opacity has typed numeric style storage and validation. | Operation 10 paint/color/effects |
| `CssProperty::FlexGrow` | `CssValue::FlexGrow` | Flex | `Existing style property` | `Property::FlexGrow` + `Value::FlexFactor` | Flex grow uses the style-owned non-negative flex factor type. | Operation 8 layout-facing properties |
| `CssProperty::FlexShrink` | `CssValue::FlexShrink` | Flex | `Existing style property` | `Property::FlexShrink` + `Value::FlexFactor` | Flex shrink uses the style-owned non-negative flex factor type. | Operation 8 layout-facing properties |
| `CssProperty::Order` | `CssValue::Order` | Flex | `Existing style property` | `Property::Order` + `Value::Order` | Flex/grid item ordering has a style-owned integer newtype. | Operation 8 layout-facing properties |
| `CssProperty::Flex` | `CssValue::Flex` | Flex | `Existing style shorthand` | `Property::Flex` + `Value::Flex` | Style shorthand canonicalization lowers to flex-grow, flex-shrink, and flex-basis. | Operation 8 layout-facing properties |
| `CssProperty::JustifyTracks` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::JustifyTracks` + `Value::AlignContent` | Grid track justification uses the style-owned content-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::AlignTracks` | `CssValue::Alignment` | Alignment | `Existing style property` | `Property::AlignTracks` + `Value::AlignContent` | Grid track alignment uses the style-owned content-alignment model. | Operation 8 layout-facing properties |
| `CssProperty::AspectRatio` | `CssValue::AspectRatio` | Sizing and spacing | `Existing style property` | `Property::AspectRatio` + `Value::AspectRatio` | Aspect ratio uses the style-owned auto-or-ratio type. | Operation 8 layout-facing properties |
| `CssProperty::ScrollbarWidth` | `CssValue::ScrollbarWidth` | Overflow and visibility | `Existing style property` | `Property::ScrollbarWidth` + `Value::ScrollbarWidth` | Scrollbar width keywords use the style-owned enum. | Operation 8 layout-facing properties |
| `CssProperty::Cursor` | `CssValue::Cursor` | Interaction | `Existing style property` | `Property::Cursor` + `Value::Cursor` | Cursor has typed style data; platform cursor realization remains outside style. | Operation 10 paint/color/effects |
| `CssProperty::PointerEvents` | `CssValue::PointerEvents` | Interaction | `Existing style property` | `Property::PointerEvents` + `Value::PointerEvents` | Pointer event participation has typed style data. | Operation 10 paint/color/effects |
| `CssProperty::UserSelect` | `CssValue::UserSelect` | Interaction | `New style property needed` | Planned `user-select` model | Style lacks typed selection interaction data. | Operation 10 paint/color/effects |
| `CssProperty::Outline` | `CssValue::Outline` | Border and outline | `New shorthand lowering needed` | Planned CSS outline width, style, and color models | Existing focus outline is not the CSS outline property and should stay distinct. | Operation 10 paint/color/effects |
| `CssProperty::OutlineColor` | `CssValue::OutlineColor` | Border and outline | `New style property needed` | Planned CSS outline color model | Style lacks CSS outline color distinct from focus outline. | Operation 10 paint/color/effects |
| `CssProperty::OutlineStyle` | `CssValue::OutlineStyle` | Border and outline | `New style property needed` | Planned CSS outline style model | Style lacks CSS outline style distinct from focus outline. | Operation 10 paint/color/effects |
| `CssProperty::OutlineWidth` | `CssValue::OutlineWidth` | Border and outline | `New style property needed` | Planned CSS outline width model | Style lacks CSS outline width distinct from focus outline. | Operation 10 paint/color/effects |
| `CssProperty::Transform` | `CssValue::Transform` | Transforms | `Existing style property` | `Property::Transform` + `Value::Transform` | Transform functions have a typed style transform list; CSS function parity needs review. | Operation 10 paint/color/effects |
| `CssProperty::TransformOrigin` | `CssValue::TransformOrigin` | Transforms | `Existing style property` | `Property::TransformOrigin` + `Value::Size` | Transform origin has typed two-axis style data; CSS position keywords need lowering review. | Operation 10 paint/color/effects |
| `CssProperty::Translate` | `CssValue::Translate` | Transforms | `New style property needed` | Planned individual translate property | Style lacks CSS individual transform property storage. | Operation 10 paint/color/effects |
| `CssProperty::Rotate` | `CssValue::Rotate` | Transforms | `New style property needed` | Planned individual rotate property | Style lacks CSS individual transform property storage. | Operation 10 paint/color/effects |
| `CssProperty::Scale` | `CssValue::Scale` | Transforms | `New style property needed` | Planned individual scale property | Style lacks CSS individual transform property storage. | Operation 10 paint/color/effects |
| `CssProperty::Filter` | `CssValue::Filter` | Paint and effects | `Symbolic style data needed` | Planned filter function list model | Filter functions should remain typed symbolic effect data until render capability resolution. | Operation 10 paint/color/effects |
| `CssProperty::BackdropFilter` | `CssValue::Filter` | Paint and effects | `Symbolic style data needed` | Planned backdrop filter function list model | Backdrop filters should remain typed symbolic effect data until render capability resolution. | Operation 10 paint/color/effects |
| `CssProperty::ClipPath` | `CssValue::ClipPath` | Paint and effects | `Symbolic style data needed` | Planned clip-path model | Basic shapes and references need symbolic paint data before render realization. | Operation 10 paint/color/effects |
| `CssProperty::Mask` | `CssValue::Mask` | Paint and effects | `Symbolic style data needed` | Planned mask layer shorthand model | Mask layers include resource-like symbolic data and need explicit lowering. | Operation 10 paint/color/effects |
| `CssProperty::MaskImage` | `CssValue::MaskImage` | Paint and effects | `Symbolic style data needed` | Planned mask image layer model | URLs/images are authored symbolic resources; loading and render resources stay outside style. | Operation 10 paint/color/effects |
| `CssProperty::MaskSize` | `CssValue::MaskSize` | Paint and effects | `New style property needed` | Planned mask size layer model | Style lacks typed mask-size layer data. | Operation 10 paint/color/effects |
| `CssProperty::MaskPosition` | `CssValue::MaskPosition` | Paint and effects | `New style property needed` | Planned mask position layer model | Style lacks typed mask-position layer data. | Operation 10 paint/color/effects |
| `CssProperty::MaskRepeat` | `CssValue::MaskRepeat` | Paint and effects | `New style property needed` | Planned mask repeat layer model | Style lacks typed mask-repeat layer data. | Operation 10 paint/color/effects |
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
| Text and font | Font family, font size, line height, text alignment, font shorthand and longhands, font feature settings, spacing, wrapping, text overflow, text decoration line/style/thickness, and text transform have typed style targets. | No Operation 9 text or font property gap remains in this ledger; `TextDecorationColor` remains in Color for Operation 10. | No property implementation |
| Generated content and lists | Pseudo-element style buckets and authored declarations can target style buckets. | `content`, list style, marker images, and counters need generated-content style data. | Operation 11 generated content/counters/lists |
| Color | `Property::Color` and `Value::Color` support concrete RGBA. | Symbolic colors, text-decoration color property path, currentColor, system colors, and modern color functions need symbolic style data. | Operation 10 paint/color/effects |
| Background | `Property::Background` stores concrete background color. | Background layers, images, position, size, repeat, origin, clip, attachment, and full shorthand lowering are missing. | Operation 10 paint/color/effects |
| Border and outline | Aggregate border color, border width, and radius/shadow primitives exist. | Side colors, side styles, individual widths/radii, border shorthands, CSS outline, and complete border style values are missing. | Operation 10 paint/color/effects |
| Paint and effects | Shadow and opacity have typed style targets. | Box decoration break, filters, backdrop filters, clip paths, masks, and render-capability symbolic payloads need typed style data. | Operation 10 paint/color/effects |
| Transforms | `Transform` and `TransformOrigin` have style targets. | Individual translate, rotate, and scale properties need typed style data. | Operation 10 paint/color/effects |
| Interaction | `Cursor` and `PointerEvents` have typed style targets. | `user-select` needs a typed interaction model; platform behavior remains outside style. | Operation 10 paint/color/effects |
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

The next implementation plan should cover Operation 10: paint/color/effects
property families.

Use this ledger instead of re-inventorying the full CSS property surface. The
paint/color/effects plan should start with the Color, Background, Border and
outline, Paint and effects, Transforms, and Interaction ledger rows that point
to Operation 10.

The paint/color/effects plan should implement style-owned models and lowering
front doors only where the ledger marks `New style property needed`,
`New shorthand lowering needed`, or `Symbolic style data needed`. It should not
add a style-to-layout adapter, generated content, timing/keyframe models, or
Operation 14 cache/invalidation generalization.

Operation 8 layout-facing rows and Operation 9 text-facing rows have been
rebased after implementation, so Operation 10 can proceed from the current
paint/color/effects gaps.
