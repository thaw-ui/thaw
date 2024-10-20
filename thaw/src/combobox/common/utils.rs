use leptos::ev;

pub fn get_dropdown_action_from_key(
    e: &ev::KeyboardEvent,
    open: bool,
    multiselect: bool,
) -> DropdownAction {
    let key = e.key();
    let code = e.code();
    let alt_key = e.alt_key();
    let ctrl_key = e.ctrl_key();
    let meta_key = e.meta_key();

    if key.len() == 1 && KeyboardKey::Space != code && !alt_key && !ctrl_key && !meta_key {
        DropdownAction::Type
    } else if !open {
        if KeyboardKey::ArrowDown == code
            || KeyboardKey::ArrowUp == code
            || KeyboardKey::Enter == code
            || KeyboardKey::Space == code
        {
            DropdownAction::Open
        } else {
            DropdownAction::None
        }
    } else if (KeyboardKey::ArrowUp == code && alt_key)
        || KeyboardKey::Enter == code
        || (!multiselect && KeyboardKey::Space == code)
    {
        DropdownAction::CloseSelect
    } else if multiselect && KeyboardKey::Space == code {
        DropdownAction::Select
    } else if KeyboardKey::Escape == code {
        DropdownAction::Close
    } else if KeyboardKey::ArrowDown == code {
        DropdownAction::Next
    } else if KeyboardKey::ArrowUp == code {
        DropdownAction::Previous
    } else if KeyboardKey::Home == code {
        DropdownAction::First
    } else if KeyboardKey::End == code {
        DropdownAction::Last
    } else if KeyboardKey::PageUp == code {
        DropdownAction::PageUp
    } else if KeyboardKey::PageDown == code {
        DropdownAction::PageDown
    } else if KeyboardKey::Tab == code {
        DropdownAction::Tab
    } else {
        DropdownAction::None
    }
}

#[derive(PartialEq)]
pub enum DropdownAction {
    None,
    Type,
    Open,
    CloseSelect,
    Select,
    Close,
    Next,
    Previous,
    First,
    Last,
    PageUp,
    PageDown,
    Tab,
}

pub enum KeyboardKey {
    ArrowLeft,
    ArrowDown,
    ArrowUp,
    Backspace,
    Enter,
    Space,
    Escape,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
}

impl PartialEq<String> for KeyboardKey {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::ArrowLeft => other == "ArrowLeft",
            Self::ArrowDown => other == "ArrowDown",
            Self::ArrowUp => other == "ArrowUp",
            Self::Backspace => other == "Backspace",
            Self::Enter => other == "Enter",
            Self::Space => other == "Space",
            Self::Escape => other == "Escape",
            Self::Home => other == "Home",
            Self::End => other == "End",
            Self::PageUp => other == "PageUp",
            Self::PageDown => other == "PageDown",
            Self::Tab => other == "Tab",
        }
    }
}
