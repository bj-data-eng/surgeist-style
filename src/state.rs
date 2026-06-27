#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum StateFlag {
    Hovered,
    Active,
    Focused,
    FocusWithin,
    PointerCaptured,
    Disabled,
    Selected,
    Pressed,
    Checked,
    Expanded,
}
