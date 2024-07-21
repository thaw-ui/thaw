use leptos::ev;

pub fn get_dropdown_action_from_key(
    e: ev::KeyboardEvent,
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
        DropdownAction::Close
    } else if KeyboardKey::ArrowDown == code {
        DropdownAction::Next
    } else if KeyboardKey::ArrowUp == code {
        DropdownAction::Previous
    } else {
        DropdownAction::None
    }
}

pub enum DropdownAction {
    None,
    Type,
    Open,
    CloseSelect,
    Close,
    Next,
    Previous,
}

enum KeyboardKey {
    ArrowDown,
    ArrowUp,
    Enter,
    Space,
}

impl PartialEq<String> for KeyboardKey {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::ArrowDown => other == "ArrowDown",
            Self::ArrowUp => other == "ArrowUp",
            Self::Enter => other == "Enter",
            Self::Space => other == "Space",
        }
    }
}
