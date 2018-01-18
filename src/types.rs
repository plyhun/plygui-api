use super::{development};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStartSize {
	Exact(u16, u16),
	Fullscreen,
}

pub type UiMemberBase = development::UiMemberCommon;

